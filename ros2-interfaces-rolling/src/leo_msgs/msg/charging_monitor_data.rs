use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChargingMonitorData {
    pub header: crate::std_msgs::msg::Header,
    pub shunt_voltage: f64,
    pub bus_voltage: f64,
    pub current: f64,
    pub power: f64,
}

impl Default for ChargingMonitorData {
    fn default() -> Self {
        ChargingMonitorData {
            header: crate::std_msgs::msg::Header::default(),
            shunt_voltage: 0.0,
            bus_voltage: 0.0,
            current: 0.0,
            power: 0.0,
        }
    }
}

impl ros2_client::Message for ChargingMonitorData {}
