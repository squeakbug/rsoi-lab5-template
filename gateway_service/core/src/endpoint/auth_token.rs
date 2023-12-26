use std::pin::Pin;

use actix_web::{
    dev::Payload,
    error::{Error as ActixWebError, ErrorUnauthorized},
    http, web, FromRequest, HttpRequest,
};
use futures::Future;
use serde_json::json;

use crate::endpoint::auth_controller::{get_okta_user, validate_token};
use crate::state::AppState;

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

#[cfg(test)]
mod tests {
    use chrono::Utc;
    use jsonwebtoken::{decode, Algorithm, DecodingKey, EncodingKey, Header, Validation};

    use crate::endpoint::auth_controller::TokenClaims;

    #[test]
    fn test_jwk_encoding() {
        let jwt_secret = "my super jwt secret";
        let now = Utc::now();
        let iat = now.timestamp() as usize;
        let max_exp_in_mins = 1000;
        let exp = (now + chrono::Duration::minutes(max_exp_in_mins)).timestamp() as usize;
        let claims: TokenClaims = TokenClaims {
            sub: String::from("user_id"),
            exp,
            iat,
        };

        let token = jsonwebtoken::encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(jwt_secret.as_ref()),
        )
        .unwrap();

        dbg!(&token);

        let decode = decode::<TokenClaims>(
            token.as_str(),
            &DecodingKey::from_secret(jwt_secret.as_ref()),
            &Validation::new(Algorithm::HS256),
        );

        let decoded_claims = decode.unwrap().claims;

        assert!(decoded_claims.sub == "user_id");
    }
}
