use actix_web::cookie::Cookie;
use actix_web::web::Data;
use actix_web::{get, post, web, HttpResponse, Responder, Result};
use chrono::prelude::*;
use jsonwebtoken::errors::Error;
use jsonwebtoken::{decode, Algorithm, DecodingKey, EncodingKey, Header, TokenData, Validation};
use reqwest::{Client, Url};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::endpoint::error_controller::ErrorResponse;
use crate::state::{AppState, User};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TokenRequestPasswordError {
    Status400(crate::models::ValidationErrorResponse),
    Status404(crate::models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

#[derive(Debug, Serialize, Deserialize)]
struct OktaResponseParams {
    #[serde(rename = "code")]
    code: String,
    #[serde(rename = "state")]
    state: String,
}

#[derive(Serialize)]
struct TokenRequestAuthorizeCode<'a> {
    code: &'a str,
    grant_type: &'a str,
    redirect_uri: &'a str,
    client_id: &'a str,
    client_secret: &'a str,
    username: &'a str,
    password: &'a str,
}

#[derive(Serialize, Deserialize)]
struct TokenRequestPassword {
    scope: String,
    grant_type: String,
    username: String,
    password: String,
    client_id: String,
    client_secret: String,
    audience: String,
}

#[derive(Deserialize, Debug)]
struct TokenResponsePassword {
    access_token: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    aud: String, // Optional. Audience
    exp: usize,  // Required (validate_exp defaults to true in validation). Expiration time (as UTC timestamp)
    iat: usize,  // Optional. Issued at (as UTC timestamp)
    iss: String, // Optional. Issuer
    nbf: usize,  // Optional. Not Before (as UTC timestamp)
    sub: String, // Optional. Subject (whom token refers to)
}

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

#[get("/callback")]
pub async fn oauth_callback(
    data: Data<AppState>,
    query: web::Query<OktaResponseParams>,
) -> Result<impl Responder, ErrorResponse> {
    let local_var_client = Client::new();

    let domain_string = &data.config.okta_oauth_domain;
    let local_var_uri_str = format!("https://{0}/oauth/token", domain_string);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    let code = query.code.as_str();
    let client_id = &data.config.okta_oauth_client_id;
    let client_secret = &data.config.okta_oauth_client_secret;
    let grant_type = "password";
    let username = "squeakbug73@outlook.com";
    let password = "Uxli2069Inux3129";
    let redirect_uri = &data.config.okta_oauth_redirect_url;

    let token_request = TokenRequestAuthorizeCode {
        client_id,
        client_secret,
        code,
        grant_type,
        redirect_uri,
        username,
        password,
    };

    local_var_req_builder = local_var_req_builder.header(reqwest::header::CONTENT_TYPE, "application/json");
    local_var_req_builder = local_var_req_builder.json(&token_request);

    let local_var_req = local_var_req_builder.build().expect("Bad bad");
    let local_var_resp = local_var_client.execute(local_var_req).await.expect("Bad bad");
    let local_var_content = local_var_resp.json::<TokenResponsePassword>().await.expect("Bad bad");

    let access_token = local_var_content.access_token;
    let google_user = get_okta_user(&access_token, domain_string).await;

    let mut db = data.user_tokens.0.lock().await;

    let user_id: String;
    let username = google_user.name;
    let maybe_usr = db.get(&username);
    if maybe_usr.is_some() {
        user_id = maybe_usr.unwrap().id.to_owned().unwrap();
    } else {
        let datetime = Utc::now();
        let id = Uuid::new_v4();
        user_id = id.to_owned().to_string();
        let user_data = User {
            id: Some(id.to_string()),
            name: username.clone(),
            verified: google_user.email_verified,
            email: google_user.email.to_owned(),
            provider: "Okta".to_string(),
            role: "user".to_string(),
            password: "".to_string(),
            photo: google_user.picture,
            created_at: Some(datetime),
            updated_at: Some(datetime),
        };

        db.insert(user_id.clone(), user_data.to_owned());
    }

    let jwt_secret = data.config.jwt_secret.clone();
    let now = Utc::now();
    let iat = now.timestamp() as usize;
    let max_exp_in_mins = 1000;
    let exp = (now + chrono::Duration::minutes(max_exp_in_mins)).timestamp() as usize;
    let claims: TokenClaims = TokenClaims { sub: user_id, exp, iat };

    let token = jsonwebtoken::encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(jwt_secret.as_ref()),
    )
    .unwrap();

    let exp_sec = actix_web::cookie::time::Duration::new(60 * 10000, 0);
    let cookie = Cookie::build("token", token)
        .path("/")
        .max_age(exp_sec)
        .http_only(true)
        .finish();

    Ok(HttpResponse::Found().cookie(cookie).finish())
}

#[get("/authorize")]
async fn oauth_login_2(data: Data<AppState>) -> Result<impl Responder, ErrorResponse> {
    let local_var_client = Client::new();

    let domain_string = &data.config.okta_oauth_domain;
    let local_var_uri_str = format!("https:{0}/authorize", domain_string);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    let client_id = data.config.okta_oauth_client_id.as_str();
    let response_type = "code";
    let scope = "openid profile email";
    let state = "shit";
    let redirect_uri = data.config.okta_oauth_redirect_url.as_str();
    local_var_req_builder = local_var_req_builder.query(&[
        ("client_id", &client_id),
        ("response_type", &response_type),
        ("scope", &scope),
        ("state", &state),
        ("redirect_uri", &redirect_uri),
    ]);

    let local_var_req = local_var_req_builder.build().expect("Bad bad");

    let redirect_url = local_var_req.url().to_string();

    Ok(actix_web::web::Redirect::to(redirect_url).see_other())
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

#[derive(Serialize)]
pub struct LoginResponse {
    pub access_token: String,
}

#[post("/authorize")]
async fn oauth_login(
    data: Data<AppState>,
    token_request_body: web::Form<TokenRequestPassword>,
) -> Result<impl Responder, ErrorResponse> {
    let local_var_client = Client::new();

    let domain_string = &data.config.okta_oauth_domain;
    let local_var_uri_str = format!("https://{0}/oauth/token", domain_string);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    local_var_req_builder = local_var_req_builder.header(reqwest::header::CONTENT_TYPE, "application/json");
    local_var_req_builder = local_var_req_builder.json(&token_request_body);

    let local_var_req = local_var_req_builder.build().expect("Bad bad");
    let local_var_resp = local_var_client.execute(local_var_req).await.expect("Bad bad");
    let local_var_content = local_var_resp.json::<TokenResponsePassword>().await.expect("Bad bad");

    let access_token = local_var_content.access_token;
    let _google_user = get_okta_user(&access_token, domain_string).await;

    let decoded = validate_token(&access_token, domain_string).await;
    if decoded.is_ok() {
        let response_body = LoginResponse { access_token };
        Ok(HttpResponse::Ok().json(response_body))
    } else {
        Ok(HttpResponse::Unauthorized().finish())
    }
}

#[get("/logout")]
async fn logout(_: Data<AppState>, _: web::Json<Claims>) -> Result<impl Responder, ErrorResponse> {
    Ok(HttpResponse::Ok())
}
