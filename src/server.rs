//! # server
//! A simple module for managing server easily.

use configuration;
use service;
use slog;
use weld;
use hyper::server::Http;
use futures_cpupool::CpuPool;

/// Holds server configuration and logger
pub struct Server<'a> {
    //Configuration of the server for future access.
    configuration: &'a configuration::Server,
    //Logger for the server. All services should create loggers from this.
    logger: slog::Logger,
}

impl<'a> Server<'a> {
    /// Creates a new instance of Server
    pub fn new(config: &'a configuration::Server) -> Server<'a> {
        Server {
            configuration: config,
            logger: weld::ROOT_LOGGER.new(o!()),
        }
    }

    /// Starts the server. **Server event loop will run on the same thread with the thread this function called. Be careful.**
    pub fn start(&self) {
        let endpoint =
            format!("{}:{}", self.configuration.host, self.configuration.port).parse().unwrap();
        info!(self.logger, "Server - Started ! {}", &endpoint);
        let thread_pool = CpuPool::new_num_cpus();

        Http::new()
            .bind(&endpoint, move || {
                Ok(service::RestService {
                    logger: weld::ROOT_LOGGER.new(o!()),
                    thread_pool: thread_pool.clone(),
                })
            })
            .unwrap()
            .run()
            .unwrap();
    }
}

