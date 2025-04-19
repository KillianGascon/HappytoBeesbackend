use crate::schema::*;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use chrono::{NaiveDateTime};

#[derive(Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = sessions)]
pub struct Session {
    pub id: i32,
    pub id_utilisateur: Option<i32>,
    pub token: Option<String>,
    pub user_agent: Option<String>,
    pub ip_address: Option<String>,
    pub date_creation: Option<NaiveDateTime>,
    pub date_expiration: Option<NaiveDateTime>,
    pub est_valide: Option<bool>,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = sessions)]
pub struct NewSession {
    pub id_utilisateur: Option<i32>,
    pub token: Option<String>,
    pub user_agent: Option<String>,
    pub ip_address: Option<String>,
    pub date_creation: Option<NaiveDateTime>,
    pub date_expiration: Option<NaiveDateTime>,
    pub est_valide: Option<bool>,
}

#[derive(AsChangeset, Deserialize)]
#[diesel(table_name = sessions)]
pub struct UpdateSession {
    pub id_utilisateur: Option<i32>,
    pub token: Option<String>,
    pub user_agent: Option<String>,
    pub ip_address: Option<String>,
    pub date_creation: Option<NaiveDateTime>,
    pub date_expiration: Option<NaiveDateTime>,
    pub est_valide: Option<bool>,
}

// Structure pour la réponse simplifiée
#[derive(Serialize)]
pub struct SessionResponse {
    pub id: i32,
    pub token: String,
    pub date_expiration: NaiveDateTime,
}