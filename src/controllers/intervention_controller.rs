use actix_web::{web, HttpResponse, Result, get, post, put};
use crate::db::Pool;
use crate::models::intervention_models::{NewIntervention, UpdateIntervention};
use crate::services::intervention_service;
use log::error;
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

#[get("/interventions")]
pub async fn get_all_interventions(pool: web::Data<Pool>) -> Result<HttpResponse> {
    let mut conn = match get_connection(&pool) {
        Ok(conn) => conn,
        Err(e) => return Ok(e),
    };

    let result = match web::block(move || intervention_service::get_all_interventions(&mut conn))
        .await {
        Ok(result) => result,
        Err(e) => {
            error!("Erreur lors de l'exécution de la requête: {}", e);
            return Ok(HttpResponse::InternalServerError().json(format!("Erreur de serveur: {}", e)));
        }
    };

    match result {
        Ok(interventions) => Ok(HttpResponse::Ok().json(interventions)),
        Err(e) => {
            error!("Erreur de base de données: {}", e);
            Ok(HttpResponse::InternalServerError().json(format!("Erreur de base de données: {}", e)))
        }
    }
}

#[get("/interventions/{id}")]
pub async fn get_intervention_by_id(pool: web::Data<Pool>, id: web::Path<i32>) -> Result<HttpResponse> {
    let mut conn = match get_connection(&pool) {
        Ok(conn) => conn,
        Err(e) => return Ok(e),
    };

    let result = match web::block(move || {
        intervention_service::get_intervention_by_id(&mut conn, id.into_inner())
    }).await {
        Ok(result) => result,
        Err(e) => {
            error!("Erreur lors de l'exécution de la requête: {}", e);
            return Ok(HttpResponse::InternalServerError().json(format!("Erreur de serveur: {}", e)));
        }
    };

    match result {
        Ok(intervention) => Ok(HttpResponse::Ok().json(intervention)),
        Err(e) => {
            error!("Erreur de base de données: {}", e);
            Ok(HttpResponse::InternalServerError().json(format!("Erreur de base de données: {}", e)))
        }
    }
}

#[post("/interventions")]
pub async fn create_intervention(pool: web::Data<Pool>, new_intervention: web::Json<NewIntervention>) -> Result<HttpResponse> {
    let mut conn = match get_connection(&pool) {
        Ok(conn) => conn,
        Err(e) => return Ok(e),
    };

    let result = match web::block(move || {
        intervention_service::create_intervention(&mut conn, new_intervention.into_inner())
    }).await {
        Ok(result) => result,
        Err(e) => {
            error!("Erreur lors de l'exécution de la requête: {}", e);
            return Ok(HttpResponse::InternalServerError().json(format!("Erreur de serveur: {}", e)));
        }
    };

    match result {
        Ok(intervention) => Ok(HttpResponse::Created().json(intervention)),
        Err(e) => {
            error!("Erreur lors de la création de l'intervention: {}", e);
            Ok(HttpResponse::InternalServerError().json(format!("Erreur de base de données: {}", e)))
        }
    }
}

#[put("/interventions/{id}")]
pub async fn update_intervention(
    pool: web::Data<Pool>,
    id: web::Path<i32>,
    intervention: web::Json<UpdateIntervention>
) -> Result<HttpResponse> {
    let mut conn = match get_connection(&pool) {
        Ok(conn) => conn,
        Err(e) => return Ok(e),
    };

    let result = match web::block(move || {
        intervention_service::update_intervention(&mut conn, id.into_inner(), intervention.into_inner())
    }).await {
        Ok(result) => result,
        Err(e) => {
            error!("Erreur lors de l'exécution de la requête: {}", e);
            return Ok(HttpResponse::InternalServerError().json(format!("Erreur de serveur: {}", e)));
        }
    };

    match result {
        Ok(intervention) => Ok(HttpResponse::Ok().json(intervention)),
        Err(e) => {
            error!("Erreur lors de la mise à jour de l'intervention: {}", e);
            Ok(HttpResponse::InternalServerError().json(format!("Erreur de base de données: {}", e)))
        }
    }
}