use clap::Parser;
use scene_compiler::build;

#[derive(Parser,Debug)]
struct Args {
    json_path: std::path::PathBuf,
    #[clap(default_value="./build")]
    build_path: std::path::PathBuf,
}

fn main() {
  let args = Args::parse();

  match args {
    Args { json_path, build_path } => {
        build(json_path, build_path);
      }
  }
}
