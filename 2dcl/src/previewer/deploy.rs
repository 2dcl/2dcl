use bevy::{
    prelude::*,
    tasks::{AsyncComputeTaskPool, Task},
};
use catalyst::EntityId;
use dcl_crypto::{Account, AuthChain, AuthLink, Signer};
use futures_lite::future;
use scene_deployer::FileData;

use super::{manual_refresh::RefreshData, ui::Messages};
use crate::deploy::{prepare_deploy_data, sign_ephemeral};

#[derive(Component)]
pub struct Deploying(Task<DeployState>);

enum DeployState {
    Signed(Account, Vec<AuthLink>),
    FilesPrepared(Account, Vec<AuthLink>, Vec<FileData>, EntityId),
    Success,
    Error(String),
}

pub fn deploy(
    mut commands: Commands,
    keyboard: Res<Input<KeyCode>>,
    mut messages: Query<&mut Messages>,
) {
    if keyboard.just_pressed(KeyCode::P) && keyboard.pressed(KeyCode::ControlLeft) {
        for mut message in messages.iter_mut() {
            message.0 = "Waiting for signature...".to_string();
        }
        let thread_pool = AsyncComputeTaskPool::get();
        let task = thread_pool.spawn(async move {
            let ephemeral_identity = dcl_crypto::Account::random();
            let chain = match sign_ephemeral(&ephemeral_identity, 300) {
                Ok(chain) => chain,
                Err(err) => return DeployState::Error(format!("{}", err)),
            };
            DeployState::Signed(ephemeral_identity, chain)
        });
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
                    let task = thread_pool.spawn(async move {
                        let server = catalyst::Server::production();

                        let (deploy_data, entity_id) =
                            match prepare_deploy_data(deploy_folder, &server) {
                                Ok(v) => v,
                                Err(err) => return DeployState::Error(format!("{}", err)),
                            };
                        DeployState::FilesPrepared(ephemeral, chain, deploy_data, entity_id)
                    });

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
                    let task = thread_pool.spawn(async move {
                        let server = catalyst::Server::production();

                        let response =
                            match scene_deployer::deploy(entity_id, deploy_data, chain, server) {
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
                    });

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
