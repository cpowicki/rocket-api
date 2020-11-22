#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;

use std::sync::{Arc, RwLock};

use api::Config;
use cache::{Cache, CacheMutation};
use rocket::response::status::Accepted;
use rocket::State;
use rocket_contrib::json::Json;

use anyhow::Result;

mod api;
mod cache;

pub type ApplicationCache = Arc<RwLock<Cache<Config>>>;

#[get("/config/<name>")]
fn get_config(name: String, state: State<ApplicationCache>) -> Option<Json<Config>> {
    match state.inner().read() {
        Ok(guard) => guard.get(&name).map(|config| Json(config.clone())),
        Err(_) => None,
    }
}

#[post("/config", format = "json", data = "<json>")]
fn post_config(json: Json<Config>, state: State<ApplicationCache>) -> Result<Accepted<String>> {
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
    let (tx, mut rx) = tokio::sync::mpsc::channel::<CacheMutation>(10);

    let cache: Cache<Config> = cache::Cache::new(100, tx);
    let state = Arc::new(RwLock::new(cache));
    let manager = state.clone();

    tokio::spawn(async move {
        while let Some(operation) = rx.recv().await {
            match operation {
                CacheMutation::Drop(name) => match manager.write() {
                    Ok(mut lock) => lock.delete(name),
                    Err(e) => println!("Failed to lock cache {:}", e),
                },
            }
        }
    });

    rocket::ignite()
        .mount("/", routes![get_config, post_config])
        .manage(state)
        .launch();
}
