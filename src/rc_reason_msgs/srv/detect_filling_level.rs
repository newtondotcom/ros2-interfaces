use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DetectFillingLevelReq {
    pub pose_frame: ::std::string::String,
    pub region_of_interest_id: ::std::string::String,
    pub region_of_interest_2d_id: ::std::string::String,
    pub load_carrier_ids: Vec<::std::string::String>,
    pub robot_pose: crate::geometry_msgs::msg::Pose,
    pub filling_level_cell_count: crate::rc_reason_msgs::msg::GridSize,
}

impl Default for DetectFillingLevelReq {
    fn default() -> Self {
        DetectFillingLevelReq {
            pose_frame: ::std::string::String::new(),
            region_of_interest_id: ::std::string::String::new(),
            region_of_interest_2d_id: ::std::string::String::new(),
            load_carrier_ids: Vec::new(),
            robot_pose: crate::geometry_msgs::msg::Pose::default(),
            filling_level_cell_count: crate::rc_reason_msgs::msg::GridSize::default(),
        }
    }
}

impl ros2_client::Message for DetectFillingLevelReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DetectFillingLevelRes {
    pub timestamp: crate::builtin_interfaces::msg::Time,
    pub load_carriers: Vec<crate::rc_reason_msgs::msg::LoadCarrierWithFillingLevel>,
    pub return_code: crate::rc_common_msgs::msg::ReturnCode,
}

impl Default for DetectFillingLevelRes {
    fn default() -> Self {
        DetectFillingLevelRes {
            timestamp: crate::builtin_interfaces::msg::Time::default(),
            load_carriers: Vec::new(),
            return_code: crate::rc_common_msgs::msg::ReturnCode::default(),
        }
    }
}

impl ros2_client::Message for DetectFillingLevelRes {}


pub struct DetectFillingLevel;
impl ros2_client::Service for DetectFillingLevel {
    type Request = DetectFillingLevelReq;
    type Response = DetectFillingLevelRes;

    fn request_type_name(&self) -> &str { "DetectFillingLevelReq" }
    fn response_type_name(&self) -> &str { "DetectFillingLevelRes" }
}
