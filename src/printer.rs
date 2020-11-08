use std::io;

pub fn print(
    result: Result<String, Box<dyn std::error::Error>>,
    mut writer: impl std::io::Write,
    mut err_writer: impl std::io::Write,
) -> io::Result<()> {
    match result {
        Ok(str) => writeln!(writer, "{}", str),
        Err(e) => writeln!(err_writer, "{}", e),
    }
}
