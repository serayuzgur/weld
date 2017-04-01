
extern crate bytes;
extern crate futures;
extern crate tokio_io;
extern crate tokio_proto;
extern crate tokio_service;

use std::io;
use std::str;
use bytes::{BytesMut, BufMut};
use tokio_io::codec::{Encoder, Decoder};


