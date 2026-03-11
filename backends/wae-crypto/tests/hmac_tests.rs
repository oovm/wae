use wae_crypto::{hmac_sign, hmac_verify, HmacAlgorithm};
use hex::ToHex;

#[test]
fn test_hmac_sha1() {
    let secret = b"test_secret";
    let data = b"hello world";
    let signature = hmac_sign(HmacAlgorithm::SHA1, secret, data).unwrap();
    let hex = signature.encode_hex::<String>();
    
    let expected = "585a25d1990961c62c8d542e831d510d88d6b6fa";
    assert_eq!(hex, expected);
}

#[test]
fn test_hmac_sha256() {
    let secret = b"test_secret";
    let data = b"hello world";
    let signature = hmac_sign(HmacAlgorithm::SHA256, secret, data).unwrap();
    let hex = signature.encode_hex::<String>();
    
    let expected = "009d677c1a196a2832e7f3366192e38c56f5684c9a4511ee1136f9a6390743e";
    assert_eq!(hex, expected);
}

#[test]
fn test_hmac_sha384() {
    let secret = b"test_secret";
    let data = b"hello world";
    let signature = hmac_sign(HmacAlgorithm::SHA384, secret, data).unwrap();
    let hex = signature.encode_hex::<String>();
    
    assert_eq!(hex.len(), 96);
}

#[test]
fn test_hmac_sha512() {
    let secret = b"test_secret";
    let data = b"hello world";
    let signature = hmac_sign(HmacAlgorithm::SHA512, secret, data).unwrap();
    let hex = signature.encode_hex::<String>();
    
    assert_eq!(hex.len(), 128);
}

#[test]
fn test_hmac_verify_correct() {
    let secret = b"test_secret";
    let data = b"hello world";
    
    let algorithms = vec![
        HmacAlgorithm::SHA1,
        HmacAlgorithm::SHA256,
        HmacAlgorithm::SHA384,
        HmacAlgorithm::SHA512,
    ];
    
    for alg in algorithms {
        let signature = hmac_sign(alg, secret, data).unwrap();
        let result = hmac_verify(alg, secret, data, &signature).unwrap();
        assert!(result);
    }
}

#[test]
fn test_hmac_verify_incorrect_signature() {
    let secret = b"test_secret";
    let data = b"hello world";
    let wrong_data = b"wrong data";
    
    let signature = hmac_sign(HmacAlgorithm::SHA256, secret, data).unwrap();
    let result = hmac_verify(HmacAlgorithm::SHA256, secret, wrong_data, &signature);
    assert!(result.is_err());
}

#[test]
fn test_hmac_verify_incorrect_secret() {
    let secret = b"test_secret";
    let wrong_secret = b"wrong_secret";
    let data = b"hello world";
    
    let signature = hmac_sign(HmacAlgorithm::SHA256, secret, data).unwrap();
    let result = hmac_verify(HmacAlgorithm::SHA256, wrong_secret, data, &signature);
    assert!(result.is_err());
}

#[test]
fn test_hmac_empty_input() {
    let secret = b"test_secret";
    let data = b"";
    
    let signature = hmac_sign(HmacAlgorithm::SHA256, secret, data).unwrap();
    let result = hmac_verify(HmacAlgorithm::SHA256, secret, data, &signature).unwrap();
    assert!(result);
}

#[test]
fn test_hmac_empty_secret() {
    let secret = b"";
    let data = b"hello world";
    
    let signature = hmac_sign(HmacAlgorithm::SHA256, secret, data).unwrap();
    let result = hmac_verify(HmacAlgorithm::SHA256, secret, data, &signature).unwrap();
    assert!(result);
}

#[test]
fn test_hmac_algorithm_clone_copy() {
    let alg = HmacAlgorithm::SHA256;
    let alg2 = alg;
    assert_eq!(alg, alg2);
    
    let alg3 = alg.clone();
    assert_eq!(alg, alg3);
}

#[test]
fn test_hmac_algorithm_debug() {
    let alg = HmacAlgorithm::SHA1;
    let debug_str = format!("{:?}", alg);
    assert!(debug_str.contains("SHA1"));
}
