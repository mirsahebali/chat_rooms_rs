#![allow(dead_code)]
#![allow(unused)]

mod screen;
mod utils;
use crate::screen::Screen;
use crate::utils::move_cursor_below_msg_line;
use std::{
    fs,
    io::{self, stdout},
    time::Duration,
};

use crossterm::{
    cursor::MoveTo,
    event::{poll, KeyEvent},
    event::{read, Event, KeyCode},
    style::Print,
    terminal::{Clear, ClearType},
    ExecutableCommand, QueueableCommand,
};

fn main() -> io::Result<()> {
    let mut stdout = stdout();
    let mut screen = Screen::new();
    let mut buffer = String::new();
    let mut messages = Vec::new();
    loop {
        while poll(Duration::ZERO)? {
            stdout.queue(Clear(ClearType::FromCursorUp))?;
            screen.print_border(&stdout)?;
            screen.print_message_upper_line(&stdout)?;
            utils::move_cursor_below_msg_line(&stdout, screen.height)?;
            match read()? {
                Event::Key(key) => {
                    match key.code {
                        KeyCode::Char(ch) => {
                            buffer.push(ch);
                        }
                        KeyCode::Enter => {
                            messages.push(buffer.clone());
                            for (index, msgs) in messages.iter().rev().enumerate() {
                                stdout
                                    .execute(MoveTo(2, screen.height - (4 + index) as u16))?
                                    .execute(Print(msgs))?;
                            }
                            buffer.clear();
                            utils::move_cursor_below_msg_line(&stdout, screen.height)?;
                        }

                        _ => {}
                    };
                }
                Event::Resize(w, h) => {
                    screen.width = w;
                    screen.height = h;
                }
                _ => {}
            }
        }
    }
}
