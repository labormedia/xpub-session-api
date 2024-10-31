use std::hash::{DefaultHasher, Hash, Hasher};

use actix_web::{
    HttpResponse,
    web,
};
use serde::{Deserialize, Serialize};
use mongodb::{
    Client,
    Collection,
    bson::{
        doc,
        Bson
    },
};
use crate::{
    model,
        handlers::{
        DB_NAME,
        COLL_NAME,
    }
};
use bitcoin::bip32;


pub type CredentialWitness = [u8; 8];

#[derive(Clone, Hash, Serialize, Deserialize)]
pub struct Nonce(u32);

#[derive(Clone, Hash, Serialize, Deserialize)]
pub struct XpubWrapper{
    #[serde(with = "serde_bytes")]
    bytes: [u8; 78],
}

impl XpubWrapper {
    fn to_bytes(self) -> [u8; 78] {
        self.bytes
    }
    fn to_xpub(self) -> bip32::Xpub {
        bip32::Xpub::decode(&self.to_bytes()).expect("Valid Xpub bytes")
    }
}

impl TryFrom<[u8; 78]> for XpubWrapper {
    type Error = bip32::Error;
    fn try_from(value: [u8; 78]) -> Result<Self, Self::Error> {
        match bip32::Xpub::decode(&value) {
            Ok(_xpub) => 
                Ok(XpubWrapper {
                    bytes: value
                }),
            Err(err) => Err(err)
        }
    }
}

impl Into<Bson> for XpubWrapper {
    fn into(self) -> Bson {
        let mut bin = std::io::Cursor::new(self.to_bytes());
        mongodb::bson::Bson::Document(mongodb::bson::Document::from_reader(&mut bin).expect("XpubWrapper value size is known."))
    }
}

#[derive(Clone, Hash, Serialize, Deserialize)]
pub struct Address<T: Hash> {
    xpub: T,
    nonce: Nonce,
    xpub_list: Vec<T>,
}

#[derive(Deserialize)]
pub struct Credentials<T: Hash> {
    witness: CredentialWitness,
    xpub: T,
    nonce: Nonce,
}

impl Address<XpubWrapper> {
    pub fn get_nonce(self: &Self) -> Nonce {
        self.nonce.clone()
    }
    pub fn update_nonce(mut self) -> Self {
        self.nonce = Nonce(self.nonce.0 + 1);
        self
    }
    pub async fn authenticate(client: web::Data<Client>, credentials: Credentials<XpubWrapper>) -> Result<Self, HttpResponse> {
        let mut hasher = DefaultHasher::new();
        credentials.xpub.hash(&mut hasher);
        credentials.nonce.hash(&mut hasher);
        let credential_hash = hasher.finish();
        let credential_witness = u64::from_be_bytes(credentials.witness);

        // Nahive authorization. TODO: implement persistent storage.
        if credential_witness != credential_hash {
            Err(HttpResponse::Unauthorized().json("Unauthorized"))
        } else {
            // Default address for now. 
            // TODO: access address from persistent storage.
            // TODO: update persistent nonce for this address.

            let collection: Collection<model::Address<XpubWrapper>> = client.database(DB_NAME).collection(COLL_NAME);

            match collection.find_one(doc! {"xpub": &credentials.xpub}).await {
                Ok(Some(address)) => {
                    let mut address_hasher = DefaultHasher::new();
                    address.xpub.hash(&mut address_hasher);
                    address.nonce.hash(&mut address_hasher);
                    // Update nonce on persistent db
                    let updated_address = address.clone().update_nonce();
                    match collection.insert_one(updated_address.clone()).await {
                        Ok(_) => { 
                            if credential_witness != address_hasher.finish() {
                                Err(HttpResponse::Unauthorized().json("Unauthorized"))
                            } else {
                                Ok(updated_address)
                            }
                         },
                        Err(err) => Err(HttpResponse::InternalServerError().body(err.to_string())),
                    }
                },
                Ok(None) => Err(HttpResponse::Unauthorized().json("Unauthorized")),
                Err(err) => Err(HttpResponse::InternalServerError().body(err.to_string())),
            }
        }
    }
}