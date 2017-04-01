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


mod codec;
mod proto;
mod service;

use service::db_service::DbService;
use proto::line_proto::LineProto;
use tokio_proto::TcpServer;
use r2d2_mysql::CreateManager;
use r2d2_mysql::MysqlConnectionManager;

use futures::{BoxFuture, Future};
use futures_cpupool::CpuPool;
use rand::Rng;
use tokio_minihttp::{Request, Response};


fn main() {
    // Specify the localhost address
    let addr = "0.0.0.0:8080".parse().unwrap();

    let thread_pool = CpuPool::new(4);

    let db_url = "mysql://root@localhost:3306/weld";
    let db_config = r2d2::Config::default();
    let db_manager = MysqlConnectionManager::new(db_url).unwrap();
    let db_pool = r2d2::Pool::new(db_config, db_manager).unwrap();

    // The builder requires a protocol and an address
    // We provide a way to *instantiate* the service for each new
    // connection; here, we just immediately return a new instance.
   TcpServer::new(tokio_minihttp::Http, addr).serve(move || {
        Ok(DbService {
            thread_pool: thread_pool.clone(),
            db_pool: db_pool.clone(),
        })
    })
}