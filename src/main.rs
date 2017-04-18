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

mod rest_service;
mod server;
mod configuration;
mod database;

use server::Server;
use configuration::Configuration;
use std::env::args;

/// Holds the shared variables of the application. 
//TODO: Is is the right way?
pub mod weld {
    //TODO: take this to a seperate file later.
    use slog;
    use slog_term;
    use slog_async;
    use slog::Drain;
    use std::sync::Arc;
    use configuration::Configuration;
    use configuration;
    use database::Database;
    use std::sync::Mutex;

    lazy_static! {
        pub static ref ROOT_LOGGER: slog::Logger = slog::Logger::root(Arc::new(slog_async::Async::new(slog_term::CompactFormat::new(slog_term::TermDecorator::new().build()).build().fuse()).build().fuse()), o!());
        pub static ref CONFIGURATION : Mutex<Configuration> = Mutex::new(Configuration::new(""));
        pub static ref DATABASE : Mutex<Database> = Mutex::new(Database::new(&configuration::Database{path:"".to_string()}));
    }
}

fn main() {
    info!(weld::ROOT_LOGGER, "Application started";"started_at" => format!("{}", time::now().rfc3339()), "version" => env!("CARGO_PKG_VERSION"));
    let mut configuration =  weld::CONFIGURATION.lock().unwrap();
    match args().nth(1) {
        Some(path) => configuration.load(path.as_str()),
        None => {
            info!(weld::ROOT_LOGGER,"Program arguments not found.");
            configuration.load("weld.json");
        }
    }
    load_db(&configuration);
    // Always call this at the end.
    Server::new(&configuration.server).start();
}

fn load_db(configuration: &Configuration){
    let mut database = weld::DATABASE.lock().unwrap();
    database.set_configuration(&configuration.database);
    database.open();
}
