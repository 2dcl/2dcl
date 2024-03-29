use super::collision::CollisionTile;
use super::scenes_io::{
    get_parcel_file_data, get_scene, read_scene_file, refresh_path, SceneData, SceneFilesMap,
};
use crate::bundles::{self, get_parcels_center_location, loading_animation};
use crate::renderer::constants::*;
use crate::renderer::scene_maker::*;
use crate::states::AppState;
use crate::{
    components::{self, *},
    resources,
};
use bevy::prelude::*;
use bevy::tasks::AsyncComputeTaskPool;
use catalyst::entity_files::ContentFile;
use catalyst::{ContentClient, Server};
use dcl_common::Parcel;
use futures_lite::future;
use image::io::Reader as ImageReader;
use image::{DynamicImage, ImageFormat};
use rmp_serde::*;
use serde::Deserialize;
use std::fs;
use std::path::Path;
use std::path::PathBuf;
use std::str::FromStr;
use std::time::SystemTime;

pub struct SceneLoaderPlugin;

#[derive(Default, Resource)]
pub struct DownloadQueue {
    parcels: Vec<Parcel>,
}

#[derive(Default, Resource)]
pub struct SpawningQueue {
    parcels: Vec<Parcel>,
}

#[derive(Default, Resource)]
pub struct DespawnedEntities {
    pub entities: Vec<Entity>,
}

impl Plugin for SceneLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(DownloadQueue::default())
            .insert_resource(DespawnedEntities::default())
            .insert_resource(SpawningQueue::default())
            .add_systems(
                Update,
                level_changer
                    .before(scene_manager)
                    .run_if(in_state(AppState::InGame)),
            )
            .add_systems(
                Update,
                scene_manager
                    .after(level_changer)
                    .run_if(in_state(AppState::InGame)),
            )
            .add_systems(
                Update,
                (
                    scene_version_downloader,
                    downloading_scenes_task_handler,
                    downloading_version_task_handler,
                    loading_sprites_task_handler,
                    loading_animation,
                )
                    .run_if(in_state(AppState::InGame)),
            )
            .add_systems(
                Update,
                spawning_queue_cleaner
                    .before(level_changer)
                    .before(scene_manager)
                    .before(scene_version_downloader)
                    .before(downloading_version_task_handler)
                    .before(loading_sprites_task_handler)
                    .before(downloading_scenes_task_handler)
                    .run_if(in_state(AppState::InGame)),
            )
            .add_systems(
                Update,
                default_scenes_despawner
                    .before(scene_manager)
                    .run_if(in_state(AppState::InGame)),
            );
    }
}

pub fn spawning_queue_cleaner(
    mut spawning_queue: ResMut<SpawningQueue>,
    scene_query: Query<&components::Scene>,
) {
    for i in (0..spawning_queue.parcels.len()).rev() {
        let parcel = &spawning_queue.parcels[i];
        for scene in scene_query.iter() {
            if scene.parcels.contains(parcel) {
                spawning_queue.parcels.remove(i);
                break;
            }
        }
    }
}
pub fn level_changer(
    mut commands: Commands,
    mut collision_map: ResMut<resources::CollisionMap>,
    mut despawned_entities: ResMut<DespawnedEntities>,
    asset_server: Res<AssetServer>,
    mut player_query: Query<(&mut components::Player, &mut GlobalTransform)>,
    scene_query: Query<(Entity, &mut components::Scene)>,
    level_query: Query<(&components::Level, &Parent)>,
    mut spawning_queue: ResMut<SpawningQueue>,
) {
    //Find the player
    let player_query = player_query.get_single_mut();

    if player_query.is_err() {
        return;
    }

    let mut player_query = player_query.unwrap();
    let current_level = player_query.0.current_level;

    //We check if we're on the correct level

    for (scene_entity, scene) in scene_query.iter() {
        if scene.parcels.contains(&player_query.0.current_parcel) {
            for (level, level_parent) in level_query.iter() {
                if **level_parent == scene_entity {
                    //If we're in a different level we change it
                    if current_level != level.id {
                        //Despawn every scene
                        for (other_scene_entity, _other_scene) in scene_query.iter() {
                            despawned_entities.entities.push(other_scene_entity);
                            commands.entity(other_scene_entity).despawn_recursive();
                        }

                        //Clear collision map
                        collision_map.tiles.clear();

                        //Spawn correct level
                        let mut de = Deserializer::from_read_ref(&scene.serialized_data);
                        let deserialized_scene: dcl2d_ecs_v1::Scene =
                            Deserialize::deserialize(&mut de).unwrap();
                        let scene_data = SceneData {
                            scene: deserialized_scene,
                            path: scene.path.clone(),
                            is_default: false,
                        };

                        spawn_scene(
                            &mut commands,
                            &asset_server,
                            &scene_data,
                            &mut collision_map,
                            current_level,
                            &mut spawning_queue,
                        );
                    }
                    break;
                }
            }
            break;
        }
    }

    if current_level == 0 {
        player_query.0.current_parcel = world_location_to_parcel(&player_query.1.translation());
    }
}

pub fn scene_manager(
    mut player_query: Query<(&mut components::Player, &mut GlobalTransform)>,
    scene_query: Query<&mut components::Scene, Without<components::Player>>,
    mut download_queue: ResMut<DownloadQueue>,
    spawning_queue: Res<SpawningQueue>,
    config: Res<resources::Config>,
) {
    //Find the player
    let player_query = player_query.get_single_mut();

    if player_query.is_err() {
        return;
    }

    let player_query = player_query.unwrap();
    let player_parcel = player_query.0.current_parcel.clone();

    let current_level = player_query.0.current_level;

    //Only continue if we're in the overworld.
    if current_level != 0 {
        return;
    }

    let mut parcels_to_spawn =
        get_all_parcels_around(&player_parcel, config.world.min_render_distance);

    //Check every scene already spawned
    for scene in scene_query.iter() {
        //We don't need to spawn parcels already spawned
        for i in (0..parcels_to_spawn.len()).rev() {
            if scene.parcels.contains(&parcels_to_spawn[i]) {
                parcels_to_spawn.remove(i);
            }
        }
    }

    //We don't need to spawn parcels being spawned
    for i in (0..parcels_to_spawn.len()).rev() {
        for spawning_parcel in &spawning_queue.parcels {
            if parcels_to_spawn[i] == *spawning_parcel {
                parcels_to_spawn.remove(i);
                break;
            }
        }
    }

    download_queue.parcels = parcels_to_spawn;
}

pub fn scene_version_downloader(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut roads_data: ResMut<RoadsData>,
    scene_files_map: Res<SceneFilesMap>,
    mut collision_map: ResMut<resources::CollisionMap>,
    mut download_queue: ResMut<DownloadQueue>,
    mut spawning_queue: ResMut<SpawningQueue>,
) {
    if download_queue.parcels.is_empty() {
        return;
    }

    //Download scenes
    let thread_pool = AsyncComputeTaskPool::get();
    let parcels_to_download = download_queue.parcels.clone();

    let scene_files_map_clone = scene_files_map.clone();
    let task_get_scene_files_to_download = thread_pool.spawn(async move {
        match get_newest_scene_files_for_parcels(parcels_to_download, &scene_files_map_clone) {
            Ok(v) => Some(v),
            Err(e) => {
                println!("{:?}", e);
                None
            }
        }
    });

    //Spawm default scene or previous version
    for parcel_to_download in &download_queue.parcels {
        match get_scene(&mut roads_data, &scene_files_map, parcel_to_download) {
            Some(scene_data) => {
                spawn_scene(
                    &mut commands,
                    &asset_server,
                    &scene_data,
                    &mut collision_map,
                    0,
                    &mut spawning_queue,
                );
            }
            None => {
                spawn_default_scene(
                    &mut commands,
                    &asset_server,
                    parcel_to_download,
                    &mut collision_map,
                    &mut spawning_queue,
                );
            }
        }
    }

    commands.spawn(GettingNewestScenes {
        task: task_get_scene_files_to_download,
    });

    download_queue.parcels.clear();
}

fn get_all_parcels_around(parcel: &Parcel, distance: usize) -> Vec<Parcel> {
    let mut parcels: Vec<Parcel> = Vec::new();
    for i in 0..distance as i16 {
        for e in 0..i {
            parcels.push(Parcel(parcel.0 + i, parcel.1 + e));
            parcels.push(Parcel(parcel.0 + e, parcel.1 + i));
            parcels.push(Parcel(parcel.0 - i, parcel.1 + e));
            parcels.push(Parcel(parcel.0 + e, parcel.1 - i));

            if e > 0 {
                parcels.push(Parcel(parcel.0 + i, parcel.1 - e));
                parcels.push(Parcel(parcel.0 - e, parcel.1 + i));
                parcels.push(Parcel(parcel.0 - i, parcel.1 - e));
                parcels.push(Parcel(parcel.0 - e, parcel.1 - i));
            }
        }

        if i > 0 {
            parcels.push(Parcel(parcel.0 - i, parcel.1 + i));
            parcels.push(Parcel(parcel.0 + i, parcel.1 - i));
            parcels.push(Parcel(parcel.0 - i, parcel.1 - i));
        }

        parcels.push(Parcel(parcel.0 + i, parcel.1 + i));
    }
    parcels
}
pub fn world_location_to_parcel(location: &Vec3) -> Parcel {
    Parcel(
        (location.x / PARCEL_SIZE_X).round() as i16,
        (location.y / PARCEL_SIZE_Y).round() as i16,
    )
}

pub fn parcel_to_world_location(parcel: Parcel) -> Vec3 {
    Vec3 {
        x: PARCEL_SIZE_X * parcel.0 as f32,
        y: PARCEL_SIZE_Y * parcel.1 as f32,
        z: PARCEL_SIZE_Y * -parcel.1 as f32,
    }
}

#[tokio::main]
pub async fn download_scene_files(
    scene_files: Vec<catalyst::entity_files::SceneFile>,
) -> dcl_common::Result<Vec<PathBuf>> {
    let server = Server::production();

    let mut scene_paths: Vec<PathBuf> = Vec::new();

    for scene_file in scene_files {
        let id_str = match &scene_file.id {
            Some(id) => id.to_string(),
            None => String::default(),
        };
        let path_str = "./assets/scenes/".to_string() + &id_str;
        let scene_path = Path::new(&path_str);

        for downloadable in scene_file.content {
            let filename = format!(
                "./assets/scenes/{}/{}",
                id_str,
                downloadable.filename.to_str().unwrap()
            );

            ContentClient::download(&server, downloadable.cid, &filename).await?;
        }
        scene_paths.push(scene_path.to_path_buf());
    }

    Ok(scene_paths)
}

#[tokio::main]
pub async fn get_newest_scene_files_for_parcels(
    parcels: Vec<Parcel>,
    scene_files_map: &SceneFilesMap,
) -> dcl_common::Result<(Vec<catalyst::entity_files::SceneFile>, Vec<Parcel>)> {
    let mut scene_files_to_download: Vec<catalyst::entity_files::SceneFile> = Vec::new();
    let mut parcels_to_download: Vec<Parcel> = Vec::new();

    let server = Server::production();
    let scene_files = ContentClient::scene_files_for_parcels(&server, &parcels).await?;
    for scene_file in scene_files {
        let id_str = match &scene_file.id {
            Some(id) => id.to_string(),
            None => String::default(),
        };
        let path_str = "./assets/scenes/".to_string() + &id_str;
        let scene_path = Path::new(&path_str);
        let mut downloadable_2dcl: Option<ContentFile> = None;

        for downloadable in scene_file.clone().content {
            if downloadable
                .filename
                .to_str()
                .unwrap()
                .ends_with("scene.2dcl")
            {
                downloadable_2dcl = Some(downloadable);
                break;
            }
        }

        if !scene_path.exists() {
            fs::create_dir_all(format!("./assets/scenes/{}", id_str))?;
        }

        if let Some(downloadable_2dcl) = downloadable_2dcl {
            let filename = format!(
                "./assets/scenes/{}/{}-temp",
                id_str,
                downloadable_2dcl.filename.to_str().unwrap()
            );
            ContentClient::download(&server, downloadable_2dcl.cid, &filename).await?;
            let mut parcels = Vec::default();
            for parcel in &scene_file.pointers {
                if let Ok(parcel) = Parcel::from_str(parcel) {
                    parcels.push(parcel);
                }
            }

            if let Some(scene_2cl) = read_scene_file(&filename) {
                let mut should_download = false;
                for parcel in &parcels {
                    match get_parcel_file_data(parcel, scene_files_map) {
                        Some(parcel_data) => {
                            if let Some(previous_2dcl_scene) = read_scene_file(parcel_data.path) {
                                if previous_2dcl_scene.timestamp != scene_2cl.timestamp {
                                    scene_files_to_download.push(scene_file);
                                    should_download = true;
                                    break;
                                }
                            }
                        }
                        None => {
                            scene_files_to_download.push(scene_file);
                            should_download = true;
                            break;
                        }
                    }
                }

                if should_download {
                    parcels_to_download.append(&mut parcels);
                }
            }
            match std::fs::remove_file(filename) {
                Ok(_) => {}
                Err(e) => println!("{}", e),
            };
        }

        if scene_path.read_dir()?.next().is_none() {
            std::fs::remove_dir(scene_path)?;
        }
    }
    Ok((scene_files_to_download, parcels_to_download))
}

#[tokio::main]
pub async fn download_level_spawn_point(parcel: &Parcel, level_id: usize) -> Vec3 {
    let server = Server::production();
    let scene_files =
        match ContentClient::scene_files_for_parcels(&server, &vec![parcel.clone()]).await {
            Ok(v) => v,
            Err(_) => {
                let scene_data = SceneData { ..default() };
                return get_parcels_center_location(&scene_data.scene.parcels);
            }
        };

    for scene_file in scene_files {
        let id_str = match &scene_file.id {
            Some(id) => id.to_string(),
            None => String::default(),
        };
        let path_str = "./assets/scenes/".to_string() + &id_str;
        let scene_path = Path::new(&path_str);
        let mut downloadable_2dcl: Option<ContentFile> = None;

        for downloadable in scene_file.clone().content {
            if downloadable
                .filename
                .to_str()
                .unwrap()
                .ends_with("scene.2dcl")
            {
                downloadable_2dcl = Some(downloadable);
                break;
            }
        }

        if !scene_path.exists()
            && fs::create_dir_all(format!("./assets/scenes/{}", id_str)).is_err()
        {
            continue;
        }

        if let Some(downloadable_2dcl) = downloadable_2dcl {
            let filename = format!(
                "./assets/scenes/{}/{}-temp",
                id_str,
                downloadable_2dcl.filename.to_str().unwrap()
            );

            if ContentClient::download(&server, downloadable_2dcl.cid, &filename)
                .await
                .is_err()
            {
                continue;
            }

            if let Some(scene_2d) = read_scene_file(filename) {
                let scene_data = SceneData {
                    scene: scene_2d,
                    ..default()
                };

                let scene_center = get_parcels_center_location(&scene_data.scene.parcels);
                return match level_id < scene_data.scene.levels.len() {
                    true => {
                        let spawn_point = scene_data.scene.levels[level_id].spawn_point.clone();
                        Vec3 {
                            x: spawn_point.x as f32 + scene_center.x,
                            y: spawn_point.y as f32 + scene_center.y,
                            z: (spawn_point.y as f32 + scene_center.y) * -1.0,
                        }
                    }
                    false => scene_center,
                };
            }
        }
    }

    get_parcels_center_location(&vec![parcel.clone()])
}

pub fn loading_sprites_task_handler(
    mut commands: Commands,
    mut tasks_loading_sprites: Query<(Entity, &mut LoadingSpriteRenderer)>,
) {
    for (entity, mut sprite) in &mut tasks_loading_sprites {
        if let Some((texture, image_size)) = future::block_on(future::poll_once(&mut sprite.task)) {
            commands.entity(entity).remove::<LoadingSpriteRenderer>();
            commands
                .entity(entity)
                .insert(bundles::SpriteRenderer::from_texture(
                    &sprite.sprite_renderer_component,
                    &sprite.transform,
                    texture,
                    image_size,
                    &sprite.parcels,
                    sprite.level_id,
                ));
        }
    }
}
fn downloading_scenes_task_handler(
    mut commands: Commands,
    mut collision_map: ResMut<resources::CollisionMap>,
    mut roads_data: ResMut<RoadsData>,
    mut scene_files_map: ResMut<SceneFilesMap>,
    mut despawned_entities: ResMut<DespawnedEntities>,
    asset_server: Res<AssetServer>,
    mut tasks_downloading_scenes: Query<(Entity, &mut DownloadingScene)>,
    scenes_query: Query<(Entity, &components::Scene)>,
    mut spawning_queue: ResMut<SpawningQueue>,
) {
    for (entity, mut downloading_scene) in &mut tasks_downloading_scenes {
        if let Some(new_paths) = future::block_on(future::poll_once(&mut downloading_scene.task)) {
            commands.entity(entity).despawn_recursive();
            if let Some(new_paths) = new_paths {
                for new_path in new_paths {
                    match refresh_path(new_path.clone(), &mut scene_files_map) {
                        Ok(_) => {}
                        Err(e) => println!("{}", e),
                    }
                }
            }

            for (entity, scene) in &scenes_query {
                for parcel_1 in &downloading_scene.parcels {
                    for parcel_2 in &scene.parcels {
                        if parcel_1 == parcel_2 {
                            if let Some(scene_data) =
                                get_scene(&mut roads_data, &scene_files_map, parcel_1)
                            {
                                if scene.timestamp.0 != scene_data.scene.timestamp {
                                    despawned_entities.entities.push(entity);
                                    commands.entity(entity).despawn_recursive();
                                    spawn_scene(
                                        &mut commands,
                                        &asset_server,
                                        &scene_data,
                                        &mut collision_map,
                                        0,
                                        &mut spawning_queue,
                                    );
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

fn downloading_version_task_handler(
    mut commands: Commands,
    mut tasks_check_scene_version: Query<(Entity, &mut GettingNewestScenes)>,
    asset_server: Res<AssetServer>,
) {
    for (entity, mut newest_scenes) in &mut tasks_check_scene_version {
        if let Some(task_result) = future::block_on(future::poll_once(&mut newest_scenes.task)) {
            commands.entity(entity).despawn_recursive();

            if let Some((scene_files, parcels)) = task_result {
                let thread_pool = AsyncComputeTaskPool::get();
                let task_download_scene_files = thread_pool.spawn(async move {
                    match download_scene_files(scene_files) {
                        Ok(v) => Some(v),
                        Err(e) => {
                            println!("{:?}", e);
                            None
                        }
                    }
                });

                if !parcels.is_empty() {
                    commands.spawn(bundles::DownloadingScene::from_task_and_parcels(
                        task_download_scene_files,
                        parcels,
                        &asset_server,
                    ));
                }
            }
        }
    }
}
fn default_scenes_despawner(
    mut commands: Commands,
    mut despawned_entities: ResMut<DespawnedEntities>,
    player_query: Query<(&components::Player, &GlobalTransform)>,
    scenes_query: Query<(Entity, &components::Scene)>,
    config: Res<resources::Config>,
) {
    let player_query = player_query.get_single();

    if player_query.is_err() {
        return;
    }
    let player_query = player_query.unwrap();
    let player_parcel = player_query.0.current_parcel.clone();
    let parcels_to_keep = get_all_parcels_around(&player_parcel, config.world.max_render_distance);

    'outer: for (entity, scene) in &scenes_query {
        for parcel in &parcels_to_keep {
            if scene.parcels.contains(parcel) {
                continue 'outer;
            }
        }
        despawned_entities.entities.push(entity);
        commands.entity(entity).despawn_recursive();
    }

    for (entity_1, scene_1) in &scenes_query {
        if despawned_entities.entities.contains(&entity_1) {
            continue;
        }
        for (entity_2, scene_2) in &scenes_query {
            if despawned_entities.entities.contains(&entity_2) {
                continue;
            }
            if entity_1 != entity_2 && (scene_1.is_default || scene_2.is_default) {
                'outer: for parcel_1 in &scene_1.parcels {
                    for parcel_2 in &scene_2.parcels {
                        if *parcel_1 == *parcel_2 {
                            if scene_1.is_default {
                                despawned_entities.entities.push(entity_1);
                                commands.entity(entity_1).despawn_recursive();
                                break 'outer;
                            } else if scene_2.is_default {
                                despawned_entities.entities.push(entity_2);
                                commands.entity(entity_2).despawn_recursive();
                                break 'outer;
                            }
                        }
                    }
                }
            }
        }
    }
}

fn spawn_default_scene(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    parcel: &Parcel,
    collision_map: &mut resources::CollisionMap,
    spawning_queue: &mut ResMut<SpawningQueue>,
) {
    let scene_data = match make_default_scene(parcel) {
        Ok(v) => v,
        Err(e) => {
            println!("{}", e);
            return;
        }
    };

    spawn_scene(
        commands,
        asset_server,
        &scene_data,
        collision_map,
        0,
        spawning_queue,
    );
}

pub fn spawn_level(
    commands: &mut Commands,
    asset_server: &AssetServer,
    scene_data: &SceneData,
    level_id: usize,
    collision_map: &mut resources::CollisionMap,
    timestamp: SystemTime,
) -> Option<Entity> {
    let scene = &scene_data.scene;
    if scene.levels.len() <= level_id {
        return None;
    }

    let level = &scene.levels[level_id];

    let level_entity = commands
        .spawn(bundles::Level {
            name: Name::new(level.name.clone()),
            level: Level {
                name: level.name.clone(),
                timestamp,
                id: level_id,
                spawn_point: Vec2 {
                    x: level.spawn_point.x as f32,
                    y: level.spawn_point.y as f32,
                },
            },
            ..default()
        })
        .id();

    for entity in level.entities.iter() {
        let spawned_entity = spawn_entity(
            commands,
            asset_server,
            collision_map,
            entity,
            scene_data,
            level_id,
        );
        commands.entity(level_entity).add_child(spawned_entity);
    }

    Some(level_entity)
}

pub fn get_parcel_spawn_point(
    parcel: &Parcel,
    level_id: usize,
    roads_data: &mut RoadsData,
    scene_files_map: &SceneFilesMap,
) -> Vec3 {
    match get_scene(roads_data, scene_files_map, parcel) {
        Some(scene_data) => {
            let scene_center = get_parcels_center_location(&scene_data.scene.parcels);
            match level_id < scene_data.scene.levels.len() {
                true => {
                    let spawn_point = scene_data.scene.levels[level_id].spawn_point.clone();
                    Vec3 {
                        x: spawn_point.x as f32 + scene_center.x,
                        y: spawn_point.y as f32 + scene_center.y,
                        z: (spawn_point.y as f32 + scene_center.y) * -1.0,
                    }
                }
                false => scene_center,
            }
        }
        None => download_level_spawn_point(parcel, level_id),
    }
}

pub fn spawn_scene(
    commands: &mut Commands,
    asset_server: &AssetServer,
    scene_data: &SceneData,
    collision_map: &mut resources::CollisionMap,
    level_id: usize,
    spawning_queue: &mut ResMut<SpawningQueue>,
) -> Option<Entity> {
    let scene = &scene_data.scene;
    let scene_entity = commands
        .spawn(bundles::Scene::from_2dcl_scene_data(scene_data))
        .id();

    if !scene.levels.is_empty() {
        spawning_queue
            .parcels
            .append(&mut scene_data.scene.parcels.clone());
        match spawn_level(
            commands,
            asset_server,
            scene_data,
            level_id,
            collision_map,
            SystemTime::now(),
        ) {
            Some(level_entity) => {
                commands.entity(scene_entity).add_child(level_entity);
            }
            None => {
                commands.entity(scene_entity).despawn_recursive();
                return None;
            }
        }
    }

    Some(scene_entity)
}

fn get_fixed_translation_by_anchor(
    size: &Vec2,
    translation: &Vec2,
    anchor: &dcl2d_ecs_v1::Anchor,
) -> Vec2 {
    match anchor {
        dcl2d_ecs_v1::Anchor::BottomCenter => Vec2 {
            x: translation.x - size.x / 2.0,
            y: translation.y + size.y,
        },
        dcl2d_ecs_v1::Anchor::BottomLeft => Vec2 {
            x: translation.x,
            y: translation.y + size.y,
        },
        dcl2d_ecs_v1::Anchor::BottomRight => Vec2 {
            x: translation.x - size.x,
            y: translation.y + size.y,
        },
        dcl2d_ecs_v1::Anchor::Center => Vec2 {
            x: translation.x - size.x / 2.0,
            y: translation.y + size.y / 2.0,
        },
        dcl2d_ecs_v1::Anchor::CenterLeft => Vec2 {
            x: translation.x,
            y: translation.y + size.y / 2.0,
        },
        dcl2d_ecs_v1::Anchor::CenterRight => Vec2 {
            x: translation.x - size.x,
            y: translation.y + size.y / 2.0,
        },
        dcl2d_ecs_v1::Anchor::Custom(vec) => Vec2 {
            x: translation.x - vec.x as f32,
            y: translation.y + size.y - vec.y as f32,
        },
        dcl2d_ecs_v1::Anchor::TopCenter => Vec2 {
            x: translation.x - size.x / 2.0,
            y: translation.y,
        },
        dcl2d_ecs_v1::Anchor::TopLeft => *translation,
        dcl2d_ecs_v1::Anchor::TopRight => Vec2 {
            x: translation.x - size.x,
            y: translation.y,
        },
    }
}

fn spawn_entity(
    commands: &mut Commands,
    asset_server: &AssetServer,
    collision_map: &mut resources::CollisionMap,
    entity: &dcl2d_ecs_v1::Entity,
    scene_data: &SceneData,
    level_id: usize,
) -> Entity {
    let scene = &scene_data.scene;
    let mut transform = Transform::default();
    let spawned_entity = commands
        .spawn(Name::new(entity.name.clone()))
        .insert(TransformBundle::default())
        .id();

    //Insert transform
    for component in entity.components.iter() {
        if let Some(transform_component) = component
            .as_any()
            .downcast_ref::<dcl2d_ecs_v1::components::Transform>()
        {
            let transform_bundle =
                bundles::Transform::new(transform_component, scene_data, level_id);
            transform = transform_bundle.transform.local;
            commands.entity(spawned_entity).insert(transform_bundle);
        };
    }

    //Spawning Entity
    commands
        .entity(spawned_entity)
        .insert(VisibilityBundle::default());

    // Inserting components
    for component in entity.components.iter() {
        if let Some(sprite_renderer) = component
            .as_any()
            .downcast_ref::<dcl2d_ecs_v1::components::SpriteRenderer>()
        {
            let mut image_path = scene_data.path.clone();

            image_path.push("assets");
            image_path.push(&sprite_renderer.sprite);

            commands
                .entity(spawned_entity)
                .insert(bundles::SpriteRenderer::load_async(
                    sprite_renderer,
                    &transform,
                    image_path,
                    asset_server,
                    scene_data.scene.parcels.clone(),
                    level_id,
                ));
        }

        if let Some(collider) = component
            .as_any()
            .downcast_ref::<dcl2d_ecs_v1::components::BoxCollider>()
        {
            let box_collider = BoxCollider {
                center: Vec2::new(collider.center.x as f32, collider.center.y as f32),
                size: Vec2::new(collider.size.width as f32, collider.size.height as f32),
                collision_type: collider.collision_type.clone(),
                parcels: scene_data.scene.parcels.clone(),
            };
            commands.entity(spawned_entity).insert(box_collider);
        }

        if let Some(collider) = component
            .as_any()
            .downcast_ref::<dcl2d_ecs_v1::components::MaskCollider>()
        {
            let mut sprite_path = std::fs::canonicalize(PathBuf::from_str(".").unwrap()).unwrap();
            sprite_path.push("assets");
            sprite_path.push(&scene_data.path);
            sprite_path.push("assets");
            sprite_path.push(&collider.sprite);

            if let Ok(mut reader) = ImageReader::open(sprite_path) {
                reader.set_format(ImageFormat::Png);
                if let Ok(DynamicImage::ImageRgba8(image)) = reader.decode() {
                    let mut pixels = image.pixels();
                    let rows = image.rows().len();
                    let columns = pixels.len() / rows;
                    let world_transform = transform.translation
                        + get_parcels_center_location(&scene_data.scene.parcels);

                    let fixed_translation = get_fixed_translation_by_anchor(
                        &Vec2 {
                            x: columns as f32,
                            y: rows as f32,
                        },
                        &world_transform.truncate(),
                        &collider.anchor,
                    );

                    let mut index = 0;
                    let channel = collider.channel.clone() as usize;

                    while pixels.len() > 0 {
                        if pixels.next().unwrap()[channel] > 0 {
                            let tile_location = fixed_translation
                                + (Vec2::new(
                                    (index % columns) as f32,
                                    (index / columns) as f32 * -1.0,
                                ) * super::collision::TILE_SIZE);
                            let collision_tile = CollisionTile {
                                location: tile_location,
                                colliision_type: collider.collision_type.clone(),
                                entity: Some(spawned_entity),
                                parcels: scene_data.scene.parcels.clone(),
                            };
                            collision_map.tiles.push(collision_tile);
                        }
                        index += 1;
                    }
                }
            }
        }

        if let Some(level_change) = component
            .as_any()
            .downcast_ref::<dcl2d_ecs_v1::components::triggers::LevelChange>(
        ) {
            let mut new_level_id = 0;

            for i in 0..scene.levels.len() {
                if scene.levels[i].name == level_change.level {
                    new_level_id = i;
                    break;
                }
            }

            let scene_center_location = get_parcels_center_location(&scene_data.scene.parcels);
            let level_change_component = LevelChange {
                level: new_level_id,
                spawn_point: Vec2::new(
                    level_change.spawn_point.x as f32 + scene_center_location.x,
                    level_change.spawn_point.y as f32 + scene_center_location.y,
                ),
                parcels: scene_data.scene.parcels.clone(),
            };
            commands
                .entity(spawned_entity)
                .insert(level_change_component);
        }
    }

    for child_entity in entity.children.iter() {
        let spawned_child_entity = spawn_entity(
            commands,
            asset_server,
            collision_map,
            child_entity,
            scene_data,
            level_id,
        );
        commands
            .entity(spawned_entity)
            .add_child(spawned_child_entity);
    }
    spawned_entity
}
