use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LocationAttributesPacket {
    pub sensor_modulation_performance: crate::off_highway_premium_radar_msgs::msg::SensorModulationPerformance,
    pub misalignment: crate::off_highway_premium_radar_msgs::msg::MisalignmentPacket,
    pub interference_indicator: crate::off_highway_premium_radar_msgs::msg::InterferenceIndicator,
    pub sensor_field_of_view: crate::off_highway_premium_radar_msgs::msg::SensorFieldOfView,
    pub sensor_coating: crate::off_highway_premium_radar_msgs::msg::SensorCoatingPacket,
}

impl Default for LocationAttributesPacket {
    fn default() -> Self {
        LocationAttributesPacket {
            sensor_modulation_performance: crate::off_highway_premium_radar_msgs::msg::SensorModulationPerformance::default(),
            misalignment: crate::off_highway_premium_radar_msgs::msg::MisalignmentPacket::default(),
            interference_indicator: crate::off_highway_premium_radar_msgs::msg::InterferenceIndicator::default(),
            sensor_field_of_view: crate::off_highway_premium_radar_msgs::msg::SensorFieldOfView::default(),
            sensor_coating: crate::off_highway_premium_radar_msgs::msg::SensorCoatingPacket::default(),
        }
    }
}

impl ros2_client::Message for LocationAttributesPacket {}
