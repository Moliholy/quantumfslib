use base58;
use base58::ToBase58;
use ipfsapi::IpfsApi;
use multihash;
use regex::Regex;

use crate::errors::QFSError;
use crate::types::ipfs::IpfsHash;
use std::fs::File;

static IPFS_HASH_PATTERN: &str = "^[a-zA-z0-9]{46}$";
static IPFS_DEFAULT_URL: &str = "127.0.0.1";
static IPFS_DEFAULT_PORT: u16 = 5001;

fn api() -> IpfsApi {
    IpfsApi::new(IPFS_DEFAULT_URL, IPFS_DEFAULT_PORT)
}

pub fn validate_ipfs_hash(hash: &str) -> bool {
    Regex::new(IPFS_HASH_PATTERN).unwrap().is_match(hash)
}

pub fn hash_bytes(bytes: &[u8]) -> String {
    multihash::encode(multihash::Hash::SHA2256, bytes)
        .unwrap()
        .as_slice()
        .to_base58()
}

pub fn stream(ipfs_hash: &IpfsHash) -> Result<impl Iterator<Item=u8>, QFSError> {
    api()
        .block_get(ipfs_hash.to_string().as_str())
        .map_err(QFSError::from)
}

pub fn fetch(ipfs_hash: &IpfsHash) -> Result<Vec<u8>, QFSError> {
    let bytes = stream(ipfs_hash)?.collect();
    Ok(bytes)
}

pub fn add(file: &File) -> Result<IpfsHash, QFSError> {
    api()
        .block_put(file.try_clone()?)
        .map_err(QFSError::from)
        .map(|hash| IpfsHash::new(hash.as_str()).unwrap())
}


#[cfg(test)]
mod tests {
    use crate::operations::ipfs::{fetch, hash_bytes, IpfsHash, validate_ipfs_hash};

    #[test]
    fn validate_ipfs_hash_with_valid_hash_should_work() {
        let hash = "QmaozNR7DZHQK1ZcU9p7QdrshMvXqWK6gpu5rmrkPdT3L4";
        let result = validate_ipfs_hash(hash);
        assert!(result);
    }

    #[test]
    fn test_fetch_with_valid_hash_should_work() {
        let hash = IpfsHash::new("QmWE6s8qazNrzGEHLfVA5PAFieT1nsoqU11pggfoWwSis5").unwrap();
        let result = fetch(&hash);
        assert!(result.is_ok());
        let content = String::from_utf8(result.unwrap()).unwrap();
        assert_eq!(content.as_str(), "Hello from IPFS Gateway Checker\n");
    }

    #[test]
    fn test_ipfs_hashing_should_work() {
        let result = hash_bytes(b"hello world");
        assert_eq!(result, "QmaozNR7DZHQK1ZcU9p7QdrshMvXqWK6gpu5rmrkPdT3L4");
    }
}