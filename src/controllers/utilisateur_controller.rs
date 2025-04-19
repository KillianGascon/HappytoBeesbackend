use actix_web::{web, HttpResponse, Result, get, post, put, delete};
use crate::db::Pool;
use crate::models::utilisateur_models::{Utilisateur, NewUtilisateur, UpdateUtilisateur, LoginCredentials};
use crate::services::utilisateur_service;
use serde::Deserialize;
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

#[get("/utilisateurs")]
pub async fn get_all_utilisateurs(pool: web::Data<Pool>) -> Result<HttpResponse> {
    let mut conn = match get_connection(&pool) {
        Ok(conn) => conn,
        Err(e) => return Ok(e),
    };

    let result = match web::block(move || utilisateur_service::get_all_utilisateurs(&mut conn))
        .await {
        Ok(result) => result,
        Err(e) => {
            error!("Erreur lors de l'exécution de la requête: {}", e);
            return Ok(HttpResponse::InternalServerError().json(format!("Erreur de serveur: {}", e)));
        }
    };

    match result {
        Ok(utilisateurs) => Ok(HttpResponse::Ok().json(utilisateurs)),
        Err(e) => {
            error!("Erreur de base de données: {}", e);
            Ok(HttpResponse::InternalServerError().json(format!("Erreur de base de données: {}", e)))
        }
    }
}

#[get("/utilisateurs/{id}")]
pub async fn get_utilisateur_by_id(pool: web::Data<Pool>, id: web::Path<i32>) -> Result<HttpResponse> {
    let mut conn = match get_connection(&pool) {
        Ok(conn) => conn,
        Err(e) => return Ok(e),
    };

    let result = match web::block(move || {
        utilisateur_service::get_utilisateur_by_id(&mut conn, id.into_inner())
    }).await {
        Ok(result) => result,
        Err(e) => {
            error!("Erreur lors de l'exécution de la requête: {}", e);
            return Ok(HttpResponse::InternalServerError().json(format!("Erreur de serveur: {}", e)));
        }
    };

    match result {
        Ok(utilisateur) => Ok(HttpResponse::Ok().json(utilisateur)),
        Err(e) => {
            error!("Erreur de base de données: {}", e);
            Ok(HttpResponse::InternalServerError().json(format!("Erreur de base de données: {}", e)))
        }
    }
}

#[derive(Deserialize)]
pub struct UserRegistration {
    pub nom: String,
    pub prenom: String,
    pub email: String,
    pub password: String,
    pub telephone: Option<String>,
    pub numero_apiculteur: Option<i32>,
    pub date_naissance: Option<NaiveDate>,
}

#[post("/utilisateurs")]
pub async fn create_utilisateur(pool: web::Data<Pool>, new_utilisateur: web::Json<NewUtilisateur>) -> Result<HttpResponse> {
    let mut conn = match get_connection(&pool) {
        Ok(conn) => conn,
        Err(e) => return Ok(e),
    };

    let result = match web::block(move || {
        utilisateur_service::create_utilisateur(&mut conn, new_utilisateur.into_inner())
    }).await {
        Ok(result) => result,
        Err(e) => {
            error!("Erreur lors de l'exécution de la requête: {}", e);
            return Ok(HttpResponse::InternalServerError().json(format!("Erreur de serveur: {}", e)));
        }
    };

    match result {
        Ok(utilisateur) => Ok(HttpResponse::Created().json(utilisateur)),
        Err(e) => {
            error!("Erreur lors de la création de l'utilisateur: {}", e);
            Ok(HttpResponse::InternalServerError().json(format!("Erreur de base de données: {}", e)))
        }
    }
}

#[put("/utilisateurs/{id}")]
pub async fn update_utilisateur(
    pool: web::Data<Pool>,
    id: web::Path<i32>,
    utilisateur: web::Json<UpdateUtilisateur>
) -> Result<HttpResponse> {
    let mut conn = match get_connection(&pool) {
        Ok(conn) => conn,
        Err(e) => return Ok(e),
    };

    let result = match web::block(move || {
        utilisateur_service::update_utilisateur(&mut conn, id.into_inner(), utilisateur.into_inner())
    }).await {
        Ok(result) => result,
        Err(e) => {
            error!("Erreur lors de l'exécution de la requête: {}", e);
            return Ok(HttpResponse::InternalServerError().json(format!("Erreur de serveur: {}", e)));
        }
    };

    match result {
        Ok(utilisateur) => Ok(HttpResponse::Ok().json(utilisateur)),
        Err(e) => {
            error!("Erreur lors de la mise à jour de l'utilisateur: {}", e);
            Ok(HttpResponse::InternalServerError().json(format!("Erreur de base de données: {}", e)))
        }
    }
}

#[delete("/utilisateurs/{id}")]
pub async fn delete_utilisateur(pool: web::Data<Pool>, id: web::Path<i32>) -> Result<HttpResponse> {
    let mut conn = match get_connection(&pool) {
        Ok(conn) => conn,
        Err(e) => return Ok(e),
    };

    let result = match web::block(move || {
        utilisateur_service::delete_utilisateur(&mut conn, id.into_inner())
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
            error!("Erreur lors de la suppression de l'utilisateur: {}", e);
            Ok(HttpResponse::InternalServerError().json(format!("Erreur de base de données: {}", e)))
        }
    }
}

#[post("/utilisateurs/login")]
pub async fn login(pool: web::Data<Pool>, credentials: web::Json<LoginCredentials>) -> Result<HttpResponse> {
    let mut conn = match pool.get() {
        Ok(conn) => conn,
        Err(e) => {
            let msg = format!("Erreur de connexion à la base de données: {}", e);
            error!("{}", msg);
            return Ok(HttpResponse::InternalServerError().json(msg));
        }
    };

    match web::block(move || utilisateur_service::authenticate_user(&mut conn, &credentials.email, &credentials.password)).await {
        Ok(Ok(auth_response)) => Ok(HttpResponse::Ok().json(auth_response)),
        Ok(Err(e)) => {
            error!("Erreur lors de l'authentification: {}", e);
            Ok(HttpResponse::Unauthorized().json(format!("Erreur lors de l'authentification: {}", e)))
        },
        Err(e) => {
            error!("Erreur lors de l'exécution de la requête: {}", e);
            Ok(HttpResponse::InternalServerError().json(format!("Erreur lors de l'exécution de la requête: {}", e)))
        }
    }
}

