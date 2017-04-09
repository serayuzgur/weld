use configuration;
use rest_service;
use tokio_proto::TcpServer;
use tokio_minihttp;
use futures;
use futures_cpupool;
use std::marker::Send;
use slog;
use weld;
use http;

pub struct Server<'a> {
    configuration: &'a configuration::Server,
    thread_pool: &'a futures_cpupool::CpuPool,
    logger: slog::Logger,
}


impl<'a> Server<'a> {
    pub fn new(config: &'a configuration::Server,
               thread_pool: &'a futures_cpupool::CpuPool)
               -> Server<'a> {
        Server {
            configuration: config,
            thread_pool: thread_pool,
            logger: weld::ROOT_LOGGER.new(o!()),
        }
    }
    pub fn start(&self) {
        info!(self.logger,
              "Server - Configuration {:?}",
              self.configuration);
        let endpoint =
            format!("{}:{}", self.configuration.host, self.configuration.port).parse().unwrap();
        info!(self.logger, "Server - Started!");
        TcpServer::new(http::Http, endpoint).serve(move || {
                                                       Ok(rest_service::RestService {
                                                              logger: weld::ROOT_LOGGER.new(o!()),
                                                          })
                                                   });

    }
}

