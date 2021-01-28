use std::collections::HashMap;

pub struct RequestBuilder {
    operation: String, // TODO enum
    path: Option<String>, // TODO to &str?
    format: Option<String>, // TODO to &str?
    headers: HashMap<String, String>,
    http_version: String
}

pub struct Request {
    operation: String,
    path: String,
    format: String,
    headers: HashMap<String, String>,
    http_version: String
}

impl ToString for Request {

    fn to_string(&self) -> String {
        let formatted_headers = self.headers.iter()
            .map(|(k,v)| format!("{}: {}\n", k, v))
            .collect::<Vec<String>>()
            .join("\n");

        format!("{} {}{} {}\n{}\n", self.operation, self.path, self.format, self.http_version, formatted_headers)
    }
}

impl RequestBuilder {

    fn new(operation: String) -> RequestBuilder {
        let mut headers = HashMap::new();
        headers.insert("Host".to_string(), "localhost".to_string());

        let http_version = "HTTP/1.1".to_string();
        let format = None;
        let path = None;

        RequestBuilder {
            path,
            format,
            http_version,
            operation,
            headers,
        }
    }
    pub fn post() -> RequestBuilder {
        RequestBuilder::new("POST".to_string())
    }

    pub fn get() -> RequestBuilder {
        RequestBuilder::new("GET".to_string())
    }

    pub fn path(mut self, path: &str) -> RequestBuilder {
        self.path = Option::from(path.to_string());
        self
    }

    pub fn json(mut self) -> RequestBuilder {
        self.format = Option::from("json".to_string());
        self
    }

    #[allow(dead_code)]
    pub fn header(mut self, key: String, value: String) -> RequestBuilder {
        self.headers.insert(key, value);
        self
    }

    pub fn build(self) -> Request { // TODO return Result<Request>?
        Request {
            operation: self.operation,
            path: self.path.unwrap(),
            headers: self.headers,
            http_version: self.http_version,
            format: match self.format {
                None => "".to_string(),
                Some(format) => format!("/{}", format).to_string()
            }
        }
    }

}