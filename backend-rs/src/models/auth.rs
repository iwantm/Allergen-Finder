use super::error::{ApiError, ErrorStruct};
use jsonwebtoken::jwk::{AlgorithmParameters, JwkSet};
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use reqwest::Client;
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome, Request};
use serde::Deserialize;
use std::sync::Arc;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use tokio::sync::RwLock;

const JWKS_REFRESH_INTERVAL: Duration = Duration::from_secs(86400);

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

    async fn fetch_jwks(&self, domain: &str) -> Result<JwkSet, ApiError<String>> {
        let last_updated = self.last_updated.read().await;

        let elapsed = match SystemTime::now().duration_since(*last_updated) {
            Ok(elapsed) => elapsed,
            Err(_) => {
                return Err(ApiError::AuthenticationError(ErrorStruct::new(
                    "Failed to unlock last_updated.",
                    None,
                )));
            }
        };

        let keys = { self.keys.read().await };

        if elapsed > JWKS_REFRESH_INTERVAL || keys.is_none() {
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
                                Ok(jwks)
                            }
                            Err(e) => Err(ApiError::AuthenticationError(ErrorStruct::new(
                                "Failed to parse JWKS response.",
                                Some(e.to_string()),
                            ))),
                        }
                    } else {
                        Err(ApiError::AuthenticationError(ErrorStruct::new(
                            "Failed to fetch JWKS.",
                            Some(format!("Response code: {}", resp.status())),
                        )))
                    }
                }
                Err(e) => Err(ApiError::AuthenticationError(ErrorStruct::new(
                    "Failed to make JWKS request.",
                    Some(e.to_string()),
                ))),
            }
        } else {
            match keys.clone() {
                Some(jwks) => Ok(jwks),
                _ => Err(ApiError::AuthenticationError(ErrorStruct::new(
                    "Failed to make JWKS request.",
                    None,
                ))),
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
    pub user_id: String,
    pub scopes: Vec<String>,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AuthenticatedUser {
    type Error = ApiError<String>;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let token = match request.headers().get_one("Authorization") {
            Some(value) if value.starts_with("Bearer ") => value[7..].to_string(),
            _ => {
                let error = ApiError::AuthenticationError(ErrorStruct::new(
                    "No token found in header",
                    None,
                ));

                request.local_cache(|| error.clone());

                return Outcome::Error((Status::Unauthorized, error));
            }
        };

        let auth0_config = match request.rocket().state::<Auth0Config>() {
            Some(config) => config,
            _ => {
                let error = ApiError::AuthenticationError(ErrorStruct::new(
                    "Auth0 config not found.",
                    None,
                ));

                request.local_cache(|| error.clone());

                return Outcome::Error((Status::Unauthorized, error));
            }
        };

        let jwk_cache = match request.rocket().state::<Arc<Jwks>>() {
            Some(cache) => cache,
            _ => {
                let error =
                    ApiError::AuthenticationError(ErrorStruct::new("JWKS cache not found.", None));

                request.local_cache(|| error.clone());

                return Outcome::Error((Status::Unauthorized, error));
            }
        };

        let jwks = match jwk_cache.fetch_jwks(&auth0_config.domain).await {
            Ok(jwks) => jwks.clone(),
            Err(e) => {
                request.local_cache(|| e.clone());
                return Outcome::Error((Status::Unauthorized, e));
            }
        };

        let header = match jsonwebtoken::decode_header(&token) {
            Ok(header) => header,
            Err(_) => {
                let error =
                    ApiError::AuthenticationError(ErrorStruct::new("Invalid token header.", None));

                request.local_cache(|| error.clone());
                return Outcome::Error((Status::Unauthorized, error));
            }
        };

        let kid = match header.kid {
            Some(kid) => kid,
            _ => {
                let error =
                    ApiError::AuthenticationError(ErrorStruct::new("Header missing kid.", None));

                request.local_cache(|| error.clone());

                return Outcome::Error((Status::Unauthorized, error));
            }
        };

        let jwk = match jwks.find(&kid) {
            Some(jwk) => jwk,
            _ => {
                let error = ApiError::AuthenticationError(ErrorStruct::new(
                    "No matching key in JWKs.",
                    None,
                ));

                request.local_cache(|| error.clone());

                return Outcome::Error((Status::Unauthorized, error));
            }
        };

        let decoding_key = match &jwk.algorithm {
            AlgorithmParameters::RSA(rsa) => {
                match DecodingKey::from_rsa_components(&rsa.n, &rsa.e) {
                    Ok(key) => key,
                    Err(_) => {
                        let error = ApiError::AuthenticationError(ErrorStruct::new(
                            "Failed to decode key.",
                            None,
                        ));

                        request.local_cache(|| error.clone());

                        return Outcome::Error((Status::Unauthorized, error));
                    }
                }
            }
            _ => {
                let error = ApiError::AuthenticationError(ErrorStruct::new(
                    "Unsupported key type in JWKS.",
                    None,
                ));

                request.local_cache(|| error.clone());

                return Outcome::Error((Status::Unauthorized, error));
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
            Err(e) => {
                let error = ApiError::AuthenticationError(ErrorStruct::new(
                    "JWT validation failed.",
                    Some(e.to_string()),
                ));

                request.local_cache(|| error.clone());

                Outcome::Error((Status::Unauthorized, error))
            }
        }
    }
}
