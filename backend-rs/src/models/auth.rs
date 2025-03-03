use super::error::AuthError;
use jsonwebtoken::jwk::{AlgorithmParameters, JwkSet};
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use reqwest::Client;
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome, Request};
use rocket::serde::json::Json;
use serde::Deserialize;
use std::sync::Arc;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use tokio::sync::RwLock;

#[derive(Debug)]
pub struct Auth0Config {
    pub domain: String,
    pub audience: String,
}

pub struct Jwks {
    keys: RwLock<Option<JwkSet>>,
    last_updated: RwLock<SystemTime>,
    http_client: Client,
}

impl Jwks {
    pub fn new(http_client: Client) -> Jwks {
        Jwks {
            keys: RwLock::new(None),
            last_updated: RwLock::new(UNIX_EPOCH),
            http_client,
        }
    }

    async fn fetch_jwks(&self, domain: &str) -> Result<JwkSet, AuthError<String>> {
        let last_updated = self.last_updated.read().await;

        let elapsed = match SystemTime::now().duration_since(*last_updated) {
            Ok(elapsed) => elapsed,
            Err(_) => {
                return Err(AuthError::JwkError(Json(
                    "Failed to unlock last_updated.".to_string(),
                )));
            }
        };

        let keys = { self.keys.read().await };

        if elapsed > Duration::from_secs(86400) || keys.is_none() {
            drop(keys);
            drop(last_updated);

            let jwks_url = format!("https://{}/.well-known/jwks.json", domain);
            match self.http_client.get(&jwks_url).send().await {
                Ok(resp) => {
                    if resp.status().is_success() {
                        match resp.json::<JwkSet>().await {
                            Ok(jwks) => {
                                *self.keys.write().await = Some(jwks.clone());
                                *self.last_updated.write().await = SystemTime::now();
                                return Ok(jwks);
                            }
                            Err(e) => {
                                return Err(AuthError::JwkError(Json(format!(
                                    "Failed to parse JWKS response: {}",
                                    e
                                ))));
                            }
                        }
                    } else {
                        return Err(AuthError::JwkError(Json(format!(
                            "Failed to fetch JWKS: HTTP {}",
                            resp.status()
                        ))));
                    }
                }
                Err(e) => {
                    return Err(AuthError::JwkError(Json(format!(
                        "Failed to make JWKS request: {}",
                        e
                    ))));
                }
            }
        } else {
            match keys.clone() {
                Some(jwks) => {
                    return Ok(jwks);
                }
                _ => {
                    return Err(AuthError::JwkError(Json(
                        "Failed to make jwk request.".to_string(),
                    )));
                }
            }
        }
    }
}

#[derive(Deserialize)]
struct Claims {
    sub: String,
    iss: String,
    aud: String,
    exp: usize,
    iat: usize,
    scope: Option<String>,
}

pub struct AuthenticatedUser {
    user_id: String,
    scopes: Vec<String>,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AuthenticatedUser {
    type Error = AuthError<String>;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let token = match request.headers().get_one("Authorization") {
            Some(value) if value.starts_with("Bearer ") => value[7..].to_string(),
            _ => {
                return Outcome::Error((
                    Status::Unauthorized,
                    AuthError::TokenError(Json("No token found in header.".to_string())),
                ));
            }
        };

        let auth0_config = match request.rocket().state::<Auth0Config>() {
            Some(config) => config,
            _ => {
                return Outcome::Error((
                    Status::InternalServerError,
                    AuthError::TokenError(Json("Auth0 config not found.".to_string())),
                ));
            }
        };

        let jwk_cache = match request.rocket().state::<Arc<Jwks>>() {
            Some(cache) => cache,
            _ => {
                return Outcome::Error((
                    Status::InternalServerError,
                    AuthError::TokenError(Json("JWKS cache not found.".to_string())),
                ));
            }
        };

        let jwks = match jwk_cache.fetch_jwks(&auth0_config.domain).await {
            Ok(jwks) => jwks.clone(),
            Err(e) => {
                return Outcome::Error((Status::Unauthorized, e));
            }
        };

        let header = match jsonwebtoken::decode_header(&token) {
            Ok(header) => header,
            Err(_) => {
                return Outcome::Error((
                    Status::Unauthorized,
                    AuthError::JwtError(Json("Invalid JWT.".to_string())),
                ));
            }
        };

        let kid = match header.kid {
            Some(kid) => kid,
            _ => {
                return Outcome::Error((
                    Status::Unauthorized,
                    AuthError::JwtError(Json("Header missing kid.".to_string())),
                ));
            }
        };

        let jwk = match jwks.find(&kid) {
            Some(jwk) => jwk,
            _ => {
                return Outcome::Error((
                    Status::Unauthorized,
                    AuthError::JwtError(Json("No matching key in JWKs.".to_string())),
                ));
            }
        };

        let decoding_key = match &jwk.algorithm {
            AlgorithmParameters::RSA(rsa) => {
                match DecodingKey::from_rsa_components(&rsa.n, &rsa.e) {
                    Ok(key) => key,
                    Err(_) => {
                        return Outcome::Error((
                            Status::Unauthorized,
                            AuthError::JwtError(Json("Failed to decode key.".to_string())),
                        ));
                    }
                }
            }
            _ => {
                return Outcome::Error((
                    Status::InternalServerError,
                    AuthError::JwtError(Json("Unsupported key type in JWKS".to_string())),
                ));
            }
        };

        let mut validation = Validation::new(Algorithm::RS256);
        validation.set_audience(&[&auth0_config.audience]);
        validation.set_issuer(&[&format!("https://{}/", auth0_config.domain)]);

        match decode::<Claims>(&token, &decoding_key, &validation) {
            Ok(data) => {
                let scopes = match &data.claims.scope {
                    Some(scope_str) => scope_str.split(' ').map(String::from).collect(),
                    _ => Vec::new(),
                };

                Outcome::Success(AuthenticatedUser {
                    user_id: data.claims.sub,
                    scopes,
                })
            }
            Err(e) => Outcome::Error((
                Status::InternalServerError,
                AuthError::JwtError(Json(format!("JWT validation failed: {}", e))),
            )),
        }
    }
}
