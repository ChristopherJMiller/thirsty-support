use serde::{Serialize, Deserialize};

/// Correlates to what probe on the Thirsty Controller the data reading is from
#[derive(Serialize, Deserialize, Debug)]
pub enum ProbePort {
  J1,
  J2,
  J3,
  J4
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
