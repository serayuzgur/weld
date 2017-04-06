//! # Weld
//!

#[doc(hidden)]
extern crate bytes;
#[doc(hidden)]
extern crate tokio_io;
#[doc(hidden)]
extern crate tokio_proto;
#[doc(hidden)]
extern crate tokio_service;
#[doc(hidden)]
extern crate tokio_minihttp;

extern crate rand;

extern crate futures;
extern crate futures_cpupool;
extern crate r2d2;
extern crate r2d2_mysql;

#[doc(hidden)]
#[macro_use]
extern crate serde_derive; // we have to define it here because macros must be at root 
extern crate serde_json; 

#[macro_use]
extern crate slog;
extern crate slog_term;

#[macro_use]
extern crate lazy_static;


extern crate time;

mod codec;
mod proto;
mod service;
mod server;
mod configuration;
mod database;


use futures_cpupool::CpuPool;
use server::Server;
use configuration::Configuration;
use database::Database;

/// Holds the shared variables of the application. 
//TODO: Is is the right way?
pub mod weld {
    //TODO: take this to a seperate file later.
    use slog;
    use slog_term;
    use slog::DrainExt;
    lazy_static! {
        pub static ref ROOT_LOGGER: slog::Logger = slog::Logger::root(slog_term::streamer().build().fuse(),o!());
    }
}

fn main() {

    //Logger
    info!(weld::ROOT_LOGGER, "Application started";"started_at" => format!("{}", time::now().rfc3339()), "version" => env!("CARGO_PKG_VERSION"));

    // Read configuration from "weld.json"
    //TODO: take it from program argumants
    let path = "weld.json";   
    let configuration: Configuration = Configuration::new(&path.to_string());
    let thread_pool = CpuPool::new_num_cpus();

    let server = Server::new(&configuration.server,&thread_pool);

    let mut database = Database::new(&configuration.database);
    
    database.open();
    info!(weld::ROOT_LOGGER,"{:?}", database.tables());
    let js = r#"{
            "id": 2,
            "title": "Obaaa",
            "author": "Seray"
    }"#;
    database.insert("posts", serde_json::from_str(js).unwrap());
    database.flush();

    let js2 = r#"{
            "id": 2,
            "title": "Obaaa",
            "author": "Seray Yeni"
    }"#;

    database.update("posts", serde_json::from_str(js2).unwrap());
    database.flush();



    // Always call this at the end.
    // server.start();
}