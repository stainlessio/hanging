pub mod evtloop;

#[derive(Debug, PartialEq)]
pub enum Event {
  DetectedNewDevice, // TODO: Surface USB descriptor information
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_it_compiles() {
    assert!(true);
  }
}
