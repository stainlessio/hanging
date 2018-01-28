#![cfg_attr(feature = "clippy", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]
#![feature(plugin)]
#![plugin(rocket_codegen)]
#![feature(conservative_impl_trait)]
#[macro_use]
extern crate bitflags;
extern crate futures;
extern crate rocket;
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
mod middleware;

use rocket::State;
use tokio_core::reactor::Remote;
use futures::sync::mpsc::Sender;

use event::Event;
use event::evtloop::{send_event, EventLoop};
use std::thread;
use std::io::Cursor;
use std::io::Read;

use rocket_contrib::Json;
use rocket::data::{self, FromData};
use rocket::{Request, Response, Data, Outcome};
use rocket::http::{ContentType, Status};
use rocket::Outcome::*;

struct SenderState {
    remote: Remote,
    sender: Sender<Event>,
}

#[derive(Debug)]
struct IntValue(i32);
impl FromData for IntValue {
    type Error = String;

    fn from_data(req: &Request, data: Data) -> data::Outcome<Self, String> {
        let ct = ContentType::new("application", "json");
        if req.content_type() != Some(&ct) {
            return Outcome::Forward(data);
        }

        let mut in_value = String::new();
        if let Err(e) = data.open().read_to_string(&mut in_value) {
            return Failure((Status::InternalServerError, format!("{:?}", e)));
        }


        match in_value.parse::<i32>() {
            Ok(v) => Success(IntValue(v).into()),
            Err(e) => Failure((Status::InternalServerError, format!("{:?}", e)))
        }
    }
}

#[get("/triggerEvent/<event>")]
fn trigger_event(state: State<SenderState>, event: Event) -> Result<Response, Status> {
    send_event(&state.remote, &state.sender, event);
    let response = Response::build()
        .status(Status::Ok)
        .header(ContentType::Plain)
        .sized_body(Cursor::new("OK"))
        .ok();

    response
}

#[get("/config")]
fn get_config() -> Json<config::Config> {
    Json(config::load_config("config.json").unwrap_or(config::Config::default()))
}

#[post("/config/<name>", data="<value>")]
fn update_config(name: String, value: IntValue) -> Result<Response, Status> {
    // println!("{:?} = {:?}", name, value);
    let response = Response::build()
        .status(Status::Ok)
        .header(ContentType::Plain)
        .sized_body(Cursor::new("OK"))
        .ok();

    response
}

fn main() {
    config::save_config("test.json").ok();
    let mut evtloop = EventLoop::new().unwrap();
    let sender = evtloop.sender.clone();
    let remote = evtloop.remote();

    thread::spawn(|| {
        rocket::ignite()
            .attach(middleware::cors::Cors::default())
            .mount("/", routes![trigger_event, get_config])
            .manage(SenderState {
                remote: remote,
                sender: sender,
            })
            .launch();
    });

    evtloop.run();
}
