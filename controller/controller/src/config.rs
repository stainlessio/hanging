#![allow(non_snake_case)]

use std::path::Path;
use std::fs::File;
use std::error::Error;
use std::default::Default;
use serde_json;
use std::boxed::Box;

pub fn load_config(filename: &str) -> Result<Config, Box<Error>> {
  let path = Path::new(filename); // TODO: Figure out CWD
  let file = File::open(&path)?;
  match serde_json::from_reader(file) {
    Ok(x) => Ok(x),
    Err(err) => {
      println!("{:?}", err);
      Err(Box::new(err))
    }
  }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
  pub tuning: Option<Tuning>,
}

impl Config {
  pub fn save_config(&self, filename: &str) -> Result<(), Box<Error>> {
    let path = Path::new(filename);
    let file = File::create(&path)?;
    serde_json::to_writer_pretty(file, self)?;
    Ok(())
  }
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Tuning {
  pub DetectedNewDevice: Option<EventType>,
  pub TickTiming: Option<InputType>,
  pub TestTrigger: Option<InputType>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum EventType {
  EventType,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum InputType {
  Trigger,
  IntSlider {
    minValue: i32,
    maxValue: i32,
    currentValue: i32,
  },
}

impl Default for Config {
  fn default() -> Self {
    Config { tuning: None }
  }
}
