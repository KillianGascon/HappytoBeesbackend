use actix_web::{web, HttpResponse, Result, get, post, put, delete};
use crate::db::Pool;
use crate::models::poids_models::{NewPoids, Poids, UpdatePoids};
use crate::services::poids_service;
use chrono::NaiveDate;
use serde::Deserialize;
use log::{error};
use serde::Serialize;

// Fonction auxiliaire pour la gestion des résultats de requête DB
async fn handle_db_result<T: Serialize>(
    result: Result<T, actix_web::Error>
) -> Result<HttpResponse> {
    match result {
        Ok(data) => Ok(HttpResponse::Ok().json(data)),
        Err(e) => {
            let msg = format!("Erreur: {}", e);
            error!("{}", msg);
            Ok(HttpResponse::InternalServerError().json(msg))
        }
    }
}

// Fonction auxiliaire pour obtenir la connexion à la base de données
fn get_connection(pool: &web::Data<Pool>) -> Result<diesel::r2d2::PooledConnection<diesel::r2d2::ConnectionManager<diesel::PgConnection>>, HttpResponse> {
    pool.get().map_err(|e| {
        let msg = format!("Erreur de connexion à la base de données: {}", e);
        error!("{}", msg);
        HttpResponse::InternalServerError().json(msg)
    })
}

#[get("/poids")]
pub async fn get_all_poids(pool: web::Data<Pool>) -> Result<HttpResponse> {
    let mut conn = match get_connection(&pool) {
        Ok(conn) => conn,
        Err(e) => return Ok(e),
    };

    let result = match web::block(move || poids_service::get_all_poids(&mut conn))
        .await {
        Ok(result) => result,
        Err(e) => {
            error!("Erreur lors de l'exécution de la requête: {}", e);
            return Ok(HttpResponse::InternalServerError().json(format!("Erreur de serveur: {}", e)));
        }
    };

    match result {
        Ok(poids) => Ok(HttpResponse::Ok().json(poids)),
        Err(e) => {
            error!("Erreur de base de données: {}", e);
            Ok(HttpResponse::InternalServerError().json(format!("Erreur de base de données: {}", e)))
        }
    }
}

#[get("/poids/{id}")]
pub async fn get_poids_by_id(pool: web::Data<Pool>, id: web::Path<i32>) -> Result<HttpResponse> {
    let mut conn = match get_connection(&pool) {
        Ok(conn) => conn,
        Err(e) => return Ok(e),
    };

    let result = match web::block(move || {
        poids_service::get_poids_by_id(&mut conn, id.into_inner())
    }).await {
        Ok(result) => result,
        Err(e) => {
            error!("Erreur lors de l'exécution de la requête: {}", e);
            return Ok(HttpResponse::InternalServerError().json(format!("Erreur de serveur: {}", e)));
        }
    };

    match result {
        Ok(poids) => Ok(HttpResponse::Ok().json(poids)),
        Err(e) => {
            error!("Erreur de base de données: {}", e);
            Ok(HttpResponse::InternalServerError().json(format!("Erreur de base de données: {}", e)))
        }
    }
}

#[get("/poids/ruche/{ruche_id}")]
pub async fn get_poids_by_ruche_id(pool: web::Data<Pool>, ruche_id: web::Path<i32>) -> Result<HttpResponse> {
    let mut conn = match get_connection(&pool) {
        Ok(conn) => conn,
        Err(e) => return Ok(e),
    };

    let result = match web::block(move || {
        poids_service::get_poids_by_ruche_id(&mut conn, ruche_id.into_inner())
    }).await {
        Ok(result) => result,
        Err(e) => {
            error!("Erreur lors de l'exécution de la requête: {}", e);
            return Ok(HttpResponse::InternalServerError().json(format!("Erreur de serveur: {}", e)));
        }
    };

    match result {
        Ok(poids) => Ok(HttpResponse::Ok().json(poids)),
        Err(e) => {
            error!("Erreur de base de données: {}", e);
            Ok(HttpResponse::InternalServerError().json(format!("Erreur de base de données: {}", e)))
        }
    }
}

#[get("/poids/ruche/{ruche_id}/last")]
pub async fn get_last_poids_by_ruche_id(pool: web::Data<Pool>, ruche_id: web::Path<i32>) -> Result<HttpResponse> {
    let mut conn = match get_connection(&pool) {
        Ok(conn) => conn,
        Err(e) => return Ok(e),
    };

    let result = match web::block(move || {
        poids_service::get_last_poids_by_ruche_id(&mut conn, ruche_id.into_inner())
    }).await {
        Ok(result) => result,
        Err(e) => {
            error!("Erreur lors de l'exécution de la requête: {}", e);
            return Ok(HttpResponse::InternalServerError().json(format!("Erreur de serveur: {}", e)));
        }
    };

    match result {
        Ok(Some(poids)) => Ok(HttpResponse::Ok().json(poids)),
        Ok(None) => Ok(HttpResponse::NotFound().json("Aucun poids trouvé pour cette ruche")),
        Err(e) => {
            error!("Erreur de base de données: {}", e);
            Ok(HttpResponse::InternalServerError().json(format!("Erreur de base de données: {}", e)))
        }
    }
}

#[get("/poids/date-range")]
pub async fn get_poids_by_date_range(
    pool: web::Data<Pool>,
    query: web::Query<(String, String)>
) -> Result<HttpResponse> {
    let mut conn = match get_connection(&pool) {
        Ok(conn) => conn,
        Err(e) => return Ok(e),
    };

    let (start_date_str, end_date_str) = query.into_inner();
    let start_date = match NaiveDate::parse_from_str(&start_date_str, "%Y-%m-%d") {
        Ok(date) => date,
        Err(_) => return Ok(HttpResponse::BadRequest().json("Format de date invalide pour start_date")),
    };
    let end_date = match NaiveDate::parse_from_str(&end_date_str, "%Y-%m-%d") {
        Ok(date) => date,
        Err(_) => return Ok(HttpResponse::BadRequest().json("Format de date invalide pour end_date")),
    };

    let result = match web::block(move || {
        poids_service::get_poids_by_date_range(&mut conn, start_date, end_date)
    }).await {
        Ok(result) => result,
        Err(e) => {
            error!("Erreur lors de l'exécution de la requête: {}", e);
            return Ok(HttpResponse::InternalServerError().json(format!("Erreur de serveur: {}", e)));
        }
    };

    match result {
        Ok(poids) => Ok(HttpResponse::Ok().json(poids)),
        Err(e) => {
            error!("Erreur de base de données: {}", e);
            Ok(HttpResponse::InternalServerError().json(format!("Erreur de base de données: {}", e)))
        }
    }
}

#[post("/poids")]
pub async fn create_poids(pool: web::Data<Pool>, new_poids: web::Json<NewPoids>) -> Result<HttpResponse> {
    let mut conn = match get_connection(&pool) {
        Ok(conn) => conn,
        Err(e) => return Ok(e),
    };

    let result = match web::block(move || {
        poids_service::create_poids(&mut conn, new_poids.into_inner())
    }).await {
        Ok(result) => result,
        Err(e) => {
            error!("Erreur lors de l'exécution de la requête: {}", e);
            return Ok(HttpResponse::InternalServerError().json(format!("Erreur de serveur: {}", e)));
        }
    };

    match result {
        Ok(poids) => Ok(HttpResponse::Created().json(poids)),
        Err(e) => {
            error!("Erreur lors de la création du poids: {}", e);
            Ok(HttpResponse::InternalServerError().json(format!("Erreur de base de données: {}", e)))
        }
    }
}

#[put("/poids/{id}")]
pub async fn update_poids(
    pool: web::Data<Pool>,
    id: web::Path<i32>,
    poids: web::Json<UpdatePoids>
) -> Result<HttpResponse> {
    let mut conn = match get_connection(&pool) {
        Ok(conn) => conn,
        Err(e) => return Ok(e),
    };

    let result = match web::block(move || {
        poids_service::update_poids(&mut conn, id.into_inner(), poids.into_inner())
    }).await {
        Ok(result) => result,
        Err(e) => {
            error!("Erreur lors de l'exécution de la requête: {}", e);
            return Ok(HttpResponse::InternalServerError().json(format!("Erreur de serveur: {}", e)));
        }
    };

    match result {
        Ok(poids) => Ok(HttpResponse::Ok().json(poids)),
        Err(e) => {
            error!("Erreur lors de la mise à jour du poids: {}", e);
            Ok(HttpResponse::InternalServerError().json(format!("Erreur de base de données: {}", e)))
        }
    }
}

#[delete("/poids/{id}")]
pub async fn delete_poids(pool: web::Data<Pool>, id: web::Path<i32>) -> Result<HttpResponse> {
    let mut conn = match get_connection(&pool) {
        Ok(conn) => conn,
        Err(e) => return Ok(e),
    };

    let result = match web::block(move || {
        poids_service::delete_poids(&mut conn, id.into_inner())
    }).await {
        Ok(result) => result,
        Err(e) => {
            error!("Erreur lors de l'exécution de la requête: {}", e);
            return Ok(HttpResponse::InternalServerError().json(format!("Erreur de serveur: {}", e)));
        }
    };

    match result {
        Ok(_) => Ok(HttpResponse::NoContent().finish()),
        Err(e) => {
            error!("Erreur lors de la suppression du poids: {}", e);
            Ok(HttpResponse::InternalServerError().json(format!("Erreur de base de données: {}", e)))
        }
    }
}

#[derive(Deserialize)]
pub struct YearQuery {
    year: i32,
}

#[derive(Deserialize)]
pub struct YearsQuery {
    years: String, // Format: "2022,2023,2024"
}

#[get("/poids/ruche/{ruche_id}/average")]
pub async fn get_annual_average_weight(
    pool: web::Data<Pool>,
    path: web::Path<i32>,
    query: web::Query<YearQuery>,
) -> Result<HttpResponse> {
    let ruche_id = path.into_inner();
    let year = query.year;
    let mut conn = match get_connection(&pool) {
        Ok(conn) => conn,
        Err(e) => return Ok(e),
    };

    match web::block(move || poids_service::get_annual_average_weight(&mut conn, ruche_id, year)).await {
        Ok(Ok(average)) => Ok(HttpResponse::Ok().json(average)),
        Ok(Err(e)) => {
            error!("Erreur de base de données: {}", e);
            Ok(HttpResponse::InternalServerError()
                .json(format!("Erreur de base de données: {}", e)))
        },
        Err(e) => {
            error!("Erreur lors de l'exécution de la requête: {}", e);
            Ok(HttpResponse::InternalServerError()
                .json(format!("Erreur lors du calcul de la moyenne annuelle: {}", e)))
        }
    }
}

#[get("/poids/ruche/{ruche_id}/monthly-average")]
pub async fn get_annual_average_weight_by_month(
    pool: web::Data<Pool>,
    path: web::Path<i32>,
    query: web::Query<YearQuery>,
) -> Result<HttpResponse> {
    let ruche_id = path.into_inner();
    let year = query.year;
    let mut conn = match get_connection(&pool) {
        Ok(conn) => conn,
        Err(e) => return Ok(e),
    };

    match web::block(move || poids_service::get_annual_average_weight_by_month(&mut conn, ruche_id, year)).await {
        Ok(Ok(averages)) => Ok(HttpResponse::Ok().json(averages)),
        Ok(Err(e)) => {
            error!("Erreur de base de données: {}", e);
            Ok(HttpResponse::InternalServerError()
                .json(format!("Erreur de base de données: {}", e)))
        },
        Err(e) => {
            error!("Erreur lors de l'exécution de la requête: {}", e);
            Ok(HttpResponse::InternalServerError()
                .json(format!("Erreur lors du calcul des moyennes mensuelles: {}", e)))
        }
    }
}

#[get("/poids/ruche/{ruche_id}/evolution")]
pub async fn get_weight_evolution(
    pool: web::Data<Pool>,
    path: web::Path<i32>,
    query: web::Query<YearsQuery>,
) -> Result<HttpResponse> {
    let ruche_id = path.into_inner();
    let years: Vec<i32> = query.years
        .split(',')
        .filter_map(|y| y.trim().parse::<i32>().ok())
        .collect();

    let mut conn = match get_connection(&pool) {
        Ok(conn) => conn,
        Err(e) => return Ok(e),
    };

    match web::block(move || poids_service::get_weight_evolution(&mut conn, ruche_id, years)).await {
        Ok(Ok(evolution)) => Ok(HttpResponse::Ok().json(evolution)),
        Ok(Err(e)) => {
            error!("Erreur de base de données: {}", e);
            Ok(HttpResponse::InternalServerError()
                .json(format!("Erreur de base de données: {}", e)))
        },
        Err(e) => {
            error!("Erreur lors de l'exécution de la requête: {}", e);
            Ok(HttpResponse::InternalServerError()
                .json(format!("Erreur lors du calcul de l'évolution du poids: {}", e)))
        }
    }
}