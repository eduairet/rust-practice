use log::{debug, info, warn};

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
