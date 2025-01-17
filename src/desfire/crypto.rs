extern crate rand;

use rand::rngs::OsRng;
use rand::RngCore;

fn generate_new_aes_key() -> Vec<u8> {
    let mut r = OsRng::new().unwrap();

    // Random bytes.
    let mut key = vec![0u8; 16];
    r.fill_bytes(&mut key);

    return key;
}
