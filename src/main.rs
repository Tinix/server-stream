use std::net::{TcpListener, TcpStream};
use std::io::prelude::*;

fn main() {
    let address = "127.0.0.1:8000";
    let listener = TcpListener::bind(&address).unwrap();
    println!("Server started {}", &address);
   

    // listening connections
    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream); 
    }
}

// manage connections
fn handle_connection(mut stream: TcpStream){
  let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();

    println!("Stream received!");
    println!("{}", String::from_utf8_lossy(&buffer[..]));
}
