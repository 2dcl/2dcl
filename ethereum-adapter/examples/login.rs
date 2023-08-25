use std::{thread, time};

use dcl_common::Result;
use ethereum_adapter::EthereumAdapter;

#[tokio::main]
async fn main() -> Result<()> {
    let mut adapter = EthereumAdapter::default();
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
