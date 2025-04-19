use crate::schema::*;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use chrono::NaiveDate;

#[derive(Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = interventions)]
pub struct Intervention {
    pub id: i32,
    pub id_ruche: Option<i32>,
    pub date_intervention: Option<NaiveDate>,
    pub description_intervention: Option<String>,
    pub photo_intervention: Option<String>,
}

#[derive(Insertable, AsChangeset, Deserialize)]
#[diesel(table_name = interventions)]
pub struct NewIntervention {
    pub id_ruche: Option<i32>,
    pub date_intervention: Option<NaiveDate>,
    pub description_intervention: Option<String>,
    pub photo_intervention: Option<String>,
}

#[derive(AsChangeset, Deserialize)]
#[diesel(table_name = interventions)]
pub struct UpdateIntervention {
    pub id: i32,
    pub id_ruche: Option<i32>,
    pub date_intervention: Option<NaiveDate>,
    pub description_intervention: Option<String>,
    pub photo_intervention: Option<String>,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = interventions)]
pub struct NewInterventionWithId {
    pub id: i32,
    pub id_ruche: Option<i32>,
    pub date_intervention: Option<NaiveDate>,
    pub description_intervention: Option<String>,
    pub photo_intervention: Option<String>,
}

#[derive(AsChangeset, Deserialize)]
#[diesel(table_name = interventions)]
pub struct UpdateInterventionWithId {
    pub id: i32,
    pub id_ruche: Option<i32>,
    pub date_intervention: Option<NaiveDate>,
    pub description_intervention: Option<String>,
    pub photo_intervention: Option<String>,
}