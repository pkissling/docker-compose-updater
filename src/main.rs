#![feature(proc_macro_hygiene, decl_macro)]

mod config;
mod from_request;
mod routes;
mod services;
mod docker;

#[macro_use]
extern crate rocket;

use docker::docker::Docker;
use rocket::config::{Config, Environment};
use crate::config::env_vars;
use crate::services::container_service::ContainerService;

fn main() {
    ensure_preconditions();

    let routes = routes![routes::update_container::update_container];
    let base_url = "/";

    let config = Config::build(Environment::Production)
        .address("0.0.0.0")
        .port(5000)
        .finalize()
        .expect("failed");

    // TODo docker system prune -a
    // TODO container not gracefully stopping!
    rocket::custom(config).mount(base_url, routes).launch();
}

fn ensure_preconditions() {
    // API_KEY must be set
    env_vars::expected_api_key();

    // check if docker socket is available and accessible
    let docker = Docker::connect("/var/run/docker.sock").unwrap();
    let service = ContainerService::new(docker);
    let exists = service.container_exists("kind_dhawan".to_string());
    println!("exists {}", exists)
}
