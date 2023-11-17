use actix::prelude::*;
use anyhow::Result;
use diesel::prelude::*;
use diesel::{self, QueryDsl, RunQueryDsl};
use serde::Deserialize;

use super::database_executor::DatabaseExecutor;
use crate::app::domain::ticket::NewTicket;
use crate::app::domain::ticket::Ticket;
use crate::schema::ticket;
use crate::schema::ticket::dsl;

/// Get all tickets message
pub struct GetTickets {
    pub username: Option<String>,
    pub flight_number: Option<String>,
}

impl Message for GetTickets {
    type Result = Result<Vec<Ticket>>;
}

impl Handler<GetTickets> for DatabaseExecutor {
    type Result = Result<Vec<Ticket>>;

    fn handle(&mut self, msg: GetTickets, _: &mut Self::Context) -> Self::Result {
        let mut conn = self.0.get()?;

        let mut predicate = dsl::ticket.into_boxed();
        if let Some(username) = msg.username {
            predicate = predicate.filter(dsl::username.eq(username));
        }
        if let Some(flight_number) = msg.flight_number {
            predicate = predicate.filter(dsl::flight_number.eq(flight_number));
        }

        let tickets = predicate.load::<Ticket>(&mut conn)?;

        Ok(tickets)
    }
}

/// The get ticket message
pub struct GetTicket {
    pub ticket_uid: uuid::Uuid,
}

impl Message for GetTicket {
    type Result = Result<Ticket>;
}

impl Handler<GetTicket> for DatabaseExecutor {
    type Result = Result<Ticket>;

    fn handle(&mut self, msg: GetTicket, _: &mut Self::Context) -> Self::Result {
        let mut conn = self.0.get()?;
        let predicate = dsl::ticket;
        let result = predicate.filter(dsl::ticket_uid.eq(msg.ticket_uid)).first(&mut conn)?;
        Ok(result)
    }
}

/// The create ticket message
pub struct CreateTicket(pub Ticket);

impl Message for CreateTicket {
    type Result = Result<Ticket>;
}

impl Handler<CreateTicket> for DatabaseExecutor {
    type Result = Result<Ticket>;

    fn handle(&mut self, msg: CreateTicket, _: &mut Self::Context) -> Self::Result {
        let mut conn = self.0.get()?;

        let new_ticket = NewTicket {
            ticket_uid: msg.0.ticket_uid,
            flight_number: msg.0.flight_number,
            price: msg.0.price,
            status: msg.0.status,
            username: msg.0.username,
        };

        let result: Ticket = diesel::insert_into(ticket::table)
            .values(&new_ticket)
            .get_result(&mut conn)?;

        Ok(result)
    }
}

/// The update ticket message
#[derive(Deserialize)]
pub struct UpdateTicket {
    pub ticket_uid: uuid::Uuid,
    pub new_ticket: Ticket,
}

impl Message for UpdateTicket {
    type Result = Result<Ticket>;
}

impl Handler<UpdateTicket> for DatabaseExecutor {
    type Result = Result<Ticket>;

    fn handle(&mut self, msg: UpdateTicket, _: &mut Self::Context) -> Self::Result {
        let mut conn = self.0.get()?;

        conn.transaction(|conn| {
            let queried: Ticket = dsl::ticket.filter(dsl::ticket_uid.eq(msg.ticket_uid)).first(conn)?;

            let updated = diesel::update(dsl::ticket.find(queried.id))
                .set((
                    dsl::id.eq(queried.id),
                    dsl::ticket_uid.eq(msg.new_ticket.ticket_uid),
                    dsl::username.eq(msg.new_ticket.username),
                    dsl::flight_number.eq(msg.new_ticket.flight_number),
                    dsl::price.eq(msg.new_ticket.price),
                    dsl::status.eq(msg.new_ticket.status),
                ))
                .get_result::<Ticket>(conn)?;

            Ok(updated)
        })
    }
}

/// The delete ticket message, needs a token
pub struct DeleteTicket {
    pub ticket_uid: uuid::Uuid,
}

impl Message for DeleteTicket {
    type Result = Result<()>;
}

impl Handler<DeleteTicket> for DatabaseExecutor {
    type Result = Result<()>;

    fn handle(&mut self, msg: DeleteTicket, _: &mut Self::Context) -> Self::Result {
        let mut conn = self.0.get()?;
        let _: Ticket = dsl::ticket
            .filter(dsl::ticket_uid.eq(msg.ticket_uid))
            .first(&mut conn)?;
        diesel::delete(dsl::ticket.filter(dsl::ticket_uid.eq(msg.ticket_uid))).execute(&mut conn)?;
        Ok(())
    }
}
