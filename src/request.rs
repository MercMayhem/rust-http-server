use httparse::{parse_headers, Header, Status};
use std::path::{Path, PathBuf};

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
    pub body: &'b str,
}

impl<'b, 'h> HttpRequest<'b, 'h> {
    pub fn new(
        buf: &'b [u8],
        header_arr: &'h mut [Header<'b>],
    ) -> Result<HttpRequest<'b, 'h>, &'static str> {
        let message = buf;

        // Find end index for request type
        let request_type_end: usize = String::from_utf8(message.to_vec())
            .expect("dead")
            .find(' ')
            .unwrap();
        let request_type: RequestType;

        // Parse request_type
        request_type = match String::from_utf8(message[..request_type_end].to_vec())
            .unwrap()
            .as_str()
        {
            "GET" => RequestType::GET,
            "POST" => RequestType::POST,
            "PUT" => RequestType::PUT,
            "DELETE" => RequestType::DELETE,
            _ => panic!("Unsupported request type"),
        };

        // Find start and end index of resource
        let resource_start = request_type_end + 2;
        let resource_end = String::from_utf8(message[resource_start..].to_vec())
            .expect("dead")
            .find(' ')
            .unwrap()
            + resource_start;

        // Find resource requested
        let path_buf = PathBuf::from(
            &String::from_utf8(message[resource_start..resource_end].to_vec()).unwrap(),
        );
        let resource = path_buf.into_boxed_path();

        // Find start of headers
        let header_start: usize = String::from_utf8(message.to_vec())
            .expect("dead")
            .find('\n')
            .unwrap()
            + 1;

        // Parsing headers
        let parsed_status: Status<(usize, &[Header])> =
            parse_headers(&message[header_start..], header_arr).unwrap();

        let headers: &[Header];
        match parsed_status {
            httparse::Status::Complete(head) => {
                headers = head.1;
            }
            httparse::Status::Partial => return Err("Partial parsing performed"),
        };

        // Parsing body
        let body: &str =
            std::str::from_utf8(&message[HttpRequest::find_body_pos(message).unwrap()..]).unwrap();

        return Ok(HttpRequest {
            message,
            resource,
            request_type,
            headers,
            body,
        });
    }

    pub fn find_body_pos(message: &[u8]) -> Option<usize> {
        for (i, ch) in message.iter().rev().enumerate() {
            if *ch == ('\n' as u8) {
                if message[message.len() - i - 2] == ('\r' as u8) {
                    return Some(message.len() - i);
                }
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::HttpRequest;

    #[test]
    fn test_find_body_pos() {
        let test_str = "POST /path/to/resource HTTP/1.1\r\nHost: example.com\r\nContent-Length: 26\r\nContent-Type: application/json\r\n\r\n{\"key\":\"value\"}";
        let message: &[u8] = test_str.as_bytes();

        let body_start = HttpRequest::find_body_pos(message).unwrap();
        assert_eq!(
            String::from_utf8(message[body_start..].to_vec()).unwrap(),
            "{\"key\":\"value\"}"
        )
    }
}
