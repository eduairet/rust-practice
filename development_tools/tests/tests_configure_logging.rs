use chrono::Local;
use development_tools::foo;
use env_logger::Builder;
use log::{debug, error, info, warn, LevelFilter};
use std::{env, io::Write, sync::Once};

#[cfg(test)]
mod tests_configure_logging {

    use super::*;

    static INIT: Once = Once::new();

    fn initialize() {
        INIT.call_once(|| {
            Builder::new()
                .parse_env(&env::var("TEST_APP").unwrap_or_default())
                .format(|buf, record| {
                    writeln!(
                        buf,
                        "{} [{}] - {}",
                        Local::now().format("%Y-%m-%dT%H:%M:%S"), // Use timestamp
                        record.level(),
                        record.args()
                    )
                })
                .filter(None, LevelFilter::Info)
                .init();
        });
    }

    #[test]
    fn test_log_levels() {
        initialize();
        warn!("[root] warn");
        info!("[root] info");
        debug!("[root] debug");
        foo::run();
    }

    #[test]
    fn test_custom_env() {
        initialize();
        info!("informational message");
        warn!("warning message");
        error!("this is an error {}", "message");
    }
}
