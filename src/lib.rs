use serde::{Serialize, Deserialize};

/// Correlates to what probe on the Thirsty Controller the data reading is from
#[derive(Serialize, Deserialize, Debug)]
pub enum ProbePort {
  J1,
  J2,
  J3,
  J4
}

impl Into<String> for ProbePort {
  fn into(self) -> String {
    match self {
      Self::J1 => "J1".to_string(),
      Self::J2 => "J2".to_string(),
      Self::J3 => "J3".to_string(),
      Self::J4 => "J4".to_string()
    }
  }
}

/// Message structure for controller probe data report
#[derive(Serialize, Deserialize, Debug)]
pub struct ControllerMessage {
  /// Controller Unique ID
  pub controller_id: String,
  /// Probe that reading is from
  pub probe: ProbePort,
  /// Value of the reading
  pub value: u32
}
