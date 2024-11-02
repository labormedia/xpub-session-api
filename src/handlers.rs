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
    HttpResponse,
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
    session: Session,
) -> Result<impl Responder, Error> {
    let credentials = credentials.into_inner();
    match model::Address::authenticate(credentials.clone()).await {
        Ok(false) => Ok("Unauthorized"),
        Ok(true) => {
            session.insert("credentials", credentials.clone()).unwrap();
            Ok("Authorized")
        },
        Err(err) => Err(InternalError::from_response("", err).into()),
    }
}


#[get("/get_address")]
pub async fn get_address(
    client: web::Data<Client>,
    session: Session,
) -> Result<impl Responder, Error> {
    match model::db::lookup_or_update_address(client, session).await {
        Ok(address) => Ok(web::Json(address)),
        Err(err) => Err(err)
    }
}

#[get("/derive_address/{first_path}/{second_path}")]
pub async fn derive_address(
    path: web::Path<(u32, u32)>,
    client: web::Data<Client>,
    session: Session,
) -> Result<impl Responder, Error> {
    let (first, second) = path.into_inner();
    let derivation_path = [first, second];
    match model::db::lookup_or_update_address(client.clone(), session).await {
        Ok(address) => {
            let mut new_address = address.clone();
            let derived_xpub = model::derivation::derive_xpub(new_address.clone().get_xpub().to_xpub(), &derivation_path);
            new_address.insert_xpub(derived_xpub.into());
            model::db::insert_or_update_address(client, new_address.clone());
            Ok(web::Json(new_address))
        },
        Err(err) => Err(err)
    }
}


// Make addresses' persistent references unique.
pub async fn create_address_index(client: &Client) -> Result<(), mongodb::error::Error>{
    let options = IndexOptions::builder().unique(true).build();
    let model = IndexModel::builder()
        .keys(doc!{
            "xpub": 1
        })
        .build();
    client
        .database(DB_NAME)
        .collection::<model::Address<model::XpubWrapper>>(COLL_NAME)
        .create_index(model)
        .await?;
    Ok(())
}