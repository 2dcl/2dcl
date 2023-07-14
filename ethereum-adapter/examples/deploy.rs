use ethereum_adapter::DeploySignState;
use std::{thread, time};

use ethereum_adapter::EthereumAdapter;
use dcl_common::Result;

#[tokio::main]
async fn main() -> Result<()> {
  let mut adapter = EthereumAdapter::new();
  let mut command = std::env::current_exe().unwrap();
  command.pop();
  command.pop();
  adapter.start(&mut command).unwrap();
  
  adapter.login();
  println!("Waiting...");
  while !adapter.is_logged_in().await {
    thread::sleep(time::Duration::from_millis(1000));
    println!("Awaiting for login...");
  }
  
  let mut deploy_folder = std::env::current_exe().unwrap();
  deploy_folder.pop();
  deploy_folder.pop();
  deploy_folder.pop();
  deploy_folder.pop();
  deploy_folder.push("ethereum-adapter/fixtures");
  
  adapter.sign_deploy(&deploy_folder).await?;

  println!("Waiting...");
  while adapter.sign_deploy_state() != DeploySignState::Signed {
    thread::sleep(time::Duration::from_millis(1000));
    println!("Awaiting for deploy to finish...");
  }

  println!("Done with Deployment");
  Ok(())
}