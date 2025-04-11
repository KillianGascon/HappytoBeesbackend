use crate::db::DbConnection;
use crate::models::intervention_models::{Intervention, NewIntervention, UpdateIntervention};
use crate::schema::interventions;
use diesel::prelude::*;
use diesel::result::Error;

/// Récupère toutes les interventions
pub fn get_all_interventions(conn: &mut DbConnection) -> Result<Vec<Intervention>, Error> {
    interventions::table.load::<Intervention>(conn)
}

/// Récupère une intervention par son ID
pub fn get_intervention_by_id(conn: &mut DbConnection, id: i32) -> Result<Intervention, Error> {
    interventions::table.find(id).first::<Intervention>(conn)
}

/// Crée une nouvelle intervention
pub fn create_intervention(conn: &mut DbConnection, new_intervention: NewIntervention) -> Result<Intervention, Error> {
    diesel::insert_into(interventions::table)
        .values(&new_intervention)
        .get_result(conn)
}

/// Met à jour une intervention existante
pub fn update_intervention(conn: &mut DbConnection, id: i32, updated_intervention: UpdateIntervention) -> Result<Intervention, Error> {
    diesel::update(interventions::table.find(id))
        .set(&updated_intervention)
        .get_result(conn)
}

/// Récupère les interventions associées à une ruche spécifique
pub fn get_interventions_by_ruche_id(conn: &mut DbConnection, ruche_id: i32) -> Result<Vec<Intervention>, Error> {
    interventions::table
        .filter(interventions::id_ruche.eq(ruche_id))
        .load::<Intervention>(conn)
}