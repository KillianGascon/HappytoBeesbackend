use crate::db::DbConnection;
use crate::models::production_models::{Production, NewProduction, UpdateProduction};
use crate::schema::production;
use diesel::prelude::*;
use diesel::result::Error;

/// Récupère toutes les productions
pub fn get_all_productions(conn: &mut DbConnection) -> Result<Vec<Production>, Error> {
    production::table.load::<Production>(conn)
}

/// Récupère une production par son ID
pub fn get_production_by_id(conn: &mut DbConnection, id: i32) -> Result<Production, Error> {
    production::table.find(id).first::<Production>(conn)
}

/// Crée une nouvelle production
pub fn create_production(conn: &mut DbConnection, new_production: NewProduction) -> Result<Production, Error> {
    diesel::insert_into(production::table)
        .values(&new_production)
        .get_result(conn)
}

/// Met à jour une production existante
pub fn update_production(conn: &mut DbConnection, id: i32, updated_production: UpdateProduction) -> Result<Production, Error> {
    diesel::update(production::table.find(id))
        .set(&updated_production)
        .get_result(conn)
}

/// Supprime une production
pub fn delete_production(conn: &mut DbConnection, id: i32) -> Result<usize, Error> {
    diesel::delete(production::table.find(id)).execute(conn)
}

/// Récupère les productions d'une ruche spécifique
pub fn get_productions_by_ruche_id(conn: &mut DbConnection, ruche_id: i32) -> Result<Vec<Production>, Error> {
    production::table
        .filter(production::id_ruche.eq(ruche_id))
        .load::<Production>(conn)
}

/// Récupère les productions d'une période spécifique
pub fn get_productions_by_date_range(
    conn: &mut DbConnection,
    start_date: chrono::NaiveDate,
    end_date: chrono::NaiveDate
) -> Result<Vec<Production>, Error> {
    production::table
        .filter(production::date_creation.between(start_date, end_date))
        .load::<Production>(conn)
}

// Statistiques de production
pub fn get_total_production_by_ruche(conn: &mut DbConnection, ruche_id: i32) -> Result<i32, Error> {
    use crate::schema::production::dsl::*;
    use diesel::dsl::sum;

    production
        .filter(id_ruche.eq(ruche_id))
        .select(sum(quantite_production))
        .first::<Option<i64>>(conn)
        .map(|result| result.unwrap_or(0) as i32)
}

pub fn get_production_by_date_range(
    conn: &mut DbConnection,
    ruche_id: i32,
    start_date: chrono::NaiveDate,
    end_date: chrono::NaiveDate,
) -> Result<Vec<Production>, Error> {
    use crate::schema::production::dsl::*;

    production
        .filter(id_ruche.eq(ruche_id))
        .filter(date_creation.ge(start_date))
        .filter(date_creation.le(end_date))
        .order(date_creation.asc())
        .load::<Production>(conn)
}