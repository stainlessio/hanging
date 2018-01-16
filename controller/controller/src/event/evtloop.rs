use std::sync::mpsc::channel;
use super::Event;

pub struct EventLoop {
  sender: std::collection::VecDeque<Event>,
}

impl EventLoop {
  pub fn new() -> Self {
    EventLoop {}
  }

  pub fn get_next_event(&mut self) -> Option<Event> {
    self.queue.pop_front()
  }

  pub fn add_event(&mut self, event: Event) {
    self.queue.push_back(event);
  }

  pub fn for_each<F>(&mut self, mut f: F)
  where
    F: FnMut(Event),
  {
    let drain = self.queue.drain(0..);
    for evt in drain {
      f(evt);
    }
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_event_loop_for_each() {
    let mut evtloop = EventLoop::new();
    evtloop.add_event(Event::DetectedNewDevice);
    evtloop.add_event(Event::Shutdown);
    let mut actual: Vec<Event> = Vec::new();
    evtloop.for_each(|evt| {
      actual.push(evt);
    });
    assert_eq!(actual.len(), 2);
    assert_eq!(actual[0], Event::DetectedNewDevice);
    assert_eq!(actual[1], Event::Shutdown);
  }
}
