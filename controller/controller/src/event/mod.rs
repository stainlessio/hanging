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

  #[test]
  fn test_get_next_event() {
    let mut evtloop = EventLoop::new();
    evtloop.add_event(Event::Shutdown);
    let actual = evtloop.get_next_event();
    assert_eq!(actual, Some(Event::Shutdown));
  }
}
