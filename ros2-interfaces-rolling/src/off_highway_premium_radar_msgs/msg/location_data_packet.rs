use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LocationDataPacket {
    pub header: crate::std_msgs::msg::Header,
    pub radial_distance: f32,
    pub radial_relative_velocity: f32,
    pub azimuth_angle: f32,
    pub elevation_angle: f32,
    pub radar_cross_section: f32,
    pub snr: f32,
    pub radial_distance_variance: f32,
    pub radial_relative_velocity_variance: f32,
    pub variance_azimuth: f32,
    pub variance_elevation: f32,
    pub distance_velocity_covariance: f32,
    pub probability_velocity_resolution: f32,
    pub probability_azimuth_angle: f32,
    pub probability_elevation_angle: f32,
    pub measurement_status: u16,
    pub index_angle_ambiguity: u16,
    pub reserved: [u8; 12],
}

impl Default for LocationDataPacket {
    fn default() -> Self {
        LocationDataPacket {
            header: crate::std_msgs::msg::Header::default(),
            radial_distance: 0.0,
            radial_relative_velocity: 0.0,
            azimuth_angle: 0.0,
            elevation_angle: 0.0,
            radar_cross_section: 0.0,
            snr: 0.0,
            radial_distance_variance: 0.0,
            radial_relative_velocity_variance: 0.0,
            variance_azimuth: 0.0,
            variance_elevation: 0.0,
            distance_velocity_covariance: 0.0,
            probability_velocity_resolution: 0.0,
            probability_azimuth_angle: 0.0,
            probability_elevation_angle: 0.0,
            measurement_status: 0,
            index_angle_ambiguity: 0,
            reserved: [0; 12],
        }
    }
}

impl ros2_client::Message for LocationDataPacket {}
