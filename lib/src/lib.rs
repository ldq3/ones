pub mod virtualization;
pub mod concurrency;

pub mod config {
    use std::{ io::Write, fs::File };
    use env_logger::Builder;
    use log::LevelFilter;

    pub fn initialize_logger() {
        let file = File::create("log/test.log").expect("无法创建日志文件");
        let mut builder = Builder::new();
        builder
            .format_timestamp(None)
            .format(|buf, record| { writeln!(buf, "{}: {}", record.level(), record.args()) })
            .filter(None, LevelFilter::Debug)
            .target(env_logger::Target::Pipe(Box::new(file)))
            .init();
    }
}
