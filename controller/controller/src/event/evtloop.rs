use super::Event;
use tokio_core::reactor::{Core, Handle, Remote};
use futures::prelude::*;
// use futures::future;
use futures::sync::mpsc;
use std::io;
use std::time::Instant;

pub struct EventLoop {
  core: Core,
  handle: Handle,
  last_tick: Instant,
  pub sender: mpsc::Sender<Event>,
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
    match self.core.run(process_events) {
      Ok(_) => (),
      Err(err) => {
        println!("Error: {:?}", err);
        ()
      }
    }
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

pub fn send_event(remote: Remote, sender: mpsc::Sender<Event>, event: Event) {
  remote.spawn(|_| {
    sender
      .send(event)
      .and_then(|evt| {
        println!("Sent {:?}", evt);
        Ok(())
      })
      .or_else(|_| Err(()))
  });
}

#[cfg(test)]
mod test {
  use super::*;
}
