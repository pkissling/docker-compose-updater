use crate::docker::socket::Socket;
use crate::docker::response::Response;

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
            true => Docker { socket },
            false => panic!("docker.sock not accessible!")
        }
    }

    #[allow(dead_code)]
    pub fn get_containers(self) -> std::io::Result<Response> {
        self.socket.get("/containers")
    }
}

fn can_access(socket: &Socket) -> bool {
    let result = socket.get("/containers");
    result.map_or(false, |r| r.is_success())
}
