#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;

use rocket::{
    http::{Status, Method},
    response::{self, Responder},
    Request,
};
use rocket_cors::{AllowedHeaders, AllowedOrigins};
use smithe_lib::player::{get_all_like, get_player};
use thiserror::Error;

pub const DEV_ADDRESS: &str = "http://localhost:8080/";
pub const DEV_ADDRESS_2: &str = "http://127.0.0.1:8080/";

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    SmitheLib(#[from] anyhow::Error),
    #[error(transparent)]
    SerdeJson(#[from] serde_json::Error),
}

impl<'r> Responder<'r> for Error {
    fn respond_to(self, req: &Request<'_>) -> response::Result<'r> {
        // todo: use open telemetry at this point
        match self {
            _ => Status::InternalServerError.respond_to(req),
        }
    }
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/<tag>")]
fn search_players(tag: String) -> Result<String, Error> {
    Ok(serde_json::to_string(&get_all_like(&tag)?)?)
}

#[get("/<id>")]
fn view_player(id: i32) -> Result<String, Error> {
    Ok(serde_json::to_string(&get_player(id)?)?)
}

fn main() {
    let allowed_origins = AllowedOrigins::some_exact(&[DEV_ADDRESS, DEV_ADDRESS_2]);

    let cors = rocket_cors::CorsOptions {
        allowed_origins,
        allowed_methods: vec![Method::Get, Method::Post, Method::Delete, Method::Put]
            .into_iter()
            .map(From::from)
            .collect(),
        allowed_headers: AllowedHeaders::some(&["Authorization", "Accept"]),
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors()
    .expect("failed to set cors");

    rocket::ignite()
        .mount("/", routes![index])
        .mount("/players", routes![search_players])
        .mount("/player", routes![view_player])
        .attach(cors)
        .launch();
}
