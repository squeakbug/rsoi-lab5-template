/*
 * OpenAPI definition
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1
 *
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PrivilegeResponse {
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "username")]
    pub username: String,
    #[serde(rename = "status")]
    pub status: String,
    #[serde(rename = "balance")]
    pub balance: i32,
}

impl PrivilegeResponse {
    pub fn new(id: i32, username: String, status: String, balance: i32) -> PrivilegeResponse {
        PrivilegeResponse {
            id,
            username,
            status,
            balance,
        }
    }
}

///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "BRONZE")]
    Bronze,
    #[serde(rename = "SILVER")]
    Silver,
    #[serde(rename = "GOLD")]
    Gold,
}

impl Default for Status {
    fn default() -> Status {
        Self::Bronze
    }
}
