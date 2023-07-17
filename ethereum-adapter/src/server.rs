
use std::sync::RwLock;
use rocket::*;
use rocket::State;
use rocket::response::content;
use rocket::serde::{Serialize, Deserialize, json::Json};

#[derive(Serialize,Deserialize, Debug, Clone)]
#[serde(crate = "rocket::serde")]
struct Address {
  address: String
}

#[derive(Serialize,Deserialize, Debug, Clone)]
#[serde(crate = "rocket::serde")]
struct Signature {
  by: Address,
  signature: String
}

#[derive(Debug, Default)]
struct AdapterState {
  address: RwLock<Option<Address>>,
  signature: RwLock<Option<Signature>>,
}

#[get("/login")]
fn login() -> content::RawHtml<&'static str> {
    content::RawHtml(include_str!("../frontend/dist/login.html"))
}


#[post("/address", data = "<address>")]
fn set_address(address: Json<Address>, state: &State<AdapterState>) {
  let mut state_address = state.address.write().unwrap();
  *state_address = Some(address.0);
}

#[get("/address")]
fn get_address(state: &State<AdapterState>) -> Json<Option<Address>> {
  let address = state.address.read().unwrap();
  Json((*address).clone())
}

#[get("/sign")]
fn sign() -> content::RawHtml<&'static str> {
    content::RawHtml(include_str!("../frontend/dist/sign.html"))
}

#[post("/signature", data = "<signature>")]
fn set_signature(signature: Json<Signature>, state: &State<AdapterState>) {
  let mut state_signature = state.signature.write().unwrap();
  *state_signature = Some(signature.0);
}

#[get("/signature")]
fn get_signature(state: &State<AdapterState>) -> Json<Option<Signature>> {
  let signature = state.signature.read().unwrap();
  Json((*signature).clone())
}

#[get("/main.js")]
fn js() -> content::RawJavaScript<&'static str> {
    content::RawJavaScript(include_str!("../frontend/dist/main.js"))
}

#[get("/main.css")]
fn css() -> content::RawCss<&'static str> {
    content::RawCss(include_str!("../frontend/dist/main.css"))
}


#[launch]
fn rocket() -> _ {
    rocket::build().manage(AdapterState::default()).mount("/", routes![
      login,
      get_address, 
      set_address,
      sign,
      set_signature,
      get_signature,
      js, 
      css
    ])
}