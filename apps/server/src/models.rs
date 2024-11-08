use crate::schema::tickets::dsl::*;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(
    Queryable,
    Selectable,
    Insertable,
    AsChangeset,
    Identifiable,
    Clone,
    Debug,
    Serialize,
    Deserialize,
)]
#[diesel(table_name = crate::schema::tickets)]
#[diesel(primary_key(id))]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Ticket {
    #[diesel(deserialize_as = i32)]
    pub id: Option<i32>,
    pub ticket: String,
    pub valid: bool,
}

impl Ticket {
    pub fn insert(&self, connection: &mut SqliteConnection) -> QueryResult<usize> {
        diesel::insert_into(crate::schema::tickets::table)
            .values(self)
            .execute(connection)
    }

    pub fn update(&self, connection: &mut SqliteConnection) -> QueryResult<usize> {
        diesel::update(tickets)
            .filter(ticket.eq(&self.ticket))
            .set(self)
            .execute(connection)
    }

    pub fn select(connection: &mut SqliteConnection, value: &str) -> QueryResult<Ticket> {
        tickets.filter(ticket.eq(value)).first(connection)
    }

    pub fn count(connection: &mut SqliteConnection) -> QueryResult<i64> {
        tickets.count().get_result(connection)
    }
}
