#![allow(dead_code)]
#![allow(unused)]

mod chat_box;
mod screen;

use crate::screen::Screen;
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

fn main() -> io::Result<()> {
    let mut stdout = stdout();
    let mut screen = Screen::new();
    let mut curr_message_buffer = String::new();
    loop {
        while poll(Duration::ZERO)? {
            stdout.queue(Clear(ClearType::FromCursorUp))?;
            screen.print_border(&stdout)?;
            screen.print_message_upper_line(&stdout)?;
            match read()? {
                Event::Key(key) => match key.code {
                    KeyCode::Char(ch) => {
                        curr_message_buffer.push(ch);
                    }
                    KeyCode::Enter => {}
                    _ => {}
                },
                Event::Resize(cols, rows) => {
                    screen.width = cols;
                    screen.height = rows;
                }
                _ => {}
            }
        }
    }
}
