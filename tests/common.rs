
pub fn init_logger() {
    let _ = env_logger::builder().filter_level(log::LevelFilter::Debug).is_test(true).try_init();
}
