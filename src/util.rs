use data_encoding::BASE64;
use rand::{thread_rng, Rng};

pub(crate) fn generate_base64_nonce() -> String {
    let mut nonce = [0u8; 48];
    thread_rng().fill(&mut nonce[..]);

    BASE64.encode(&nonce)
}
