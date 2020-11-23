use super::print::Print;
use std::io;

pub struct SimplePrinter<'a> {
    std_writer: &'a mut dyn io::Write,
    err_writer: &'a mut dyn io::Write,
}

impl<'a> SimplePrinter<'a> {
    pub fn new(std: &'a mut dyn io::Write, err: &'a mut dyn io::Write) -> Self {
        SimplePrinter {
            std_writer: std,
            err_writer: err,
        }
    }
}

impl<'a> Print for SimplePrinter<'a> {
    fn print(&mut self, result: &Result<String, Box<dyn std::error::Error>>) -> io::Result<()> {
        match result {
            Ok(str) => writeln!(self.std_writer, "{}", str),
            Err(e) => writeln!(self.err_writer, "{}", e),
        }
    }
}
