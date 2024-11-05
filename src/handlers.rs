use actix_web::{
    web,
    get,
    post,
    Responder,
    Error,
    error::{
        InternalError,
        ErrorInsufficientStorage,
        ErrorUnauthorized,
    },
};
use actix_session::Session;

use mongodb::{bson::doc, Client};

use crate::model;

pub const DB_NAME: &str = "xpub-session-api";
pub const COLL_NAME: &str = "addresses";

#[get("/info")]
// This will be the general information page for this API.
pub async fn info() -> Result<impl Responder, Error> {
    Ok(r#"
        Services:
        /login
        /derive_address/{first_index}/{second_index}
        /get_address
        /create_psbt
    "#)
}

#[post("/login")]
/// Login handler
pub async fn login(
    client: web::Data<Client>,
    credentials: web::Json<model::Credentials<model::XpubWrapper>>,
    session: Session,
) -> Result<impl Responder, Error> {
    let credentials = credentials.into_inner();
    match model::UserAddress::authenticate(credentials.clone()).await {
        Ok(false) => Err(ErrorUnauthorized("Unauthorized")),
        Ok(true) => {
            session.insert("credentials", credentials.clone())?;
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
    match model::db::lookup_or_update_address(client.clone(), session.clone()).await {
        Ok(address) => {
            let size = address.get_xpub_list_ref().len();
            if size > 255 {
                return Err(ErrorInsufficientStorage(size))
            }
            let mut new_address = address.clone();
            let derived_xpub = model::derivation::derive_xpub(&new_address.get_xpub(), &derivation_path);
            new_address.insert_xpub(derived_xpub.into());
            match model::db::update_address(client, new_address.clone()).await {
                Ok(updated_address) => Ok(web::Json(updated_address)),
                Err(err) => Err(InternalError::from_response("", err).into()),
            }
            
        },
        Err(err) => Err(err)
    }
}

/// fn create_psbt builds a psbt from a list of Txin transaction inputs, the recipient's address, 
/// the sender's public keys and the output and input amounts for the transation.
#[post("/create_psbt")]
pub async fn create_psbt(
    client: web::Data<Client>,
    psbt_web: web::Json<model::psbt::PsbtSerialized>,
    session: Session,
) -> Result<impl Responder, Error> {
    let credentials = match session.get("credentials")? {
        Some(credential) => credential,
        None => {
            return Err(ErrorUnauthorized("Unauthorized"));
        }
    };
    match model::UserAddress::authenticate(credentials).await {
        Ok(false) => Err(ErrorUnauthorized("Unauthorized")),
        Ok(true) => {
            let psbt = psbt_web.into_inner().try_into_psbt()?;
            Ok(web::Json(psbt))
        },
        Err(err) => Err(InternalError::from_response("", err).into()),
    }
}