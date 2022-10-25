use std::{
    io::{Read, Write},
    net::TcpStream,
    str,
};

fn main() {
    let mut stream = TcpStream::connect("localhost:3000").unwrap();
    stream.write_all(b"Hello world").unwrap();
    stream.flush();

    let mut buffer = [0; 5];
    stream.read(&mut buffer).unwrap();

    println!("Got response: {:?}", str::from_utf8(&buffer).unwrap())
}
