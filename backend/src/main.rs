#[macro_use]
extern crate rocket;

use rocket::http::Status;
use smithe_lib::player::get_all_like;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/<tag>")]
fn search_players(tag: &str) -> Result<String, Status> {
    Ok(
        serde_json::to_string(&get_all_like(tag).map_err(|_| Status::InternalServerError)?)
            .unwrap(),
    )
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/players", routes![search_players])
}
