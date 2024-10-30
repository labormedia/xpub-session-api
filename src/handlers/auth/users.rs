pub struct Address<T> {
    xpub: T,
    nonce: [u8;32],
    xpub_list: Vec<T>
}