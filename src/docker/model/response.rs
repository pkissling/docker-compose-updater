use std::collections::HashMap;
use std::fmt;

pub struct Response {
    status_code: i16,
    #[allow(dead_code)]
    headers: HashMap<String, String>,
    #[allow(dead_code)]
    body: String,
}

impl fmt::Display for Response {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Status [{}]\nBody:\n{}", self.status_code, self.body)
    }
}

impl Response {
    pub fn new(response: String) -> Self {
        let mut lines = response.lines();

        // status line
        let status_line = lines.next().unwrap();
        let status_code = extract_status_code(status_line).unwrap();

        // header
        let response_headers = lines.by_ref()
            .take_while(|&line| !line.is_empty())
            .collect::<Vec<&str>>();
        let headers = parse_headers(response_headers);

        // body
        let mut body = String::new();
        for l in lines.into_iter() {
            body = body + l;
        }

        Response {
            status_code,
            headers,
            body
        }
    }

    pub fn _is_success(&self) -> bool {
        return self.status_code < 300
    }
}

fn parse_headers(headers: Vec<&str>) -> HashMap<String, String>{
    if headers.len() == 0 {
        return HashMap::new()
    }


    headers.iter()
        .map(|&h| h.split(": "))
        .map(|mut parts| (parts.next().unwrap(), parts.next().unwrap()))
        .map(|t| (String::from(t.0), String::from(t.1)))
        .collect()
}

fn extract_status_code(status_line: &str) -> Option<i16> {
    let parts = status_line.split_whitespace();
    let status_code = parts.skip(1).next().unwrap();

    if !status_code.chars().all(char::is_numeric) {
        return None;
    }

    if !status_code.len() == 3 {
        return None;
    }

    Some(status_code.parse().unwrap())
}