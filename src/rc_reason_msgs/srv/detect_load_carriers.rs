use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DetectLoadCarriersReq {
    pub pose_frame: ::std::string::String,
    pub region_of_interest_id: ::std::string::String,
    pub region_of_interest_2d_id: ::std::string::String,
    pub load_carrier_ids: Vec<::std::string::String>,
    pub robot_pose: crate::geometry_msgs::msg::Pose,
}

impl Default for DetectLoadCarriersReq {
    fn default() -> Self {
        DetectLoadCarriersReq {
            pose_frame: ::std::string::String::new(),
            region_of_interest_id: ::std::string::String::new(),
            region_of_interest_2d_id: ::std::string::String::new(),
            load_carrier_ids: Vec::new(),
            robot_pose: crate::geometry_msgs::msg::Pose::default(),
        }
    }
}

impl ros2_client::Message for DetectLoadCarriersReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DetectLoadCarriersRes {
    pub timestamp: crate::builtin_interfaces::msg::Time,
    pub load_carriers: Vec<crate::rc_reason_msgs::msg::LoadCarrier>,
    pub return_code: crate::rc_common_msgs::msg::ReturnCode,
}

impl Default for DetectLoadCarriersRes {
    fn default() -> Self {
        DetectLoadCarriersRes {
            timestamp: crate::builtin_interfaces::msg::Time::default(),
            load_carriers: Vec::new(),
            return_code: crate::rc_common_msgs::msg::ReturnCode::default(),
        }
    }
}

impl ros2_client::Message for DetectLoadCarriersRes {}


pub struct DetectLoadCarriers;
impl ros2_client::Service for DetectLoadCarriers {
    type Request = DetectLoadCarriersReq;
    type Response = DetectLoadCarriersRes;

    fn request_type_name(&self) -> &str { "DetectLoadCarriersReq" }
    fn response_type_name(&self) -> &str { "DetectLoadCarriersRes" }
}
