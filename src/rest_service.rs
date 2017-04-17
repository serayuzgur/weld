extern crate serde_json;
extern crate futures;
extern crate futures_cpupool;

use std::vec::Vec;

use weld;
use slog;
use hyper::{Get, Post,Put,Delete, StatusCode};
use hyper::server::{Service, Request, Response};
use hyper;
use futures::Stream;
use futures::Future;


pub struct RestService {
    pub logger: slog::Logger,
    pub thread_pool: futures_cpupool::CpuPool,

}

impl RestService {
    /// Prepares an error response , logs it, wraps to BoxFuture.
    pub fn error(
                 response: Response,
                 code: StatusCode,
                 message: &str)
                 -> Result<Response,hyper::Error>{
        // error!(self.logger, "{}",&message);
        return Ok(response.with_status(code).with_body(message.to_string()));
    }

    /// Prepares an success response, wraps to BoxFuture.
    pub fn success(
                   response: Response,
                   value: &serde_json::Value)
                   -> Result<Response,hyper::Error> {
        return Ok(response.with_body(serde_json::to_vec(&value).unwrap()));
    }

    pub fn read_body(req: Request)-> Vec<u8>{
        req.body().fold(Vec::<u8>::new(), |mut v, chunk| {
                                v.extend(&chunk[..]);
                                futures::future::ok::<_, hyper::Error>(v)
        }).wait().unwrap()
    }
}

impl Service for RestService {
    type Request = Request;
    type Response = Response;
    type Error = hyper::Error;
    type Future = futures::BoxFuture<Response, hyper::Error>;

    fn call(&self, req: Request) -> Self::Future {
        self.thread_pool.spawn_fn(move || {
            let path: &String = &req.path().into();
            let parts = path.split("/").filter(|x| !x.is_empty()).collect::<Vec<_>>().clone();
            let mut db = weld::DATABASE.lock().unwrap();
            let response = Response::new();
            // response.with_header("Content-Type", "application/json");
            match parts.len() {
                // Table list
                0 => return RestService::success(response,&serde_json::to_value(&db.tables()).unwrap()),      
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
                                        return RestService::error(response,
                                                        StatusCode::PreconditionFailed,
                                                        &format!("Non parsable id Error: {}", e))
                                    }

                                }
                            }
                        }
                        None => {}
                    }
                    match req.method(){
                        &Get=>{
                            match db.read(table, &id) {
                                Some(record) => return RestService::success(response, &record),
                                None => return RestService::error(response, StatusCode::NotFound, "Record not found"),
                            }
                        }
                        &Post=>{
                            let mut payload: serde_json::Value = serde_json::from_slice(Self::read_body(req).as_slice()).unwrap();
                            match db.insert(table,payload.as_object_mut().unwrap()) {
                                Some(record) => {db.flush(); return RestService::success(response, &record)}
                                None => return RestService::error(response, StatusCode::NotFound, "Record not found"),
                            }         
                        }
                        &Put=>{
                            let  payload: serde_json::Value = serde_json::from_slice(Self::read_body(req).as_slice()).unwrap();
                            match db.update(table,payload.as_object().unwrap().clone()) {
                                Some(record) => {db.flush(); return RestService::success(response, &record)}
                                None => return RestService::error(response, StatusCode::NotFound, "Record not found"),
                            }
                        }
                        &Delete=>{
                            match db.delete(table,&id) {
                                Some(record) => {db.flush();return RestService::success(response, &record)}
                                None => return RestService::error(response, StatusCode::NotFound, "Record not found"),
                            }
                        }
                        _ =>{
                            RestService::error(response, StatusCode::MethodNotAllowed, "Method Not Allowed")
                        }
                    }
                }
                _ => {
                    return RestService::error(response, StatusCode::InternalServerError, "Nested structures are not implemented yet.");
                }
            }}).boxed()      
        }
    }


