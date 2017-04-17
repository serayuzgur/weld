use configuration;
use rest_service;
use slog;
use weld;
use hyper::server::Http;
use futures_cpupool::CpuPool;

pub struct Server<'a> {
    configuration: &'a configuration::Server,
    logger: slog::Logger,
}


impl<'a> Server<'a> {
    pub fn new(config: &'a configuration::Server) -> Server<'a> {
        Server {
            configuration: config,
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
        let thread_pool = CpuPool::new_num_cpus();

        Http::new()
            .bind(&endpoint, move || {
                Ok(rest_service::RestService {
                       logger: weld::ROOT_LOGGER.new(o!()),
                       thread_pool: thread_pool.clone(),
                   })
            })
            .unwrap()
            .run()
            .unwrap();
    }
}

