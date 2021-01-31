use crate::from_request::api_key::ApiKey;

// TODO create trait which allows mapping errors to http status.
// TODO when error is returned from function, translate to http errror
#[put("/containers/<container_name>")]
pub fn update_container(container_name: String, _api_key: ApiKey) {
    println!("START: Update [{}]", container_name);

    // TODO

    println!("END: Update [{}]", container_name);
}

#[derive(Debug)]
pub struct HttpStatusCode {
    value: i16,
    msg: String,
}

impl HttpStatusCode {
    #[allow(dead_code)]
    pub fn new(value: i16, msg: String) -> Self {
        HttpStatusCode { value, msg }
    }
}
