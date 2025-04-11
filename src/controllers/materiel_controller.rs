use actix_web::{web, HttpResponse, Result, get, post, put, delete};
use crate::db::Pool;
use crate::models::materiel_models::{Materiel, NewMateriel, UpdateMateriel};
use crate::services::materiel_service;
use chrono::NaiveDate;
use serde::Deserialize;
use log::{error};
use serde::Serialize;

/// Fonction auxiliaire pour la gestion des résultats de requête DB
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

/// Fonction auxiliaire pour obtenir la connexion à la base de données
fn get_connection(pool: &web::Data<Pool>) -> Result<diesel::r2d2::PooledConnection<diesel::r2d2::ConnectionManager<diesel::PgConnection>>, HttpResponse> {
    pool.get().map_err(|e| {
        let msg = format!("Erreur de connexion à la base de données: {}", e);
        error!("{}", msg);
        HttpResponse::InternalServerError().json(msg)
    })
}

/// Récupère tous les matériels
#[get("/materiels")]
pub async fn get_all_materiels(pool: web::Data<Pool>) -> Result<HttpResponse> {
    let mut conn = match get_connection(&pool) {
        Ok(conn) => conn,
        Err(e) => return Ok(e),
    };

    let result = match web::block(move || materiel_service::get_all_materiels(&mut conn))
        .await {
        Ok(result) => result,
        Err(e) => {
            error!("Erreur lors de l'exécution de la requête: {}", e);
            return Ok(HttpResponse::InternalServerError().json(format!("Erreur de serveur: {}", e)));
        }
    };

    match result {
        Ok(materiels) => Ok(HttpResponse::Ok().json(materiels)),
        Err(e) => {
            error!("Erreur de base de données: {}", e);
            Ok(HttpResponse::InternalServerError().json(format!("Erreur de base de données: {}", e)))
        }
    }
}

/// Récupère un matériel par son ID
#[get("/materiels/{id}")]
pub async fn get_materiel_by_id(pool: web::Data<Pool>, id: web::Path<i32>) -> Result<HttpResponse> {
    let mut conn = match get_connection(&pool) {
        Ok(conn) => conn,
        Err(e) => return Ok(e),
    };

    let result = match web::block(move || {
        materiel_service::get_materiel_by_id(&mut conn, id.into_inner())
    }).await {
        Ok(result) => result,
        Err(e) => {
            error!("Erreur lors de l'exécution de la requête: {}", e);
            return Ok(HttpResponse::InternalServerError().json(format!("Erreur de serveur: {}", e)));
        }
    };

    match result {
        Ok(materiel) => Ok(HttpResponse::Ok().json(materiel)),
        Err(e) => {
            error!("Erreur de base de données: {}", e);
            Ok(HttpResponse::InternalServerError().json(format!("Erreur de base de données: {}", e)))
        }
    }
}

/// Crée un nouveau matériel
#[post("/materiels")]
pub async fn create_materiel(pool: web::Data<Pool>, new_materiel: web::Json<NewMateriel>) -> Result<HttpResponse> {
    let mut conn = match get_connection(&pool) {
        Ok(conn) => conn,
        Err(e) => return Ok(e),
    };

    let result = match web::block(move || {
        materiel_service::create_materiel(&mut conn, new_materiel.into_inner())
    }).await {
        Ok(result) => result,
        Err(e) => {
            error!("Erreur lors de l'exécution de la requête: {}", e);
            return Ok(HttpResponse::InternalServerError().json(format!("Erreur de serveur: {}", e)));
        }
    };

    match result {
        Ok(materiel) => Ok(HttpResponse::Created().json(materiel)),
        Err(e) => {
            error!("Erreur lors de la création du matériel: {}", e);
            Ok(HttpResponse::InternalServerError().json(format!("Erreur de base de données: {}", e)))
        }
    }
}

/// Met à jour un matériel existant
#[put("/materiels/{id}")]
pub async fn update_materiel(
    pool: web::Data<Pool>,
    id: web::Path<i32>,
    materiel: web::Json<UpdateMateriel>
) -> Result<HttpResponse> {
    let mut conn = match get_connection(&pool) {
        Ok(conn) => conn,
        Err(e) => return Ok(e),
    };

    let result = match web::block(move || {
        materiel_service::update_materiel(&mut conn, id.into_inner(), materiel.into_inner())
    }).await {
        Ok(result) => result,
        Err(e) => {
            error!("Erreur lors de l'exécution de la requête: {}", e);
            return Ok(HttpResponse::InternalServerError().json(format!("Erreur de serveur: {}", e)));
        }
    };

    match result {
        Ok(materiel) => Ok(HttpResponse::Ok().json(materiel)),
        Err(e) => {
            error!("Erreur lors de la mise à jour du matériel: {}", e);
            Ok(HttpResponse::InternalServerError().json(format!("Erreur de base de données: {}", e)))
        }
    }
}

/// Supprime un matériel
#[delete("/materiels/{id}")]
pub async fn delete_materiel(pool: web::Data<Pool>, id: web::Path<i32>) -> Result<HttpResponse> {
    let mut conn = match get_connection(&pool) {
        Ok(conn) => conn,
        Err(e) => return Ok(e),
    };

    let result = match web::block(move || {
        materiel_service::delete_materiel(&mut conn, id.into_inner())
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
            error!("Erreur lors de la suppression du matériel: {}", e);
            Ok(HttpResponse::InternalServerError().json(format!("Erreur de base de données: {}", e)))
        }
    }
}

/// Récupère les matériels par type
#[get("/materiels/type/{type_materiel}")]
pub async fn get_materiels_by_type(pool: web::Data<Pool>, type_materiel: web::Path<String>) -> Result<HttpResponse> {
    let mut conn = match get_connection(&pool) {
        Ok(conn) => conn,
        Err(e) => return Ok(e),
    };

    let result = match web::block(move || {
        materiel_service::get_materiels_by_type(&mut conn, type_materiel.into_inner())
    }).await {
        Ok(result) => result,
        Err(e) => {
            error!("Erreur lors de l'exécution de la requête: {}", e);
            return Ok(HttpResponse::InternalServerError().json(format!("Erreur de serveur: {}", e)));
        }
    };

    match result {
        Ok(materiels) => Ok(HttpResponse::Ok().json(materiels)),
        Err(e) => {
            error!("Erreur de base de données: {}", e);
            Ok(HttpResponse::InternalServerError().json(format!("Erreur de base de données: {}", e)))
        }
    }
}

/// Récupère les matériels disponibles
#[get("/materiels/disponibles")]
pub async fn get_available_materiels(pool: web::Data<Pool>) -> Result<HttpResponse> {
    let mut conn = match get_connection(&pool) {
        Ok(conn) => conn,
        Err(e) => return Ok(e),
    };

    let result = match web::block(move || {
        materiel_service::get_materiels_by_etat(&mut conn, "disponible".to_string())
    }).await {
        Ok(result) => result,
        Err(e) => {
            error!("Erreur lors de l'exécution de la requête: {}", e);
            return Ok(HttpResponse::InternalServerError().json(format!("Erreur de serveur: {}", e)));
        }
    };

    match result {
        Ok(materiels) => Ok(HttpResponse::Ok().json(materiels)),
        Err(e) => {
            error!("Erreur de base de données: {}", e);
            Ok(HttpResponse::InternalServerError().json(format!("Erreur de base de données: {}", e)))
        }
    }
}

/// Récupère les matériels par état
#[get("/materiels/etat/{etat}")]
pub async fn get_materiels_by_etat(pool: web::Data<Pool>, etat: web::Path<String>) -> Result<HttpResponse> {
    let mut conn = match get_connection(&pool) {
        Ok(conn) => conn,
        Err(e) => return Ok(e),
    };

    let result = match web::block(move || {
        materiel_service::get_materiels_by_etat(&mut conn, etat.into_inner())
    }).await {
        Ok(result) => result,
        Err(e) => {
            error!("Erreur lors de l'exécution de la requête: {}", e);
            return Ok(HttpResponse::InternalServerError().json(format!("Erreur de serveur: {}", e)));
        }
    };

    match result {
        Ok(materiels) => Ok(HttpResponse::Ok().json(materiels)),
        Err(e) => {
            error!("Erreur de base de données: {}", e);
            Ok(HttpResponse::InternalServerError().json(format!("Erreur de base de données: {}", e)))
        }
    }
}

