use crate::renderer::scenes_io::{read_3dcl_scene, read_scene_file};
use catalyst::entity_files::ContentFile;
use catalyst::{ContentClient, Server};
use dcl_common::Parcel;
use tempdir::TempDir;

const MIN_PARCEL: i16 = -152;
const MAX_PARCEL: i16 = 152;
const MAX_PARCELS_PER_QUERY: usize =
    (MAX_PARCEL + 1 - MIN_PARCEL) as usize * (MAX_PARCEL + 1 - MIN_PARCEL) as usize;

pub fn where_command() -> dcl_common::Result<()> {
    println!("Finding 2dcl scenes...");
    let result = print_2dcl_scenes(MAX_PARCELS_PER_QUERY);
    println!("Finished");
    result
}

fn print_2dcl_scenes(max_parcels_per_query: usize) -> dcl_common::Result<()> {
    let mut parcels = Vec::default();
    for x in MIN_PARCEL..=MAX_PARCEL {
        for y in MIN_PARCEL..=MAX_PARCEL {
            parcels.push(Parcel(x, y));

            if parcels.len() >= max_parcels_per_query {
                if print_2dcl_scenes_for_parcels(&parcels).is_err() {
                    return print_2dcl_scenes(max_parcels_per_query / 2);
                }
                parcels.clear();
            }
        }
    }

    if print_2dcl_scenes_for_parcels(&parcels).is_err() {
        return print_2dcl_scenes(max_parcels_per_query / 2);
    }

    Ok(())
}

#[tokio::main]
async fn print_2dcl_scenes_for_parcels(parcels: &Vec<Parcel>) -> dcl_common::Result<()> {
    if parcels.is_empty() {
        return Ok(());
    }
    let server = Server::production();
    let scene_files = ContentClient::scene_files_for_parcels(&server, parcels).await?;
    let tmp_dir = TempDir::new("where_downloads").unwrap();

    for scene_file in scene_files {
        let id_str = match &scene_file.id {
            Some(id) => id.to_string(),
            None => String::default(),
        };
        let scene_path = tmp_dir.path().join(&id_str);
        let mut downloadable_json: Option<ContentFile> = None;
        let mut downloadable_2dcl: Option<ContentFile> = None;

        for downloadable in scene_file.clone().content {
            if downloadable
                .filename
                .to_str()
                .unwrap()
                .ends_with("scene.2dcl")
            {
                downloadable_2dcl = Some(downloadable);
                if downloadable_json.is_some() {
                    break;
                }
            } else if downloadable
                .filename
                .to_str()
                .unwrap()
                .ends_with("scene.json")
            {
                downloadable_json = Some(downloadable);
                if downloadable_2dcl.is_some() {
                    break;
                }
            }
        }

        if let (Some(downloadable_json), Some(downloadable_2dcl)) =
            (downloadable_json, downloadable_2dcl)
        {
            if !scene_path.exists() {
                std::fs::create_dir_all(&scene_path)?;
            }

            let file_3d = scene_path
                .clone()
                .join(&id_str)
                .join(downloadable_json.filename.to_str().unwrap());

            ContentClient::download(&server, downloadable_json.cid, &file_3d).await?;

            if let Ok(scene_3d) = read_3dcl_scene(file_3d) {
                let file_2d = scene_path
                    .clone()
                    .join(id_str)
                    .join(downloadable_2dcl.filename.to_str().unwrap());

                ContentClient::download(&server, downloadable_2dcl.cid, &file_2d).await?;

                if let Some(scene_2d) = read_scene_file(&file_2d) {
                    println!(
                        "{} -> {}",
                        parcels_to_string(&scene_3d.scene.parcels),
                        scene_2d.name
                    );
                }
            }
        }
    }
    Ok(())
}

fn parcels_to_string(parcels: &Vec<Parcel>) -> String {
    if parcels.is_empty() {
        return String::default();
    }

    if parcels.len() == 1 {
        return format!("({}, {})", parcels[0].0, parcels[0].1);
    }

    let mut output_string = "[".to_string();
    for parcel in parcels {
        output_string += &format!(" ({}, {}),", parcel.0, parcel.1);
    }

    output_string.pop();

    output_string += " ]";

    output_string
}
