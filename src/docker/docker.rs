use crate::docker::socket::Socket;
use crate::docker::model::response::Response;

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
        Docker { socket }
        // match can_access(&socket) {
        //     true => Docker { socket },
        //     false => panic!("docker.sock not accessible!")
        // }
    }

    #[allow(dead_code)]
    pub fn get_containers(self) -> std::io::Result<Response> {
        self.socket.get("/containers/json")
    }
}

// fn can_access(socket: &Socket) -> bool { // TODO return std::io::Result<Response>?
//     let result = socket.get("/ping");
//     match result {
//         Ok(_) => true,
//         Err(_) => false
//     }
// }
