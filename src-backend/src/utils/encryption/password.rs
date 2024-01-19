use hex;
use ring::{pbkdf2, rand::SecureRandom};

const PBKDF2_ITERATIONS: u32 = 101_101;
const PBKDF2_KEY_LEN: usize = 32;
const PBKDF2_SALT_LEN: usize = 16;

pub struct PasswordHasher {
    rng: ring::rand::SystemRandom,
}

impl PasswordHasher {
    pub fn new() -> Self {
        Self {
            rng: ring::rand::SystemRandom::new(),
        }
    }

    pub fn hash_password(&self, password: &str) -> Result<(String, String), anyhow::Error> {
        let mut salt = [0u8; PBKDF2_SALT_LEN];
        self.rng.fill(&mut salt).unwrap();

        let mut pbkdf2_hash = [0u8; PBKDF2_KEY_LEN];
        pbkdf2::derive(
            pbkdf2::PBKDF2_HMAC_SHA256,
            std::num::NonZeroU32::new(PBKDF2_ITERATIONS).unwrap(),
            &salt,
            password.as_bytes(),
            &mut pbkdf2_hash,
        );

        let derived_key = hex::encode(pbkdf2_hash);
        let salt_hex = hex::encode(&salt);

        Ok((derived_key, salt_hex))
    }

    pub fn verify_password(&self, password: &str, salt: &str, derived_key: &str) -> bool {
        let salt = hex::decode(salt).unwrap();
        let derived_key = hex::decode(derived_key).unwrap();
        let mut pbkdf2_hash = [0u8; PBKDF2_KEY_LEN];
        pbkdf2::derive(
            pbkdf2::PBKDF2_HMAC_SHA256,
            std::num::NonZeroU32::new(PBKDF2_ITERATIONS).unwrap(),
            &salt,
            password.as_bytes(),
            &mut pbkdf2_hash,
        );
        let derived_key_vec: Vec<u8> = derived_key.into();
        pbkdf2_hash.to_vec() == derived_key_vec
    }
}
