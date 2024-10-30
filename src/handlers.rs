use std::hash::{
    Hash,
    Hasher
};
use actix_web::{
    web,
    Responder,
    Error,
    error::InternalError,
};
use actix_session::storage::RedisSessionStore;
use actix_session::Session;
use bitcoin::bip32::Xpub;

use crate::model;

// This will be the general information page for this API.
pub async fn hello() -> Result<impl Responder, Error> {
    Ok("World!")
}

/// Login handler
pub async fn login(
    credentials: web::Json<model::Credentials<model::CredentialWitness>>,
    session: Session,
) -> Result<impl Responder, Error> {
    let credentials = credentials.into_inner();

    match model::Address::authenticate(credentials) {
        Ok(address) => {
            session.insert("nonce", address.get_nonce()).unwrap();
            Ok("Authorized")
        },
        Err(err) => return Err(InternalError::from_response("", err).into()),
    }
}