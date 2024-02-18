#![recursion_limit = "512"]

mod request;
mod html;
mod util;

use std::env;
use util::{get_html_files, get_index_file, get_files_and_dirs};
use httparse::EMPTY_HEADER;
use request::HttpRequest;
use std::io::Read;
use std::net::{TcpListener, TcpStream};
use std::time::Duration;


fn process_request(request: &HttpRequest){
    match request.request_type {
        request::RequestType::GET => {
            let curr_dir = env::current_dir().unwrap().into_boxed_path();

            let dir_resource = curr_dir.join(&request.resource);

            if dir_resource.try_exists().unwrap() {
                println!("Resource {:?} exists", dir_resource);
                if dir_resource.is_dir(){
                    let html_files = get_html_files(&dir_resource).unwrap();
                    let index_files = get_index_file(html_files.clone());
                    
                    if index_files == None{
                        let body = html::diplay_directory_listing(&dir_resource);
                        println!("{}", body);
                    } else {
                        todo!()
                    }
                }
            }
            else {println!("Resource {:?} doesn't exist", dir_resource)};
        },
        _ => println!("Implement this later")
    }
}

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
    process_request(&request)
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
