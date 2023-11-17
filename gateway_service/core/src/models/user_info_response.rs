use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserInfoResponse {
    /// Информация о билетах пользоватлея
    #[serde(rename = "tickets", skip_serializing_if = "Option::is_none")]
    pub tickets: Option<Vec<crate::models::TicketResponse>>,
    #[serde(rename = "privilege", skip_serializing_if = "Option::is_none")]
    pub privilege: Option<Box<crate::models::PrivilegeShortInfo>>,
}
