use core::fmt::{self, Write};

use limine_protocol::structures::{Terminal, TerminalWriteFn};

pub struct TerminalWriter {
    terminal: &'static mut Terminal,
    write: TerminalWriteFn,
}

impl TerminalWriter {
    /// Create a new terminal writer from the request
    pub fn new() -> TerminalWriter {
        // Our terminal request
        use crate::TERMINAL;

        // Get the terminal response
        let res = unsafe { TERMINAL.response.unwrap().as_ref() };

        // The very first terminal
        let first_term = unsafe {
            res.get_terminals_mut()
                .unwrap() // Get a mutable slice of the terminals, we must hope the bootloader did it right, which is why this is unsafe
                .first_mut() // Get the first item
                .unwrap() // It should exist, if not panic
        };

        let term_write = res.write;

        Self {
            terminal: *first_term,
            write: term_write,
        }
    }

    /// Put it in the `TERM_WRITER` static
    pub fn init() {
        unsafe { TERM_WRITER = Some(Self::new()) }
    }
}

impl Write for TerminalWriter {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        (self.write)(self.terminal, s.as_ptr(), s.len() as u64);
        Ok(())
    }
}

pub static mut TERM_WRITER: Option<TerminalWriter> = None;

macro_rules! print {
    ($($arg:tt)*) => (#[allow(unused_unsafe)] unsafe { $crate::writer::TERM_WRITER.as_mut().unwrap().write_fmt(format_args!($($arg)*)).unwrap() });
}

pub(crate) use print;

macro_rules! println {
    () => (print!("\n"));
    ($($arg:tt)*) => (print!("{}\n", format_args!($($arg)*)));
}

pub(crate) use println;
