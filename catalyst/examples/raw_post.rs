extern crate catalyst;

use catalyst::*;
use dcl_common::{ Parcel, Result };
use serde::Serialize;

#[derive(Serialize)]
struct ParcelPointer<'a> {
    pointers: &'a Vec<Parcel>
}

#[derive(Serialize)]
struct EntityIds<'a> {
    ids: &'a Vec<EntityId>
}



#[tokio::main]
async fn main() -> Result<()> {
    let server = Server::production();

    let ids = EntityIds { ids: &vec![EntityId::new("bafkreiabfxgn375iwwgtx2i5zhhtge2affusbt7sndnf7wqbkeuz4f36ki")] };
    let response: reqwest::Response = server.raw_post("/content/entities/active", &ids).await?;
    let body = response.text().await?;

    println!("{}", body);

    Ok(())
}
