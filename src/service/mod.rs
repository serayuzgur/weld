pub mod utils;
pub mod query;

use weld;
use slog;
use hyper::{Get, Post, Put, Delete, StatusCode};
use hyper::server::{Service, Request, Response};
use hyper;
use futures::{Stream, Future, BoxFuture};
use futures_cpupool::CpuPool;
use serde_json::{from_slice, Value, to_value};
use database::errors::Errors::{NotFound, BadData, Duplicate};

pub struct RestService {
    pub logger: slog::Logger,
    pub thread_pool: CpuPool,
}

impl RestService {
    #[inline]
    /// Gets records or spesific record from db and returns as a result.
    fn get(paths: Vec<String>, response: Response) -> BoxFuture<Response, hyper::Error> {
        let mut db = weld::DATABASE.lock().unwrap();
        match db.read(&mut paths.clone()) {
            Ok(record) => return utils::success(response, StatusCode::Ok, &record),
            Err(error) => {
                match error {
                    NotFound(msg) => utils::error(response, StatusCode::NotFound, msg.as_str()),
                    _ => utils::error(response, StatusCode::InternalServerError, "Server Error"),
                }
            }
        }
    }

    #[inline]
    /// Creates the record. Returns the persisted version.
    fn post(req: Request,
            paths: Vec<String>,
            response: Response)
            -> BoxFuture<Response, hyper::Error> {
        req.body()
            .concat()
            .and_then(move |body| {
                let mut db = weld::DATABASE.lock().unwrap();
                let payload: Value = from_slice(body.to_vec().as_slice()).unwrap();
                match db.insert(&mut paths.clone(), payload) {
                    Ok(record) => {
                        db.flush();
                        utils::success(response, StatusCode::Created, &record)
                    }
                    Err(error) => {
                        match error {
                            NotFound(msg) => {
                                utils::error(response, StatusCode::NotFound, msg.as_str())
                            }
                            BadData(msg) => {
                                utils::error(response, StatusCode::Conflict, msg.as_str())
                            }
                            Duplicate(msg) => {
                                utils::error(response, StatusCode::Conflict, msg.as_str())
                            }
                        }
                    }
                }
            })
            .boxed()
    }

    #[inline]
    /// Updates the record. Returns the persisted version.
    fn put(req: Request,
           paths: Vec<String>,
           response: Response)
           -> BoxFuture<Response, hyper::Error> {
        req.body()
            .concat()
            .and_then(move |body| {
                let mut db = weld::DATABASE.lock().unwrap();
                let payload: Value = from_slice(body.to_vec().as_slice()).unwrap();
                match db.update(&mut paths.clone(), payload) {
                    Ok(record) => {
                        db.flush();
                        return utils::success(response, StatusCode::Ok, &record);
                    }
                    Err(error) => {
                        if let NotFound(msg) = error {
                            utils::error(response, StatusCode::NotFound, msg.as_str())
                        } else {
                            utils::error(response, StatusCode::InternalServerError, "Server Error")
                        }
                    }
                }
            })
            .boxed()
    }

    #[inline]
    /// Deletes the record. Returns the data back.
    fn delete(paths: Vec<String>, response: Response) -> BoxFuture<Response, hyper::Error> {
        let mut db = weld::DATABASE.lock().unwrap();
        match db.delete(&mut paths.clone()) {
            Ok(record) => {
                db.flush();
                return utils::success(response, StatusCode::Ok, &record);
            }
            Err(error) => {
                if let NotFound(msg) = error {
                    return utils::error(response, StatusCode::NotFound, msg.as_str());
                } else {
                    utils::error(response, StatusCode::NotFound, "Server Error")
                }
            }
        }
    }
}

impl Service for RestService {
    type Request = Request;
    type Response = Response;
    type Error = hyper::Error;
    type Future = BoxFuture<Response, hyper::Error>;

    fn call(&self, req: Request) -> Self::Future {
        let path_parts = utils::split_path(req.path().to_string());
        let response = Response::new();
        // Table list
        if let 0 = path_parts.len() {
            let db = weld::DATABASE.lock().unwrap();
            utils::success(response, StatusCode::Ok, &to_value(&db.tables()).unwrap())
        } else {
            // Record list or record
            match req.method() {
                &Get => Self::get(path_parts, response),   
                &Post => Self::post(req, path_parts, response),   
                &Put => Self::put(req, path_parts, response),   
                &Delete => Self::delete(path_parts, response),
                _ => utils::error(response, StatusCode::MethodNotAllowed, "Method Not Allowed"),
            }
        }
    }
}
