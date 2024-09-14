use env_loader::load::load;
use log::debug;
use std::env;

pub fn init_env() {
    env_logger::init();
    match load(".env") {
        Ok(_) => {}
        Err(err) => panic!("{err:?}"),
    }
}

pub fn get_system_variable(variable_name: &str) -> String {
    match env::var(variable_name) {
        Ok(addr) => {
            debug!("Variable initialized: {}", &addr);
            addr
        }
        Err(err) => {
            panic!("{}", err)
        }
    }
}
