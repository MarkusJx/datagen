use log::LevelFilter;
use log4rs::append::console::ConsoleAppender;
use log4rs::config::{Appender, Root};
use log4rs::Config;

pub fn init_logger(log_level: Option<LevelFilter>) {
    log4rs::init_config(
        Config::builder()
            .appender(
                Appender::builder().build("stdout", Box::new(ConsoleAppender::builder().build())),
            )
            .build(
                Root::builder()
                    .appender("stdout")
                    .build(log_level.unwrap_or(LevelFilter::Off)),
            )
            .unwrap(),
    )
    .unwrap();
}
