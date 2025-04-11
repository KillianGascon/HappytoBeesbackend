use crate::schema::*;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use chrono::NaiveDate;

#[derive(Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = poids)]
pub struct Poids {
    pub id: i32,
    pub id_ruche: Option<i32>,
    pub poids_ruche: Option<i32>,
    pub date_creation: Option<NaiveDate>,
}

#[derive(Insertable, AsChangeset, Deserialize)]
#[diesel(table_name = poids)]
pub struct NewPoids {
    pub id_ruche: Option<i32>,
    pub poids_ruche: Option<i32>,
    pub date_creation: Option<NaiveDate>,
}

#[derive(AsChangeset, Deserialize)]
#[diesel(table_name = poids)]
pub struct UpdatePoids {
    pub id: i32,
    pub id_ruche: Option<i32>,
    pub poids_ruche: Option<i32>,
    pub date_creation: Option<NaiveDate>,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = poids)]
pub struct NewPoidsWithId {
    pub id: i32,
    pub id_ruche: Option<i32>,
    pub poids_ruche: Option<i32>,
    pub date_creation: Option<NaiveDate>,
}

#[derive(AsChangeset, Deserialize)]
#[diesel(table_name = poids)]
pub struct UpdatePoidsWithId {
    pub id: i32,
    pub id_ruche: Option<i32>,
    pub poids_ruche: Option<i32>,
    pub date_creation: Option<NaiveDate>,
}