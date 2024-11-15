use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ComputeGraspsReq {
    pub pose_frame: ::std::string::String,
    pub region_of_interest_id: ::std::string::String,
    pub load_carrier_id: ::std::string::String,
    pub load_carrier_compartment: crate::rc_reason_msgs::msg::Compartment,
    pub item_models: Vec<crate::rc_reason_msgs::msg::ItemModel>,
    pub suction_surface_length: f64,
    pub suction_surface_width: f64,
    pub robot_pose: crate::geometry_msgs::msg::Pose,
    pub collision_detection: crate::rc_reason_msgs::msg::CollisionDetection,
}

impl Default for ComputeGraspsReq {
    fn default() -> Self {
        ComputeGraspsReq {
            pose_frame: ::std::string::String::new(),
            region_of_interest_id: ::std::string::String::new(),
            load_carrier_id: ::std::string::String::new(),
            load_carrier_compartment: crate::rc_reason_msgs::msg::Compartment::default(),
            item_models: Vec::new(),
            suction_surface_length: 0.0,
            suction_surface_width: 0.0,
            robot_pose: crate::geometry_msgs::msg::Pose::default(),
            collision_detection: crate::rc_reason_msgs::msg::CollisionDetection::default(),
        }
    }
}

impl ros2_client::Message for ComputeGraspsReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ComputeGraspsRes {
    pub timestamp: crate::builtin_interfaces::msg::Time,
    pub items: Vec<crate::rc_reason_msgs::msg::Item>,
    pub load_carriers: Vec<crate::rc_reason_msgs::msg::LoadCarrier>,
    pub grasps: Vec<crate::rc_reason_msgs::msg::SuctionGrasp>,
    pub return_code: crate::rc_common_msgs::msg::ReturnCode,
}

impl Default for ComputeGraspsRes {
    fn default() -> Self {
        ComputeGraspsRes {
            timestamp: crate::builtin_interfaces::msg::Time::default(),
            items: Vec::new(),
            load_carriers: Vec::new(),
            grasps: Vec::new(),
            return_code: crate::rc_common_msgs::msg::ReturnCode::default(),
        }
    }
}

impl ros2_client::Message for ComputeGraspsRes {}


pub struct ComputeGrasps;
impl ros2_client::Service for ComputeGrasps {
    type Request = ComputeGraspsReq;
    type Response = ComputeGraspsRes;

    fn request_type_name(&self) -> &str { "ComputeGraspsReq" }
    fn response_type_name(&self) -> &str { "ComputeGraspsRes" }
}
