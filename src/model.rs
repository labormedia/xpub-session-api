use std::hash::{DefaultHasher, Hash, Hasher};

use actix_web::HttpResponse;
use serde::{Deserialize, Serialize};

pub type CredentialWitness = [u8; 8];
pub type Nonce = [u8; 32];

#[derive(Hash, Serialize)]
pub struct Address<T: Hash + Default> {
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

impl<T: Hash + Default> Address<T> {
    pub fn get_nonce(self: &Self) -> Nonce {
        self.nonce
    }
    pub fn authenticate(credentials: Credentials<T>) -> Result<Self, HttpResponse> {
        let mut hasher = DefaultHasher::new();
        credentials.xpub.hash(&mut hasher);
        credentials.nonce.hash(&mut hasher);

        // Nahive authorization. TODO: implement persistent storage.
        if u64::from_be_bytes(credentials.witness) != hasher.finish() {
            Err(HttpResponse::Unauthorized().json("Unauthorized"))
        } else {
            // Default address for now. 
            // TODO: access address from persistent storage.
            // TODO: update persistent nonce for this address.
            Ok( Address {
                xpub: T::default(),
                nonce: Nonce::default(),
                xpub_list: Vec::new(),
            })
        }

    }
}