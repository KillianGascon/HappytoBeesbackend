// src/models.rs
use crate::schema::*;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use chrono::NaiveDate;

#[derive(Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = ruche)]
pub struct Ruche {
    pub id: i32,
    pub id_apiculteur: Option<i32>,
    pub photo_ruche: Option<String>,
    pub numero_ruche: Option<i32>,
    pub nom_ruche: Option<String>,
    pub nombre_cadres_corp: Option<i32>,
    pub nombre_hausses: Option<i32>,
    pub nombre_cadres_hausse: Option<i32>,
    pub nombre_cadre_couvain: Option<i32>,
    pub nombre_cadre_nourriture: Option<i32>,
    pub nombre_cadre_libre: Option<i32>,
}

#[derive(Insertable, AsChangeset, Deserialize)]
#[diesel(table_name = ruche)]
#[diesel(treat_none_as_null = true)]
pub struct NewRuche {
    pub id_apiculteur: Option<i32>,
    pub photo_ruche: Option<String>,
    pub numero_ruche: Option<i32>,
    pub nom_ruche: Option<String>,
    pub nombre_cadres_corp: Option<i32>,
    pub nombre_hausses: Option<i32>,
    pub nombre_cadres_hausse: Option<i32>,
    pub nombre_cadre_couvain: Option<i32>,
    pub nombre_cadre_nourriture: Option<i32>,
    pub nombre_cadre_libre: Option<i32>,
}

#[derive(AsChangeset, Deserialize, Serialize)]
#[diesel(table_name = ruche)]
#[diesel(treat_none_as_null = true)]
pub struct UpdateRuche {
    pub id: i32,
    pub id_apiculteur: Option<i32>,
    pub photo_ruche: Option<String>,
    pub numero_ruche: Option<i32>,
    pub nom_ruche: Option<String>,
    pub nombre_cadres_corp: Option<i32>,
    pub nombre_hausses: Option<i32>,
    pub nombre_cadres_hausse: Option<i32>,
    pub nombre_cadre_couvain: Option<i32>,
    pub nombre_cadre_nourriture: Option<i32>,
    pub nombre_cadre_libre: Option<i32>,
}
#[derive(Insertable, Deserialize)]
#[diesel(table_name = ruche)]
pub struct NewRucheWithId {
    pub id: i32,
    pub id_apiculteur: Option<i32>,
    pub photo_ruche: Option<String>,
    pub numero_ruche: Option<i32>,
    pub nom_ruche: Option<String>,
    pub nombre_cadres_corp: Option<i32>,
    pub nombre_hausses: Option<i32>,
    pub nombre_cadres_hausse: Option<i32>,
    pub nombre_cadre_couvain: Option<i32>,
    pub nombre_cadre_nourriture: Option<i32>,
    pub nombre_cadre_libre: Option<i32>,
}

#[derive(AsChangeset, Deserialize)]
#[diesel(table_name = ruche)]
pub struct UpdateRucheWithId {
    pub id: i32,
    pub id_apiculteur: Option<i32>,
    pub photo_ruche: Option<String>,
    pub numero_ruche: Option<i32>,
    pub nom_ruche: Option<String>,
    pub nombre_cadres_corp: Option<i32>,
    pub nombre_hausses: Option<i32>,
    pub nombre_cadres_hausse: Option<i32>,
    pub nombre_cadre_couvain: Option<i32>,
    pub nombre_cadre_nourriture: Option<i32>,
    pub nombre_cadre_libre: Option<i32>,
}