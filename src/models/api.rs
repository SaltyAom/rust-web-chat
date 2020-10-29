use serde::Serialize;

#[derive(Serialize)]
pub struct APIResult<'a> {
    pub success: bool,
    pub detail: &'a str,
}

#[derive(Serialize)]
pub struct APIStaticResult {
    pub success: bool,
    pub detail: &'static str,
}

pub struct APIResponse;

impl APIResponse {
    pub const SUCCESS: APIStaticResult = API_SUCCESS;
    pub const FAILURE: APIStaticResult = API_FAILURE;
}

pub const API_SUCCESS: APIStaticResult = APIStaticResult {
    success: true,
    detail: "",
};
pub const API_FAILURE: APIStaticResult = APIStaticResult {
    success: false,
    detail: "",
};
