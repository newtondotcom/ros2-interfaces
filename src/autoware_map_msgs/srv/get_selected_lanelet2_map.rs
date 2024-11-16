use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetSelectedLanelet2MapReq {
    pub cell_ids: Vec<::std::string::String>,
}

impl Default for GetSelectedLanelet2MapReq {
    fn default() -> Self {
        GetSelectedLanelet2MapReq {
            cell_ids: Vec::new(),
        }
    }
}

impl ros2_client::Message for GetSelectedLanelet2MapReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetSelectedLanelet2MapRes {
    pub header: crate::std_msgs::msg::Header,
    pub lanelet2_cells: crate::autoware_map_msgs::msg::LaneletMapBin,
}

impl Default for GetSelectedLanelet2MapRes {
    fn default() -> Self {
        GetSelectedLanelet2MapRes {
            header: crate::std_msgs::msg::Header::default(),
            lanelet2_cells: crate::autoware_map_msgs::msg::LaneletMapBin::default(),
        }
    }
}

impl ros2_client::Message for GetSelectedLanelet2MapRes {}


pub struct GetSelectedLanelet2Map;
impl ros2_client::Service for GetSelectedLanelet2Map {
    type Request = GetSelectedLanelet2MapReq;
    type Response = GetSelectedLanelet2MapRes;

    fn request_type_name(&self) -> &str { "GetSelectedLanelet2MapReq" }
    fn response_type_name(&self) -> &str { "GetSelectedLanelet2MapRes" }
}
