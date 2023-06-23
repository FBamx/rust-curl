use super::constants;
use std::collections::HashMap;

use url::{self, Url};

#[derive(Debug)]
pub struct Request {
    pub method: String,
    pub uri: String,
    pub version: String,
    pub headers: HashMap<String, String>,
    pub body: Option<Vec<u8>>,
}

impl Request {
    pub fn new(url: &str) -> Self {
        let parsed_url = url.parse::<Url>().unwrap();
        let host = parsed_url.host().unwrap().to_string();
        let user_agent = constants::user_agent();
        let mut headers = HashMap::new();
        headers.insert("Host".to_owned(), host);
        headers.insert("Accept".to_owned(), "*/*".to_owned());
        headers.insert("User-Agent".to_owned(), user_agent);
        Request {
            method: "GET".to_owned(),
            uri: parsed_url.path().to_owned(),
            version: constants::HTTP_VERSION.to_owned(),
            headers,
            body: None,
        }
    }

    pub fn to_string(&self) -> String {
        let request_line = format!("{} {} {}", self.method, self.uri, self.version);
        let mut headers = String::from("");
        for (k, v) in self.headers.iter() {
            headers = format!("{}{}: {}\r\n", headers, k.to_string(), v.to_string());
        }
        format!("{}\r\n{}\r\n", request_line, headers)
    }
}
