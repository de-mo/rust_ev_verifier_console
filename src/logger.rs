use log::LevelFilter;
use log4rs::{
    append::{console::ConsoleAppender, file::FileAppender},
    config::{Appender, Root},
    encode::pattern::PatternEncoder,
    Config,
};
use rust_verifier::Config as VerifierConfig;

/// Init the logger with or without stdout
pub fn init_logger(config: &'static VerifierConfig, level: LevelFilter, with_console: bool) {
    // File logger
    let file = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{d} {l} - {m}{n}")))
        .build(config.log_file_path())
        .unwrap();
    let mut root_builder = Root::builder().appender("file");
    let mut config_builder =
        Config::builder().appender(Appender::builder().build("file", Box::new(file)));

    // Console logger
    if with_console {
        let stdout = ConsoleAppender::builder()
            .encoder(Box::new(PatternEncoder::new("{h({l})} - {m}{n}")))
            .build();
        root_builder = root_builder.appender("stdout");
        config_builder =
            config_builder.appender(Appender::builder().build("stdout", Box::new(stdout)));
    }

    let config = config_builder.build(root_builder.build(level)).unwrap();
    let _handle = log4rs::init_config(config).unwrap();
}
