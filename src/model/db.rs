use super::*;

// model::db::lookup(client, credentials).await

pub async fn insert_address_from_credentials(collection: Collection<Address<XpubWrapper>>, credentials: Credentials<XpubWrapper>) -> Result<Address<XpubWrapper>, HttpResponse> {
    let address = if Address::authenticate(credentials.clone()).await? {
        Address::from_credentials(credentials.clone())
    } else {
        return Err(HttpResponse::Unauthorized().json("Unauthorized"))
    };
    match collection.insert_one(address.clone()).await {
        Ok(_) => Ok(address),
        Err(err) => Err(HttpResponse::InternalServerError().body(err.to_string())),
    }
}

pub async fn address_lookup(client: web::Data<Client>, credentials: Credentials<XpubWrapper>) -> Result<Address<XpubWrapper>, HttpResponse> {
    let credential_xpub: bip32::Xpub = credentials.xpub.clone().to_xpub();
    let collection: Collection<Address<XpubWrapper>> = client.database(DB_NAME).collection(COLL_NAME);
            match collection.find_one(doc! {"xpub": &credentials.xpub}).await {
                Ok(Some(address)) => {
                    let address_xpub: bip32::Xpub = address.xpub.clone().to_xpub();
                    let mut address_message = address_xpub.to_string().to_owned();
                    address_message.push_str(&address.nonce.clone().to_str());
                    let message_hash = signed_msg_hash(&address_message);
                    Ok(address)
                },
                Ok(None) => Err(HttpResponse::NotFound().json("NotFound")),
                Err(err) => Err(HttpResponse::InternalServerError().body(err.to_string())),
            }
}

pub async fn update_address_nonce(collection: Collection<Address<XpubWrapper>>, address: Address<XpubWrapper>) -> Result<Address<XpubWrapper>, HttpResponse>  {
    // Update nonce on persistent db
    let updated_address = address.clone().update_nonce(); // TODO: Unleash the nonce updating procedure.
    match collection.insert_one(updated_address.clone()).await {
        Ok(_) => Ok(updated_address),
        Err(err) => Err(HttpResponse::InternalServerError().body(err.to_string())),
    }
}