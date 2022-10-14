use crossterm::ExecutableCommand;
use std::io::stdout;

/// A modifiable cursor.
pub struct Cursor;
impl Cursor {
    /// Hides or shows the cursor
    pub fn set_visible(visible: bool) -> crossterm::Result<()> {
        match visible {
            true => stdout().execute(crossterm::cursor::Show)?,
            false => stdout().execute(crossterm::cursor::Hide)?,
        };
        Ok(())
    }

    /// Moves the cursor left.
    pub fn move_right(n: u16) -> crossterm::Result<()> {
        stdout().execute(crossterm::cursor::MoveRight(n))?;
        Ok(())
    }

    /// Moves the cursor left.
    pub fn move_left(n: u16) -> crossterm::Result<()> {
        stdout().execute(crossterm::cursor::MoveLeft(n))?;
        Ok(())
    }

    /// Moves the cursor down.
    pub fn move_down(n: u16) -> crossterm::Result<()> {
        stdout().execute(crossterm::cursor::MoveDown(n))?;
        Ok(())
    }

    /// Moves the cursor up.
    pub fn move_up(n: u16) -> crossterm::Result<()> {
        stdout().execute(crossterm::cursor::MoveUp(n))?;
        Ok(())
    }
}
