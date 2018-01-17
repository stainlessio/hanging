use super::Event;
use tokio_core::reactor::{Core, Handle};
use futures::future;
use std::io;
use std::time::{Duration, Instant};
use std::thread::sleep_ms;

pub struct EventLoop {
  core: Core,
  handle: Handle,
  last_tick: Instant,
}

impl EventLoop {
  pub fn new() -> io::Result<(EventLoop)> {
    let core = Core::new()?;
    let handle = core.handle();

    Ok(EventLoop {
      core: core,
      handle: handle,
      last_tick: Instant::now(),
    })
  }

  pub fn run_once(&mut self) -> io::Result<bool> {
    self.core.turn(Some(Duration::from_millis(125)));
    if self.last_tick.elapsed() > Duration::from_millis(250) {
      // Call Registered Tick Handlers
      println!("tick");
      self.last_tick = Instant::now();
      Ok(true)
    } else {
      Ok(false)
    }
  }

  pub fn add(&self, event: Event) {
    self.handle.spawn(future::ok::<(), ()>(()));
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_eventloop_new_works() {
    let mut evtloop = EventLoop::new().unwrap();
    let actual = evtloop.run_once();
    assert!(actual.is_ok());
  }

  #[test]
  fn test_eventloop_tick_triggered() {
    let mut evtloop = EventLoop::new().unwrap();
    let actual = evtloop.run_once().unwrap();
    assert!(!actual);
    sleep_ms(500);
    let actual = evtloop.run_once().unwrap();
    assert!(actual);
  }
}
