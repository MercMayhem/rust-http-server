use std::path::{Path, PathBuf};

use httparse::{parse_headers, Header, Status};

#[derive(Debug)]
pub enum RequestType {
    GET,
    POST,
    PUT,
    DELETE,
}

pub struct HttpRequest<'b: 'h, 'h> {
    pub message: &'b [u8],
    pub request_type: RequestType,
    pub resource: Box<Path>,
    pub headers: &'h [Header<'b>],
    // pub body: & 'b str
}

impl<'b, 'h> HttpRequest<'b, 'h> {
    pub fn new(
        buf: &'b [u8],
        header_arr: &'h mut [Header<'b>],
    ) -> Result<HttpRequest<'b, 'h>, &'static str> {
        let message = buf;

        let request_type_end: usize = String::from_utf8(message.to_vec())
            .expect("dead")
            .find(' ')
            .unwrap();
        let request_type: RequestType;

        request_type = match String::from_utf8(message[..request_type_end].to_vec()).unwrap().as_str(){
            "GET" =>  RequestType::GET,
            "POST" => RequestType::POST,
            "PUT" => RequestType::PUT,
            "DELETE" => RequestType::DELETE,
            _ => panic!("Unsupported request type")
        };

        let resource_start = request_type_end + 1;
        let resource_end = String::from_utf8(message[resource_start..].to_vec())
            .expect("dead")
            .find(' ')
            .unwrap() + resource_start;
        
        let path_buf = PathBuf::from(&String::from_utf8(message[resource_start..resource_end].to_vec()).unwrap());
        let resource = path_buf.into_boxed_path();

        let header_start: usize = String::from_utf8(message.to_vec())
            .expect("dead")
            .find('\n')
            .unwrap()
            + 1;

        let parsed_status: Status<(usize, &[Header])> =
            parse_headers(&message[header_start..], header_arr).unwrap();

        let headers: &[Header];
        match parsed_status {
            httparse::Status::Complete(head) => {
                headers = head.1;
            }
            httparse::Status::Partial => return Err("Partial parsing performed"),
        };

        return Ok(HttpRequest { message, request_type, resource, headers });
    }
}
