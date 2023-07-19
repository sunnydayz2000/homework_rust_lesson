use sha2::{Sha256, Sha512, Digest};

pub fn sha256(input: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(input);
    hasher.finalize()
        .iter()
        .map(|b| format!("{:02x}", b))
        .collect::<String>()

}
