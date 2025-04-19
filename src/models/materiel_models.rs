use crate::schema::*;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = materiel)]
pub struct Materiel {
    pub id: i32,
    pub id_ruche: Option<i32>,
    pub nom_materiel: Option<String>,
    pub type_materiel: Option<String>,
    pub etat_materiel: Option<String>,
}

#[derive(Insertable, AsChangeset, Deserialize)]
#[diesel(table_name = materiel)]
pub struct NewMateriel {
    pub id_ruche: Option<i32>,
    pub nom_materiel: Option<String>,
    pub type_materiel: Option<String>,
    pub etat_materiel: Option<String>,
}

#[derive(AsChangeset, Deserialize)]
#[diesel(table_name = materiel)]
pub struct UpdateMateriel {
    pub id: i32,
    pub id_ruche: Option<i32>,
    pub nom_materiel: Option<String>,
    pub type_materiel: Option<String>,
    pub etat_materiel: Option<String>,
}
#[derive(Insertable, Deserialize)]
#[diesel(table_name = materiel)]
pub struct NewMaterielWithId {
    pub id: i32,
    pub id_ruche: Option<i32>,
    pub nom_materiel: Option<String>,
    pub type_materiel: Option<String>,
    pub etat_materiel: Option<String>,
}

#[derive(AsChangeset, Deserialize)]
#[diesel(table_name = materiel)]
pub struct UpdateMaterielWithId {
    pub id: i32,
    pub id_ruche: Option<i32>,
    pub nom_materiel: Option<String>,
    pub type_materiel: Option<String>,
    pub etat_materiel: Option<String>,
}
