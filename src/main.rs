//! # Weld
//!

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

mod service;
mod server;
mod configuration;
mod database;
mod weld;

use server::Server;
use configuration::Configuration;
use std::env::args;

fn main() {
    info!(weld::ROOT_LOGGER, "Application started";"started_at" => format!("{}", time::now().rfc3339()), "version" => env!("CARGO_PKG_VERSION"));
    let mut configuration = weld::CONFIGURATION.lock().unwrap();
    if let Some(path) = args().nth(1) {
        configuration.load(path.as_str())
    } else {
        info!(weld::ROOT_LOGGER, "Program arguments not found.");
        configuration.load("weld.json");
    }
    load_db(&configuration);
    // Always call this at the end.
    Server::new(&configuration.server).start();
}

fn load_db(configuration: &Configuration) {
    let mut database = weld::DATABASE.lock().unwrap();
    database.set_configuration(&configuration.database);
    database.open();
}
