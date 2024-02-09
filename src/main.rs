use std::net::{TcpListener, TcpStream};

fn handle_connection(stream: TcpStream) {
    match stream.peer_addr(){
        Ok(addr) => println!("connection with address: {:?}", addr),
        Err(e) => println!("Error: {:?}", e)
    }
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:80")?;

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_connection(stream);
            }
            Err(e) => println!("Error: {:?}", e)
        }
    }
    Ok(())
}

