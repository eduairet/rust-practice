use development_tools::foo;
use env_logger::Builder;
use std::{env, sync::Once};

#[cfg(test)]
mod tests_configure_logging {

    use super::*;

    static INIT: Once = Once::new();

    fn initialize() {
        INIT.call_once(|| {
            Builder::new()
                .parse_env(&env::var("TEST_APP").unwrap_or_default())
                .init();
        });
    }

    #[test]
    fn test_log_levels() {
        initialize();
        log::warn!("[root] warn");
        log::info!("[root] info");
        log::debug!("[root] debug");
        foo::run();
    }

    #[test]
    fn test_custom_env() {
        initialize();
        log::info!("informational message");
        log::warn!("warning message");
        log::error!("this is an error {}", "message");
    }
}
