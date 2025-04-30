use actix_web::{web, HttpResponse, Result, get, post, put, delete};
use crate::db::Pool;
use crate::models::ruche_models::{NewRuche, UpdateRuche};
use crate::services::ruche_service;
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

#[get("/ruches")]
pub async fn get_all_ruches(pool: web::Data<Pool>) -> Result<HttpResponse> {
    let mut conn = match get_connection(&pool) {
        Ok(conn) => conn,
        Err(e) => return Ok(e),
    };

    let result = match web::block(move || ruche_service::get_all_ruches(&mut conn))
        .await {
        Ok(result) => result,
        Err(e) => {
            error!("Erreur lors de l'exécution de la requête: {}", e);
            return Ok(HttpResponse::InternalServerError().json(format!("Erreur de serveur: {}", e)));
        }
    };

    match result {
        Ok(ruches) => Ok(HttpResponse::Ok().json(ruches)),
        Err(e) => {
            error!("Erreur de base de données: {}", e);
            Ok(HttpResponse::InternalServerError().json(format!("Erreur de base de données: {}", e)))
        }
    }
}

#[get("/ruches/{id}")]
pub async fn get_ruche_by_id(pool: web::Data<Pool>, id: web::Path<i32>) -> Result<HttpResponse> {
    let mut conn = match get_connection(&pool) {
        Ok(conn) => conn,
        Err(e) => return Ok(e),
    };

    let result = match web::block(move || {
        ruche_service::get_ruche_by_id(&mut conn, id.into_inner())
    }).await {
        Ok(result) => result,
        Err(e) => {
            error!("Erreur lors de l'exécution de la requête: {}", e);
            return Ok(HttpResponse::InternalServerError().json(format!("Erreur de serveur: {}", e)));
        }
    };

    match result {
        Ok(ruche) => Ok(HttpResponse::Ok().json(ruche)),
        Err(e) => {
            error!("Erreur de base de données: {}", e);
            Ok(HttpResponse::InternalServerError().json(format!("Erreur de base de données: {}", e)))
        }
    }
}

// Méthode pour récupérer les ruches d'un utilisateur spécifique
#[get("/ruches/getByUtilisateur/{id}")]
pub async fn get_ruches_by_utilisateur(pool: web::Data<Pool>, id: web::Path<i32>, ) -> Result<HttpResponse> {
    let mut conn = match get_connection(&pool) {
        Ok(conn) => conn,
        Err(e) => return Ok(e),
    };

    let result = match web::block(move || {
        ruche_service::get_ruches_by_utilisateur(&mut conn, id.into_inner())
    })
        .await
    {
        Ok(result) => result,
        Err(e) => {
            error!("Erreur lors de l'exécution de la requête: {}", e);
            return Ok(HttpResponse::InternalServerError().json(format!(
                "Erreur de serveur: {}",
                e
            )));
        }
    };

    match result {
        Ok(ruches) => Ok(HttpResponse::Ok().json(ruches)),
        Err(e) => {
            error!("Erreur de base de données: {}", e);
            Ok(HttpResponse::InternalServerError().json(format!(
                "Erreur de base de données: {}",
                e
            )))
        }
    }
}

#[post("/ruches")]
pub async fn create_ruche(pool: web::Data<Pool>, new_ruche: web::Json<NewRuche>) -> Result<HttpResponse> {
    let mut conn = match get_connection(&pool) {
        Ok(conn) => conn,
        Err(e) => return Ok(e),
    };

    let result = match web::block(move || {
        ruche_service::create_ruche(&mut conn, new_ruche.into_inner())
    }).await {
        Ok(result) => result,
        Err(e) => {
            error!("Erreur lors de l'exécution de la requête: {}", e);
            return Ok(HttpResponse::InternalServerError().json(format!("Erreur de serveur: {}", e)));
        }
    };

    match result {
        Ok(ruche) => Ok(HttpResponse::Created().json(ruche)),
        Err(e) => {
            error!("Erreur lors de la création de la ruche: {}", e);
            Ok(HttpResponse::InternalServerError().json(format!("Erreur de base de données: {}", e)))
        }
    }
}

#[put("/ruches/{id}")]
pub async fn update_ruche(
    pool: web::Data<Pool>,
    id: web::Path<i32>,
    ruche: web::Json<UpdateRuche>
) -> Result<HttpResponse> {
    let mut conn = match get_connection(&pool) {
        Ok(conn) => conn,
        Err(e) => return Ok(e),
    };

    let result = match web::block(move || {
        ruche_service::update_ruche(&mut conn, id.into_inner(), ruche.into_inner())
    }).await {
        Ok(result) => result,
        Err(e) => {
            error!("Erreur lors de l'exécution de la requête: {}", e);
            return Ok(HttpResponse::InternalServerError().json(format!("Erreur de serveur: {}", e)));
        }
    };

    match result {
        Ok(ruche) => Ok(HttpResponse::Ok().json(ruche)),
        Err(e) => {
            error!("Erreur lors de la mise à jour de la ruche: {}", e);
            Ok(HttpResponse::InternalServerError().json(format!("Erreur de base de données: {}", e)))
        }
    }
}

#[delete("/ruches/{id}")]
pub async fn delete_ruche(pool: web::Data<Pool>, id: web::Path<i32>) -> Result<HttpResponse> {
    let mut conn = match get_connection(&pool) {
        Ok(conn) => conn,
        Err(e) => return Ok(e),
    };

    let result = match web::block(move || {
        ruche_service::delete_ruche(&mut conn, id.into_inner())
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
            error!("Erreur lors de la suppression de la ruche: {}", e);
            Ok(HttpResponse::InternalServerError().json(format!("Erreur de base de données: {}", e)))
        }
    }
}