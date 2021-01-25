use crate::docker::socket::Socket;
use crate::docker::response::Response;

pub struct Docker {
    socket: Socket
}

impl Docker {

    pub fn connect(socket: &str) -> std::io::Result<Self> {
        let s = Socket::new(socket)?;
        Ok(Docker::new(s)?)
    }

    fn new(mut socket: Socket) -> std::io::Result<Self> {
        let response = test_socket(&mut socket)?;
        assert!(response.is_success());

        Ok(Docker { socket })
    }

    pub fn get_containers(self) -> std::io::Result<Response> {
        self.socket.get("/containers")
    }
}

fn test_socket(socket: &Socket) -> std::io::Result<Response> {
    socket.get("/containers")
}
