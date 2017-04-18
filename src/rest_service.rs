extern crate serde_json;
extern crate futures;
extern crate futures_cpupool;

use std::vec::Vec;

use weld;
use slog;
use hyper::{Get, Post, Put, Delete, StatusCode};
use hyper::server::{Service, Request, Response};
use hyper;
use futures::Stream;
use futures::Future;
use futures::future::ok;

pub struct RestService {
    pub logger: slog::Logger,
    pub thread_pool: futures_cpupool::CpuPool,
}

impl RestService {
    /// Prepares an error response , logs it, wraps to BoxFuture.
    pub fn error(response: Response,
                 code: StatusCode,
                 message: &str)
                 -> futures::BoxFuture<Response, hyper::Error> {
        // error!(self.logger, "{}",&message);
        return ok(response.with_status(code).with_body(message.to_string())).boxed();
    }

    /// Prepares an success response, wraps to BoxFuture.
    pub fn success(response: Response,
                   value: &serde_json::Value)
                   -> futures::BoxFuture<Response, hyper::Error> {
        return ok(response.with_body(serde_json::to_vec(&value).unwrap())).boxed();
    }

    /// Splits '/'  and filters empty strings
    fn split_path(path: String) -> Vec<String> {
        path.split("/").filter(|x| !x.is_empty()).map(String::from).collect::<Vec<String>>()
    }

    /// Helps to decide id value.
    fn decide_id(part: Option<&String>) -> Result<i64, String> {
        match part {
            Some(val) => {
                if !val.is_empty() {
                    match i64::from_str_radix(val, 10) {
                        Ok(parsed) => Ok(parsed),
                        Err(e) => return Err(format!("Non parsable id Error: {}", e)),
                    }
                } else {
                    Ok(-1)
                }
            }
            None => Ok(-1),
        }
    }

    #[inline]
    /// Gets records or spesific record from db and returns as a result.
    fn get(table: String,
           id: i64,
           response: Response)
           -> futures::BoxFuture<Response, hyper::Error> {
        let mut db = weld::DATABASE.lock().unwrap();
        match db.read(table.as_str(), &id) {
            Some(record) => return Self::success(response, &record),
            None => return Self::error(response, StatusCode::NotFound, "Record not found"),
        }
    }

    #[inline]
    /// Creates the record. Returns the persisted version.
    fn post(req: Request,
            table: String,
            response: Response)
            -> futures::BoxFuture<Response, hyper::Error> {
        req.body()
            .concat()
            .and_then(move |body| {
                let mut db = weld::DATABASE.lock().unwrap();
                let mut payload: serde_json::Value =
                    serde_json::from_slice(body.to_vec().as_slice()).unwrap();
                match db.insert(table.as_str(), payload.as_object_mut().unwrap()) {
                    Some(record) => {
                        db.flush();
                        return Self::success(response, &record);
                    }
                    None => return Self::error(response, StatusCode::NotFound, "Record not found"),
                }
            })
            .boxed()
    }

    #[inline]
    /// Updates the record. Returns the persisted version.
    fn put(req: Request,
           table: String,
           id: i64,
           response: Response)
           -> futures::BoxFuture<Response, hyper::Error> {
        // TODO:: use path id when updating
        req.body()
            .concat()
            .and_then(move |body| {
                let mut db = weld::DATABASE.lock().unwrap();
                let mut payload: serde_json::Value =
                    serde_json::from_slice(body.to_vec().as_slice()).unwrap();
                match db.update(table.as_str(), payload.as_object().unwrap().clone()) {
                    Some(record) => {
                        db.flush();
                        return Self::success(response, &record);
                    }
                    None => return Self::error(response, StatusCode::NotFound, "Record not found"),
                }
            })
            .boxed()
    }

    #[inline]
    /// Deletes the record. Returns the data back.
    fn delete(table: String,
              id: i64,
              response: Response)
              -> futures::BoxFuture<Response, hyper::Error> {
        let mut db = weld::DATABASE.lock().unwrap();
        match db.delete(table.as_str(), &id) {
            Some(record) => {
                db.flush();
                return Self::success(response, &record);
            }
            None => return Self::error(response, StatusCode::NotFound, "Record not found"),
        }
    }
}

impl Service for RestService {
    type Request = Request;
    type Response = Response;
    type Error = hyper::Error;
    type Future = futures::BoxFuture<Response, hyper::Error>;

    fn call(&self, req: Request) -> Self::Future {
        let parts = Self::split_path(req.path().to_string());
        let response = Response::new().with_header(hyper::header::ContentType::json());

        match parts.len() {
            // Table list
            0 => {
                let db = weld::DATABASE.lock().unwrap();
                Self::success(response, &serde_json::to_value(&db.tables()).unwrap())
            }  
            1 | 2 => {
                // Record list or record
                let table = parts.get(0).unwrap().clone();
                let id = match Self::decide_id(parts.get(1)) {
                    Ok(result) => result,
                    Err(e) => {
                        return Self::error(response, StatusCode::PreconditionFailed, e.as_str());
                    }
                };

                match req.method() {
                    &Get => Self::get(table, id, response),   
                    &Post => Self::post(req, table, response),   
                    &Put => Self::put(req, table, id, response),   
                    &Delete => Self::delete(table, id, response),
                    _ => Self::error(response, StatusCode::MethodNotAllowed, "Method Not Allowed"),
                }
            }
            _ => {
                return Self::error(response,
                                   StatusCode::InternalServerError,
                                   "Nested structures are not implemented yet.");
            }
        }
    }
}
