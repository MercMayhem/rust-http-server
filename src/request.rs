use std::path::Path;

use httparse::{parse_headers, Header, Status, EMPTY_HEADER};

pub enum RequestType {
    GET,
    POST,
    PUT,
    DELETE,
}

pub struct HttpRequest<'b: 'h, 'h> {
    pub message: &'b [u8],
    // pub request_type: RequestType,
    // pub resource: Box<Path>,
    pub headers: &'h [Header<'b>],
    // pub body: & 'b str
}

impl<'b, 'h> HttpRequest<'b, 'h> {
    pub fn new(
        buf: &'b [u8],
        header_arr: &'h mut [Header<'b>],
    ) -> Result<HttpRequest<'b, 'h>, &'static str> {
        let message = buf;

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

        return Ok(HttpRequest { message, headers });
    }
}
