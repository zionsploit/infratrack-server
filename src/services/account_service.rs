use std::{collections::BTreeMap, sync::Arc};
use axum::{http::StatusCode, response::IntoResponse, Extension, Json};
use hmac::{Hmac, Mac};
use jwt::{AlgorithmType, Header, SignWithKey, Token };
use magic_crypt::{new_magic_crypt, MagicCryptTrait};
use sha2::Sha384;
use crate::{
    enums::response_enum::{ResponseErrorMessage, ResponseOkMessage, VerifiedToken}, 
    structs::{
        account_struct::{AccountLogin, AccountRegistration, ReturnAccountInformation, TokenVerification}, 
        pool_conn_struct::PoolConnectionState
    }, utils::token::verified_token};

// POST METHOD
pub async fn register_account (
    Extension(sql_pool): Extension<Arc<PoolConnectionState>>,
    Json(request): Json<AccountRegistration>,
) -> impl IntoResponse {

    let env_password_magic_key = dotenvy::var("PASSWORD_MAGICKEY");

    if let Err(e) = env_password_magic_key {
        return (StatusCode::INTERNAL_SERVER_ERROR, format!("{:?}: {}", ResponseErrorMessage::ReadingPasswordKeyNotFound, e));
    }

    let encypt_aes_password = if request.password.clone().eq(&request.confirmpassword) {
        let mc = new_magic_crypt!(env_password_magic_key.unwrap(), 256);

        let password_base64 = mc.encrypt_bytes_to_base64(&request.password);
        password_base64
    } else {
        return (StatusCode::BAD_REQUEST, format!("{:?}: password and confirm password doesn't match", ResponseErrorMessage::PasswordIsNotSame));
    };
    

    let find_account_by_username: Result<ReturnAccountInformation, sqlx::Error> = sqlx::query_as("SELECT * from account WHERE user_name = ?").bind(&request.username).fetch_one(&sql_pool.connection).await;

    if let Ok(_) = find_account_by_username {
        return (StatusCode::BAD_REQUEST, format!("{:?}", ResponseErrorMessage::UsernameIsExists));
    }

    match 
        sqlx::query("INSERT INTO account (first_name, middle_name, last_name, user_name, password) VALUES (?, ?, ?, ?, ?)")
            .bind(request.firstname)
            .bind(request.middlename)
            .bind(request.lastname)
            .bind(request.username)
            .bind(encypt_aes_password)
            .execute(&sql_pool.connection).await {
        Ok(_) => (StatusCode::OK, format!("{:?}", ResponseOkMessage::NewAccountCreated)),
        Err(error) => (StatusCode::INTERNAL_SERVER_ERROR, format!("{:?} {}", ResponseErrorMessage::ExecutingQueryError, error))
    }
}

pub async fn login_account (
    Extension(sql_pool): Extension<Arc<PoolConnectionState>>,
    Json(request): Json<AccountLogin>
) -> impl IntoResponse {
    let env_password_magic_key = dotenvy::var("PASSWORD_MAGICKEY");

    if let Err(e) = env_password_magic_key {
        return (StatusCode::INTERNAL_SERVER_ERROR, format!("{:?}: {}", ResponseErrorMessage::ReadingPasswordKeyNotFound, e));
    }

    let encyrpt_aes_password = {
        let mc = new_magic_crypt!(env_password_magic_key.as_ref().unwrap(), 256);
        mc.encrypt_bytes_to_base64(&request.password)
    };

    match sqlx::query_as("SELECT * FROM account where user_name = ? AND password = ?")
        .bind(request.username)
        .bind(encyrpt_aes_password)
        .fetch_one(&sql_pool.connection).await {
            Ok(result) => {
                let account_information: ReturnAccountInformation = result;

                let jwt_key: Hmac<Sha384> = Hmac::new_from_slice(env_password_magic_key.unwrap().as_bytes()).unwrap();
                
                let header = Header {
                    algorithm: AlgorithmType::Hs384,
                    ..Default::default()
                };

                let mut claims: BTreeMap<String, ReturnAccountInformation> = BTreeMap::new();

                claims.insert("sub".to_owned(), account_information);

                let token = Token::new(header, claims).sign_with_key(&jwt_key);

                if let Err(e) = token {
                    return (StatusCode::INTERNAL_SERVER_ERROR, format!("{:?}: {}", ResponseErrorMessage::ErrorGeneratingToken, e));
                }

                (StatusCode::OK, format!("{:?}", token.unwrap().as_str()))
            }
            Err(_) => (StatusCode::BAD_REQUEST, format!("{:?}", ResponseErrorMessage::AccountNotFound))
        }
}

pub async fn verify_token (
    Json(request): Json<TokenVerification>
) -> impl IntoResponse {

    match verified_token(&request.token) {
        VerifiedToken::TokenIsValid => (StatusCode::OK, format!("{:?}", ResponseOkMessage::TokenIsValid)),
        VerifiedToken::TokenIsNotValid => (StatusCode::FORBIDDEN, format!("{:?}", ResponseErrorMessage::TokenIsNotValid))
    }

}