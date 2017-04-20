/// Holds the shared variables of the application.
use slog;
use slog_term;
use slog_async;
use slog::Drain;
use std::sync::Arc;
use configuration::Configuration;
use configuration;
use database::Database;
use std::sync::Mutex;

lazy_static! {
    pub static ref ROOT_LOGGER: slog::Logger = slog::Logger::root(Arc::new(slog_async::Async::new(slog_term::CompactFormat::new(slog_term::TermDecorator::new().build()).build().fuse()).build().fuse()), o!());
    pub static ref CONFIGURATION : Mutex<Configuration> = Mutex::new(Configuration::new(""));
    pub static ref DATABASE : Mutex<Database> = Mutex::new(Database::new(&configuration::Database{path:"".to_string()}));
}
