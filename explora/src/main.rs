use explora::logger;

fn main() {
    logger::init();
    log::trace!("Test trace");
    log::debug!("Test debug");
    log::info!("Test info");
    log::warn!("Test warn");
    log::error!("Test error");

    let x = 5;
    log::info!("x = {}", x);
}
