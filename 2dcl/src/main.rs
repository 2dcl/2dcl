use dcl_common::Result;
//use tokio::process::Command;

mod renderer;
mod compiler;
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

   build
   {  
    json_path: std::path::PathBuf,
    #[clap(default_value="./build")]
    build_path: std::path::PathBuf,
   },
   preview
   {
    #[clap(default_value="./scene.2dcl")]
    path: std::path::PathBuf
   }
}



fn main() -> Result<()> {
  let args = Cli::parse();

  match args.action {
    Some(Action::build {    json_path,
      build_path, }) => {
        compiler::run(json_path, build_path);
      }
    ,
    Some(Action::preview { path}) => {
        renderer::preview_scene(path);
      }
    ,
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
