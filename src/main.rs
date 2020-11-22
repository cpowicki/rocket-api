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

#[get("/config/<name>")]
fn get_config(name: String, state: State<RwLock<Cache>>) -> Option<Json<Config>> {
    match state.inner().read() {
        Ok(guard) => guard.get(&name).map(|config| Json(config.clone())),
        Err(_) => None,
    }
}

#[post("/config", format = "json", data = "<config>")]
fn post_config(config: Json<Config>, state: State<RwLock<Cache>>) -> Result<Accepted<String>> {
    state
        .inner()
        .write()
        .and_then(|mut guard| {
            guard.insert(config.into_inner());
            Ok(Accepted(Some("Success".to_owned())))
        })
        .map_err(|_| anyhow::anyhow!("Internal Server Error"))
}

fn main() {
    let cache = cache::Cache::default();

    rocket::ignite()
        .mount("/", routes![get_config, post_config])
        .manage(RwLock::new(cache))
        .launch();
}
