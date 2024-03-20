use std::io;

pub const ADDR: &str = "127.0.0.1:8000";

pub fn accept_user_input(input_msg: &str) -> String {
    let mut buf = String::new();
    println!("{input_msg}");
    io::stdin()
        .read_line(&mut buf)
        .map_err(|err| eprintln!("ERROR: accpting user input, {err}"))
        .unwrap();
    buf
}
