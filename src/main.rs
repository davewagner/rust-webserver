use std::net::TcpListener;

fn main() {
    // Bind the TCP listener to the address and port
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        println!("Connection established from: {}", stream.peer_addr().unwrap());

    }
}