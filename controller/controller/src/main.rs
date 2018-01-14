#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]
#![feature(plugin)]
#![plugin(rocket_codegen)]
#[macro_use]
extern crate bitflags;
extern crate rocket;

mod event;
mod device;
use std::process;

fn main() {
    let mut evtloop = event::EventLoop::new();
    evtloop.add_event(event::Event::DetectedNewDevice);
    evtloop.add_event(event::Event::DetectedNewDevice);
    evtloop.add_event(event::Event::DetectedNewDevice);
    evtloop.add_event(event::Event::Shutdown);
    evtloop.for_each(|evt| {
        match evt {
            event::Event::Shutdown => process::exit(0),
            event::Event::DetectedNewDevice => {
                println!("Detected new device!");
            }
        }
    });

    process::exit(1);
}
