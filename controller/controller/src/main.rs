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
extern crate serde;
extern crate serde_json;
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
use std::io::Cursor;
// use std::time::Duration;

use rocket_contrib::Json;
use rocket::response::Response;
use rocket::http::{hyper, ContentType, Status};

struct SenderState {
    remote: Remote,
    sender: Sender<Event>,
}

#[get("/triggerEvent/<event>")]
fn trigger_event(state: State<SenderState>, event: Event) -> Result<Response, Status> {
    send_event(&state.remote, &state.sender, event);
    let response = Response::build()
        .status(Status::Ok)
        .header(ContentType::Plain)
        .header(hyper::header::AccessControlAllowOrigin::Any)
        .sized_body(Cursor::new("OK"))
        .ok();

    response
}

#[get("/config")]
fn get_config() -> Json<config::Config> {
    Json(config::load_config("config.json").unwrap_or(config::Config::default()))
}

fn main() {
    config::save_config("test.json").ok();
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
