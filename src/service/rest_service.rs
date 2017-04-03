extern crate serde_json;
extern crate tokio_minihttp;

extern crate futures;
extern crate futures_cpupool;
extern crate r2d2;
extern crate r2d2_mysql;

use std::io;
use futures::{BoxFuture, Future};
use tokio_minihttp::{Request, Response};
use tokio_service::Service;

pub struct RestService {}

impl Service for RestService {
    type Request = Request;
    type Response = Response;
    type Error = io::Error;
    type Future = BoxFuture<Response, io::Error>;

    fn call(&self, req: Request) -> Self::Future {
        let msg = User {
            oid: 123,
            name: "John".to_string(),
            surname: "Doe".to_string(),
        };


        let json = serde_json::to_string(&msg).unwrap();
        let mut response = Response::new();
        response.header("Content-Type", "application/json");
        response.body(&json);
        futures::future::ok(response).boxed()
    }
}

#[derive(Serialize, Deserialize)]
struct User {
    oid: i32,
    name: String,
    surname: String,
}

