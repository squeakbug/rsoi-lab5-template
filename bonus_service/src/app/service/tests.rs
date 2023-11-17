#![cfg(test)]

use async_trait::async_trait;

use crate::app::domain::privilege_history::NewPrivilegeHistory;
use crate::app::models;
use crate::app::models::BalanceHistory;
use crate::app::models::PrivilegeRequest;
use crate::app::models::PrivilegeResponse;
use crate::app::repository::privilege_repository::PrivilegeRepository;
use crate::app::service::privilege_service::PrivilegeService;
use crate::app::service::privilege_service_impl::PrivilegeServiceImpl;
use crate::app::service::service_error::Result;

#[derive(Clone)]
struct PrivilegeRepositoryMock;

#[async_trait]
impl PrivilegeRepository for PrivilegeRepositoryMock {
    async fn get_privilege(&self, _: i32) -> Result<PrivilegeResponse> {
        let response = PrivilegeResponse {
            id: 0,
            balance: 0,
            status: String::from("BRONZE"),
            username: String::from(""),
        };
        Ok(response)
    }

    async fn get_privileges(&self, _: Option<String>) -> Result<Vec<PrivilegeResponse>> {
        let response = vec![PrivilegeResponse {
            id: 0,
            balance: 0,
            status: String::from("BRONZE"),
            username: String::from(""),
        }];
        Ok(response)
    }

    async fn create_privilege(&self, _: &models::PrivilegeCreateRequest) -> Result<models::PrivilegeResponse> {
        let result = models::PrivilegeResponse {
            balance: 0,
            id: 0,
            username: String::from("Test"),
            status: String::from("BRONZE"),
        };
        Ok(result)
    }

    async fn edit_privilege(&self, _: i32, _: &PrivilegeRequest) -> Result<PrivilegeResponse> {
        let response = PrivilegeResponse {
            id: 0,
            balance: 0,
            status: String::from("BRONZE"),
            username: String::from(""),
        };
        Ok(response)
    }

    async fn create_privilege_history(&self, _: NewPrivilegeHistory) -> Result<BalanceHistory> {
        Ok(BalanceHistory {
            balance_diff: Some(0),
            date: Some(String::from("")),
            operation_type: Some(String::from("")),
            ticket_uid: Some(String::from("")),
        })
    }

    async fn get_privilege_history(&self, _: Option<i32>, _: Option<uuid::Uuid>) -> Result<Vec<BalanceHistory>> {
        let response = vec![BalanceHistory {
            date: Some(chrono::Local::now().to_string()),
            balance_diff: Some(0),
            ticket_uid: Some(uuid::Uuid::default().to_string()),
            operation_type: Some(String::from("")),
        }];
        Ok(response)
    }
}

#[actix_rt::test]
async fn test_get_privilege() {
    let repo = PrivilegeRepositoryMock;
    let sut = PrivilegeServiceImpl {
        privilege_repository: Box::new(repo),
    };

    let result = sut.get_privilege(0).await;

    assert!(result.unwrap().id == 0);
}

#[actix_rt::test]
async fn test_get_privileges() {
    let repo = PrivilegeRepositoryMock;
    let sut = PrivilegeServiceImpl {
        privilege_repository: Box::new(repo),
    };

    let result = sut.get_privileges(None).await;

    assert!(result.unwrap()[0].id == 0);
}

#[actix_rt::test]
async fn test_update_privilege() {
    let repo = PrivilegeRepositoryMock;
    let sut = PrivilegeServiceImpl {
        privilege_repository: Box::new(repo),
    };

    let result = sut
        .edit_privilege(
            0,
            &PrivilegeRequest {
                balance_diff: 0,
                ticket_uid: uuid::Uuid::default(),
                username: String::from("test"),
                operation_type: String::from("PAID"),
            },
        )
        .await;
}
