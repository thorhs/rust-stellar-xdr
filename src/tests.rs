#[cfg(test)]
use super::*;
use xdr_codec::{Pack, Unpack};

use data_encoding::BASE64;
use std::convert::TryFrom;
use std::io::Cursor;
use std::str::FromStr;
use std::time::Duration;

static SPK: &'static str = "GDMISGCFLPSRGPVN4OZC3V4QMWNKH7JBD5WIUQFWWFCK5PZQYR4TQBCD";
static SSK: &'static str = "SBA6RV3Z5QHGPSHGJE6HPSF6U3C5LF637YRXJLTIPTCKYCB72NPHVA4X";
static CPK: &'static str = "GAU6MMBZVRBA3BVPVQXM7GWMYNSJKLJ456ZDJDANXZESR55DIKDYJY2L";
static CSK: &'static str = "SDIDB5OPSXBIBA22A4PA74PZOPEXP56QFSXLJGUK2HYGT6O2DIZQB2ZH";

#[test]
fn test_parse_transaction_envelope() {
    let data = "AAAAAGL8HQvQkbK2HA3WVjRrKmjX00fG8sLI7m0ERwJW/AX3AAAACgAAAAAAAAABAAAAAAAAAAAAAAABAAAAAAAAAAAAAAAArqN6LeOagjxMaUP96Bzfs9e0corNZXzBWJkFoK7kvkwAAAAAO5rKAAAAAAAAAAABVvwF9wAAAEAKZ7IPj/46PuWU6ZOtyMosctNAkXRNX9WCAI5RnfRk+AyxDLoDZP/9l3NvsxQtWj9juQOuoBlFLnWu8intgxQA";

    let mut decoded = BASE64.decode(data.as_bytes()).unwrap();

    let te = TransactionEnvelope::unpack(&mut Cursor::new(&mut decoded))
        .unwrap()
        .0;

    assert_eq!(te.tx.fee, 10);
    assert_eq!(te.tx.seqNum, 1);
    match te.tx.operations[0].body {
        OperationBody::CREATE_ACCOUNT(ca) => assert_eq!(ca.startingBalance, 1000000000),
        _ => assert!(false, "Not a CreateAccountOp"),
    }
}

#[test]
fn test_convert_vec_to_uint256() {
    let expected: uint256 = uint256([
        0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
        25, 26, 27, 28, 29, 30, 31,
    ]);

    let input: Vec<u8> = vec![
        0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
        25, 26, 27, 28, 29, 30, 31,
    ];

    let output = uint256::try_from(&input[..]).unwrap();

    assert_eq!(expected, output);
}

#[test]
fn test_secret_key_conversion() {
    let encoded = "SBXJINOE4VUEH7QF7IIDDTPLBGBKZCQN6J4YMUM4JPSG7QW55NQRBIWX";
    let decoded = uint256([
        0x6e, 0x94, 0x35, 0xc4, 0xe5, 0x68, 0x43, 0xfe, 0x05, 0xfa, 0x10, 0x31, 0xcd, 0xeb, 0x09,
        0x82, 0xac, 0x8a, 0x0d, 0xf2, 0x79, 0x86, 0x51, 0x9c, 0x4b, 0xe4, 0x6f, 0xc2, 0xdd, 0xeb,
        0x61, 0x10,
    ]);

    let decode: StellarKey = encoded.parse().unwrap();

    match decode {
        StellarKey::SecretKey(sk) => assert_eq!(sk, decoded),
        _ => assert!(false, "Not decoded as secret key"),
    }
}

#[test]
fn test_public_key_conversion() {
    let encoded = "GBWCUJFSFMQSTXBILBKLLB4FJNZLTGLADUP3JGSZDXF2Q3MSIKCTID7R";
    let decoded = uint256([
        0x6c, 0x2a, 0x24, 0xb2, 0x2b, 0x21, 0x29, 0xdc, 0x28, 0x58, 0x54, 0xb5, 0x87, 0x85, 0x4b,
        0x72, 0xb9, 0x99, 0x60, 0x1d, 0x1f, 0xb4, 0x9a, 0x59, 0x1d, 0xcb, 0xa8, 0x6d, 0x92, 0x42,
        0x85, 0x34,
    ]);

    let decode: StellarKey = encoded.parse().unwrap();

    match decode {
        StellarKey::PublicKey(pk) => assert_eq!(pk, decoded),
        _ => assert!(false, "Not decoded as public key"),
    }
}

#[test]
fn test_string_to_signer_key() {
    let encoded = "SBXJINOE4VUEH7QF7IIDDTPLBGBKZCQN6J4YMUM4JPSG7QW55NQRBIWX";
    let decoded = uint256([
        0x6e, 0x94, 0x35, 0xc4, 0xe5, 0x68, 0x43, 0xfe, 0x05, 0xfa, 0x10, 0x31, 0xcd, 0xeb, 0x09,
        0x82, 0xac, 0x8a, 0x0d, 0xf2, 0x79, 0x86, 0x51, 0x9c, 0x4b, 0xe4, 0x6f, 0xc2, 0xdd, 0xeb,
        0x61, 0x10,
    ]);

    let sk: SignerKey = encoded.parse().unwrap();

    assert_eq!(sk, SignerKey::SIGNER_KEY_TYPE_ED25519(decoded));
}

#[test]
fn test_string_to_public_key() {
    let encoded = "GBWCUJFSFMQSTXBILBKLLB4FJNZLTGLADUP3JGSZDXF2Q3MSIKCTID7R";
    let decoded = uint256([
        0x6c, 0x2a, 0x24, 0xb2, 0x2b, 0x21, 0x29, 0xdc, 0x28, 0x58, 0x54, 0xb5, 0x87, 0x85, 0x4b,
        0x72, 0xb9, 0x99, 0x60, 0x1d, 0x1f, 0xb4, 0x9a, 0x59, 0x1d, 0xcb, 0xa8, 0x6d, 0x92, 0x42,
        0x85, 0x34,
    ]);

    let pk: PublicKey = encoded.parse().unwrap();

    assert_eq!(pk, PublicKey::PUBLIC_KEY_TYPE_ED25519(decoded));
}

#[test]
fn test_public_key_to_string() {
    let encoded = "GBWCUJFSFMQSTXBILBKLLB4FJNZLTGLADUP3JGSZDXF2Q3MSIKCTID7R";
    let decoded = uint256([
        0x6c, 0x2a, 0x24, 0xb2, 0x2b, 0x21, 0x29, 0xdc, 0x28, 0x58, 0x54, 0xb5, 0x87, 0x85, 0x4b,
        0x72, 0xb9, 0x99, 0x60, 0x1d, 0x1f, 0xb4, 0x9a, 0x59, 0x1d, 0xcb, 0xa8, 0x6d, 0x92, 0x42,
        0x85, 0x34,
    ]);

    let pk: PublicKey = PublicKey::PUBLIC_KEY_TYPE_ED25519(decoded);

    assert_eq!(pk.to_string(), encoded);
}

#[test]
fn test_signer_key_to_string() {
    let encoded = "SBXJINOE4VUEH7QF7IIDDTPLBGBKZCQN6J4YMUM4JPSG7QW55NQRBIWX";
    let decoded = uint256([
        0x6e, 0x94, 0x35, 0xc4, 0xe5, 0x68, 0x43, 0xfe, 0x05, 0xfa, 0x10, 0x31, 0xcd, 0xeb, 0x09,
        0x82, 0xac, 0x8a, 0x0d, 0xf2, 0x79, 0x86, 0x51, 0x9c, 0x4b, 0xe4, 0x6f, 0xc2, 0xdd, 0xeb,
        0x61, 0x10,
    ]);

    let sk: SignerKey = SignerKey::SIGNER_KEY_TYPE_ED25519(decoded);

    assert_eq!(sk.to_string(), encoded);
}

#[test]
fn test_generate_challenge() {
    let pk = SPK;
    let sk = SSK;
    let ck = CPK;

    let skp = KeyPair {
        sk: SignerKey::from_str(sk).unwrap(),
        pk: PublicKey::from_str(pk).unwrap(),
    };
    let cpk: PublicKey = PublicKey::from_str(ck).unwrap();

    let ac = generate_auth_challenge(
        &skp,
        &cpk,
        "https://isk.toti.is",
        Duration::new(300, 0),
        NetworkType::TestNetwork,
    );

    let mut buf = Vec::new();
    ac.pack(&mut buf).unwrap();
    println!("Auth challenge: {}", BASE64.encode(&buf));
}
