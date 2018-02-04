use rocket_cors;
use rocket_cors::{AllowedHeaders, AllowedMethods, AllowedOrigins};
use std::str::FromStr;

pub struct Cors {}

impl Cors {
  pub fn default() -> rocket_cors::Cors {
    let origins = AllowedOrigins::all();
    let headers = AllowedHeaders::all();
    let methods: AllowedMethods = ["Get", "Post"]
      .iter()
      .map(|s| FromStr::from_str(s).unwrap())
      .collect();

    rocket_cors::Cors {
      allowed_origins: origins,
      allowed_headers: headers,
      allowed_methods: methods,
      ..Default::default()
    }
  }
}
