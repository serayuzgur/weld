extern crate serde_json;
extern crate rand;
extern crate tokio_minihttp;

extern crate futures;
extern crate futures_cpupool;
extern crate r2d2;
extern crate r2d2_mysql;

use std::io;
use futures::{BoxFuture, Future};
use futures_cpupool::CpuPool;
use rand::Rng;
use r2d2_mysql::MysqlConnectionManager;
use tokio_minihttp::{Request, Response};
use tokio_service::Service;

pub struct DbService {
    pub thread_pool: CpuPool,
    pub db_pool: r2d2::Pool<MysqlConnectionManager>,
}

impl Service for DbService {
    type Request = Request;
    type Response = Response;
    type Error = io::Error;
    type Future = BoxFuture<Response, io::Error>;

    fn call(&self, req: Request) -> Self::Future {

        let random_id = rand::thread_rng().gen_range(0, 10_000);
        let db = self.db_pool.clone();
        let msg = self.thread_pool.spawn_fn(move || {
            let mut conn = db.get().unwrap();
            let rows = conn.query("SELECT * FROM User WHERE oid = 1").unwrap();
            let mut row = rows.last().unwrap().unwrap();

            Ok(User {
                   oid: row.get("oid").unwrap(),
                   name: row.get("name").unwrap(),
                   surname: row.get("surname").unwrap(),
               })
        });

        msg.map(|msg| {
                     let json = serde_json::to_string(&msg).unwrap();
                     let mut response = Response::new();
                     response.header("Content-Type", "application/json");
                     response.body(&json);
                     response
                 })
            .boxed()
    }
}

#[derive(Serialize, Deserialize)]
struct User {
    oid: i32,
    name: String,
    surname: String,
}

