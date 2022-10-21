//! # weld
//! This module holds the static shared variables of the application. All of them could be used without fear of concurrency.
use slog::Logger;
use slog_term::{CompactFormat, TermDecorator};
use slog_async::Async;
use slog::Drain;
use std::sync::Arc;
use configuration::Configuration;
use database::Database;
use std::sync::Mutex;

//Initializes lazy statics.
lazy_static! {
    /// Root logger for the application. Helps to manage log output from one place. All loggers will use this.
    pub static ref ROOT_LOGGER: Logger = Logger::root(Arc::new(Async::new(CompactFormat::new(TermDecorator::new().build()).build().fuse()).build().fuse()), o!());
    
    /// Configuration of the application. Provides mutable synchronised configuration access.
    pub static ref CONFIGURATION : Mutex<Configuration> = Mutex::new(Configuration::new());
    
    /// Database of the application. Provides mutable synchronised db access. 
    pub static ref DATABASE : Mutex<Database> = Mutex::new(Database::new());
}
