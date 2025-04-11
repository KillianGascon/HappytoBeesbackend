use actix_web::{web, App, HttpServer, middleware::Logger};
use dotenv::dotenv;
use std::{env, io::Write};
use log::{info, error};
use chrono::Local;

// Les modules et contrôleurs seront ajoutés après la création des fichiers correspondants
mod db;
mod schema;
mod services;
mod controllers;
mod middleware;
mod models;

use crate::controllers::intervention_controller;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Chargement des variables d'environnement
    dotenv().ok();

    // Configuration du logger
    env_logger::builder()
        .format(|buf, record| {
            writeln!(
                buf,
                "{} [{}] - {}",
                Local::now().format("%Y-%m-%d %H:%M:%S"),
                record.level(),
                record.args()
            )
        })
        .filter_level(log::LevelFilter::Info)
        .init();

    // Récupération des variables d'environnement
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL doit être définie dans le fichier .env");

    let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = env::var("PORT").unwrap_or_else(|_| "3000".to_string())
        .parse::<u16>().expect("PORT doit être un nombre");

    // Création du pool de connexions à la base de données
    let pool = match db::establish_connection_pool(&database_url) {
        Ok(pool) => {
            info!("Connexion à la base de données établie avec succès");
            pool
        },
        Err(e) => {
            error!("Échec de la connexion à la base de données: {}", e);
            panic!("Impossible de démarrer l'application sans base de données");
        }
    };

    info!("Démarrage du serveur sur {}:{}", host, port);

    // Configuration et démarrage du serveur HTTP
    HttpServer::new(move || {
        // Configuration CORS
        let cors = actix_cors::Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);

        App::new()
            .wrap(Logger::default())
            .wrap(cors)
            .app_data(web::Data::new(pool.clone()))
            .service(web::scope("/api")
                .service(intervention_controller::get_all_interventions)
                .service(intervention_controller::get_intervention_by_id)
                .service(intervention_controller::create_intervention)
                .service(intervention_controller::update_intervention)

                // Routes d'utilisateur
                .service(controllers::utilisateur_controller::get_all_utilisateurs)
                .service(controllers::utilisateur_controller::get_utilisateur_by_id)
                .service(controllers::utilisateur_controller::create_utilisateur)
                .service(controllers::utilisateur_controller::update_utilisateur)
                .service(controllers::utilisateur_controller::delete_utilisateur)
                .service(controllers::utilisateur_controller::login)
                
                // Routes de ruche
                .service(controllers::ruche_controller::get_all_ruches)
                .service(controllers::ruche_controller::get_ruche_by_id)
                .service(controllers::ruche_controller::create_ruche)
                .service(controllers::ruche_controller::update_ruche)
                .service(controllers::ruche_controller::delete_ruche)
                
                // Routes de production
                .service(controllers::production_controller::get_all_productions)
                .service(controllers::production_controller::get_production_by_id)
                .service(controllers::production_controller::create_production)
                .service(controllers::production_controller::update_production)
                .service(controllers::production_controller::delete_production)

                // Routes de poids
                .service(controllers::poids_controller::get_all_poids)
                .service(controllers::poids_controller::get_poids_by_id)
                .service(controllers::poids_controller::create_poids)
                .service(controllers::poids_controller::update_poids)
                .service(controllers::poids_controller::delete_poids)
                .service(controllers::poids_controller::get_annual_average_weight)

                // Routes de matériel
                .service(controllers::materiel_controller::get_all_materiels)
                .service(controllers::materiel_controller::get_materiel_by_id)
                .service(controllers::materiel_controller::create_materiel)
                .service(controllers::materiel_controller::update_materiel)
                .service(controllers::materiel_controller::delete_materiel)
                .service(controllers::materiel_controller::get_materiels_by_etat)
            )
    })
        .bind((host, port))?
        .run()
        .await
}