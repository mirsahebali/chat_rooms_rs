use room_rs::ADDR;
use std::{
    io::{self, Read},
    net::{TcpListener, TcpStream},
    thread,
};

#[derive(Debug)]
struct Server {
    listener: TcpListener,
}

impl Server {
    fn start() -> Self {
        Self {
            listener: TcpListener::bind(ADDR)
                .map_err(|err| panic!("ERROR: listening on address, {ADDR}, \n {err}"))
                .unwrap(),
        }
    }
    fn listen_stream(&self) -> io::Result<()> {
        for stream in self.listener.incoming() {
            match stream {
                Ok(stream) => {
                    println!(
                        "ACCEPTING connections from: {}",
                        stream.peer_addr().unwrap()
                    );
                    thread::spawn(move || loop {
                        read_from_server(&stream)
                            .map_err(|err| panic!("ERROR, reading from server, {err}"))
                            .unwrap();
                    });
                }
                Err(err) => {
                    eprintln!("ERROR: accepting connections, {err}");
                }
            }
        }
        Ok(())
    }
}

fn read_from_server(mut stream: &TcpStream) -> io::Result<()> {
    let mut buf = [0_u8; 512];
    stream
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

fn main() -> io::Result<()> {
    let server = Server::start();
    server.listen_stream()?;
    Ok(())
}
