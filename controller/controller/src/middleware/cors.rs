use rocket::{Request, Response};
use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::{hyper};

#[derive(Default)]
pub struct Cors {

}

impl Fairing for Cors {
  fn info(&self) -> Info {
    Info {
      name: "Enable Cors allow any",
      kind: Kind::Response
    }
  }

  fn on_response(&self, request: &Request, response: &mut Response) {
    response.set_header(hyper::header::AccessControlAllowOrigin::Any);
  }
}