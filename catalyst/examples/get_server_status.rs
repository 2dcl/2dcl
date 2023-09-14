extern crate catalyst;

use catalyst::{LambdasClient, Server};
use dcl_common::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let server = Server::development();
    let status = LambdasClient::status(&server).await?;
    println!("{:?}", status);
    Ok(())
}
