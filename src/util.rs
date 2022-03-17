use sha2::Digest;

/// Computes the SHA256 digest of a byte vector
pub fn sha256_digest(bytes: &[u8]) -> String {
    format!("sha256:{:x}", sha2::Sha256::digest(bytes))
}

/// Determines whether the given string is an OCI digest
pub fn is_digest(s: &str) -> bool {
    s.len() == 71 && s.starts_with("sha256")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_digest() {
        assert!(is_digest("sha256:e6ff4e95ec2fa250bad3f4646a53252ab039b5d1afc778209f42b30da5eaebc0"));
        assert!(!is_digest("v1.17"));
    }

    #[test]
    fn test_sha256_digest() {
        assert_eq!(sha256_digest("thisisatest\n".as_bytes()), 
            "sha256:f4bb45533d30329f994c4462bb9d3662881836931ffdf4a418a4339e5a4d57ac")
    }
}
