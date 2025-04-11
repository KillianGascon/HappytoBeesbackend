use crate::db::DbConnection;
use crate::models::session_models::{Session, NewSession, UpdateSession};
use crate::schema::sessions;
use diesel::prelude::*;
use diesel::result::Error;
use chrono::{Utc, Duration};

/// Récupère toutes les sessions
pub fn get_all_sessions(conn: &mut DbConnection) -> Result<Vec<Session>, Error> {
    sessions::table.load::<Session>(conn)
}

/// Récupère une session par son ID
pub fn get_session_by_id(conn: &mut DbConnection, id: i32) -> Result<Session, Error> {
    sessions::table.find(id).first::<Session>(conn)
}

/// Crée une nouvelle session
pub fn create_session(conn: &mut DbConnection, new_session: NewSession) -> Result<Session, Error> {
    diesel::insert_into(sessions::table)
        .values(&new_session)
        .get_result(conn)
}

/// Met à jour une session existante
pub fn update_session(conn: &mut DbConnection, id: i32, updated_session: UpdateSession) -> Result<Session, Error> {
    diesel::update(sessions::table.find(id))
        .set(&updated_session)
        .get_result(conn)
}

/// Supprime une session
pub fn delete_session(conn: &mut DbConnection, id: i32) -> Result<usize, Error> {
    diesel::delete(sessions::table.find(id)).execute(conn)
}

/// Récupère les sessions d'un utilisateur spécifique
pub fn get_sessions_by_user_id(conn: &mut DbConnection, user_id: i32) -> Result<Vec<Session>, Error> {
    sessions::table
        .filter(sessions::id_utilisateur.eq(user_id))
        .load::<Session>(conn)
}

/// Vérifie si une session est valide
pub fn is_session_valid(conn: &mut DbConnection, token: &str) -> Result<bool, Error> {
    let count = sessions::table
        .filter(sessions::token.eq(token))
        .filter(sessions::expire_at.gt(chrono::Utc::now().naive_utc()))
        .count()
        .get_result::<i64>(conn)?;
    Ok(count > 0)
}

/// Invalide toutes les sessions d'un utilisateur
pub fn invalidate_user_sessions(conn: &mut DbConnection, user_id: i32) -> Result<usize, Error> {
    diesel::update(sessions::table.filter(sessions::id_utilisateur.eq(user_id)))
        .set(sessions::expire_at.eq(chrono::Utc::now().naive_utc()))
        .execute(conn)
}

pub fn get_session_by_token(conn: &mut DbConnection, token_value: &str) -> Result<Option<Session>, Error> {
    use crate::schema::sessions::dsl::*;

    sessions
        .filter(token.eq(token_value))
        .filter(est_valide.eq(true))
        .filter(date_expiration.gt(diesel::dsl::now))
        .first::<Session>(conn)
        .optional()
}

pub fn create_user_session(
    conn: &mut DbConnection,
    user_id: i32,
    token_value: String,
    user_agent: Option<String>,
    ip: Option<String>,
) -> Result<Session, Error> {
    let now = Utc::now().naive_utc();
    let expires_at = (Utc::now() + Duration::days(3)).naive_utc();

    let new_session = NewSession {
        id_utilisateur: Some(user_id),
        token: Some(token_value),
        user_agent,
        ip_address: ip,
        date_creation: Some(now),
        date_expiration: Some(expires_at),
        est_valide: Some(true),
    };

    create_session(conn, new_session)
}

pub fn invalidate_session(conn: &mut DbConnection, session_id: i32) -> Result<Session, Error> {
    let invalidate = UpdateSession {
        id_utilisateur: None,
        token: None,
        user_agent: None,
        ip_address: None,
        date_creation: None,
        date_expiration: None,
        est_valide: Some(false),
    };

    update_session(conn, session_id, invalidate)
}

pub fn cleanup_expired_sessions(conn: &mut DbConnection) -> Result<usize, Error> {
    use crate::schema::sessions::dsl::*;

    diesel::delete(sessions.filter(date_expiration.lt(diesel::dsl::now)))
        .execute(conn)
}