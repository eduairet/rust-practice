use development_tools::foo;
use std::sync::Once;

#[cfg(test)]
mod tests_configure_logging {
    use super::*;

    static INIT: Once = Once::new();

    fn initialize() {
        INIT.call_once(|| {
            env_logger::init();
        });
    }

    #[test]
    fn tests_log_levels() {
        initialize();
        log::warn!("[root] warn");
        log::info!("[root] info");
        log::debug!("[root] debug");
        foo::run();
    }
}
