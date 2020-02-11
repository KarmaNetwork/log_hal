#![no_std]

use core::panic::PanicInfo;
use core::fmt::{ Arguments };

pub enum Level {
    Panic,
    Error,
    Warn,
    Info,
    Debug,
}

pub trait Log: Default {
    /// Log message
    fn log(&self, level: Level, args: &Arguments);

    fn tag(&mut self, t: &'static str);

    fn debug(&self, args: &Arguments) {
        self.log(Level::Debug, &format_args!("D: {}", args));
    }

    fn info(&self, args: &Arguments) {
        self.log(Level::Info, &format_args!("I: {}", args));
    }

    fn warn(&self, args: &Arguments) {
        self.log(Level::Warn, &format_args!("W: {}", args));
    }

    fn error(&self, args: &Arguments) {
        self.log(Level::Error, &format_args!("E: {}", args));
    }

    fn panic(&self, info: &PanicInfo) {
        if let Some(location) = info.location() {
            let payload = info.payload().downcast_ref::<&str>().unwrap();
            self.log(Level::Panic, &format_args!("P: {} \n\t at {} :{}:{}", 
                                                 payload,
                                                 location.file(),
                                                 location.line(),
                                                 location.column()));
        }
    }

}

