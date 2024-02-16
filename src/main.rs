mod request;
use httparse::EMPTY_HEADER;
use request::HttpRequest;
use std::io::Read;
use std::net::{TcpListener, TcpStream};
use std::time::Duration;

fn handle_connection(mut stream: TcpStream) {
    match stream.peer_addr() {
        Ok(addr) => println!("connection with address: {:?}", addr),
        Err(e) => println!("Error: {:?}", e),
    };
    let _ = stream.set_read_timeout(Some(Duration::new(5, 0)));
    let mut buf: [u8; 4096] = [0; 4096];

    let _bytes: usize = stream.read(&mut buf).unwrap();
    let mut header_arr = [EMPTY_HEADER; 64];

    let request = HttpRequest::new(&buf, header_arr.as_mut_slice()).unwrap();
    
    println!("Type: {:?}", request.request_type);
    println!("Header: {:?}", request.headers);
    println!("Resource: {:?}", request.resource);
    println!("Body: {}", request.body)
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:80")?;

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_connection(stream);
            }
            Err(e) => println!("Error: {:?}", e),
        }
    }
    Ok(())
}
