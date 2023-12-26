use std::vec;

use async_trait::async_trait;
use chrono;
use reqwest::Client;

use crate::models;
use crate::service::gateway_service::GatewayService;
use crate::service::service_error::{Result, ServiceError};

#[derive(Clone)]
pub struct GatewayServiceImpl {
    pub flight_base_path: String,
    pub ticket_base_path: String,
    pub bonus_base_path: String,

    client: Client,
}

impl GatewayServiceImpl {
    pub fn new(flight_base_path: String, ticket_base_path: String, bonus_base_path: String) -> Self {
        GatewayServiceImpl {
            flight_base_path,
            ticket_base_path,
            bonus_base_path,
            client: Client::new(),
        }
    }

    pub async fn get_full_tickets(&self, token: String, username: String) -> Result<Vec<models::TicketResponse>> {
        let my_token: &String = &token;
        let futures = self
            .get_tickets(my_token.clone(), username)
            .await?
            .into_iter()
            .map(|ticket| async move {
                let flight = self.get_flight(my_token.clone(), ticket.flight_number).await.unwrap();
                models::TicketResponse {
                    date: flight.date,
                    flight_number: flight.flight_number,
                    from_airport: flight.from_airport,
                    to_airport: flight.to_airport,
                    price: flight.price,
                    status: Some(ticket.status),
                    ticket_uid: Some(ticket.ticket_uid),
                }
            });

        let mut result = vec![];
        for f in futures {
            result.push(f.await);
        }

        Ok(result)
    }

    pub async fn create_privilege(
        &self,
        token: String,
        input: models::PrivilegeCreateRequest,
    ) -> Result<models::PrivilegeResponse> {
        use bonus_service_api::apis as bapis;
        use bonus_service_api::models as bmodels;

        let bonus_conf = bapis::configuration::Configuration {
            base_path: self.bonus_base_path.clone(),
            client: self.client.clone(),
            ..Default::default()
        };
        let privilege_req = bmodels::PrivilegeCreateRequest {
            balance: input.balance,
            status: input.status,
            username: input.username,
        };
        let privilege_resp = bapis::bonus_restapi_operations_api::create_bonus(token, &bonus_conf, privilege_req)
            .await
            .map_err(|_| ServiceError::BadClientData)?;

        let result = models::PrivilegeResponse {
            id: privilege_resp.id,
            balance: privilege_resp.balance,
            username: privilege_resp.username,
            status: privilege_resp.status,
        };

        Ok(result)
    }

    pub async fn get_privilege(&self, token: String, username: String) -> Result<models::PrivilegeResponse> {
        use bonus_service_api::apis as bapis;

        let bonus_conf = bapis::configuration::Configuration {
            base_path: self.bonus_base_path.clone(),
            client: self.client.clone(),
            ..Default::default()
        };
        let user_bonuses = bapis::bonus_restapi_operations_api::list_bonuses(token, &bonus_conf, Some(&username))
            .await
            .map_err(|_| ServiceError::BadClientData)?;

        let user_bonuses = match user_bonuses.len() {
            0 => Err(ServiceError::NotFoundError),
            _ => Ok(user_bonuses),
        }?;

        // У одного пользователя может быть только 1 аккаунт
        let user_bonus = user_bonuses.first().unwrap().to_owned();

        let result = models::PrivilegeResponse {
            id: user_bonus.id,
            balance: user_bonus.balance,
            username: user_bonus.username,
            status: user_bonus.status,
        };

        Ok(result)
    }

    pub async fn get_user_history(&self, token: String, username: String) -> Result<Vec<models::BalanceHistory>> {
        use bonus_service_api::apis as bapis;

        let bonus_conf = bapis::configuration::Configuration {
            base_path: self.bonus_base_path.clone(),
            client: self.client.clone(),
            ..Default::default()
        };
        let response =
            bapis::bonus_restapi_operations_api::list_bonus_history(token, &bonus_conf, Some(&username), None)
                .await
                .map_err(|_| ServiceError::BadClientData)?;

        let result = response
            .into_iter()
            .map(|history| models::BalanceHistory {
                balance_diff: history.balance_diff,
                date: history.date,
                operation_type: history.operation_type,
                ticket_uid: history.ticket_uid,
            })
            .collect::<Vec<_>>();

        Ok(result)
    }

    pub async fn get_ticket_history(
        &self,
        token: String,
        ticket_uid: uuid::Uuid,
    ) -> Result<Vec<models::BalanceHistory>> {
        use bonus_service_api::apis as bapis;

        let bonus_conf = bapis::configuration::Configuration {
            base_path: self.bonus_base_path.clone(),
            client: self.client.clone(),
            ..Default::default()
        };
        let response = bapis::bonus_restapi_operations_api::list_bonus_history(
            token,
            &bonus_conf,
            None,
            Some(&ticket_uid.to_string()),
        )
        .await
        .map_err(|_| ServiceError::BadClientData)?;

        let result = response
            .into_iter()
            .map(|history| models::BalanceHistory {
                balance_diff: history.balance_diff,
                date: history.date,
                operation_type: history.operation_type,
                ticket_uid: Some(ticket_uid.to_string()),
            })
            .collect::<Vec<_>>();

        Ok(result)
    }

    pub async fn update_privilege(
        &self,
        token: String,
        privilege_id: i32,
        request: models::PrivilegeRequest,
    ) -> Result<models::PrivilegeShortInfo> {
        use bonus_service_api::apis as bapis;

        let bonus_conf = bapis::configuration::Configuration {
            base_path: self.bonus_base_path.clone(),
            client: self.client.clone(),
            ..Default::default()
        };
        let privelege_req = bonus_service_api::models::privilege_request::PrivilegeRequest {
            balance_diff: request.balance_diff,
            ticket_uid: request.ticket_uid,
            username: request.username,
            operation_type: request.operation,
        };

        let response = bapis::bonus_restapi_operations_api::edit_bonus(token, &bonus_conf, privilege_id, privelege_req)
            .await
            .unwrap();

        let result = models::PrivilegeShortInfo {
            balance: Some(response.balance),
            status: Some(response.status),
        };

        Ok(result)
    }

    pub async fn get_tickets(&self, token: String, username: String) -> Result<Vec<models::TicketInfo>> {
        use ticket_service_api::apis as tapis;

        let ticket_conf = tapis::configuration::Configuration {
            base_path: self.ticket_base_path.clone(),
            client: self.client.clone(),
            ..Default::default()
        };
        let tickets = tapis::ticket_restapi_operations_api::list_tickets(token, &ticket_conf, Some(&username), None)
            .await
            .map_err(|_| ServiceError::BadClientData)?
            .into_iter()
            .map(|ticket| models::TicketInfo {
                id: ticket.id,
                ticket_uid: ticket.ticket_uid,
                flight_number: ticket.flight_number,
                price: ticket.price,
                status: ticket.status,
                username: ticket.username,
            })
            .collect::<Vec<_>>();

        Ok(tickets)
    }

    pub async fn get_ticket(&self, token: String, ticket_uid: uuid::Uuid) -> Result<models::TicketInfo> {
        use ticket_service_api::apis as tapis;

        let ticket_conf = tapis::configuration::Configuration {
            base_path: self.ticket_base_path.clone(),
            client: self.client.clone(),
            ..Default::default()
        };
        tapis::ticket_restapi_operations_api::get_ticket(token, &ticket_conf, &ticket_uid.to_string())
            .await
            .map_err(|_| ServiceError::NotFoundError)
            .map(|ticket| models::TicketInfo {
                id: ticket.id,
                ticket_uid: ticket.ticket_uid,
                flight_number: ticket.flight_number,
                price: ticket.price,
                status: ticket.status,
                username: ticket.username,
            })
    }

    pub async fn create_ticket(&self, token: String, input: models::TicketRequest) -> Result<()> {
        use ticket_service_api::apis as tapis;
        use ticket_service_api::models as tmodels;

        let ticket_conf = tapis::configuration::Configuration {
            base_path: self.ticket_base_path.clone(),
            client: self.client.clone(),
            ..Default::default()
        };
        let ticket_req = tmodels::TicketRequest {
            flight_number: input.flight_number,
            ticket_uid: input.ticket_uid,
            price: input.price,
            username: input.username,
            status: input.status,
        };
        tapis::ticket_restapi_operations_api::create_ticket(token, &ticket_conf, ticket_req)
            .await
            .map_err(|_| ServiceError::NotFoundError)
    }

    pub async fn get_flight(&self, token: String, flight_number: String) -> Result<models::FlightResponse> {
        use flight_service_api::apis as fapis;

        let flight_conf = fapis::configuration::Configuration {
            base_path: self.flight_base_path.clone(),
            client: self.client.clone(),
            ..Default::default()
        };
        let flight_resp =
            fapis::flight_restapi_operations_api::list_flights(token, &flight_conf, None, None, Some(&flight_number))
                .await
                .map_err(|_| ServiceError::NotFoundError)?
                .items;

        let flight_by_number = match flight_resp {
            None => Err(ServiceError::NotFoundError),
            _ => Ok(flight_resp.unwrap().first().unwrap().to_owned()),
        }?;

        let result = models::FlightResponse {
            flight_number: flight_by_number.flight_number,
            from_airport: flight_by_number.from_airport,
            to_airport: flight_by_number.to_airport,
            date: flight_by_number.date,
            price: flight_by_number.price,
        };

        Ok(result)
    }

    pub async fn delete_ticket(&self, token: String, ticket_uid: uuid::Uuid) -> Result<()> {
        use ticket_service_api::apis as tapis;

        let ticket_conf = tapis::configuration::Configuration {
            base_path: self.ticket_base_path.clone(),
            client: self.client.clone(),
            ..Default::default()
        };
        tapis::ticket_restapi_operations_api::edit_ticket1(token, &ticket_conf, &ticket_uid.to_string())
            .await
            .map_err(|_| ServiceError::NotFoundError)
    }
}

#[async_trait]
impl GatewayService for GatewayServiceImpl {
    async fn get_flights(
        &self,
        token: String,
        page: Option<i32>,
        size: Option<i32>,
    ) -> Result<models::PaginationResponse> {
        use flight_service_api::apis as fapis;

        let flight_conf = fapis::configuration::Configuration {
            base_path: self.flight_base_path.clone(),
            client: self.client.clone(),
            ..Default::default()
        };
        fapis::flight_restapi_operations_api::list_flights(token, &flight_conf, page, size, None)
            .await
            .map_err(|_| ServiceError::BadClientData)
            .map(|flights| models::PaginationResponse {
                page: flights.page,
                page_size: flights.page_size,
                total_elements: flights.total_elements,
                items: Some(
                    flights
                        .items
                        .unwrap()
                        .into_iter()
                        .map(|flight| models::FlightResponse {
                            date: flight.date,
                            flight_number: flight.flight_number,
                            from_airport: flight.from_airport,
                            to_airport: flight.to_airport,
                            price: flight.price,
                        })
                        .collect::<Vec<_>>(),
                ),
            })
    }

    async fn get_privilege_with_history(
        &self,
        token: String,
        username: String,
    ) -> Result<models::PrivilegeInfoResponse> {
        use bonus_service_api::apis as bapis;

        let bonus_conf = bapis::configuration::Configuration {
            base_path: self.bonus_base_path.clone(),
            client: self.client.clone(),
            ..Default::default()
        };
        let privilege_resp_future =
            bapis::bonus_restapi_operations_api::list_bonuses(token.clone(), &bonus_conf, Some(&username));

        let user_history_future = self.get_user_history(token, username.clone());

        let privilege_resp = privilege_resp_future.await.map_err(|_| ServiceError::BadClientData)?;

        let user_bonuses = match privilege_resp.len() {
            0 => Err(ServiceError::NotFoundError),
            _ => Ok(privilege_resp),
        }?;

        // У одного пользователя может быть только 1 аккаунт
        let user_bonus = user_bonuses.first().unwrap().to_owned();

        let user_history = user_history_future.await?;

        let result = models::PrivilegeInfoResponse {
            balance: Some(user_bonus.balance),
            history: Some(user_history),
            status: Some(user_bonus.status),
        };

        Ok(result)
    }

    async fn get_user_info(&self, token: String, username: String) -> Result<models::UserInfoResponse> {
        let tickets_future = self.get_full_tickets(token.clone(), username.clone());
        let privilege_future = self.get_privilege(token, username.clone());

        let tickets = tickets_future.await?;
        let privilege = privilege_future.await?;
        let result = models::UserInfoResponse {
            privilege: Some(Box::new(models::PrivilegeShortInfo {
                balance: Some(privilege.balance),
                status: Some(privilege.status),
            })),
            tickets: Some(tickets),
        };

        Ok(result)
    }

    async fn get_user_tickets(&self, token: String, username: String) -> Result<Vec<models::TicketResponse>> {
        self.get_full_tickets(token, username).await
    }

    async fn get_ticket_by_uid(
        &self,
        token: String,
        _: String,
        ticket_uid: uuid::Uuid,
    ) -> Result<models::TicketResponse> {
        let ticket_info = self.get_ticket(token.clone(), ticket_uid).await?;

        let flight_number = ticket_info.flight_number;
        let flight_response = self.get_flight(token, flight_number.clone()).await?;
        let result = models::TicketResponse {
            date: flight_response.date,
            ticket_uid: Some(ticket_uid),
            flight_number: Some(flight_number),
            from_airport: flight_response.from_airport,
            to_airport: flight_response.to_airport,
            price: flight_response.price,
            status: Some(ticket_info.status),
        };
        Ok(result)
    }

    async fn buy_ticket(
        &self,
        token: String,
        username: String,
        ticket_req: models::TicketPurchaseRequest,
    ) -> Result<models::TicketPurchaseResponse> {
        let flight_number = ticket_req.flight_number.unwrap();
        let flight_future = self.get_flight(token.clone(), flight_number.clone());

        let privilege_resp = match self.get_privilege(token.clone(), username.clone()).await {
            Err(ServiceError::NotFoundError) => {
                let req = models::PrivilegeCreateRequest {
                    balance: 0,
                    username: username.clone(),
                    status: String::from("BRONZE"),
                };
                self.create_privilege(token.clone(), req).await
            }
            Err(err) => Err(err),
            Ok(resp) => Ok(resp),
        }?;

        let balance_diff;
        let paid_by_money;
        let paid_by_bonuses;
        let operation;
        let paid_from_balance: Option<bool> = ticket_req.paid_from_balance;
        if paid_from_balance.is_some() && paid_from_balance.unwrap() {
            let cur_balance = privilege_resp.balance;
            let ticket_price = ticket_req.price.unwrap();
            if cur_balance < ticket_price {
                balance_diff = cur_balance;
                paid_by_money = ticket_price - cur_balance;
                paid_by_bonuses = balance_diff;
            } else {
                balance_diff = cur_balance - ticket_price;
                paid_by_money = 0;
                paid_by_bonuses = balance_diff;
            }
            operation = String::from("DEBIT_THE_ACCOUNT");
        } else {
            balance_diff = ticket_req.price.unwrap() / 10;
            paid_by_money = ticket_req.price.unwrap();
            paid_by_bonuses = 0;
            operation = String::from("FILL_IN_BALANCE");
        }

        let ticket_uid = uuid::Uuid::new_v4();
        let privilege_req = models::PrivilegeRequest {
            balance_diff,
            ticket_uid,
            username: username.clone(),
            operation,
        };
        let privilege_resp_future = self.update_privilege(token.clone(), privilege_resp.id, privilege_req);

        let ticket_service_req = models::TicketRequest {
            flight_number: flight_number.clone(),
            ticket_uid,
            price: ticket_req.price.unwrap(),
            username,
            status: String::from("PAID"),
        };
        self.create_ticket(token, ticket_service_req).await?;

        let privilege_resp = privilege_resp_future.await?;
        let flight = flight_future.await?;
        let result = models::TicketPurchaseResponse {
            date: Some(chrono::Local::now().to_string()),
            flight_number: Some(flight_number),
            from_airport: flight.from_airport,
            to_airport: flight.to_airport,
            paid_by_bonuses: Some(paid_by_bonuses),
            paid_by_money: Some(paid_by_money),
            ticket_uid: Some(ticket_uid),
            status: Some(String::from("PAID")),
            price: ticket_req.price,
            privilege: Some(Box::new(privilege_resp)),
        };

        Ok(result)
    }

    async fn return_ticket(&self, token: String, username: String, ticket_uid: uuid::Uuid) -> Result<()> {
        // Проверка, есть ли билет у пользователя
        let _ = self.get_ticket(token.clone(), ticket_uid).await?;

        let ticket_history_future = self.get_ticket_history(token.clone(), ticket_uid);
        let privilege_future = self.get_privilege(token.clone(), username.clone());

        let ticket_history = ticket_history_future.await?;
        let last_history_entry = match ticket_history.last() {
            None => return Err(ServiceError::NotFoundError),
            Some(entry) => entry,
        };

        let last_op_type = last_history_entry.clone().operation_type.unwrap();
        let balance_diff;
        let operation_to_return;
        let privilege = privilege_future.await?;
        if last_op_type == *String::from("FILL_IN_BALANCE") {
            // Если бонусы были начислены, то списываем их
            let last_balance_diff = last_history_entry.balance_diff.unwrap();
            balance_diff = if privilege.balance > last_balance_diff {
                last_balance_diff
            } else {
                privilege.balance
            };
            operation_to_return = String::from("DEBIT_THE_ACCOUNT");
        } else if last_op_type == *String::from("DEBIT_THE_ACCOUNT") {
            // Если бонусы были списаны, то начисляем их
            balance_diff = last_history_entry.balance_diff.unwrap();
            operation_to_return = String::from("FILL_IN_BALANCE");
        } else if last_op_type == *String::from("FILLED_BY_MONEY") {
            // Если покупка совершена только за деньги, то ни списываем, ни начисляем
            balance_diff = 0;
            operation_to_return = String::from("FILL_IN_BALANCE");
        } else {
            return Err(ServiceError::InternalError);
        }

        let privilege_id = privilege.id;
        let privilege_req = models::PrivilegeRequest {
            balance_diff,
            username: username.clone(),
            ticket_uid,
            operation: operation_to_return,
        };
        let update_privilege_future = self.update_privilege(token.clone(), privilege_id, privilege_req);
        let delete_ticket_future = self.delete_ticket(token, ticket_uid);
        update_privilege_future.await?;
        delete_ticket_future.await
    }
}
