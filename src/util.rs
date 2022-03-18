use sha2::Digest;

/// Computes the SHA256 digest of a byte vector
pub fn sha256_digest(bytes: &[u8]) -> String {
    format!("sha256:{:x}", sha2::Sha256::digest(bytes))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sha256_digest() {
        assert_eq!(sha256_digest("thisisatest\n".as_bytes()), 
            "sha256:f4bb45533d30329f994c4462bb9d3662881836931ffdf4a418a4339e5a4d57ac")
    }
}
