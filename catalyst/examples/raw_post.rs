extern crate catalyst;

use catalyst::*;
use dcl_common::{Parcel, Result};
use serde::Serialize;

#[derive(Serialize)]
struct ParcelPointer<'a> {
    pointers: &'a Vec<Parcel>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let server = Server::production();

    let ids = ParcelPointer {
        pointers: &vec![Parcel(0, 0)],
    };

    let response: reqwest::Response = server.raw_post("/content/entities/active", &ids).await?;
    let body = response.text().await?;

    println!("{}", body);

    Ok(())
}
