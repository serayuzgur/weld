//! # Weld
//!

#[doc(hidden)]
extern crate bytes;
#[doc(hidden)]
extern crate futures;
#[doc(hidden)]
extern crate tokio_io;
#[doc(hidden)]
extern crate tokio_proto;
#[doc(hidden)]
extern crate tokio_service;

mod codec;
mod proto;
mod service;

use service::echo_rev_service::EchoRev;
use proto::line_proto::LineProto;
use tokio_proto::TcpServer;

fn main() {
    // Specify the localhost address
    let addr = "0.0.0.0:8080".parse().unwrap();

    // The builder requires a protocol and an address
    let server = TcpServer::new(LineProto, addr);
    // We provide a way to *instantiate* the service for each new
    // connection; here, we just immediately return a new instance.
    server.serve(|| Ok(EchoRev));
}