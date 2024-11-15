use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DetectItemsReq {
    pub pose_frame: ::std::string::String,
    pub region_of_interest_id: ::std::string::String,
    pub load_carrier_id: ::std::string::String,
    pub load_carrier_compartment: crate::rc_reason_msgs::msg::Compartment,
    pub item_models: Vec<crate::rc_reason_msgs::msg::ItemModel>,
    pub robot_pose: crate::geometry_msgs::msg::Pose,
}

impl Default for DetectItemsReq {
    fn default() -> Self {
        DetectItemsReq {
            pose_frame: ::std::string::String::new(),
            region_of_interest_id: ::std::string::String::new(),
            load_carrier_id: ::std::string::String::new(),
            load_carrier_compartment: crate::rc_reason_msgs::msg::Compartment::default(),
            item_models: Vec::new(),
            robot_pose: crate::geometry_msgs::msg::Pose::default(),
        }
    }
}

impl ros2_client::Message for DetectItemsReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DetectItemsRes {
    pub timestamp: crate::builtin_interfaces::msg::Time,
    pub items: Vec<crate::rc_reason_msgs::msg::Item>,
    pub load_carriers: Vec<crate::rc_reason_msgs::msg::LoadCarrier>,
    pub return_code: crate::rc_common_msgs::msg::ReturnCode,
}

impl Default for DetectItemsRes {
    fn default() -> Self {
        DetectItemsRes {
            timestamp: crate::builtin_interfaces::msg::Time::default(),
            items: Vec::new(),
            load_carriers: Vec::new(),
            return_code: crate::rc_common_msgs::msg::ReturnCode::default(),
        }
    }
}

impl ros2_client::Message for DetectItemsRes {}


pub struct DetectItems;
impl ros2_client::Service for DetectItems {
    type Request = DetectItemsReq;
    type Response = DetectItemsRes;

    fn request_type_name(&self) -> &str { "DetectItemsReq" }
    fn response_type_name(&self) -> &str { "DetectItemsRes" }
}
