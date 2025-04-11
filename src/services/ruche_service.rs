use crate::db::DbConnection;
use crate::models::ruche_models::{Ruche, NewRuche, UpdateRuche};
use crate::schema::ruche;
use diesel::prelude::*;
use diesel::result::Error;

/// Récupère toutes les ruches
pub fn get_all_ruches(conn: &mut DbConnection) -> Result<Vec<Ruche>, Error> {
    ruche::table.load::<Ruche>(conn)
}

/// Récupère une ruche par son ID
pub fn get_ruche_by_id(conn: &mut DbConnection, id: i32) -> Result<Ruche, Error> {
    ruche::table.find(id).first::<Ruche>(conn)
}

/// Crée une nouvelle ruche
pub fn create_ruche(conn: &mut DbConnection, new_ruche: NewRuche) -> Result<Ruche, Error> {
    diesel::insert_into(ruche::table)
        .values(&new_ruche)
        .get_result(conn)
}

/// Met à jour une ruche existante
pub fn update_ruche(conn: &mut DbConnection, id: i32, updated_ruche: UpdateRuche) -> Result<Ruche, Error> {
    diesel::update(ruche::table.find(id))
        .set(&updated_ruche)
        .get_result(conn)
}

/// Supprime une ruche
pub fn delete_ruche(conn: &mut DbConnection, id: i32) -> Result<usize, Error> {
    diesel::delete(ruche::table.find(id)).execute(conn)
}

pub fn get_ruche_by_nom(conn: &mut DbConnection, nom: String) -> Result<Vec<Ruche>, Error> {
    ruche::table.filter(ruche::nom_ruche.eq(nom)).load::<Ruche>(conn)
}

// Fonctions supplémentaires pour enrichir le service
pub fn get_ruches_by_user_id(conn: &mut DbConnection, user_id: i32) -> Result<Vec<Ruche>, Error> {
    ruche::table.filter(ruche::id_apiculteur.eq(user_id)).load::<Ruche>(conn)
}
//pub fn get_ruches_by_numero_ruche(conn: &mut DbConnection, numero_ruche: String) -> Result<Vec<Ruche>, Error> {
//    ruche::table.filter(ruche::numero_ruche.eq(numero_ruche)).load::<Ruche>(conn)
//}
pub fn count_ruches(conn: &mut DbConnection) -> Result<i64, Error> {
    ruche::table.count().get_result(conn)
}