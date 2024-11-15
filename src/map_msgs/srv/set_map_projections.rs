use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetMapProjectionsReq {

}

impl Default for SetMapProjectionsReq {
    fn default() -> Self {
        SetMapProjectionsReq {

        }
    }
}

impl ros2_client::Message for SetMapProjectionsReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetMapProjectionsRes {
    pub projected_maps_info: Vec<crate::map_msgs::msg::ProjectedMapInfo>,
}

impl Default for SetMapProjectionsRes {
    fn default() -> Self {
        SetMapProjectionsRes {
            projected_maps_info: Vec::new(),
        }
    }
}

impl ros2_client::Message for SetMapProjectionsRes {}


pub struct SetMapProjections;
impl ros2_client::Service for SetMapProjections {
    type Request = SetMapProjectionsReq;
    type Response = SetMapProjectionsRes;

    fn request_type_name(&self) -> &str { "SetMapProjectionsReq" }
    fn response_type_name(&self) -> &str { "SetMapProjectionsRes" }
}
