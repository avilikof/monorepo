use log::debug;
pub fn announce_initialization_finish() {
    template("Initialization finished!");
}

pub fn announce_initialization_start() {
    template("Starting initialization!");
}

fn template(message: &str) {
    debug!("");
    debug!("************************");
    debug!("{message}");
    debug!("************************");
    debug!("");
}
