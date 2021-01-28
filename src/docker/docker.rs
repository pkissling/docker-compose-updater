use crate::docker::model::response::Response;
use crate::docker::socket::Socket;

pub struct Docker {
    #[allow(dead_code)]
    socket: Socket
}

impl Docker {
    pub fn connect(socket: &str) -> std::io::Result<Self> {
        let s = Socket::new(socket)?;
        Ok(Docker::new(s))
    }

    fn new(socket: Socket) -> Self {
        match can_access(&socket) {
            Ok(_) => Docker { socket },
            Err(e) => panic!("Error: {}", e)
        }
    }

    #[allow(dead_code)]
    pub fn get_containers(self) -> std::io::Result<Response> {
        self.socket.get("/containers/json")
    }
}

fn can_access(socket: &Socket) -> std::io::Result<Response> {
    socket.get("/_ping")
}
