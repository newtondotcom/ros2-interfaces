use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Scenario {
    pub current_scenario: ::std::string::String,
    pub activating_scenarios: Vec<::std::string::String>,
}

impl Scenario {
    pub const EMPTY: &'static str = "Empty";
    pub const LANEDRIVING: &'static str = "LaneDriving";
    pub const PARKING: &'static str = "Parking";
}

impl Default for Scenario {
    fn default() -> Self {
        Scenario {
            current_scenario: ::std::string::String::new(),
            activating_scenarios: Vec::new(),
        }
    }
}

impl ros2_client::Message for Scenario {}
