use crate::schema::*;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use chrono::NaiveDate;

#[derive(Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = production)]
pub struct Production {
    pub id: i32,
    pub id_ruche: Option<i32>,
    pub quantite_production: Option<i32>,
    pub date_creation: Option<NaiveDate>,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = production)]
pub struct NewProduction {
    pub id_ruche: Option<i32>,
    pub quantite_production: Option<i32>,
    pub date_creation: Option<NaiveDate>,
}

#[derive(AsChangeset, Deserialize)]
#[diesel(table_name = production)]
pub struct UpdateProduction {
    pub id_ruche: Option<i32>,
    pub quantite_production: Option<i32>,
    pub date_creation: Option<NaiveDate>,
}