#![allow(clippy::too_many_arguments)]
#![allow(clippy::type_complexity)]
use dcl_common::Result;

mod avatar_spritesheet_maker;
mod content_discovery;
mod metamask_login;
mod previewer;
mod renderer;

pub mod bundles;
pub mod components;
pub mod deploy;
pub mod resources;
pub mod states;

use clap::Parser;
use tempdir::TempDir;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    action: Option<Action>,
}

#[derive(clap::Subcommand)]
enum Action {
    Preview {
        #[clap(default_value = "./")]
        source_path: std::path::PathBuf,
        #[clap(default_value = "./build/")]
        destination_path: std::path::PathBuf,
    },
    Build {
        #[clap(default_value = "./")]
        source_path: std::path::PathBuf,
        #[clap(default_value = "./build/")]
        destination_path: std::path::PathBuf,
    },
    Deploy {
        #[clap(default_value = "./")]
        source_path: std::path::PathBuf,
    },
    Where,
    ImportAvatar {
        eth_address: String,
    },
    Clean,
}

#[tokio::main]
pub async fn main() -> Result<()> {
    let args = Cli::parse();

    match args.action {
        Some(Action::Preview {
            source_path,
            destination_path,
        }) => {
            previewer::preview(source_path, destination_path);
        }
        Some(Action::Build {
            source_path,
            destination_path,
        }) => {
            scene_compiler::compile(source_path, destination_path).unwrap();
        }
        Some(Action::Deploy { source_path }) => {
            let tmp_dir = TempDir::new("temp_build").unwrap();
            scene_compiler::compile(source_path, &tmp_dir).unwrap();
            deploy::deploy(tmp_dir).await?;
        }
        Some(Action::Clean) => {
            let current_path = std::env::current_exe().unwrap();
            let current_path = current_path.parent().unwrap();
            std::env::set_current_dir(current_path).unwrap();

            if let Err(e) = renderer::scenes_io::clear_all_downloaded_scenes() {
                println!("{}", e);
            }
        }
        Some(Action::Where) => {
            let scenes = content_discovery::find_2d_scenes_str().await?;
            println!("{}", scenes);
        }
        Some(Action::ImportAvatar { eth_address }) => {
            avatar_spritesheet_maker::start(&eth_address).await?;
        }
        None => {
            renderer::start();
        }
    }
    Ok(())
}
