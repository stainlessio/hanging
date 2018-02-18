use super::Event;
use tokio_core::reactor::{Core, Handle, Remote};
use futures::prelude::*;
use futures::sync::mpsc;
use std::io;
use std::time::Instant;
use device::{Device, UnitFeatures};

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
      match res {
        Event::DetectedNewDevice => {
          let mut device = Device {
            id: "foo".to_owned(),
            features: UnitFeatures::default(),
            port: None,
          };
          match device.open("COM6") {
            Err(x) => println!("Failed to open device: {:?}", x),
            _ => (),
          };
        }
        x => println!("Received Event: {:?}", x),
      };
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

pub fn send_event(remote: &Remote, borrowed_sender: &mpsc::Sender<Event>, event: Event) {
  let sender = borrowed_sender.clone();
  remote.spawn(|_| sender.send(event).and_then(|_| Ok(())).or_else(|_| Err(())));
}

#[cfg(test)]
mod test {
  use super::*;
}
