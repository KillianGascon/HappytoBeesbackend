use actix_web::{web, HttpResponse, Result, get, post, put, delete};
use crate::db::Pool;
use crate::models::production_models::{Production, NewProduction, UpdateProduction};
use crate::services::production_service;
use chrono::NaiveDate;
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

// Obtenir toutes les productions
#[get("/productions")]
pub async fn get_all_productions(pool: web::Data<Pool>) -> Result<HttpResponse> {
    let mut conn = match get_connection(&pool) {
        Ok(conn) => conn,
        Err(e) => return Ok(e),
    };

    let result = match web::block(move || production_service::get_all_productions(&mut conn))
        .await {
        Ok(result) => result,
        Err(e) => {
            error!("Erreur lors de l'exécution de la requête: {}", e);
            return Ok(HttpResponse::InternalServerError().json(format!("Erreur de serveur: {}", e)));
        }
    };

    match result {
        Ok(productions) => Ok(HttpResponse::Ok().json(productions)),
        Err(e) => {
            error!("Erreur de base de données: {}", e);
            Ok(HttpResponse::InternalServerError().json(format!("Erreur de base de données: {}", e)))
        }
    }
}

// Obtenir une production par ID
#[get("/productions/{id}")]
pub async fn get_production_by_id(pool: web::Data<Pool>, id: web::Path<i32>) -> Result<HttpResponse> {
    let mut conn = match get_connection(&pool) {
        Ok(conn) => conn,
        Err(e) => return Ok(e),
    };

    let result = match web::block(move || {
        production_service::get_production_by_id(&mut conn, id.into_inner())
    }).await {
        Ok(result) => result,
        Err(e) => {
            error!("Erreur lors de l'exécution de la requête: {}", e);
            return Ok(HttpResponse::InternalServerError().json(format!("Erreur de serveur: {}", e)));
        }
    };

    match result {
        Ok(production) => Ok(HttpResponse::Ok().json(production)),
        Err(e) => {
            error!("Erreur de base de données: {}", e);
            Ok(HttpResponse::InternalServerError().json(format!("Erreur de base de données: {}", e)))
        }
    }
}

// Obtenir les productions par ruche
#[get("/productions/ruche/{ruche_id}")]
pub async fn get_productions_by_ruche_id(pool: web::Data<Pool>, ruche_id: web::Path<i32>) -> Result<HttpResponse> {
    let mut conn = match get_connection(&pool) {
        Ok(conn) => conn,
        Err(e) => return Ok(e),
    };

    let result = match web::block(move || {
        production_service::get_productions_by_ruche_id(&mut conn, ruche_id.into_inner())
    }).await {
        Ok(result) => result,
        Err(e) => {
            error!("Erreur lors de l'exécution de la requête: {}", e);
            return Ok(HttpResponse::InternalServerError().json(format!("Erreur de serveur: {}", e)));
        }
    };

    match result {
        Ok(productions) => Ok(HttpResponse::Ok().json(productions)),
        Err(e) => {
            error!("Erreur de base de données: {}", e);
            Ok(HttpResponse::InternalServerError().json(format!("Erreur de base de données: {}", e)))
        }
    }
}

// Créer une nouvelle production
#[post("/productions")]
pub async fn create_production(pool: web::Data<Pool>, new_production: web::Json<NewProduction>) -> Result<HttpResponse> {
    let mut conn = match get_connection(&pool) {
        Ok(conn) => conn,
        Err(e) => return Ok(e),
    };

    let result = match web::block(move || {
        production_service::create_production(&mut conn, new_production.into_inner())
    }).await {
        Ok(result) => result,
        Err(e) => {
            error!("Erreur lors de l'exécution de la requête: {}", e);
            return Ok(HttpResponse::InternalServerError().json(format!("Erreur de serveur: {}", e)));
        }
    };

    match result {
        Ok(production) => Ok(HttpResponse::Created().json(production)),
        Err(e) => {
            error!("Erreur lors de la création de la production: {}", e);
            Ok(HttpResponse::InternalServerError().json(format!("Erreur de base de données: {}", e)))
        }
    }
}

// Mettre à jour une production
#[put("/productions/{id}")]
pub async fn update_production(
    pool: web::Data<Pool>,
    id: web::Path<i32>,
    production: web::Json<UpdateProduction>
) -> Result<HttpResponse> {
    let mut conn = match get_connection(&pool) {
        Ok(conn) => conn,
        Err(e) => return Ok(e),
    };

    let result = match web::block(move || {
        production_service::update_production(&mut conn, id.into_inner(), production.into_inner())
    }).await {
        Ok(result) => result,
        Err(e) => {
            error!("Erreur lors de l'exécution de la requête: {}", e);
            return Ok(HttpResponse::InternalServerError().json(format!("Erreur de serveur: {}", e)));
        }
    };

    match result {
        Ok(production) => Ok(HttpResponse::Ok().json(production)),
        Err(e) => {
            error!("Erreur lors de la mise à jour de la production: {}", e);
            Ok(HttpResponse::InternalServerError().json(format!("Erreur de base de données: {}", e)))
        }
    }
}

// Supprimer une production
#[delete("/productions/{id}")]
pub async fn delete_production(pool: web::Data<Pool>, id: web::Path<i32>) -> Result<HttpResponse> {
    let mut conn = match get_connection(&pool) {
        Ok(conn) => conn,
        Err(e) => return Ok(e),
    };

    let result = match web::block(move || {
        production_service::delete_production(&mut conn, id.into_inner())
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
            error!("Erreur lors de la suppression de la production: {}", e);
            Ok(HttpResponse::InternalServerError().json(format!("Erreur de base de données: {}", e)))
        }
    }
}

// Statistiques de production par ruche
#[get("/productions/ruche/{ruche_id}/statistiques")]
pub async fn get_production_statistics_by_ruche(
    pool: web::Data<Pool>,
    ruche_id: web::Path<i32>
) -> Result<HttpResponse> {
    let ruche_id = ruche_id.into_inner();
    let mut conn = match get_connection(&pool) {
        Ok(conn) => conn,
        Err(e) => return Ok(e),
    };

    match web::block(move || production_service::get_total_production_by_ruche(&mut conn, ruche_id)).await {
        Ok(Ok(total_production)) => Ok(HttpResponse::Ok().json(total_production)),
        Ok(Err(e)) => {
            error!("Erreur de base de données: {}", e);
            Ok(HttpResponse::InternalServerError()
                .json(format!("Erreur de base de données: {}", e)))
        },
        Err(e) => {
            error!("Erreur lors de l'exécution de la requête: {}", e);
            Ok(HttpResponse::InternalServerError()
                .json(format!("Erreur lors de la récupération des statistiques de production: {}", e)))
        },
    }
}

// Productions par période
#[derive(serde::Deserialize)]
pub struct DateRange {
    debut: NaiveDate,
    fin: NaiveDate,
}

#[get("/productions/ruche/{ruche_id}/date_range")]
pub async fn get_productions_by_date_range(
    pool: web::Data<Pool>,
    ruche_id: web::Path<i32>,
    query: web::Query<DateRange>
) -> Result<HttpResponse> {
    let ruche_id = ruche_id.into_inner();
    let mut conn = match get_connection(&pool) {
        Ok(conn) => conn,
        Err(e) => return Ok(e),
    };

    match web::block(move || production_service::get_production_by_date_range(
        &mut conn,
        ruche_id,
        query.debut,
        query.fin
    )).await {
        Ok(Ok(productions)) => Ok(HttpResponse::Ok().json(productions)),
        Ok(Err(e)) => {
            error!("Erreur de base de données: {}", e);
            Ok(HttpResponse::InternalServerError()
                .json(format!("Erreur de base de données: {}", e)))
        },
        Err(e) => {
            error!("Erreur lors de l'exécution de la requête: {}", e);
            Ok(HttpResponse::InternalServerError()
                .json(format!("Erreur lors de la récupération des productions: {}", e)))
        },
    }
}