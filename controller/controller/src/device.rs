bitflags! {
  pub struct UnitFeatures: u8 {
    const ACCEL = 0b00000001;
    const MAG   = 0b00000010;
    const GYRO  = 0b00000100;
    const LEDS  = 0b00001000;
    const SERVO = 0b00010000;
  }
}

impl Default for UnitFeatures {
  fn default() -> UnitFeatures {
    UnitFeatures::ACCEL |
    UnitFeatures::MAG |
    UnitFeatures::GYRO |
    UnitFeatures::LEDS
  }
}

pub struct Device {
  id : String,
  features : UnitFeatures
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_default() {
    let actual : UnitFeatures = Default::default();
    assert!(actual.contains(UnitFeatures::ACCEL));
    assert!(actual.contains(UnitFeatures::MAG));
    assert!(actual.contains(UnitFeatures::GYRO));
    assert!(actual.contains(UnitFeatures::LEDS));
  }
}