#![allow(non_snake_case)]
use rocket_contrib::Json;

pub fn load_config(filename: &str) -> Config {
  Config {
    tuning: Tuning {
      DetectedNewDevice: InputType::Trigger,
    },
  }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
  tuning: Tuning,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Tuning {
  DetectedNewDevice: InputType,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum InputType {
  Trigger,
}
