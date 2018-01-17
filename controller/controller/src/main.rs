#![cfg_attr(feature = "clippy", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]
#![feature(plugin)]
#![plugin(rocket_codegen)]
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

use event::Event;
use event::evtloop::EventLoop;

#[get("/shutdown")]
fn post_shutdown_message() -> &'static str {
    "OK"
}

fn main() {
    let mut evtloop = EventLoop::new().unwrap();
    loop {
        if evtloop.run_once().unwrap() {
            evtloop.add(Event::Shutdown);
        }
    }
    // evtloop.add_event(Event::DetectedNewDevice);
    // evtloop.add_event(Event::DetectedNewDevice);
    // evtloop.add_event(Event::DetectedNewDevice);
    // evtloop.add_event(Event::Shutdown);
    // evtloop.for_each(|evt| match evt {
    //     Event::Shutdown => process::exit(0),
    //     Event::DetectedNewDevice => {
    //         println!("Detected new device!");
    //     }
    // });

    process::exit(1);
}
