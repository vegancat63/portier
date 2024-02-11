use diesel::prelude::*;
// use crate::schema::accounts;
use serde::Serialize;

#[derive(Queryable, Selectable, Serialize)]
#[diesel(table_name = crate::schema::accounts)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Account {
    pub id: i32,
    pub title: String,
    pub description: Option<String>,
    pub account_type: Option<String>,
    pub protocol: Option<String>,
    pub login: Option<String>,
    pub password: Option<String>,
    pub host: Option<String>,
    pub port: Option<String>,
    pub url: Option<String>,
}
