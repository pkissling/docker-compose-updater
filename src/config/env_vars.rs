use std::env;

pub fn expected_api_key() -> String {
    // TODO generate new uuid if not set?
    env::var("API_KEY").expect("'API_KEY' not set")
}
