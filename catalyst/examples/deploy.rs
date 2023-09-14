extern crate catalyst;

use catalyst::*;
use dcl_common::Result;
use httpmock::{Method::POST, MockServer};
use reqwest::multipart::Form;

#[tokio::main]
async fn main() -> Result<()> {
    let server = get_server();

    let mut form = reqwest::multipart::Form::new();
    form = form.part(
        "entityId",
        reqwest::multipart::Part::text(
            "bafkreihwww4kfevwi4nhhspz6fun3x3ludup3z3eaxihupfg7yvffxsp44",
        ),
    );

    form = add_authchain(form);
    form = add_file(form);

    let response = ContentClient::deploy_entity(&server, form).await?;
    println!("{:?}", response);
    Ok(())
}

fn get_server() -> Server {
    let response = include_str!("../fixtures/deploy_timestamp.json");

    let server = MockServer::start();

    server.mock(|when, then| {
        when.method(POST).path("/content/entities");
        then.status(200).body(response);
    });

    Server::new(server.url(""))
}

//The authentication chain needed for deployment is documented here: https://docs.decentraland.org/contributor/auth/authchain/
fn add_authchain(mut form: Form) -> Form {
    form = form.part(
        format!("authChain[0][type]"),
        reqwest::multipart::Part::text("SIGNER"),
    );
    form = form.part(
        format!("authChain[0][payload]"),
        reqwest::multipart::Part::text("0x3e0443619c66688be60586e4c1a74d7057d7b4d0"),
    );
    form = form.part(
        format!("authChain[0][signature]"),
        reqwest::multipart::Part::text(""),
    );

    form = form.part(
        format!("authChain[1][type]"),
        reqwest::multipart::Part::text("ECDSA_EPHEMERAL"),
    );
    form = form.part(
    format!("authChain[1][payload]"),
    reqwest::multipart::Part::text("Decentraland Login\nEphemeral address: 0x307C0640279B45346A0e554466228495A2bfd575\nExpiration: 2023-09-14T21:22:58.000Z"),
  );
    form = form.part(
        format!("authChain[1][signature]"),
        reqwest::multipart::Part::text("some_signature"),
    );

    form = form.part(
        format!("authChain[2][type]"),
        reqwest::multipart::Part::text("ECDSA_SIGNED_ENTITY"),
    );
    form = form.part(
        format!("authChain[2][payload]"),
        reqwest::multipart::Part::text(
            "bafkreihwww4kfevwi4nhhspz6fun3x3ludup3z3eaxihupfg7yvffxsp44",
        ),
    );
    form.part(
        format!("authChain[2][signature]"),
        reqwest::multipart::Part::text("some_signature"),
    )
}

fn add_file(form: Form) -> Form {
    let some_bytes = vec![];
    let part = reqwest::multipart::Part::bytes(some_bytes)
        .file_name("bafkreihwww4kfevwi4nhhspz6fun3x3ludup3z3eaxihupfg7yvffxsp44")
        .mime_str("application/octet-stream")
        .unwrap();

    form.part(
        "bafkreihwww4kfevwi4nhhspz6fun3x3ludup3z3eaxihupfg7yvffxsp44",
        part,
    )
}
