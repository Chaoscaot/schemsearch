use std::fmt::Debug;
use std::io::Write;
use indicatif::TermLike;

#[derive(Debug)]
pub struct MaschineStdErr { pub(crate) size: u16}

impl TermLike for MaschineStdErr {
    fn width(&self) -> u16 {
        self.size
    }

    fn move_cursor_up(&self, _: usize) -> std::io::Result<()> {
        Ok(())
    }

    fn move_cursor_down(&self, _: usize) -> std::io::Result<()> {
        Ok(())
    }

    fn move_cursor_right(&self, _: usize) -> std::io::Result<()> {
        Ok(())
    }

    fn move_cursor_left(&self, _: usize) -> std::io::Result<()> {
        Ok(())
    }

    fn write_line(&self, s: &str) -> std::io::Result<()> {
        writeln!(std::io::stderr(), "{}", s)
    }

    fn write_str(&self, s: &str) -> std::io::Result<()> {
        write!(std::io::stderr(), "{}", s)
    }

    fn clear_line(&self) -> std::io::Result<()> {
        Ok(())
    }

    fn flush(&self) -> std::io::Result<()> {
        std::io::stderr().flush()
    }
}