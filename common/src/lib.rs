use axum::{
    async_trait,
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
};
use serde::Deserialize;
use tower_jwt::{DecodingKey, JwtLayer, RequestClaim, Validation};

#[derive(Clone, Debug, Deserialize)]
pub struct Claim {
    pub name: String,
    pub sub: String,
    pub scopes: Vec<Scope>,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Scope {
    Admin,
    Order,
}

impl Claim {
    pub fn has_scope(&self, scope: Scope) -> Result<(), StatusCode> {
        if self.scopes.contains(&scope) {
            Ok(())
        } else {
            Err(StatusCode::FORBIDDEN)
        }
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for Claim {
    type Rejection = StatusCode;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let jwt = parts
            .extensions
            .get::<RequestClaim<Claim>>()
            .ok_or(StatusCode::UNAUTHORIZED)?;

        Ok(jwt.claim.clone())
    }
}

pub fn get_jwt_layer() -> JwtLayer<Claim, DecodingKey> {
    let mut validation = Validation::default();
    validation.required_spec_claims.clear();

    JwtLayer::new(validation, DecodingKey::from_secret("secret".as_bytes()))
}
