#![cfg_attr(feature = "clippy", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]
#![feature(plugin)]
#![plugin(rocket_codegen)]
#![feature(conservative_impl_trait)]
#[macro_use]
extern crate bitflags;
extern crate futures;
extern crate rocket;
extern crate tokio_core;
extern crate tokio_io;

mod event;
mod device;

use std::process;
use rocket::State;
use tokio_core::reactor::Remote;
use futures::sync::mpsc::Sender;

use event::Event;
use event::evtloop::{send_event, EventLoop};
use std::thread;
use std::time::Duration;

struct SenderState {
    remote: Remote,
    sender: Sender<Event>,
}

// #[get("/shutdown")]
// fn post_shutdown_message(state: State<SenderState>) -> &'static str {
//     send_event(&state.remote, &state.sender, Event::Shutdown);
//     "OK"
// }

#[get("/add_device")]
fn add_device(state: State<SenderState>) -> &'static str {
    send_event(&state.remote, &state.sender, Event::DetectedNewDevice);
    "OK"
}

fn main() {
    let mut evtloop = EventLoop::new().unwrap();
    let sender = evtloop.sender.clone();
    let remote = evtloop.remote();

    thread::spawn(|| {
        rocket::ignite()
            .mount("/", routes![add_device])
            .manage(SenderState {
                remote: remote,
                sender: sender,
            })
            .launch();
    });

    evtloop.run();
}
