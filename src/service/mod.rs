//! # service
//! This is the service layer of the application.
//! All requests taken by the server will be consumed by the services under this module.
pub mod utils;
pub mod query_api;

use weld;
use slog;
use hyper::{Delete, Error, Get, Post, Put, StatusCode};
use hyper::server::{Request, Response, Service};
use hyper;

use futures::{Future, Stream};
use futures_cpupool::CpuPool;
use serde_json::{from_slice, to_value};
use database::errors::Errors::{Conflict, NotFound};

use self::query_api::Queries;

type FutureBox = Box<Future<Item = Response, Error = Error>>;

/// A Simple struct to represent rest service.
pub struct RestService {
    /// Logger of the service.
    pub logger: slog::Logger,
    /// Thread pool sent from the Server in order to manage threading.
    pub thread_pool: CpuPool,
}

impl RestService {
    #[inline]
    /// Gets the desired data from the path and returns.
    /// To reach service Http Method must be GET.
    /// It works in acceptor thread. Since it is fast for small databases it is ok to work like this.
    /// Later all services must be handled under a new thread.
    fn get(paths: Vec<String>, queries: Option<Queries>, response: Response) -> FutureBox {
        let mut db = weld::DATABASE.lock().unwrap();
        match db.read(&mut paths.clone(), queries) {
            Ok(record) => return utils::success(response, StatusCode::Ok, &record),
            Err(error) => match error {
                NotFound(msg) => utils::error(response, StatusCode::NotFound, msg.as_str()),
                _ => utils::error(response, StatusCode::InternalServerError, "Server Error"),
            },
        }
    }

    /// Creates the resource to the desired path and returns.
    /// To reach service Http Method must be POST.
    /// It reads request in acceptor thread. Does all other operations at a differend thread.
    #[inline]
    fn post(req: Request, paths: Vec<String>, response: Response) -> FutureBox {
        Box::new(req.body().concat2().and_then(move |body| {
            let mut db = weld::DATABASE.lock().unwrap();
            match from_slice(body.to_vec().as_slice()) {
                Ok(payload) => match db.insert(&mut paths.clone(), payload) {
                    Ok(record) => {
                        db.flush();
                        utils::success(response, StatusCode::Created, &record)
                    }
                    Err(error) => match error {
                        NotFound(msg) => utils::error(response, StatusCode::NotFound, msg.as_str()),
                        Conflict(msg) => utils::error(response, StatusCode::Conflict, msg.as_str()),
                    },
                },
                Err(_) => utils::error(
                    response,
                    StatusCode::BadRequest,
                    "Request body must be a valid json.",
                ),
            }
        }))
    }

    /// Updates the resource at the desired path and returns.
    /// To reach service Http Method must be PUT.
    /// It reads request in acceptor thread. Does all other operations at a differend thread.
    #[inline]
    fn put(req: Request, paths: Vec<String>, response: Response) -> FutureBox {
        Box::new(req.body().concat2().and_then(move |body| {
            let mut db = weld::DATABASE.lock().unwrap();
            match from_slice(body.to_vec().as_slice()) {
                Ok(payload) => match db.update(&mut paths.clone(), payload) {
                    Ok(record) => {
                        db.flush();
                        info!(weld::ROOT_LOGGER, "PUT: DB Event");
                        return utils::success(response, StatusCode::Ok, &record);
                    }
                    Err(error) => {
                        if let NotFound(msg) = error {
                            utils::error(response, StatusCode::NotFound, msg.as_str())
                        } else {
                            utils::error(response, StatusCode::InternalServerError, "Server Error")
                        }
                    }
                },
                Err(_) => utils::error(
                    response,
                    StatusCode::BadRequest,
                    "Request body must be a valid json.",
                ),
            }
        }))
    }

    /// Deletes the resource at the desired path and returns.
    /// To reach service Http Method must be DELETE.
    /// It reads request in acceptor thread. Does all other operations at a differend thread.
    #[inline]
    fn delete(paths: Vec<String>, response: Response) -> FutureBox {
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

/// Service implementation for the RestService. It is required by tokio to make it work with our service.
impl Service for RestService {
    /// Type of the request
    type Request = Request;
    /// Type of the response
    type Response = Response;
    /// Type of the error
    type Error = hyper::Error;

    type Future = FutureBox;
    /// Type of the future

    /// Entry point of the service. Pases path nad method and redirect to the correct function.
    fn call(&self, req: Request) -> FutureBox {
        let path_parts = utils::split_path(req.path().to_string());
        let response = Response::new();
        // Table list
        if let 0 = path_parts.len() {
            // TODO: return as homepage with links
            let db = weld::DATABASE.lock().unwrap();
            utils::success(response, StatusCode::Ok, &to_value(&db.tables()).unwrap())
        } else {
            // Record list or record
            match req.method() {
                &Get => {
                    let queries = query_api::parse(req.query());
                    Self::get(path_parts, queries, response)
                }
                &Post => Self::post(req, path_parts, response),
                &Put => Self::put(req, path_parts, response),
                &Delete => Self::delete(path_parts, response),
                _ => utils::error(response, StatusCode::MethodNotAllowed, "Method Not Allowed"),
            }
        }
    }
}
