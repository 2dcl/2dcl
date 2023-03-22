use dcl_common::Result;

mod previewer;
mod renderer;

pub mod bundles;
pub mod components;
pub mod resources;

use clap::Parser;

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
}

fn main() -> Result<()> {
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
        None => {
            renderer::start();
        }
    }
    Ok(())
}
