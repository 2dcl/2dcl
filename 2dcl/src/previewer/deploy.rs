use std::path::Path;

use super::{manual_refresh::RefreshData, ui::Messages};
use bevy::{
    prelude::*,
    tasks::{AsyncComputeTaskPool, Task},
};
use catalyst::EntityId;
use dcl_crypto::{Account, AuthChain, AuthLink, Signer};
use futures_lite::future;
use scene_deployer::FileData;

#[derive(Component)]
pub struct Deploying(Task<DeployState>);

enum DeployState {
    Signed(Account, Vec<AuthLink>),
    FilesPrepared(Account, Vec<AuthLink>, Vec<FileData>, EntityId),
    Success,
    Error(String),
}

pub fn deploy_input(
    mut commands: Commands,
    keyboard: Res<Input<KeyCode>>,
    mut messages: Query<&mut Messages>,
) {
    if keyboard.just_pressed(KeyCode::P) && keyboard.pressed(KeyCode::ControlLeft) {
        for mut message in messages.iter_mut() {
            message.0 = "Waiting for signature...".to_string();
        }
        let thread_pool = AsyncComputeTaskPool::get();
        let task = thread_pool.spawn(async move { sign_ephemeral(300) });
        commands.spawn(Deploying(task));
    }
}

pub fn handle_tasks(
    mut commands: Commands,
    mut deploying_tasks: Query<(Entity, &mut Deploying)>,
    mut messages: Query<&mut Messages>,
    refresh_data: Res<RefreshData>,
) {
    for (entity, mut task) in &mut deploying_tasks {
        if let Some(state) = future::block_on(future::poll_once(&mut task.0)) {
            match state {
                DeployState::Signed(ephemeral, chain) => {
                    println!("Preparing files...");
                    for mut message in messages.iter_mut() {
                        message.0 = "Preparing files...".to_string();
                    }

                    let thread_pool = AsyncComputeTaskPool::get();
                    let deploy_folder = refresh_data.destination_path.clone();
                    let task = thread_pool
                        .spawn(async move { prepare_deploy_data(ephemeral, chain, deploy_folder) });

                    commands.entity(entity).insert(Deploying(task));
                }
                DeployState::FilesPrepared(ephemeral, mut chain, deploy_data, entity_id) => {
                    println!("Uploading files...");
                    for mut message in messages.iter_mut() {
                        message.0 = "Uploading files...".to_string();
                    }

                    let payload = &entity_id.0;
                    let signature = ephemeral.sign(payload);
                    chain.push(AuthLink::EcdsaPersonalSignedEntity {
                        payload: payload.clone(),
                        signature,
                    });

                    let chain = AuthChain::from(chain);

                    let thread_pool = AsyncComputeTaskPool::get();
                    let task =
                        thread_pool.spawn(async move { deploy(entity_id, deploy_data, chain) });

                    commands.entity(entity).insert(Deploying(task));
                }
                DeployState::Success => {
                    println!("Scene deployed succesfully.");
                    for mut message in messages.iter_mut() {
                        message.0 = "Scene deployed succesfully.".to_string();
                    }
                    commands.entity(entity).despawn();
                }
                DeployState::Error(err) => {
                    println!("{}", err);
                    for mut message in messages.iter_mut() {
                        message.0 = err.clone();
                    }
                    commands.entity(entity).despawn();
                }
            }
        }
    }
}

#[tokio::main]
async fn prepare_deploy_data<T>(
    ephemeral_identity: dcl_crypto::Account,
    auth_chain: Vec<AuthLink>,
    deploy_folder: T,
) -> DeployState
where
    T: AsRef<Path>,
{
    let server = catalyst::Server::production();
    let (deploy_data, entity_id) =
        match crate::deploy::prepare_deploy_data(deploy_folder, &server).await {
            Ok(v) => v,
            Err(err) => return DeployState::Error(format!("{}", err)),
        };
    DeployState::FilesPrepared(ephemeral_identity, auth_chain, deploy_data, entity_id)
}

#[tokio::main]
async fn sign_ephemeral(duration_in_secs: u64) -> DeployState {
    let ephemeral_identity = dcl_crypto::Account::random();
    let chain = match crate::deploy::sign_ephemeral(&ephemeral_identity, duration_in_secs).await {
        Ok(chain) => chain,
        Err(err) => return DeployState::Error(format!("{}", err)),
    };
    DeployState::Signed(ephemeral_identity, chain)
}

#[tokio::main]
async fn deploy(
    entity_id: EntityId,
    deploy_data: Vec<FileData>,
    auth_chain: AuthChain,
) -> DeployState {
    let server = catalyst::Server::production();
    let response = match scene_deployer::deploy(entity_id, deploy_data, auth_chain, server).await {
        Ok(v) => v,
        Err(err) => return DeployState::Error(format!("{}", err)),
    };

    if response.status() == 200 {
        DeployState::Success
    } else {
        let error = match response.text().await {
            Ok(error) => error,
            Err(error) => format!("{}", error),
        };

        println!("{}", error);
        DeployState::Error(error)
    }
}
