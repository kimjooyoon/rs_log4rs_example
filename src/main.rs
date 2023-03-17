use log::{debug, error, info};

fn main() {
    log4rs::init_file("log4rs.yml", Default::default()).unwrap();

    debug!("debug zzz");
    error!("this is error");
    info!("what the info");
}