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
        let parts = path.split("/").filter(|x| !x.is_empty()).collect::<Vec<_>>();

        let mut db = weld::DATABASE.lock().unwrap();
        let mut response = Response::new();
        response.header("Content-Type", "application/json");

        match parts.len() {
            0 => {
                // Table list
                let json = serde_json::to_string(&db.tables()).unwrap();
                response.body(&json);
                return futures::future::ok(response).boxed();
            }
            1 | 2 => {
                // Record list or record
                let table = parts.get(0).unwrap();
                let mut id = -1;
                match parts.get(1) {
                    Some(val) => {
                        if !val.is_empty() {
                            match i64::from_str_radix(val, 10) {
                                Ok(parsed) => id = parsed,
                                Err(e) => {
                                    error!(self.logger, "Non parsable id Error: {}", e);
                                    response.status_code(412, "Non parsable id Error: {}");
                                    response.body("Non parsable id Error: {}");
                                    return futures::future::ok(response).boxed();
                                }
                            }
                        }
                    }
                    None => {}
                }
                match db.read(table, &id) {
                    Some(record) => {
                        let json = serde_json::to_string(&record).unwrap();
                        response.body(&json);
                        return futures::future::ok(response).boxed();
                    }
                    None => {
                        error!{self.logger,"Record not found"}
                        response.status_code(404, "Record not found");
                        response.body("Record not found");
                        return futures::future::ok(response).boxed();
                    }
                }
            }

            _ => {
                error!{self.logger,"Nested structures are not implemented yet."}
                response.status_code(500, "Nested structures are not implemented yet.");
                response.body("Nested structures are not implemented yet.");
                return futures::future::ok(response).boxed();
            }
        }
    }
}

