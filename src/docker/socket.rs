use std::{os::unix::net::UnixStream};
use std::io::prelude::*;
use crate::docker::model::response::Response;
use crate::docker::model::request::{Request, RequestBuilder};

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
        let request = RequestBuilder::get()
            .path(path)
            .build();

        self.send(request)
    }

    #[allow(dead_code)]
    pub fn post(&self, path: &str) -> std::io::Result<Response> {
        let request = RequestBuilder::post()
            .path(path)
            .json()
            .build();

        self.send(request)
    }

    fn send(&self, request: Request) -> std::io::Result<Response> {
        println!("Request:");
        println!("{}", request.to_string()); // impl format

        let mut stream = self.stream.try_clone().unwrap(); // TODO looks hacky
        stream.write_all(request.to_string().as_bytes())?; // TODO impl TODO

        const BUFFER_SIZE: usize = 1024;
        let mut buffer: [u8; BUFFER_SIZE] = [0; BUFFER_SIZE];
        stream.read(&mut buffer)?;

        let response = String::from_utf8(buffer.to_vec()).unwrap();
        println!("Response:");
        println!("{}", response);

        Ok(Response::new(response))
    }
}