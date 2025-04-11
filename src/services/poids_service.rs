use crate::db::DbConnection;
use crate::models::poids_models::{Poids, NewPoids, UpdatePoids};
use crate::schema::poids;
use diesel::prelude::*;
use diesel::result::Error;
use chrono::{NaiveDate, Datelike};
use std::collections::HashMap;

/// Récupère tous les poids
pub fn get_all_poids(conn: &mut DbConnection) -> Result<Vec<Poids>, Error> {
    poids::table.load::<Poids>(conn)
}

/// Récupère un poids par son ID
pub fn get_poids_by_id(conn: &mut DbConnection, id: i32) -> Result<Poids, Error> {
    poids::table.find(id).first::<Poids>(conn)
}

/// Crée un nouveau poids
pub fn create_poids(conn: &mut DbConnection, new_poids: NewPoids) -> Result<Poids, Error> {
    diesel::insert_into(poids::table)
        .values(&new_poids)
        .get_result(conn)
}

/// Met à jour un poids existant
pub fn update_poids(conn: &mut DbConnection, id: i32, updated_poids: UpdatePoids) -> Result<Poids, Error> {
    diesel::update(poids::table.find(id))
        .set(&updated_poids)
        .get_result(conn)
}

/// Supprime un poids
pub fn delete_poids(conn: &mut DbConnection, id: i32) -> Result<usize, Error> {
    diesel::delete(poids::table.find(id)).execute(conn)
}

/// Récupère les poids d'une ruche spécifique
pub fn get_poids_by_ruche_id(conn: &mut DbConnection, ruche_id: i32) -> Result<Vec<Poids>, Error> {
    poids::table
        .filter(poids::id_ruche.eq(ruche_id))
        .load::<Poids>(conn)
}

/// Récupère les poids d'une période spécifique
pub fn get_poids_by_date_range(
    conn: &mut DbConnection,
    start_date: chrono::NaiveDate,
    end_date: chrono::NaiveDate
) -> Result<Vec<Poids>, Error> {
    poids::table
        .filter(poids::date_creation.between(start_date, end_date))
        .load::<Poids>(conn)
}

/// Récupère le dernier poids enregistré pour une ruche
pub fn get_last_poids_by_ruche_id(conn: &mut DbConnection, ruche_id: i32) -> Result<Option<Poids>, Error> {
    poids::table
        .filter(poids::id_ruche.eq(ruche_id))
        .order(poids::date_creation.desc())
        .first::<Poids>(conn)
        .optional()
}

pub fn get_annual_average_weight(
    conn: &mut DbConnection,
    ruche_id: i32,
    year: i32
) -> Result<f64, Error> {
    use crate::schema::poids::dsl::*;

    let start_date = NaiveDate::from_ymd_opt(year, 1, 1).unwrap();
    let end_date = NaiveDate::from_ymd_opt(year + 1, 1, 1).unwrap();

    let poids_records = poids
        .filter(id_ruche.eq(ruche_id))
        .filter(date_creation.ge(start_date))
        .filter(date_creation.lt(end_date))
        .select((poids_ruche, date_creation))
        .load::<(Option<i32>, Option<NaiveDate>)>(conn)?;

    if poids_records.is_empty() {
        return Ok(0.0);
    }

    let mut total_weight = 0;
    let mut count = 0;

    for (weight, _date) in poids_records {
        if let Some(w) = weight {
            total_weight += w;
            count += 1;
        }
    }

    if count > 0 {
        Ok(total_weight as f64 / count as f64)
    } else {
        Ok(0.0)
    }
}

pub fn get_annual_average_weight_by_month(
    conn: &mut DbConnection,
    ruche_id: i32,
    year: i32
) -> Result<HashMap<u32, f64>, Error> {
    use crate::schema::poids::dsl::*;

    let start_date = NaiveDate::from_ymd_opt(year, 1, 1).unwrap();
    let end_date = NaiveDate::from_ymd_opt(year + 1, 1, 1).unwrap();

    let poids_records = poids
        .filter(id_ruche.eq(ruche_id))
        .filter(date_creation.ge(start_date))
        .filter(date_creation.lt(end_date))
        .select((poids_ruche, date_creation))
        .load::<(Option<i32>, Option<NaiveDate>)>(conn)?;

    let mut monthly_weights: HashMap<u32, Vec<i32>> = HashMap::new();

    // Regrouper les poids par mois
    for (weight, date) in poids_records {
        if let (Some(w), Some(d)) = (weight, date) {
            let month = d.month();
            monthly_weights.entry(month).or_insert_with(Vec::new).push(w);
        }
    }

    // Calculer la moyenne pour chaque mois
    let mut result: HashMap<u32, f64> = HashMap::new();
    for (month, weights) in monthly_weights {
        let sum: i32 = weights.iter().sum();
        let count = weights.len() as f64;
        result.insert(month, sum as f64 / count);
    }

    Ok(result)
}

pub fn get_weight_evolution(
    conn: &mut DbConnection,
    ruche_id: i32,
    years: Vec<i32>
) -> Result<HashMap<i32, f64>, Error> {
    let mut result: HashMap<i32, f64> = HashMap::new();

    for year in years {
        let avg = get_annual_average_weight(conn, ruche_id, year)?;
        result.insert(year, avg);
    }

    Ok(result)
}