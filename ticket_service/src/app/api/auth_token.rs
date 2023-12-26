use std::pin::Pin;

use actix_web::{
    dev::Payload,
    error::{Error as ActixWebError, ErrorUnauthorized},
    http, web, FromRequest, HttpRequest,
};
use futures::Future;
use jsonwebtoken::errors::Error;
use jsonwebtoken::{decode, Algorithm, DecodingKey, TokenData, Validation};
use reqwest::{Client, Url};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::app::api::state::AppState;

#[derive(Deserialize, Default, Debug)]
pub struct OktaUserResult {
    pub sub: String,
    pub nickname: String,
    pub name: String,
    pub picture: String,
    pub updated_at: String,
    pub email: String,
    pub email_verified: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenClaims {
    pub sub: String,
    pub iat: usize,
    pub exp: usize,
}

#[derive(Deserialize, Default, Debug)]
pub struct KeyResult {
    pub kty: String,
    #[serde(rename = "use")]
    pub usage: String,
    pub n: String,
    pub e: String,
    pub kid: String,
    pub x5t: String,
    pub alg: String,
}

#[derive(Deserialize, Default, Debug)]
pub struct KeysResult {
    pub keys: Vec<KeyResult>,
}

pub async fn get_okta_user(access_token: &str, domain_string: &str) -> OktaUserResult {
    let client = Client::new();

    let local_var_uri_str = format!("https://{0}/v1/userinfo", domain_string);
    let uri_str = Url::parse(&local_var_uri_str).expect("Bad url");

    let response = client
        .get(uri_str)
        .bearer_auth(access_token)
        .send()
        .await
        .expect("Bad send request");
    response.json::<OktaUserResult>().await.expect("Bad desirealization")
}

pub async fn validate_token(access_token: &str, domain_string: &str) -> Result<TokenData<TokenClaims>, Error> {
    let client = Client::new();

    let mut validation = Validation::new(Algorithm::RS256);
    validation.set_audience(&[format!("https://{0}/api/v2/", domain_string)]);

    let local_var_uri_str = format!("https://{0}/.well-known/jwks.json", domain_string);
    let uri_str = Url::parse(&local_var_uri_str).expect("Bad url");

    let response = client
        .get(uri_str)
        .bearer_auth(access_token)
        .send()
        .await
        .expect("Bad send request");
    let des_result = response.json::<KeysResult>().await.unwrap();

    let n = &des_result.keys[0].n;
    let e = &des_result.keys[0].e;
    decode::<TokenClaims>(
        access_token,
        &DecodingKey::from_rsa_components(n, e).unwrap(),
        &validation,
    )
}

pub struct AuthenticationGuard {
    pub nickname: String,
    pub access_token: String,
}

impl FromRequest for AuthenticationGuard {
    type Error = ActixWebError;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let access_token = req.cookie("token").map(|c| c.value().to_string()).or_else(|| {
            req.headers()
                .get(http::header::AUTHORIZATION)
                .map(|h| h.to_str().unwrap().split_at(7).1.to_string())
        });

        if access_token.is_none() {
            return Box::pin(async move {
                Err(ErrorUnauthorized(
                    json!({"status": "fail", "message": "You are not logged in, please provide token"}),
                ))
            });
        }

        let data = req.app_data::<web::Data<AppState>>().unwrap().clone();

        Box::pin(async move {
            let decode = validate_token(access_token.clone().unwrap().as_ref(), &data.config.okta_oauth_domain).await;
            match decode {
                Ok(_) => {
                    let _google_user =
                        get_okta_user(&access_token.clone().unwrap(), &data.config.okta_oauth_domain).await;
                    Ok(AuthenticationGuard {
                        nickname: _google_user.nickname.clone(),
                        access_token: access_token.clone().unwrap(),
                    })
                }

                Err(_) => Err(ErrorUnauthorized(
                    json!({"status": "fail", "message": "Invalid token or usre doesn't exists"}),
                )),
            }
        })
    }
}
