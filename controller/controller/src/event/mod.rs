use std::str::FromStr;

use rocket::request::FromParam;
use rocket::http::RawStr;

pub mod evtloop;

#[derive(Debug, PartialEq)]
pub enum Event {
  DetectedNewDevice, // TODO: Surface USB descriptor information
}

impl<'a> FromParam<'a> for Event {
  type Error = &'a RawStr;

  fn from_param(param: &'a RawStr) -> Result<Self, Self::Error> {
    param.parse::<Event>().map_err(|_| param)
  }
}

impl FromStr for Event {
  type Err = ();

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      "DetectedNewDevice" => Ok(Event::DetectedNewDevice),
      _ => Err(()),
    }
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_it_compiles() {
    assert!(true);
  }
}
