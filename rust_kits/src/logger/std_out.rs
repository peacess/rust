use std::time::SystemTime;

use log::{Level, Metadata, Record, SetLoggerError};

struct StdoutLogger;
impl log::Log for StdoutLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Debug
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let ts = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
            println!(
                "{:0>15} {:>6} {:?}:{:?} - {}",
                ts.as_nanos(),
                record.level(),
                record.module_path(),
                record.line(),
                record.args()
            );
        }
    }

    fn flush(&self) {}
}

pub fn init() -> Result<(), SetLoggerError> {
    log::set_logger(&StdoutLogger)
}
