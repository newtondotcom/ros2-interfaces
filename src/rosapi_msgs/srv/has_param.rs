use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HasParamReq {
    pub name: ::std::string::String,
}

impl Default for HasParamReq {
    fn default() -> Self {
        HasParamReq {
            name: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for HasParamReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HasParamRes {
    pub exists: bool,
}

impl Default for HasParamRes {
    fn default() -> Self {
        HasParamRes {
            exists: false,
        }
    }
}

impl ros2_client::Message for HasParamRes {}


pub struct HasParam;
impl ros2_client::Service for HasParam {
    type Request = HasParamReq;
    type Response = HasParamRes;

    fn request_type_name(&self) -> &str { "HasParamReq" }
    fn response_type_name(&self) -> &str { "HasParamRes" }
}
