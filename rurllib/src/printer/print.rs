use std::io;

pub trait Print {
    fn print(&mut self, result: &Result<String, Box<dyn std::error::Error>>) -> io::Result<()>;
}
