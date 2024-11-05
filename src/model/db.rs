use super::*;
use actix_web::{
    Error,
    error::InternalError
};
use actix_session::Session;
use mongodb::{
    Client,
    IndexModel,
    options::IndexOptions,
};
use crate::model;
// model::db::lookup(client, credentials).await

pub async fn insert_address_from_credentials(
    collection: Collection<model::UserAddress<XpubWrapper>>, 
    credentials: Credentials<XpubWrapper>
) -> Result<model::UserAddress<XpubWrapper>, HttpResponse> {
    let address = if model::UserAddress::authenticate(credentials.clone()).await? {
        model::UserAddress::from_credentials(credentials.clone())
    } else {
        return Err(HttpResponse::Unauthorized().json("Unauthorized"))
    };
    match collection.insert_one(address.clone()).await {
        Ok(_) => Ok(address),
        Err(err) => Err(HttpResponse::InternalServerError().body(err.to_string())),
    }
}

pub async fn lookup_or_update_address(
    client: web::Data<Client>,
    session: Session,
) -> Result<model::UserAddress<XpubWrapper>, Error> {
    match session.get::<model::Credentials<model::XpubWrapper>>("credentials")? {
        Some(credential) => {
            let internal_address = model::UserAddress::from_credentials(credential.clone());
            let address = match model::db::address_lookup(client, credential.clone()).await {
                Ok(lookup_address) => lookup_address,
                Err(err) => return Err(InternalError::from_response("", err).into())
            };
            Ok(address)
        },
        None => Err(InternalError::from_response("", HttpResponse::Unauthorized().json("Unauthorized")).into())
    }
}

pub async fn address_lookup(
    client: web::Data<Client>, 
    credentials: Credentials<XpubWrapper>
) -> Result<model::UserAddress<XpubWrapper>, HttpResponse> {
    let credential_xpub: bip32::Xpub = credentials.xpub.clone().to_xpub();
    let collection: Collection<model::UserAddress<XpubWrapper>> = client.database(DB_NAME).collection(COLL_NAME);
            match collection.find_one(doc! {"xpub": &credentials.xpub}).await {
                Ok(Some(address)) => {
                    Ok(address)
                },
                Ok(None) => Err(HttpResponse::NotFound().json("NotFound")),
                Err(err) => Err(HttpResponse::InternalServerError().body(err.to_string())),
            }
}

pub async fn update_address(
    client: web::Data<Client>, 
    updated_address: model::UserAddress<XpubWrapper>
) -> Result<model::UserAddress<XpubWrapper>, HttpResponse> {
    let collection: Collection<model::UserAddress<XpubWrapper>> = client.database(DB_NAME).collection(COLL_NAME);
    let filter_doc = doc! { 
        "xpub": updated_address.clone().get_xpubwrapper()
    };
    let update_doc = doc! { 
        "$set": doc! { 
            "xpub_list": updated_address.clone().get_xpub_list()
        } 
    };
    match collection.update_one(filter_doc, update_doc).await {
        Ok(_) => Ok(updated_address),
        Err(err) => Err(HttpResponse::InternalServerError().body(err.to_string())),
    }
}

pub async fn update_address_nonce(
    collection: Collection<model::UserAddress<XpubWrapper>>,
    address: model::UserAddress<XpubWrapper>
) -> Result<model::UserAddress<XpubWrapper>, HttpResponse>  {
    // Update nonce on persistent db
    let updated_address = address.clone().update_nonce(); // TODO: Unleash the nonce updating procedure.
    match collection.insert_one(updated_address.clone()).await {
        Ok(_) => Ok(updated_address),
        Err(err) => Err(HttpResponse::InternalServerError().body(err.to_string())),
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
        .collection::<model::UserAddress<model::XpubWrapper>>(COLL_NAME)
        .create_index(model)
        .await?;
    Ok(())
}