use configuration::Configuration;
use service::rest_service;
use tokio_proto::TcpServer;
use tokio_minihttp;
use futures;
use futures_cpupool;
use std::marker::Send;
use slog;

pub struct Server<'a> {
    configuration: &'a Configuration,
    thread_pool: &'a futures_cpupool::CpuPool,
    logger: slog::Logger
}


impl<'a> Server<'a> {
    pub fn new(config: &'a Configuration, thread_pool: &'a futures_cpupool::CpuPool, logger: &slog::Logger) -> Server<'a> {
        Server {
            configuration: config,
            thread_pool: thread_pool,
            logger: logger.new(o!()),
        }
    }
    pub fn start(&self) {
        //TODO: Make it loop
        let listener = self.configuration.server
            .listeners
            .last()
            .unwrap();
        
        // info!(root_logger, "Application started";"started_at" => format!("{}", time::now().rfc3339()));

        info!(self.logger,"Listener {:?}", &listener);


        let endpoint = format!("{}:{}", &listener.host, &listener.port).parse().unwrap();


        println!("Server Started!");
        TcpServer::new(tokio_minihttp::Http, endpoint).serve(move || {
                                                                 Ok(rest_service::RestService {})
                                                             });

        // Finish loop;

    }
}

