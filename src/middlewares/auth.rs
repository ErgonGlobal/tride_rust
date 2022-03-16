use actix_web::error::ErrorUnauthorized;
use actix_web::{dev, Error, FromRequest, HttpMessage, HttpRequest};
use futures::future::{err, ok, Ready};
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use serde::{Deserialize, Serialize};

pub struct AuthorizationService {
    pub user: String,
    pub path: String,
}
#[derive(Debug, Serialize, Deserialize)]

pub struct Claims {
    pub id: String,
    pub exp: usize,
    pub iat: usize,
    pub firstName: String,
    pub lastName: String,
}

impl FromRequest for AuthorizationService {
    type Error = Error;
    type Future = Ready<Result<AuthorizationService, Error>>;

    fn from_request(_req: &HttpRequest, _payload: &mut dev::Payload) -> Self::Future {
        let _auth = _req.headers().get("Authorization");
        let _path = _req.uri().path().to_string();
        println!("{}", _path);
        match _auth {
            Some(_) => {
                let _split: Vec<&str> = _auth.unwrap().to_str().unwrap().split("Bearer").collect();
                let token = _split[1].trim();

             

                // ok(AuthorizationService { authorization: token.to_string()  , path: _path})
                let key = "jwt-ergon-secret".as_ref();
                match decode::<Claims>(
                    token,
                    &DecodingKey::from_secret(key),
                    &Validation::new(Algorithm::HS256),
                ) {
                    Ok(_token) => ok(AuthorizationService {
                        user: _token.claims.id.to_string(),
                        path: _path,
                    }),
                    Err(_e) => err(ErrorUnauthorized("invalid token!")),
                }
                /*
                let _config: Config = Config {};
                let _var = _config.get_config_with_key("SECRET_KEY");
                let key = _var.as_bytes();
                match decode::<Claims>(
                    token,
                    &DecodingKey::from_secret(key),
                    &Validation::new(Algorithm::HS256),
                ) {
                    Ok(_token) => ok(AuthorizationService),
                    Err(_e) => err(ErrorUnauthorized("invalid token!")),
                }
                */
            }
            None => err(ErrorUnauthorized("Unauthorized")),
        }
    }
}
