use std::{os::unix::net::UnixStream};
use std::io::prelude::*;
use crate::docker::response::Response;

pub struct Socket {
    stream: UnixStream
}

impl Socket {
    pub fn new(stream: &str) -> std::io::Result<Self> {
        // match Path::new("/var/run/docker.sock").exists() {
        //     true => Ok(()),
        //     false => Err("docker.sock does not exist"),
        // }

        let stream = UnixStream::connect(stream)?;
        Ok(Socket { stream })
    }

    pub fn get(&self, path: &str) -> std::io::Result<Response> {
        let path = format!("GET {}", path);
        self.send(path)
    }

    #[allow(dead_code)]
    pub fn post(&self, path: String) -> std::io::Result<Response> {
        let path = format!("POST {}", path);
        self.send(path)
    }

    fn send(&self, mut request: String) -> std::io::Result<Response> {
        append_data_format(&mut request);
        append_protocol(&mut request);
        append_newline(&mut request);
        append_host_header(&mut request);
        append_newline(&mut request);

        let request = "GET /containers/json HTTP/1.1\nHost: localhost\n\n";
        println!("Request:");
        println!("{}", request);

        let mut stream = self.stream.try_clone().unwrap(); // TODO
        stream.write_all(request.as_bytes())?;

        const BUFFER_SIZE: usize = 1024;
        let mut buffer: [u8; BUFFER_SIZE] = [0; BUFFER_SIZE];
        stream.read(&mut buffer)?;

        let response = String::from_utf8(buffer.to_vec()).unwrap();
        println!("Response:");
        println!("{}", response);

        Ok(Response::new(response))
    }
}


fn append_data_format(payload: &mut String) {
    payload.push_str("/json");

}
fn append_protocol(payload: &mut String) {
    payload.push_str(" HTTP/1.1");
}

fn append_newline(payload: &mut String) {
    payload.push_str("\n");
}

fn append_host_header(payload: &mut String) {
    payload.push_str("Host: localhost\n");
}
