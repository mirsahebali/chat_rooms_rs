use std::io::{self, Stdout};

use crossterm::{
    cursor::{MoveTo, MoveToColumn, MoveToRow},
    ExecutableCommand,
};

pub fn move_cursor_below_msg_line(mut stdout: &Stdout, height: u16) -> io::Result<()> {
    stdout.execute(MoveTo(2, height - 2))?;
    Ok(())
}
