extern crate serde_json;
extern crate tokio_minihttp;

extern crate futures;
extern crate futures_cpupool;

use std::io;
use std::vec::Vec;
use futures::{BoxFuture, Future};
use http::{Request, Response};
use tokio_service::Service;
use weld;
use slog;

pub struct RestService {
    pub logger: slog::Logger,
}

impl RestService {
    /// Prepares an error response , logs it, wraps to BoxFuture.
    pub fn error(&self,
                 response: Response,
                 code: u32,
                 message: &str)
                 -> BoxFuture<Response, io::Error> {
        let mut response_mut = response;
        error!(self.logger, "{}",&message);
        response_mut.status_code(code, message);
        response_mut.body(message);
        return futures::future::ok(response_mut).boxed();
    }

    /// Prepares an success response, wraps to BoxFuture.
    pub fn success(&self,
                   response: Response,
                   value: &serde_json::Value)
                   -> BoxFuture<Response, io::Error> {
        let mut response_mut = response;
        let body = serde_json::to_string(&value).unwrap();
        response_mut.body(&body);
        debug!(self.logger,"Response {}",&body);
        return futures::future::ok(response_mut).boxed();
    }
}

impl Service for RestService {
    type Request = Request;
    type Response = Response;
    type Error = io::Error;
    type Future = BoxFuture<Response, io::Error>;

    fn call(&self, req: Request) -> Self::Future {
        let path = req.path();
        info!(self.logger, "{} : {} - {:?}", &path, req.method(),req);
        let parts = path.split("/").filter(|x| !x.is_empty()).collect::<Vec<_>>();

        let mut db = weld::DATABASE.lock().unwrap();
        let mut response = Response::new();
        response.header("Content-Type", "application/json");

        match parts.len() {
            // Table list
            0 => return self.success(response,&serde_json::to_value(&db.tables()).unwrap()),      
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
                                    return self.error(response,
                                                      412,
                                                      &format!("Non parsable id Error: {}", e))
                                }

                            }
                        }
                    }
                    None => {}
                }
                match req.method(){
                    "GET"=>{
                        match db.read(table, &id) {
                            Some(record) => return self.success(response, &record),
                            None => return self.error(response, 404, "Record not found"),
                        }
                    }
                    "POST"=>{
                        let mut payload: serde_json::Value = serde_json::from_str(req.body()).unwrap();
                        let mut payload_mut = payload.as_object_mut().unwrap();
                        match db.insert(table,payload_mut) {
                            Some(record) => {db.flush(); return self.success(response, &record)}
                            None => return self.error(response, 404, "Record not found"),
                        }
                    }
                    "PUT"=>{
                        let payload: serde_json::Value = serde_json::from_str(req.body()).unwrap();
                        match db.update(table,payload.as_object().unwrap().clone()) {
                            Some(record) => {db.flush(); return self.success(response, &record)}
                            None => return self.error(response, 404, "Record not found"),
                        }
                    }
                    "DELETE"=>{
                        match db.delete(table,&id) {
                            Some(record) => {db.flush();return self.success(response, &record)}
                            None => return self.error(response, 404, "Record not found"),
                        }
                    }
                    _ =>{
                        self.error(response, 405, "Method Not Allowed")
                    }
                }
            }
            _ => {
                return self.error(response, 500, "Nested structures are not implemented yet.");
            }
        }
    }
}

