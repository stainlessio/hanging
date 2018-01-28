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

pub fn save_config(filename: &str) -> Result<(), Box<Error>> {
  let path = Path::new(filename);
  let mut file = File::create(&path)?;
  let config = Config {
    tuning: Some(Tuning {
      DetectedNewDevice: None,
      TickTiming: Some(InputType::IntSlider {
        minValue: 1, maxValue: 60, currentValue: 30
      }),
      TestTrigger: Some(InputType::Trigger),
    })
  };
  serde_json::to_writer_pretty(file, &config)?;
  Ok(())
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
  tuning: Option<Tuning>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Tuning {
  DetectedNewDevice: Option<EventType>,
  TickTiming: Option<InputType>,
  TestTrigger: Option<InputType>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum EventType {
  EventType,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum InputType {
  Trigger,
  IntSlider {minValue: i32, maxValue: i32, currentValue: i32},
}

impl Default for Config {
  fn default() -> Self {
    Config {
      tuning: None
    }
  }
}
