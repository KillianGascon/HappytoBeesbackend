use crate::db::DbConnection;
use crate::models::materiel_models::{Materiel, NewMateriel, UpdateMateriel};
use crate::schema::materiel;
use diesel::prelude::*;
use diesel::result::Error;

/// Récupère tous les matériels
pub fn get_all_materiels(conn: &mut DbConnection) -> Result<Vec<Materiel>, Error> {
    materiel::table.load::<Materiel>(conn)
}

/// Récupère un matériel par son ID
pub fn get_materiel_by_id(conn: &mut DbConnection, id: i32) -> Result<Materiel, Error> {
    materiel::table.find(id).first::<Materiel>(conn)
}

/// Crée un nouveau matériel
pub fn create_materiel(conn: &mut DbConnection, new_materiel: NewMateriel) -> Result<Materiel, Error> {
    diesel::insert_into(materiel::table)
        .values(&new_materiel)
        .get_result(conn)
}

/// Met à jour un matériel existant
pub fn update_materiel(conn: &mut DbConnection, id: i32, updated_materiel: UpdateMateriel) -> Result<Materiel, Error> {
    diesel::update(materiel::table.find(id))
        .set(&updated_materiel)
        .get_result(conn)
}

/// Supprime un matériel
pub fn delete_materiel(conn: &mut DbConnection, id: i32) -> Result<usize, Error> {
    diesel::delete(materiel::table.find(id)).execute(conn)
}

/// Récupère les matériels par type
pub fn get_materiels_by_type(conn: &mut DbConnection, type_materiel: String) -> Result<Vec<Materiel>, Error> {
    materiel::table
        .filter(materiel::type_materiel.eq(type_materiel))
        .load::<Materiel>(conn)
}

/// Récupère les matériels par état
pub fn get_materiels_by_etat(conn: &mut DbConnection, etat: String) -> Result<Vec<Materiel>, Error> {
    materiel::table
        .filter(materiel::etat_materiel.eq(etat))
        .load::<Materiel>(conn)
}