
use std::sync::RwLock;
use rocket::*;
use rocket::State;
use rocket::response::content;
use rocket::serde::{Serialize, Deserialize, json::Json};

#[derive(Debug, Default)]
struct AdapterState {
  address: RwLock<Option<Address>>
}

#[get("/login")]
fn login() -> content::RawHtml<&'static str> {
    content::RawHtml(include_str!("../frontend/dist/index.html"))
}

#[derive(Serialize,Deserialize, Debug, Clone)]
#[serde(crate = "rocket::serde")]
struct Address {
  address: String
}

#[post("/address", data = "<address>")]
fn save_address(address: Json<Address>, state: &State<AdapterState>) {
  let mut state_address = state.address.write().unwrap();
  *state_address = Some(address.0);
}

#[get("/main.js")]
fn js() -> content::RawJavaScript<&'static str> {
    content::RawJavaScript(include_str!("../frontend/dist/main.js"))
}

#[get("/main.css")]
fn css() -> content::RawCss<&'static str> {
    content::RawCss(include_str!("../frontend/dist/main.css"))
}

#[get("/address")]
fn get_address(state: &State<AdapterState>) -> Json<Option<Address>> {
  let address = state.address.read().unwrap();
  Json((*address).clone())
}

#[launch]
fn rocket() -> _ {
    rocket::build().manage(AdapterState::default()).mount("/", routes![
      login, 
      js, 
      css,
      get_address, 
      save_address
    ])
}