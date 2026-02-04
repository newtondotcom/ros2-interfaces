use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EmergencySignal {
    pub fleet_names: Vec<::std::string::String>,
    pub is_emergency: bool,
}

impl Default for EmergencySignal {
    fn default() -> Self {
        EmergencySignal {
            fleet_names: Vec::new(),
            is_emergency: false,
        }
    }
}

impl ros2_client::Message for EmergencySignal {}
