use tokio_service::Service;
use futures::{future, Future, BoxFuture};
use std::io;
pub struct EchoRev;

impl Service for EchoRev {
    type Request = String;
    type Response = String;
    type Error = io::Error;
    type Future = BoxFuture<Self::Response, Self::Error>;

    fn call(&self, req: Self::Request) -> Self::Future {
        let rev: String = req.chars().rev().collect();
        future::ok(rev).boxed()
    }
}
