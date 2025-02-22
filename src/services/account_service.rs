use std::{collections::BTreeMap, sync::Arc};
use axum::{http::StatusCode, response::IntoResponse, Extension, Json};
use hmac::{Hmac, Mac};
use jwt::{AlgorithmType, Header, SignWithKey, Token };
use magic_crypt::{new_magic_crypt, MagicCryptTrait};
use sha2::Sha384;
use uuid::Uuid;
use tracing::error;
use crate::{
    enums::response_enum::{ResponseErrorMessage, ResponseOkMessage, VerifiedToken}, 
    structs::{
        account_credentials_struct::{JwtStructure, RequestAccountCredentials, ResponseAccountCredentials}, account_struct::{AccountLogin, TokenVerification}, pool_conn_struct::PoolConnectionState
    }, utils::token::verified_token};

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

    match sqlx::query_as("SELECT * FROM account_credentials where username = ? AND password = ?")
        .bind(request.username)
        .bind(encyrpt_aes_password)
        .fetch_one(&sql_pool.connection).await {
            Ok(result) => {
                let account_information: ResponseAccountCredentials = result;

                let jwt_key: Hmac<Sha384> = Hmac::new_from_slice(env_password_magic_key.unwrap().as_bytes()).unwrap();
                
                let header = Header {
                    algorithm: AlgorithmType::Hs384,
                    ..Default::default()
                };

                let mut claims: BTreeMap<String, JwtStructure> = BTreeMap::new();

                claims.insert("sub".to_owned(), JwtStructure {
                    id: account_information.id.to_string(),
                    username: account_information.username,
                    created_at: account_information.created_at,
                    updated_at: account_information.updated_at
                });

                let token = Token::new(header, claims).sign_with_key(&jwt_key);

                if let Err(e) = token {
                    return (StatusCode::INTERNAL_SERVER_ERROR, format!("{:?}: {}", ResponseErrorMessage::ErrorGeneratingToken, e));
                }

                (StatusCode::OK, token.unwrap().as_str().to_string())
            }
            Err(err) => {
                error!("{}", err);
                (StatusCode::BAD_REQUEST, format!("{:?}", ResponseErrorMessage::AccountNotFound))
            }
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

pub async fn add_account_credentials (
    Extension(sql_pool): Extension<Arc<PoolConnectionState>>,
    Json(request): Json<RequestAccountCredentials>
) -> impl IntoResponse {

    let check_if_username_is_exists: Result<ResponseAccountCredentials, sqlx::Error> = sqlx::query_as("SELECT * FROM account_credentials WHERE username LIKE ? OR recovery_email LIKE ?")
        .bind(format!("{}", &request.username))
        .bind(format!("{}", &request.recover_email))
        .fetch_one(&sql_pool.connection).await;

    if let Ok(_) = check_if_username_is_exists {
        return (StatusCode::CONFLICT, format!("{:?}", ResponseErrorMessage::AlreadyExists));
    };

    let env_password_magic_key = dotenvy::var("PASSWORD_MAGICKEY");

    if let Err(e) = env_password_magic_key {
        return (StatusCode::INTERNAL_SERVER_ERROR, format!("{:?}: {}", ResponseErrorMessage::ReadingPasswordKeyNotFound, e));
    }

    let encypt_aes_password = if request.password.clone().eq(&request.confirm_password) {
        let mc = new_magic_crypt!(env_password_magic_key.unwrap(), 256);

        let password_base64 = mc.encrypt_bytes_to_base64(&request.password);
        password_base64
    } else {
        return (StatusCode::BAD_REQUEST, format!("{:?}", ResponseErrorMessage::PasswordIsNotSame));
    };

    let id: Uuid = Uuid::new_v4();

    match sqlx::query("INSERT INTO account_credentials (id, username, recovery_email, password) VALUES (?, ?, ?, ?)")
        .bind(id.to_string())
        .bind(request.username)
        .bind(request.recover_email)
        .bind(encypt_aes_password).execute(&sql_pool.connection.to_owned()).await {
            Ok(_) => (StatusCode::OK, format!("{:?}", ResponseOkMessage::NewDataCreated)),
            Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, format!("{:?}", ResponseErrorMessage::ExecutingQueryError))
        }
}