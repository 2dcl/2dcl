
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

  println!("{:?}", adapter.address());
  Ok(())
}