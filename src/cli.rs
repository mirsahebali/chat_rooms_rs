use room_rs::{accept_user_input, ADDR};
use std::{
    io::{self, Read, Write},
    net::TcpStream,
    sync::Arc,
    thread,
};
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
    println!("Message: {buf_str}");
    Ok(())
}
fn write_to_server(mut stream: &TcpStream) -> io::Result<()> {
    let buf = accept_user_input("Enter message to send to server");
    stream
        .write_all(buf.as_bytes())
        .map_err(|err| panic!("ERROR: writing to server,{err}"))
        .unwrap();
    Ok(())
}
fn main() -> io::Result<()> {
    let stream = TcpStream::connect(ADDR)
        .map_err(|err| {
            panic!("ERROR: connecting to stream, {err}");
        })
        .unwrap();
    let stream_arc = Arc::new(stream);
    let stream = Arc::clone(&stream_arc);
    thread::spawn(move || loop {
        read_from_server(&stream)
            .map_err(|err| {
                panic!("ERROR: reading from server, {err}");
            })
            .unwrap();
    });

    let stream = Arc::clone(&stream_arc);
    loop {
        write_to_server(&stream)
            .map_err(|err| {
                panic!("ERROR: writing to server, {err}");
            })
            .unwrap();
    }
}
