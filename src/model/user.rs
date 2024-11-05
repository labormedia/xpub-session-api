use bitcoin::{
    Address,
    Psbt,
};
use crate::model;

pub struct User {
    _id: u32,
    address: model::UserAddress<model::XpubWrapper>,
    psbt_list: Vec<Psbt>,
    salted_fingerprint: model::SaltedFingerPrint,
}