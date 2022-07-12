extern crate catalyst;

use catalyst::*;

#[tokio::main]
async fn main() -> Result<()> {
  let server = Server::production();
  let parcel = Parcel(0,0);
  let scenes = LambdaClient::scenes(&server, &parcel, &parcel).await?;

  for scene in scenes {
    println!(" - {:?}", scene);
  }
  Ok(())
}
