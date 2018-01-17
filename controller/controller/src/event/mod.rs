pub mod evtloop;

#[derive(Debug, PartialEq)]
pub enum Event {
  Shutdown,
  DetectedNewDevice, // TODO: Surface USB descriptor information
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_it_compiles() {
    assert!(true);
  }

  #[test]
  fn test_shutdown_exists() {
    let expected = Event::Shutdown;
    assert_eq!(expected, Event::Shutdown);
  }
}
