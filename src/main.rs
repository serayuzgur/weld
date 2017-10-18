#![warn(missing_docs)]
//! # Weld
//! Weld is a fake API generator for easily mocking back-end services.
//! It is heavily inspired from [JSON Server](https://github.com/typicode/json-server).
//! Weld also claims that you can create APIs with **zero coding** in less than **30 seconds**.
//! Providing a `db.json` file will generate crud operations, cascading manipulation and query parametric filter support.

extern crate hyper;
extern crate rand;
extern crate futures;
extern crate futures_cpupool;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
#[macro_use]
extern crate slog;
extern crate slog_term;
extern crate slog_async;
#[macro_use]
extern crate lazy_static;
extern crate time;

pub mod service;
pub mod server;
pub mod configuration;
pub mod database;
pub mod weld;

use server::Server;
use std::env::args;

fn main() {
    info!(weld::ROOT_LOGGER, "Application started";
    "started_at" => format!("{}", time::now().rfc3339()), "version" => env!("CARGO_PKG_VERSION"));

    load_config();

    load_db();

    start_server();
}

/// Loads configuration.
fn load_config() {
    let mut configuration = weld::CONFIGURATION.lock()
        .expect("Configuration is not accesible. Terminating...");

    //No arg ? so use default.
    if let Some(path) = args().nth(1) {
        configuration.load(path.as_str())
    } else {
        info!(weld::ROOT_LOGGER, "Program arguments not found.");
        configuration.load("weld.json");
    }
}

/// Loads database.
fn load_db() {
    let mut db = weld::DATABASE.lock().expect("Database is not accesible. Terminating...");
    db.load(&weld::CONFIGURATION.lock().unwrap().database);
}

/// Starts server.
fn start_server() {
    let config = weld::CONFIGURATION.lock().unwrap().server.clone();
    Server::new(&config).start();
}