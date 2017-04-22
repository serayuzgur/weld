pub mod utils;
pub mod query;

use weld;
use slog;
use hyper::{Get, Post, Put, Delete, StatusCode};
use hyper::server::{Service, Request, Response};
use hyper;
use futures::{Stream, Future, BoxFuture};
use futures_cpupool::CpuPool;
use serde_json;
use database::Errors::{NotFound, BadData, Duplicate};

pub struct RestService {
    pub logger: slog::Logger,
    pub thread_pool: CpuPool,
}

impl RestService {
    #[inline]
    /// Gets records or spesific record from db and returns as a result.
    fn get(paths: &mut Vec<String>, response: Response) -> BoxFuture<Response, hyper::Error> {
        let mut db = weld::DATABASE.lock().unwrap();
        match db.read(paths) {
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
    fn post(req: Request, table: String, response: Response) -> BoxFuture<Response, hyper::Error> {
        req.body()
            .concat()
            .and_then(move |body| {
                let mut db = weld::DATABASE.lock().unwrap();
                let mut payload: serde_json::Value =
                    serde_json::from_slice(body.to_vec().as_slice()).unwrap();
                match db.insert(table.as_str(), payload.as_object_mut().unwrap()) {
                    Ok(record) => {
                        db.flush();
                        return utils::success(response, StatusCode::Created, &record);
                    }
                    Err(error) => {
                        match error {
                            NotFound(msg) => {
                                utils::error(response, StatusCode::NotFound, msg.as_str())
                            }
                            Duplicate(msg) => {
                                utils::error(response, StatusCode::Conflict, msg.as_str())
                            }
                            _ => {
                                utils::error(response,
                                             StatusCode::InternalServerError,
                                             "Server Error")
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
           table: String,
           id: i64,
           response: Response)
           -> BoxFuture<Response, hyper::Error> {
        // TODO:: use path id when updating
        req.body()
            .concat()
            .and_then(move |body| {
                let mut db = weld::DATABASE.lock().unwrap();
                let mut payload: serde_json::Value =
                    serde_json::from_slice(body.to_vec().as_slice()).unwrap();
                match db.update(table.as_str(), payload.as_object().unwrap().clone()) {
                    Ok(record) => {
                        db.flush();
                        return utils::success(response, StatusCode::Ok, &record);
                    }
                    Err(error) => {
                        match error {
                            NotFound(msg) => {
                                utils::error(response, StatusCode::NotFound, msg.as_str())
                            }
                            BadData(msg) => {
                                utils::error(response, StatusCode::Conflict, msg.as_str())
                            }
                            _ => {
                                utils::error(response,
                                             StatusCode::InternalServerError,
                                             "Server Error")
                            }
                        }
                    }
                }
            })
            .boxed()
    }

    #[inline]
    /// Deletes the record. Returns the data back.
    fn delete(table: String, id: i64, response: Response) -> BoxFuture<Response, hyper::Error> {
        let mut db = weld::DATABASE.lock().unwrap();
        match db.delete(table.as_str(), &id) {
            Ok(record) => {
                db.flush();
                return utils::success(response, StatusCode::Ok, &record);
            }
            Err(error) => {
                match error {
                    NotFound(msg) => {
                        return utils::error(response, StatusCode::NotFound, msg.as_str());
                    }
                    _ => utils::error(response, StatusCode::NotFound, "Server Error"),
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
        let mut path_parts = utils::split_path(req.path().to_string());
        let response = Response::new();

        match path_parts.len() {
            // Table list
            0 => {
                let db = weld::DATABASE.lock().unwrap();
                utils::success(response,
                               StatusCode::Ok,
                               &serde_json::to_value(&db.tables()).unwrap())
            }  
            _ => {
                // Record list or record
                let table = path_parts.get(0).unwrap().clone();
                let id = match utils::decide_id(path_parts.get(1)) {
                    Ok(result) => result,
                    Err(e) => {
                        return utils::error(response, StatusCode::PreconditionFailed, e.as_str());
                    }
                };

                match req.method() {
                    &Get => Self::get(&mut path_parts, response),   
                    &Post => Self::post(req, table, response),   
                    &Put => Self::put(req, table, id, response),   
                    &Delete => Self::delete(table, id, response),
                    _ => utils::error(response, StatusCode::MethodNotAllowed, "Method Not Allowed"),
                }
            }
        }
    }
}
