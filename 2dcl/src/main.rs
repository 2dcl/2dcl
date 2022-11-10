use dcl_common::Result;
//use tokio::process::Command;

mod renderer;
mod previewer;

//mod ws;

use clap::Parser;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    action: Option<Action>,
}

#[derive(clap::Subcommand)]
enum Action {
   Preview
   {
    #[clap(default_value="./")]
    source_path: std::path::PathBuf,
    #[clap(default_value="./build/")]
    destination_path: std::path::PathBuf
   },
   Build
   {
    #[clap(default_value="./")]
    source_path: std::path::PathBuf,
    #[clap(default_value="./build/")]
    destination_path: std::path::PathBuf
   }
}



fn main() -> Result<()> {
  let args = Cli::parse();

  match args.action {
    Some(Action::Preview {source_path, destination_path}) => {
        previewer::preview(source_path, destination_path);
      }
    ,
    Some(Action::Build {source_path, destination_path}) => {
      scene_compiler::compile(source_path, destination_path);
    }
    None =>
    {
      renderer::start();
    }
  }
      
 
//    ws::start().await?;

    // spawn kernel process
  //  let mut command = std::env::current_exe()?;
  //  command.pop();
  //  command.push("kernel");
  //  Command::new(command).spawn().expect("failed to spawn");

    //

    Ok(())
}
