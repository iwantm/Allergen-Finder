use super::error::AuthError;
use jsonwebtoken::jwk::JwkSet;
use reqwest::Client;
use rocket::serde::json::Json;
use std::sync::{Arc, RwLock};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

struct Auth0Config {
    authority: String,
    audience: String,
}

struct Jwks {
    keys: RwLock<Option<JwkSet>>,
    last_updated: RwLock<SystemTime>,
    http_client: Client,
}

impl Jwks {
    fn new(http_client: Client) -> Jwks {
        Jwks {
            keys: RwLock::new(None),
            last_updated: RwLock::new(UNIX_EPOCH),
            http_client,
        }
    }

    async fn fetch_jwks(&self, domain: &str) -> Result<Jwks, AuthError<String>> {
        let elapsed = match self.last_updated.read() {
            Ok(last_updated) => match SystemTime::now().duration_since(*last_updated) {
                Ok(elapsed) => elapsed,
                Err(_) => {
                    return Err(AuthError::JwkError(Json(
                        "Failed to unlock last_updated.".to_string(),
                    )))
                }
            },
            Err(_) => {
                return Err(AuthError::JwkError(Json(
                    "Failed to unlock last_updated.".to_string(),
                )));
            }
        };

        let keys = match self.keys.read() {
            Ok(keys) => keys,
            Err(_) => {
                return Err(AuthError::JwkError(Json(
                    "Failed to unlock keys.".to_string(),
                )));
            }
        };

        let _ = if elapsed > Duration::from_secs(86400) || keys.is_none() {
            let jwks_url = format!("https://{}/.well-known/jwks.json", domain);
            match self.http_client.get(&jwks_url).send().await {
                Ok(resp) => {
                    if resp.status().is_success() {
                        match resp.json::<JwkSet>().await {
                            Ok(jwks) => {
                                *self.keys.write().unwrap() = Some(jwks.clone());
                                *self.last_updated.write().unwrap() = SystemTime::now();
                                Ok(jwks)
                            }
                            Err(_) => Err(AuthError::JwkError(Json(
                                "Failed to parse JWKS response".to_string(),
                            ))),
                        }
                    } else {
                        Err(AuthError::JwkError(Json(format!(
                            "Failed to fetch JWKS: HTTP {}",
                            resp.status()
                        ))))
                    }
                }
                Err(_) => {
                    return Err(AuthError::JwkError(Json(
                        "Failed to make jwk request.".to_string(),
                    )));
                }
            }
        } else {
            match self.keys.read().unwrap().clone() {
                Some(jwks) => Ok(jwks),
                None => Err(AuthError::JwkError(Json(
                    "Failed to make jwk request.".to_string(),
                ))),
            }
        };
        Err(AuthError::JwkError(Json(
            "Failed to unlock last_updated.".to_string(),
        )))
    }
}
