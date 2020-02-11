extern crate log_hal;

use log_hal::Log;
use log_hal::Level;
use core::fmt::Arguments;

struct LogStd {
    tag: &'static str,
}

impl Log for LogStd {
    fn tag(&mut self, t: &'static str) {
        self.tag = t;
    }

    fn log(&self, level: Level, args: Arguments) {
        println!("{}", args);
    }
}

impl Default for LogStd {
    fn default() -> Self {
        LogStd {
            tag: ""
        }
    }
}

#[test]
fn test() {
    let logger = LogStd::default();
    logger.debug(format_args!("{}", "hello"));
}

