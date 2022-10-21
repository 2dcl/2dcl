use clap::Parser;
use compiler::build;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    action: Action,
}

#[derive(clap::Subcommand)]
enum Action {

   build
   {  
    json_path: std::path::PathBuf,
    #[clap(default_value="./build")]
    build_path: std::path::PathBuf,
   },
}



fn main() {
  let args = Cli::parse();

  match args.action {
    Action::build {    json_path,
      build_path, } => {
        build(json_path, build_path);
      }
  }

}
