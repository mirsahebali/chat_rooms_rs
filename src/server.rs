use room_rs::{ADDR, PORT};
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
}

impl Server {
    fn start() -> Self {
        let port = env::var("PORT").unwrap_or_else(|_| PORT.to_string());
        Self {
            listener: TcpListener::bind(format!("{ADDR}:{port}").to_string())
                .map_err(|err| panic!("ERROR: listening on address, {ADDR}:{port}, \n {err}"))
                .unwrap(),
        }
    }
    fn start_listening(&self) -> io::Result<()> {
        for stream in self.listener.incoming() {
            match stream {
                Ok(stream) => {
                    println!(
                        "ACCEPTING connections from: {}",
                        stream.peer_addr().unwrap()
                    );
                    let stream = Arc::new(stream);
                    let stream_read = Arc::clone(&stream);
                    thread::spawn(move || loop {
                        read_from_server(&stream_read)
                            .map_err(|err| panic!("ERROR, reading from server, {err}"))
                            .unwrap();
                    });

                    // let stream_write = Arc::clone(&stream);
                    // thread::spawn(move || loop {
                    //     write_to_clients(&stream_write)
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

fn write_to_clients(mut stream: &TcpStream) -> io::Result<()> {
    Ok(())
}

fn main() -> io::Result<()> {
    let server = Server::start();
    server.start_listening()?;
    Ok(())
}
