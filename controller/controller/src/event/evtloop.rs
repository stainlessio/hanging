use super::Event;
use tokio_core::reactor::{Core, Handle, Remote};
use futures::prelude::*;
use futures::future;
use futures::sync::mpsc;
use std::io;
use std::time::{Duration, Instant};
use std::thread::sleep_ms;

pub struct EventLoop {
  core: Core,
  handle: Handle,
  last_tick: Instant,
  sender: mpsc::Sender<Event>,
  receiver: mpsc::Receiver<Event>,
}

impl EventLoop {
  pub fn new() -> io::Result<(EventLoop)> {
    let core = Core::new()?;
    let handle = core.handle();

    let (tx, rx) = mpsc::channel(1); // TODO: Add a tunable capacity

    Ok(EventLoop {
      core: core,
      handle: handle,
      last_tick: Instant::now(),
      sender: tx,
      receiver: rx,
    })
  }

  pub fn remote(&self) -> Remote {
    self.core.remote()
  }

  pub fn run(&mut self) {
    let process_events = self.receiver.by_ref().for_each(|res| {
      println!("{:?}", res);
      Ok(())
    });
    self.core.run(process_events);
  }

  pub fn add(self, event: Event) {
    self.handle.spawn(
      self
        .sender
        .send(event)
        .and_then(|_| Ok(()))
        .or_else(|_| Err(())),
    );
  }
}

pub fn wrap_event_in_future(sender: &Sender<Event>, event: Event) -> Send {
  sender.send(event).and_then(|_| Ok(())).or_else(|_| Err(()))
}

#[cfg(test)]
mod test {
  use super::*;
}
