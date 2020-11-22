#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;

use std::sync::RwLock;

use api::Config;
use cache::Cache;
use rocket::response::status::Accepted;
use rocket::State;
use rocket_contrib::json::Json;

use anyhow::Result;

mod api;
mod cache;

pub type ConfigCache = Cache<Config>;

#[get("/config/<name>")]
fn get_config(name: String, state: State<RwLock<ConfigCache>>) -> Option<Json<Config>> {
    match state.inner().read() {
        Ok(guard) => guard.get(&name).map(|config| Json(config.clone())),
        Err(_) => None,
    }
}

#[post("/config", format = "json", data = "<json>")]
fn post_config(json: Json<Config>, state: State<RwLock<ConfigCache>>) -> Result<Accepted<String>> {
    state
        .inner()
        .write()
        .and_then(|mut guard| {
            let config = json.into_inner();

            guard.insert(config.get_name().to_owned(), config);
            Ok(Accepted(Some("Success".to_owned())))
        })
        .map_err(|_| anyhow::anyhow!("Internal Server Error"))
}

#[tokio::main]
async fn main() {
    let cache: ConfigCache = cache::Cache::default();

    tokio::spawn(async move {});

    rocket::ignite()
        .mount("/", routes![get_config, post_config])
        .manage(RwLock::new(cache))
        .launch();
}
