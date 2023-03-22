use clap::Parser;
use scene_compiler::compile;

#[derive(Parser, Debug)]
struct Args {
    source_path: std::path::PathBuf,
    #[clap(default_value = "./build")]
    destination_path: std::path::PathBuf,
}

fn main() {
    let args = Args::parse();

    let Args {
        source_path,
        destination_path,
    } = args;
    let result = compile(source_path, destination_path);

    if result.is_err() {
        println!("{}", result.unwrap_err());
    }
}
