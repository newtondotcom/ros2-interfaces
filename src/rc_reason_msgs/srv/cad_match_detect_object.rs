use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CadMatchDetectObjectReq {
    pub template_id: ::std::string::String,
    pub pose_frame: ::std::string::String,
    pub region_of_interest_id: ::std::string::String,
    pub load_carrier_id: ::std::string::String,
    pub load_carrier_compartment: crate::rc_reason_msgs::msg::Compartment,
    pub robot_pose: crate::geometry_msgs::msg::Pose,
    pub collision_detection: crate::rc_reason_msgs::msg::CollisionDetection,
    pub pose_prior_ids: Vec<::std::string::String>,
    pub data_acquisition_mode: ::std::string::String,
}

impl Default for CadMatchDetectObjectReq {
    fn default() -> Self {
        CadMatchDetectObjectReq {
            template_id: ::std::string::String::new(),
            pose_frame: ::std::string::String::new(),
            region_of_interest_id: ::std::string::String::new(),
            load_carrier_id: ::std::string::String::new(),
            load_carrier_compartment: crate::rc_reason_msgs::msg::Compartment::default(),
            robot_pose: crate::geometry_msgs::msg::Pose::default(),
            collision_detection: crate::rc_reason_msgs::msg::CollisionDetection::default(),
            pose_prior_ids: Vec::new(),
            data_acquisition_mode: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for CadMatchDetectObjectReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CadMatchDetectObjectRes {
    pub timestamp: crate::builtin_interfaces::msg::Time,
    pub matches: Vec<crate::rc_reason_msgs::msg::Match>,
    pub grasps: Vec<crate::rc_reason_msgs::msg::Grasp>,
    pub load_carriers: Vec<crate::rc_reason_msgs::msg::LoadCarrier>,
    pub return_code: crate::rc_common_msgs::msg::ReturnCode,
}

impl Default for CadMatchDetectObjectRes {
    fn default() -> Self {
        CadMatchDetectObjectRes {
            timestamp: crate::builtin_interfaces::msg::Time::default(),
            matches: Vec::new(),
            grasps: Vec::new(),
            load_carriers: Vec::new(),
            return_code: crate::rc_common_msgs::msg::ReturnCode::default(),
        }
    }
}

impl ros2_client::Message for CadMatchDetectObjectRes {}


pub struct CadMatchDetectObject;
impl ros2_client::Service for CadMatchDetectObject {
    type Request = CadMatchDetectObjectReq;
    type Response = CadMatchDetectObjectRes;

    fn request_type_name(&self) -> &str { "CadMatchDetectObjectReq" }
    fn response_type_name(&self) -> &str { "CadMatchDetectObjectRes" }
}
