use tokio::process::Command;
use dcl_common::Result;

mod ws;
mod renderer;

#[tokio::main]
async fn main() -> Result<()> {
  ws::start().await?;

  // spawn kernel process
  let mut command = std::env::current_exe()?;
  command.pop();
  command.push("kernel");
  Command::new(command).spawn().expect("failed to spawn");


  renderer::start();

  Ok(())

}
