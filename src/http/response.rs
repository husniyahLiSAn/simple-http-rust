use std::{fmt::Display, io};

use super::request::Version;
use super::request::HttpRequest;

#[derive(Debug)]
pub struct HttpResponse {
    version: Version,
    status: ResponseStatus,
    content_length: usize,
    accept_ranges: AcceptRanges,
    pub response_body: String,
    pub current_path: String
}

impl HttpResponse {
    pub fn new(request: &HttpRequest) -> io::Result<HttpResponse> {
        let version: Version = Version::V2_0;
        let mut status: ResponseStatus = ResponseStatus::NotFound;
        let mut content_length: usize = 0;
        let mut accept_ranges: AcceptRanges = AcceptRanges::None;
        let current_path = request.resource.path.clone();
        let mut response_body = String::new();

        let server_root_path = std::env::current_dir()?;
        let resource = request.resource.path.clone();
        let new_path =  server_root_path.join(resource);
        if new_path.exists() {
            if new_path.is_file() {
                let content = std::fs::read_to_string(&new_path)?;
                response_body.push_str(&content);
                content_length = content.len();
                status = ResponseStatus::OK;
                accept_ranges = AcceptRanges::Bytes;
            } else {
                let four_o_four = "<html>
                <body>
                <h1>404 NOT FOUND</h1>
                </body>
                </html>";
                content_length = four_o_four.len();
                
                let content = format!(
                    "{} {}\n{}\ncontent-length: {}\r\n\r\n{}", version, status, accept_ranges, content_length, four_o_four);

                response_body.push_str(&content);
            }
        }
        Ok(HttpResponse {
            version, status, content_length, accept_ranges, response_body, current_path
        })
    }
}

#[derive(Debug)]
enum ResponseStatus {
    OK = 200,
    NotFound = 404
}

impl Display for ResponseStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt:: Result {
        let msg = match self {
            ResponseStatus::OK => "200 OK",
            ResponseStatus::NotFound => "404 NOT FOUND"
        };
        write!(f, "{}", msg)
    }
}

#[derive(Debug)]
enum AcceptRanges {
    Bytes,
    None
}

impl Display for AcceptRanges {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt:: Result {
        let msg = match self {
            AcceptRanges::Bytes => "accept-ranges: bytes",
            AcceptRanges::None => "accept-ranges: none"
        };
        write!(f, "{}", msg)
    }
}