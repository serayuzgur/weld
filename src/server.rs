use configuration;
use service::rest_service;
use tokio_proto::TcpServer;
use tokio_minihttp;
use futures;
use futures_cpupool;
use std::marker::Send;
use slog;
use weld;

pub struct Server<'a> {
    configuration: &'a configuration::Server,
    thread_pool: &'a futures_cpupool::CpuPool,
    logger: slog::Logger
}


impl<'a> Server<'a> {
    pub fn new(config: &'a configuration::Server, thread_pool: &'a futures_cpupool::CpuPool) -> Server<'a> {
        Server {
            configuration: config,
            thread_pool: thread_pool,
            logger: weld::ROOT_LOGGER.new(o!()),
        }
    }
    pub fn start(&self) {
        //TODO: Make it loop
        let listener = self.configuration
            .listeners
            .last()
            .unwrap();
        
        info!(self.logger,"Listener {:?}", &listener);


        let endpoint = format!("{}:{}", &listener.host, &listener.port).parse().unwrap();


        info!(self.logger,"Server Started!");
        TcpServer::new(tokio_minihttp::Http, endpoint).serve(move || {
                                                                 Ok(rest_service::RestService {})
                                                             });

        // Finish loop;

    }
}

