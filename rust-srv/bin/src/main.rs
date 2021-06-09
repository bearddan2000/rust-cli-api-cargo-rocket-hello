#![feature(decl_macro)]
#[macro_use] extern crate rocket;

use rocket::response::content::Json;

fn main() {
    rocket::ignite()
      .mount("/api", routes![hello])
      .launch();
}

#[get("/")]
fn hello() -> Json<&'static str> {
  Json("{
    'status': 'success',
    'message': 'Hello API!'
  }")
}
