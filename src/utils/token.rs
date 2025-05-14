use std::collections::BTreeMap;

use crate::{enums::response_enum::VerifiedToken, structs::account_struct::AccountVerification};
use hmac::{Hmac, Mac};
use jwt::{Header, Token, VerifyWithKey};
use sha2::Sha384;
use tracing::error;

pub fn verified_token (token: &str) -> VerifiedToken {
    let env_password_magic_key = dotenvy::var("PASSWORD_MAGICKEY");

    if let Err(e) = env_password_magic_key {
        error!("Error For Reading ENV: {}", e);
        return VerifiedToken::TokenIsNotValid;
    }



    let jwt_key: Hmac<Sha384> = Hmac::new_from_slice(env_password_magic_key.unwrap().as_bytes()).unwrap();

    let token_verification: Result<Token<Header, BTreeMap<String, AccountVerification>, _>, jwt::Error> = token.verify_with_key(&jwt_key);
  
    match token_verification {
        Ok(_) => VerifiedToken::TokenIsValid,
        Err(err) => {
            println!("{:?}", err);
            return VerifiedToken::TokenIsNotValid;
        }
    }
}