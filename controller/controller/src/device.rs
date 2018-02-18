use std::io::prelude::*;
use serial::prelude::*;
use serial;
use std::io;
use std::time::Duration;

bitflags! {
  pub struct UnitFeatures: u8 {
  const LED   = 0b00000001;
  const ACCEL = 0b00000010;
  const MAG   = 0b00000100;
  const GYRO  = 0b00001000;
  const TEMP  = 0b00010000;
  const BARO  = 0b00100000;
  const SERVO = 0b01000000;
  }
}

impl Default for UnitFeatures {
  fn default() -> UnitFeatures {
    UnitFeatures::ACCEL | UnitFeatures::MAG | UnitFeatures::GYRO | UnitFeatures::LED
  }
}

impl From<u8> for UnitFeatures {
  fn from(byte: u8) -> Self {
    Self { bits: byte }
  }
}

pub struct Device {
  // TODO: Move default creation to impl fn
  pub id: String,
  pub features: UnitFeatures,
  pub port: Option<Box<SerialPort>>,
}

impl Device {
  pub fn open(&mut self, device: &str) -> io::Result<()> {
    println!("Opening Serial Port");
    let mut port = serial::open(device).unwrap();

    port.reconfigure(&|settings| {
      println!("Configuring");
      settings.set_baud_rate(serial::Baud115200)?;
      Ok(())
    })?;

    port.set_timeout(Duration::from_secs(10))?;

    let mut buf: Vec<u8> = (0..3).collect();
    buf[0] = ('C' as u8);
    buf[1] = ('\n' as u8);
    println!("Writing");
    port.write(&buf[0..1])?;
    println!("Reading");
    port.read(&mut buf[..])?;
    println!("{:?}", buf);
    self.port = Some(Box::new(port));
    Ok(())
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_default() {
    let actual: UnitFeatures = Default::default();
    assert!(actual.contains(UnitFeatures::ACCEL));
    assert!(actual.contains(UnitFeatures::MAG));
    assert!(actual.contains(UnitFeatures::GYRO));
    assert!(actual.contains(UnitFeatures::LEDS));
  }
}
