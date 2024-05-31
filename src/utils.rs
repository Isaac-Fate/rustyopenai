#[allow(unused)]
pub fn init_test_logger() {
    let _ = env_logger::builder().filter_level(log::LevelFilter::Debug).is_test(true).try_init();
}
