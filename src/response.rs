use std::collections::HashMap;

use chrono::Local;

pub struct HttpResponse<'a> {
    status: u8,
    headers: HashMap<String, String>,
    body: &'a str,
}

impl<'a> HttpResponse<'a> {
    pub fn from_html_body(html: &'a str) -> HttpResponse<'a> {
        let status = 200;
        let mut headers = HashMap::new();

        headers.insert(
            "Content-Length".to_string(),
            format!("{}", html.bytes().len()),
        );
        headers.insert(
            "Content-Type".to_string(),
            "text/html; charset=utf-8".to_string(),
        );
        headers.insert("Server".to_string(), "Amans-Rust-Server/0.1".to_string());
        headers.insert("Date".to_string(), Local::now().to_rfc2822().to_string());

        let body = html;

        return HttpResponse {
            status,
            headers,
            body,
        };
    }

    pub fn from_file(file_bytes: &'a str) -> HttpResponse<'a> {
        let status = 200;
        let mut headers = HashMap::new();

        headers.insert(
            "Content-Length".to_string(),
            format!("{}", file_bytes.len()),
        );

        headers.insert(
            "Content-Type".to_string(),
            "application/octet-stream".to_string(),
        );

        headers.insert("Date".to_string(), Local::now().to_rfc2822().to_string());
        headers.insert("Server".to_string(), "Amans-Rust-Server/0.1".to_string());

        return HttpResponse {
            status,
            headers,
            body: file_bytes,
        };
    }

    fn push_headers(&self, ret: &mut String) {
        for (key, value) in self.headers.iter() {
            ret.push_str(format!("{}: {}\r\n", key.as_str(), value.as_str()).as_str());
        }
        ret.push_str("\r\n")
    }
}

impl<'a> ToString for HttpResponse<'a> {
    fn to_string(&self) -> String {
        let mut ret = String::new();

        ret.push_str("HTTP/1.1 ");
        ret.push_str(format!("{} ", self.status).as_str());

        match self.status {
            200 => ret.push_str("OK"),
            _ => println!("We'll cross this bridge when we get to it"),
        }

        ret.push_str("\r\n");

        self.push_headers(&mut ret);
        ret.push_str(self.body);

        return ret;
    }
}
