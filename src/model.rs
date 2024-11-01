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
        Bson,
        to_document,
    },
};
use crate::{
    handlers::{
        DB_NAME,
        COLL_NAME,
    }
};
use bitcoin::{
    bip32,
    secp256k1,
    sign_message::{
        MessageSignature,
        signed_msg_hash,
    },
};
pub mod derivation;

#[derive(Clone, Serialize, Deserialize)]
pub struct CredentialWitness(
    #[serde(with = "serde_bytes")]
    [u8; 65]
);

impl CredentialWitness {
    fn get_slice(self) -> [u8; 65] {
        self.0
    }
}

#[derive(Clone, Hash, Serialize, Deserialize, Debug)]
pub struct Nonce(u32);

impl Nonce {
    fn to_str(self) -> String {
        self.0.to_string()
    }
}

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
        mongodb::bson::Bson::Document(to_document(&self).expect("Known size"))
    }
}

#[derive(Clone, Hash, Serialize, Deserialize)]
pub struct Address<T: Hash> {
    xpub: T,
    nonce: Nonce,
    xpub_list: Vec<T>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Credentials<T: Hash> {
    witness: CredentialWitness,
    xpub: T,
    nonce: Nonce,
}

impl<T: Hash> Credentials<T> {
    pub fn get_nonce(self) -> Nonce {
        self.nonce.clone()
    }
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
        let credential_xpub: bip32::Xpub = credentials.xpub.clone().to_xpub();
        let public_key = credential_xpub.public_key;
        let mut message = credential_xpub.to_string().to_owned();
        message.push_str(&credentials.nonce.to_str());
        let credential_signature: MessageSignature = match MessageSignature::from_slice(&credentials.witness.get_slice()) {
            Ok(signature) => signature,
            Err(err) => return Err(HttpResponse::Unauthorized().json("Unauthorized")),
        };
        let is_signed = match derivation::verify(public_key, &message, credential_signature) {
            Ok(is_signed) => is_signed,
            Err(err) => return Err(HttpResponse::Unauthorized().json("Unauthorized")),
        };

        if !is_signed {
            Err(HttpResponse::Unauthorized().json("Unauthorized"))
        } else {
            // TODO: update persistent nonce for this address.

            let collection: Collection<Address<XpubWrapper>> = client.database(DB_NAME).collection(COLL_NAME);

            match collection.find_one(doc! {"xpub": &credentials.xpub}).await {
                Ok(Some(address)) => {
                    let address_xpub: bip32::Xpub = address.xpub.clone().to_xpub();
                    let mut address_message = address_xpub.to_string().to_owned();
                    address_message.push_str(&address.nonce.clone().to_str());
                    println!("to sign B {}", address_message);
                    let message_hash = signed_msg_hash(&address_message);
                    // Update nonce on persistent db
                    let updated_address = address.clone();// .update_nonce(); // TODO: Unleash the nonce updating procedure.
                    match collection.insert_one(updated_address.clone()).await {
                        Ok(_) => { 
                            if credential_xpub.encode() != address.xpub.to_bytes() {
                                Err(HttpResponse::Unauthorized().json("Unauthorized"))
                            } else {
                                Ok(updated_address)
                            }
                         },
                        Err(err) => Err(HttpResponse::InternalServerError().body(err.to_string())),
                    }
                },
                Ok(None) => Err(HttpResponse::NotFound().json("NotFound")),
                Err(err) => Err(HttpResponse::InternalServerError().body(err.to_string())),
            }
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Dummy {
    witness: String,
    xpub: String,
    nonce: String,
}