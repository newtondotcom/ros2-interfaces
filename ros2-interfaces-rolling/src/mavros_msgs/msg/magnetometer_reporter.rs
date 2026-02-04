use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MagnetometerReporter {
    pub header: crate::std_msgs::msg::Header,
    pub report: u8,
    pub confidence: f32,
    pub compass_id: u8,
    pub cal_mask: u8,
    pub cal_status: u8,
    pub autosaved: u8,
    pub fitness: f32,
    pub ofs_x: f32,
    pub ofs_y: f32,
    pub ofs_z: f32,
    pub diag_x: f32,
    pub diag_y: f32,
    pub diag_z: f32,
    pub offdiag_x: f32,
    pub offdiag_y: f32,
    pub offdiag_z: f32,
    pub orientation_confidence: f32,
    pub old_orientation: u8,
    pub new_orientation: u8,
    pub scale_factor: f32,
}

impl Default for MagnetometerReporter {
    fn default() -> Self {
        MagnetometerReporter {
            header: crate::std_msgs::msg::Header::default(),
            report: 0,
            confidence: 0.0,
            compass_id: 0,
            cal_mask: 0,
            cal_status: 0,
            autosaved: 0,
            fitness: 0.0,
            ofs_x: 0.0,
            ofs_y: 0.0,
            ofs_z: 0.0,
            diag_x: 0.0,
            diag_y: 0.0,
            diag_z: 0.0,
            offdiag_x: 0.0,
            offdiag_y: 0.0,
            offdiag_z: 0.0,
            orientation_confidence: 0.0,
            old_orientation: 0,
            new_orientation: 0,
            scale_factor: 0.0,
        }
    }
}

impl ros2_client::Message for MagnetometerReporter {}
