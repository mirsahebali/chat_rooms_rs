#![allow(unused)]
use std::{
    io::{self, Read},
    net::{SocketAddr, TcpStream},
    sync::Arc,
    time,
};

pub const ADDR: &str = "127.0.0.1";
pub const PORT: &str = "8000";
pub fn accept_user_input(input_msg: &str) -> String {
    let mut buf = String::new();
    println!("{input_msg}");
    io::stdin()
        .read_line(&mut buf)
        .map_err(|err| eprintln!("ERROR: accpting user input, {err}"))
        .unwrap();
    buf
}
// Single Chat primitive
#[derive(Debug)]
pub struct Message {
    pub message: String,
    pub time: time::Instant,
}

#[derive(Debug)]
pub struct Client<'a> {
    // Local address of the current client
    pub addr: SocketAddr,
    pub messages: Vec<Message>,
    // Local address of the connected Server
    pub connected_to: &'a TcpStream,
}

impl<'a> Client<'a> {
    pub fn read_from_server(&mut self) -> io::Result<()> {
        let mut buf = [0_u8; 512];
        self.connected_to
            .read(&mut buf)
            .map_err(|err| panic!("ERROR cloning stream, {err}"))
            .unwrap();
        let mut buf_str = String::new();
        for b in buf {
            if b == 0 {
                break;
            }
            buf_str.push(b as char);
        }
        println!("{buf_str}");
        Ok(())
    }
    pub fn write_to_server() {}
}
