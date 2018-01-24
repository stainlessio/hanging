#![cfg_attr(feature = "clippy", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]
#![feature(plugin)]
#![plugin(rocket_codegen)]
#![feature(conservative_impl_trait)]
#[macro_use]
extern crate bitflags;
extern crate futures;
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;
extern crate tokio_core;
extern crate tokio_io;

mod event;
mod device;
mod config;

// use std::process;
use rocket::State;
use tokio_core::reactor::Remote;
use futures::sync::mpsc::Sender;

use event::Event;
use event::evtloop::{send_event, EventLoop};
use std::thread;
// use std::time::Duration;

use rocket_contrib::Json;

struct SenderState {
    remote: Remote,
    sender: Sender<Event>,
}

#[get("/triggerEvent/<event>")]
fn trigger_event(state: State<SenderState>, event: Event) -> &'static str {
    send_event(&state.remote, &state.sender, event);
    "OK"
}

#[get("/config")]
fn get_config() -> Json<config::Config> {
    Json(config::load_config(""))
}

fn main() {
    let mut evtloop = EventLoop::new().unwrap();
    let sender = evtloop.sender.clone();
    let remote = evtloop.remote();

    thread::spawn(|| {
        rocket::ignite()
            .mount("/", routes![trigger_event, get_config])
            .manage(SenderState {
                remote: remote,
                sender: sender,
            })
            .launch();
    });

    evtloop.run();
}
