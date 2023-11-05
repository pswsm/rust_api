#[macro_use]
extern crate rocket;

use rocket::serde::json::Json;
use rocket::serde::Serialize;

mod sql;

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct CResp<T> {
    value: T,
}

#[get("/aboutme")]
async fn aboutme() -> Json<CResp<String>> {
    Json(CResp {
        value: 100000.to_string(),
    })
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![aboutme])
}
