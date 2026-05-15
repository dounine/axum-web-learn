use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct PaginationParams {
    #[validate(range(min = 1, message = "page must be greater than or equal to 1"))]
    #[serde(default = "default_page")]
    pub page: u32,
    #[validate(range(min = 1, max = 100, message = "size must be between 1 and 100"))]
    #[serde(default = "default_size")]
    pub size: u8,
}
fn default_page() -> u32 {
    1
}
fn default_size() -> u8 {
    10
}
