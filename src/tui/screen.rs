use std::{
    io::{self, stdout, Stdout, Write},
    thread,
    time::Duration,
};

use crossterm::{
    cursor::{MoveLeft, MoveRight, MoveTo, MoveToNextLine},
    event::{self, read, EnableMouseCapture, Event, KeyCode, KeyEvent, ModifierKeyCode},
    event::{poll, MouseEvent},
    style::{Print, Stylize},
    terminal::{self, Clear, ClearType},
    ExecutableCommand, QueueableCommand,
};

const LINE: &str = "═";

//
#[derive(Debug)]
pub struct Screen<'a> {
    pub width: u16,
    pub height: u16,
    pub x_border: &'a str,
    pub y_border: &'a str,
}

impl<'a> Screen<'a> {
    pub fn new() -> Self {
        let (mut width, mut height) = terminal::size()
            .map_err(|err| eprintln!("ERROR: getting the terminal size: {err}"))
            .unwrap();
        Self {
            width,
            height,
            x_border: "━",
            y_border: "┃",
        }
    }
    pub fn print_border(&self, mut stdout: &Stdout) -> io::Result<()> {
        for i in 0..self.width {
            stdout
                .execute(MoveTo(i, 0))?
                .execute(Print(self.x_border.magenta()))?;
        }
        for i in 0..self.width {
            stdout
                .execute(MoveTo(i, self.height))?
                .execute(Print(self.x_border.magenta()))?;
        }
        for j in 0..self.height {
            stdout
                .execute(MoveTo(0, j))?
                .execute(Print(self.y_border.magenta()))?;
        }

        for j in 0..self.height {
            stdout
                .execute(MoveTo(self.width, j))?
                .execute(Print(self.y_border.magenta()))?;
        }
        stdout
            .execute(MoveTo(0, 0))?
            .execute(Print("┏".magenta()))?
            .execute(MoveTo(self.width, 0))?
            .execute(Print("┓".magenta()))?
            .execute(MoveTo(0, self.height))?
            .execute(Print("┗".magenta()))?
            .execute(MoveTo(self.width, self.height))?
            .execute(Print("┛".magenta()))?;
        Ok(())
    }

    pub fn print_message_upper_line(&self, mut stdout: &Stdout) -> io::Result<()> {
        stdout
            .execute(MoveTo(1, self.height - 3))?
            .execute(Print(LINE.repeat(self.width as usize - 2).cyan()))?;

        stdout.execute(MoveToNextLine(1))?;
        stdout.execute(MoveRight(2));
        Ok(())
    }
}
