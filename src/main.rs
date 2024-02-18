#![recursion_limit = "512"]

mod html;
mod request;
mod response;
mod util;

use httparse::EMPTY_HEADER;
use request::HttpRequest;
use response::HttpResponse;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::time::Duration;
use std::{env, fs};
use util::{get_files_and_dirs, get_html_files, get_index_file};

fn process_request(request: &HttpRequest) -> Option<String> {
    match request.request_type {
        request::RequestType::GET => {
            let curr_dir = env::current_dir().unwrap().into_boxed_path();
            let body: String;
            let dir_resource = curr_dir.join(&request.resource);

            if dir_resource.try_exists().unwrap() {
                println!("Resource {:?} exists", dir_resource);
                if dir_resource.is_dir() {
                    let html_files = get_html_files(&dir_resource).unwrap();
                    let index_file = get_index_file(html_files.clone());

                    if index_file == None {
                        body = html::diplay_directory_listing(&dir_resource);
                    } else {
                        let index = fs::read(index_file.unwrap()).unwrap();
                        body = String::from_utf8(index).unwrap();
                    }

                    let response = HttpResponse::from_html_body(&body);
                    println!("{}", response.to_string());

                    return Some(response.to_string());
                }
            } else {
                println!("Resource {:?} doesn't exist", dir_resource)
            };
        }
        _ => println!("Implement this later"),
    }

    None
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
    let response = process_request(&request).unwrap_or("".to_string());

    let _ = stream.write_all(response.as_bytes());
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
