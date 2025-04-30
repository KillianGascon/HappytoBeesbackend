use crate::db::DbConnection;
use crate::models::utilisateur_models::{Utilisateur, NewUtilisateur, UpdateUtilisateur, TokenClaims, AuthResponse};
use crate::schema::utilisateur;
use diesel::prelude::*;
use diesel::result::Error;
use argon2::{
    password_hash::{PasswordHasher, PasswordVerifier, SaltString},
    Argon2, PasswordHash,
};
use rand_core::OsRng;
use jsonwebtoken::{encode, Header, EncodingKey};
use std::env;
use chrono::{Utc, Duration};
use log::{error, info};

/// Récupère tous les utilisateurs
pub fn get_all_utilisateurs(conn: &mut DbConnection) -> Result<Vec<Utilisateur>, Error> {
    utilisateur::table.load::<Utilisateur>(conn)
}

/// Récupère un utilisateur par son ID
pub fn get_utilisateur_by_id(conn: &mut DbConnection, id: i32) -> Result<Utilisateur, Error> {
    utilisateur::table.find(id).first::<Utilisateur>(conn)
}

/// Crée un nouvel utilisateur
pub fn create_utilisateur(
    conn: &mut DbConnection,
    mut new_utilisateur: NewUtilisateur,
) -> Result<Utilisateur, Error> {
    // Vérification et hachage du mot de passe
    if let Some(password) = &new_utilisateur.mot_de_passe {
        match hash_password(password) {
            Ok(hashed_password) => {
                new_utilisateur.mot_de_passe = Some(hashed_password);
            }
            Err(e) => {
                log::error!("Erreur lors du hachage du mot de passe : {}", e);
                return Err(diesel::result::Error::RollbackTransaction);
            }
        }
    } else {
        log::error!("Mot de passe manquant pour l'utilisateur.");
        return Err(diesel::result::Error::RollbackTransaction);
    }

    // Insertion dans la base de données
    diesel::insert_into(utilisateur::table)
        .values(&new_utilisateur)
        .get_result(conn)
}


/// Met à jour un utilisateur existant
pub fn update_utilisateur(conn: &mut DbConnection, id: i32, updated_utilisateur: UpdateUtilisateur) -> Result<Utilisateur, Error> {
    diesel::update(utilisateur::table.find(id))
        .set(&updated_utilisateur)
        .get_result(conn)
}

/// Supprime un utilisateur
pub fn delete_utilisateur(conn: &mut DbConnection, id: i32) -> Result<usize, Error> {
    diesel::delete(utilisateur::table.find(id)).execute(conn)
}

/// Récupère un utilisateur par son email
pub fn get_utilisateur_by_email(conn: &mut DbConnection, email: String) -> Result<Utilisateur, Error> {
    utilisateur::table
        .filter(utilisateur::mail.eq(email))
        .first::<Utilisateur>(conn)
}

/// Vérifie si un email existe déjà
pub fn email_exists(conn: &mut DbConnection, email: String) -> Result<bool, Error> {
    let count = utilisateur::table
        .filter(utilisateur::mail.eq(email))
        .count()
        .get_result::<i64>(conn)?;
    Ok(count > 0)
}

pub fn authenticate_user(
    conn: &mut DbConnection,
    email: &str,
    password: &str,
) -> Result<Option<AuthResponse>, Error> {
    use crate::schema::utilisateur::dsl::*;

    info!("Tentative d'authentification pour l'email: {}", email);

    let user_result = utilisateur
        .filter(mail.eq(email))
        .first::<Utilisateur>(conn)
        .optional()?;

    if let Some(user) = user_result {
        info!("Utilisateur trouvé avec l'ID: {}", user.id);

        if let Some(stored_hash) = &user.mot_de_passe {
            if verify_password(password, stored_hash) {
                info!("Authentification réussie pour l'utilisateur: {}", user.id);
                // Gestion d'erreur pour convertir l'erreur JWT en erreur Diesel
                let token = match generate_jwt_token(&user) {
                    Ok(token) => token,
                    Err(e) => {
                        error!("Erreur lors de la génération du token JWT: {}", e);
                        return Err(Error::RollbackTransaction);
                    }
                };
                return Ok(Some(AuthResponse {
                    user,
                    token,
                }));
            } else {
                info!("Mot de passe incorrect pour l'utilisateur: {}", user.id);
            }
        } else {
            info!("Aucun mot de passe stocké pour l'utilisateur: {}", user.id);
        }
    } else {
        info!("Aucun utilisateur trouvé avec l'email: {}", email);
    }

    Ok(None)
}

// Fonctions utilitaires pour le hachage de mots de passe
fn hash_password(password: &str) -> Result<String, argon2::password_hash::Error> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    argon2
        .hash_password(password.as_bytes(), &salt)
        .map(|hash| hash.to_string())
}

fn verify_password(password: &str, hash: &str) -> bool {
    let parsed_hash = match PasswordHash::new(hash) {
        Ok(h) => h,
        Err(e) => {
            error!("Erreur lors de l'analyse du hash: {}", e);
            return false;
        }
    };

    Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok()
}

fn generate_jwt_token(user: &Utilisateur) -> Result<String, jsonwebtoken::errors::Error> {
    let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET doit être défini");
    let now = Utc::now();
    let expires_at = now + Duration::days(3);

    let claims = TokenClaims {
        sub: user.mail.clone().unwrap_or_default(),
        user_id: user.id,
        exp: expires_at.timestamp() as usize,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(jwt_secret.as_bytes()),
    )
}