extern crate serde_json;
extern crate tokio_minihttp;

extern crate futures;
extern crate futures_cpupool;

use std::io;
use std::vec::Vec;
use futures::{BoxFuture, Future};
use tokio_minihttp::{Request, Response};
use tokio_service::Service;
use weld;
use slog;

pub struct RestService {
    pub paths: Vec<String>,
    pub logger: slog::Logger,
}

impl Service for RestService {
    type Request = Request;
    type Response = Response;
    type Error = io::Error;
    type Future = BoxFuture<Response, io::Error>;

    fn call(&self, req: Request) -> Self::Future {
        let path = req.path();
        info!(self.logger, "{}", &path);

        let mut db = weld::DATABASE.lock().unwrap();
        info!(self.logger, "DB aquired");
        let mut response = Response::new();
        response.header("Content-Type", "application/json");

        match db.read("posts", &1) {
            Some(record) => {
                let json = serde_json::to_string(&record).unwrap();
                response.body(&json);
            }
            None => {}
        }

        futures::future::ok(response).boxed()
    }
}

