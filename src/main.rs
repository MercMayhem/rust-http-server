mod request;
use httparse::{parse_headers, EMPTY_HEADER};
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

    let bytes: usize = stream.read(&mut buf).unwrap();

    let message: String = String::from_utf8(buf[..bytes].to_vec()).unwrap();
    let request_type_end: usize = message
        .find(' ')
        .unwrap();
    let header_start: usize = message.find('\n').unwrap() + 1;
    let mut headers = [EMPTY_HEADER; 20];

    let parsed_headers = parse_headers(message[header_start..].as_bytes(), &mut headers).unwrap();

    println!("data: {}", message);
    println!("request type: {}", message[..request_type_end].to_string());
    println!("header start: {:?}", header_start);
    println!("header: \n{}", message[header_start..].to_string());
    println!("{:?}", parsed_headers);
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
