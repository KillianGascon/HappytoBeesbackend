use crate::schema::utilisateur;
use chrono::NaiveDate;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = utilisateur)]
pub struct Utilisateur {
    pub id: i32,
    pub nom_apiculteur: Option<String>,
    pub prenom_apiculteur: Option<String>,
    pub mail: Option<String>,
    pub telephone: Option<String>,
    pub mot_de_passe: Option<String>,
    pub numero_apiculteur: Option<i32>,
    pub date_naissance: Option<NaiveDate>,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = utilisateur)]
#[diesel(treat_none_as_null = true)]
pub struct NewUtilisateur {
    pub nom_apiculteur: Option<String>,
    pub prenom_apiculteur: Option<String>,
    pub mail: Option<String>,
    pub telephone: Option<String>,
    pub mot_de_passe: Option<String>,
    pub numero_apiculteur: Option<i32>,
    pub date_naissance: Option<NaiveDate>,
}

#[derive(AsChangeset, Deserialize, Serialize)]
#[diesel(table_name = utilisateur)]
#[diesel(treat_none_as_null = true)]
pub struct UpdateUtilisateur {
    pub nom_apiculteur: Option<String>,
    pub prenom_apiculteur: Option<String>,
    pub mail: Option<String>,
    pub telephone: Option<String>,
    pub mot_de_passe: Option<String>,
    pub numero_apiculteur: Option<i32>,
    pub date_naissance: Option<NaiveDate>,
}

#[derive(Serialize, Deserialize)]
pub struct TokenClaims {
    pub sub: String,
    pub user_id: i32,
    pub exp: usize,
}

#[derive(Serialize)]
pub struct AuthResponse {
    pub token: String,
    pub user: Utilisateur,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginCredentials {
    pub email: String,
    pub password: String,
}