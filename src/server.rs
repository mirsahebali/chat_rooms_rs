#![allow(unused)]

use room_rs::{Client, ADDR, PORT};
use std::{
    env,
    io::{self, Read},
    net::{TcpListener, TcpStream},
    sync::Arc,
    thread,
};

#[derive(Debug)]
struct Server {
    listener: TcpListener,
    clients: Vec<Arc<TcpStream>>,
}

impl Server {
    fn start() -> Self {
        let port = env::var("PORT").unwrap_or_else(|_| PORT.to_string());
        Self {
            listener: TcpListener::bind(format!("{ADDR}:{port}").to_string())
                .map_err(|err| panic!("ERROR: listening on address, {ADDR}:{port}, \n {err}"))
                .unwrap(),
            clients: Vec::new(),
        }
    }
    fn start_listening(&mut self) -> io::Result<()> {
        for stream in self.listener.incoming() {
            match stream {
                Ok(stream) => {
                    println!(
                        "ACCEPTING connections from: {}",
                        stream.peer_addr().unwrap()
                    );
                    let stream = Arc::new(stream);

                    // INFO: Clone the stream and push in the clients vector
                    self.clients.push(stream.clone());

                    // INFO: Cloning the stream for "Reading" from client
                    let stream_read = Arc::clone(&stream);
                    thread::spawn(move || loop {
                        read_from_server(&stream_read)
                            .map_err(|err| panic!("ERROR, reading from server, {err}"))
                            .unwrap();
                    });

                    // let mut stream_write = Arc::clone(&stream);
                    // thread::spawn(move || loop {
                    //     Self::write_to_clients(&mut stream_write)
                    //         .map_err(|err| panic!("ERROR, writing to clients, {err}"))
                    //         .unwrap();
                    // });
                }
                Err(err) => {
                    eprintln!("ERROR: accepting connections, {err}");
                }
            }
        }
        Ok(())
    }

    pub fn write_to_all_clients(&mut self) -> io::Result<()> {
        for clients in self.clients.iter_mut() {}
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
    let mut server = Server::start();
    server.start_listening()?;
    Ok(())
}
