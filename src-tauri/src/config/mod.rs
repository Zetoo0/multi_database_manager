use log::info;

pub mod log4rs_conf;

pub fn load_config() {
    log4rs_conf::load_config();

    info!("Config initialized")
}
