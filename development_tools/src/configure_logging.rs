use log::{debug, info, warn, LevelFilter};
use log4rs::{
    append::file::FileAppender,
    config::{Appender, Config, Root},
    encode::pattern::PatternEncoder,
};
use std::error::Error;

pub mod foo {
    use super::*;

    pub mod bar {
        use super::*;

        pub fn run() {
            warn!("[bar] warn");
            info!("[bar] info");
            debug!("[bar] debug");
        }
    }

    pub fn run() {
        warn!("[foo] warn");
        info!("[foo] info");
        debug!("[foo] debug");
        bar::run();
    }
}

/// Configure logging to write to a file
///
/// # Examples
///
/// ```ignore
/// use log::info;
/// use development_tools::log_to_file;
///
/// let result = log_to_file();
/// info!("Doc test!");
/// assert!(result.is_ok());
/// ```
pub fn log_to_file() -> Result<(), Box<dyn Error>> {
    let logfile = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{l} - {m}\n")))
        .build("log/output.log")?;

    let config = Config::builder()
        .appender(Appender::builder().build("logfile", Box::new(logfile)))
        .build(Root::builder().appender("logfile").build(LevelFilter::Info))?;

    log4rs::init_config(config)?;

    Ok(())
}
