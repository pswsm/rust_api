#[macro_use]
extern crate rocket;

use rocket::serde::msgpack::MsgPack;

#[get("/aboutme")]
async fn aboutme() -> MsgPack<String> {
    MsgPack("Testing 1-2".to_string())
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![aboutme])
}
