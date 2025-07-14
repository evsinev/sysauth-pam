use humantime::format_rfc3339_millis;
use log::{Level, LevelFilter, Metadata, Record};
use std::fs::OpenOptions;
use std::io::Write;
use std::time::SystemTime;

pub struct FileLogger {
    pub file_path: String,
}

impl FileLogger {

    pub fn try_init(file_path: String, level: LevelFilter) {
        let file_logger = FileLogger {
            file_path,
        };

        if log::set_boxed_logger(Box::new(file_logger)).is_ok() {
            log::set_max_level(level);
        }
    }
}

impl log::Log for FileLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Trace
    }

    fn log(&self, record: &Record) {
        let message = format!(
            "{} {} - {}\n"
            , format_rfc3339_millis(SystemTime::now())
            , record.level()
            , record.args()
        );

        let mut file = OpenOptions::new()
            .append(true)
            .create(true)
            .open(self.file_path.as_str())
            .expect("Can't open file");

        let _written = file.write(message.as_bytes())
            .expect("Can't write to file");

        file.sync_all().expect("Can't sync all");
    }

    fn flush(&self) {}
}

#[cfg(test)]
mod tests {
    use file_logger::FileLogger;
    use log::{debug, LevelFilter};

    #[test]
    fn test_file_logger() {
        FileLogger::try_init("./test_file_logger.txt".to_string(), LevelFilter::Trace);
        debug!("Test log message");
    }

}