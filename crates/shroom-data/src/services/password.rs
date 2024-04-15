use argon2::Argon2;
use password_hash::{
    rand_core::OsRng, PasswordHash, PasswordHashString, PasswordHasher, PasswordVerifier,
    SaltString,
};

#[derive(Debug, Default)]
pub struct PwService {
    hasher: Argon2<'static>,
}

impl PwService {
    pub fn generate_hash(&self, password: &str) -> PasswordHashString {
        let salt = SaltString::generate(&mut OsRng);
        self.hasher
            .hash_password(password.as_bytes(), &salt)
            .unwrap()
            .serialize()
    }

    pub fn verify_password(&self, password: &str, hash: &str) -> bool {
        let hash = PasswordHash::new(hash).expect("Password hash is invalid");
        self.hasher
            .verify_password(password.as_bytes(), &hash)
            .is_ok()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hash_verify() {
        let pw = PwService::default();

        let hash = pw.generate_hash("test");
        assert!(pw.verify_password("test", hash.as_str()));
        assert!(!pw.verify_password("test1", hash.as_str()));
    }
}
