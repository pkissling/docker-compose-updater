use crate::docker::docker::Docker;

pub struct ContainerService {
    docker: Docker
}

impl ContainerService {
    pub fn new(docker: Docker) -> Self {
        ContainerService {
            docker
        }
    }

    pub fn container_exists(self, name : String) -> bool {
        let response = self.docker.get_container(name);
        response.is_ok()
    }
}
