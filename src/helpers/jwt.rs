use crate::helpers::error::ActixError;
use crate::helpers::config::Config;
use actix_web::dev::ServiceRequest;
use actix_web::web::Data;
use actix_web::HttpMessage;
use actix_web::Result;
use actix_web_httpauth::extractors::bearer::BearerAuth;
use actix_web_httpauth::extractors::AuthenticationError;
use actix_web_httpauth::headers::www_authenticate::bearer::Bearer;
use eyre::eyre;
use jsonwebtoken::decode;
use jsonwebtoken::encode;
use jsonwebtoken::DecodingKey;
use jsonwebtoken::EncodingKey;
use jsonwebtoken::Header;
use jsonwebtoken::Validation;
use serde::{Deserialize, Serialize};
use tracing::{error, info};

pub struct UserId(pub i64);

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    sub: i64,
    iss: String,
    exp: usize,
}

pub async fn issue_token(secret: &[u8], user_id: i64) -> eyre::Result<String> {
    let my_claims = Claims {
        sub: user_id,
        iss: "myissuer".to_owned(),
        exp: 9000000000,
    };

    Ok(encode(
        &Header::default(),
        &my_claims,
        &EncodingKey::from_secret(secret),
    )?)
}

pub async fn token_validator(
    req: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest> {
    let config: &Data<Config> = req.app_data().ok_or_else(|| {
        let e: ActixError = eyre!("No config!").into();
        e
    })?;

    let token = credentials.token();
    let validation = Validation {
        iss: Some("myissuer".to_string()),
        ..Default::default()
    };

    let token_data = decode::<Claims>(
        &token,
        &DecodingKey::from_secret(config.jwt_secret.as_bytes()),
        &validation,
    );

    match token_data {
        Ok(token_data) => {
            info!("{:?}", token_data.claims);
            info!("{:?}", token_data.header);

            req.extensions_mut().insert(UserId(token_data.claims.sub));

            Ok(req)
        }
        Err(e) => {
            error!("{}", e);
            Err(AuthenticationError::new(Bearer::default()).into())
        }
    }
}
