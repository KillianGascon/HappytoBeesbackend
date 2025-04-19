use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpMessage,
};
use futures::future::{ready, LocalBoxFuture, Ready};
use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm};
use std::env;
use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize)]
pub struct TokenClaims {
    pub sub: String,
    pub exp: usize,
    pub iat: usize,
}

pub struct Auth;

impl<S, B> Transform<S, ServiceRequest> for Auth
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = AuthMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthMiddleware { service }))
    }
}

pub struct AuthMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for AuthMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let auth_header = req.headers().get("Authorization");

        // Vérifier la présence du header d'autorisation
        if auth_header.is_none() {
            req.into_parts(); // Consommation de la requête
            return Box::pin(async {
                Err(Error::from(actix_web::error::ErrorUnauthorized("Authorization header manquant")))
            });
        }

        // Extraire le token du header Bearer
        let auth_header = auth_header.unwrap().to_str().unwrap_or_default();
        if !auth_header.starts_with("Bearer ") {
            req.into_parts(); // Consommation de la requête
            return Box::pin(async {
                Err(Error::from(actix_web::error::ErrorUnauthorized("Format du token invalide")))
            });
        }

        let token = auth_header[7..].trim();

        // Décoder et vérifier le JWT
        let jwt_secret = match env::var("JWT_SECRET") {
            Ok(secret) => secret,
            Err(_) => {
                req.into_parts(); // Consommation de la requête
                return Box::pin(async {
                    Err(Error::from(actix_web::error::ErrorInternalServerError("Configuration JWT manquante")))
                });
            }
        };

        let validation = Validation::new(Algorithm::HS256);

        match decode::<TokenClaims>(
            token,
            &DecodingKey::from_secret(jwt_secret.as_bytes()),
            &validation,
        ) {
            Ok(token_data) => {
                // Ajouter les claims décodés à la requête
                req.extensions_mut().insert(token_data.claims);
                let fut = self.service.call(req);
                Box::pin(async move {
                    let res = fut.await?;
                    Ok(res)
                })
            }
            Err(_) => {
                req.into_parts(); // Consommation de la requête
                Box::pin(async {
                    Err(Error::from(actix_web::error::ErrorUnauthorized("Token invalide ou expiré")))
                })
            }
        }
    }
}