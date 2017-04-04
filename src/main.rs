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

use server::Server;

use futures_cpupool::CpuPool;
use configuration::Configuration;

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
    // take it from program argumants
    let path = "weld.json";   
    let configuration: Configuration = Configuration::new(&path.to_string());
    let thread_pool = CpuPool::new_num_cpus();

    let server = Server::new(&configuration,&thread_pool);


//     let db_url = "mysql://root@localhost:3306/weld";
//     let db_config = r2d2::Config::default();
//     let db_manager = MysqlConnectionManager::new(db_url).unwrap();
//     let db_pool = r2d2::Pool::new(db_config, db_manager).unwrap();

//     // The builder requires a protocol and an address
//     // We provide a way to *instantiate* the service for each new
//     // connection; here, we just immediately return a new instance.
//   

    // Always call this at the end.
    server.start();
}