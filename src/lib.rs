use serde::{Serialize, Deserialize};

/// Correlates to what probe on the Thirsty Controller the data reading is from
#[derive(Serialize, Deserialize, Debug)]
enum ProbePort {
  J1,
  J2,
  J3,
  J4
}

/// Message structure for controller probe data report
#[derive(Serialize, Deserialize, Debug)]
struct ControllerMessage {
  /// Controller Unique ID
  controller_id: String,
  /// Probe that reading is from
  probe: ProbePort,
  /// Value of the reading
  value: u32
}
