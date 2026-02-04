use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FreeParkingSpots {
    pub spots: Vec<::std::string::String>,
}

impl Default for FreeParkingSpots {
    fn default() -> Self {
        FreeParkingSpots {
            spots: Vec::new(),
        }
    }
}

impl ros2_client::Message for FreeParkingSpots {}
