use log::{info, LevelFilter};
use log4rs::{
    append::{
        console::ConsoleAppender,
        file::FileAppender,
        rolling_file::{
            policy::compound::{roll::fixed_window::FixedWindowRoller, trigger::size::SizeTrigger},
            RollingFileAppender,
        },
    },
    config::{Appender, Config, Root},
    encode::pattern::PatternEncoder,
};
use std::process;
use std::{fs, path::PathBuf};

const TRIGGER_FILE_SIZE: u64 = 5 * 1024 * 1024; // 5MB

pub fn load_config() {
    load_config_prod();

    if cfg!(dev) {
        // load_config_dev();
        info!("Logger initialized");
        info!("Starting writepad in dev mode");
        return;
    }

    if cfg!(debug_assertions) {
        info!("Logger initialized");
        info!("Starting writepad in debug mode");
        return;
    }

    info!("Logger initialized");
    info!("Starting writepad");
}

// fn load_config_dev(){
//     log4rs::init_file("log4rs_conf.yaml", Default::default()).unwrap_or_else(|e| {
//         eprintln!(
//             "error: unable to initialize logger using log4rs_conf.yaml: {}",
//             e
//         );
//         process::abort();
//     });
// }

fn load_config_prod() {
    let mut file_path: PathBuf = "log/writepad.log".into();
    let mut rolling_file_path: PathBuf = "log/writepad_rolling.log".into();
    let mut old_rolling_pattern: PathBuf = "log/old-writepad_rolling-{}.log".into();

    if !cfg!(dev) {
        let home_dir = dirs::home_dir().unwrap_or_else(|| {
            eprintln!("error: unable to get home directory");
            process::abort();
        });

        let op_dir = home_dir.join(".officepad");
        let wp_dir = op_dir.join("writepad");
        let log_dir = wp_dir.join("log");
        file_path = log_dir.join("writepad.log");
        rolling_file_path = log_dir.join("writepad_rolling.log");
        old_rolling_pattern = log_dir.join("old-writepad_rolling-{}.log");

        fs::create_dir_all(&log_dir).unwrap_or_else(|e| {
            eprintln!("error: unable to create log directory: {}", e);
            process::abort();
        });
    }
    let stdout = ConsoleAppender::builder()
        .encoder(Box::new(PatternEncoder::new(
            "{d(%+)(utc)} [{f}:{L}] {h({l})} {M}:{m}{n}",
        )))
        .build();

    let file = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new(
            "[{d(%Y-%m-%dT%H:%M:%S%.6f)} {h({l}):<5.5} {M}] {m}{n}",
        )))
        .build(file_path)
        .unwrap();

    let rolling_policy = log4rs::append::rolling_file::policy::compound::CompoundPolicy::new(
        Box::new(SizeTrigger::new(TRIGGER_FILE_SIZE)),
        Box::new(
            FixedWindowRoller::builder()
                .build(old_rolling_pattern.to_str().unwrap(), 2)
                .unwrap(),
        ),
    );

    let rolling_file = RollingFileAppender::builder()
        .encoder(Box::new(PatternEncoder::new(
            "[{d(%Y-%m-%dT%H:%M:%S%.6f)} {h({l}):<5.5} {M}] {m}{n}",
        )))
        .build(rolling_file_path, Box::new(rolling_policy))
        .unwrap();

    let log_level = if cfg!(debug_assertions) {
        LevelFilter::Debug
    } else {
        LevelFilter::Info
    };

    let config = Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(stdout)))
        .appender(Appender::builder().build("file", Box::new(file)))
        .appender(Appender::builder().build("rolling_file", Box::new(rolling_file)))
        .build(
            Root::builder()
                .appender("stdout")
                .appender("file")
                .appender("rolling_file")
                .build(log_level),
        )
        .unwrap();

    log4rs::init_config(config).unwrap_or_else(|e| {
        eprintln!("error: unable to initialize logger: {}", e);
        process::abort();
    });
}
