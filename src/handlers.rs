use std::hash::{
    Hash,
    Hasher
};
use actix_web::{
    web,
    get,
    post,
    Responder,
    Error,
    error::InternalError,
};
use actix_session::storage::RedisSessionStore;
use actix_session::Session;
use bitcoin::bip32::Xpub;
use serde_json;

use mongodb::{bson::doc, options::IndexOptions, Client, Collection, IndexModel};

use crate::model;

pub const DB_NAME: &str = "xpub-session-api";
pub const COLL_NAME: &str = "addresses";

#[get("/hello")]
// This will be the general information page for this API.
pub async fn hello() -> Result<impl Responder, Error> {
    Ok("World!")
}

#[post("/login")]
/// Login handler
pub async fn login(
    client: web::Data<Client>,
    credentials: web::Json<model::Credentials<model::XpubWrapper>>,
    //credentials: web::Json<model::Dummy>,
    session: Session,
) -> Result<impl Responder, Error> {
    let credentials = credentials.into_inner();

    match model::Address::authenticate(client, credentials).await {
        Ok(address) => {
            session.insert("nonce", address.get_nonce()).unwrap();
            Ok("Authorized")
        },
        Err(err) => return Err(InternalError::from_response("", err).into()),
    }
}

// Make addresses persistent references unique.
pub async fn create_address_index(client: &Client) -> Result<(), mongodb::error::Error>{
    let options = IndexOptions::builder().unique(true).build();
    let model = IndexModel::builder()
        .keys(doc!{
            "xpub": 1
        })
        .build();
    client
        .database(DB_NAME)
        .collection::<model::Address<model::XpubWrapper>>(COLL_NAME)  // TODO: Change the type of Address<T> to Address<Xpub>
        .create_index(model)
        .await?;
    Ok(())
}