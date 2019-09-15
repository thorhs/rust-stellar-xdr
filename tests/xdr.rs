use rust_stellar_xdr as xdr;
use xdr_codec::Unpack;

use data_encoding::BASE64;
use std::convert::TryFrom;
use std::io::Cursor;

#[test]
fn test_parse_transaction_envelope() {
    let data = "AAAAAGL8HQvQkbK2HA3WVjRrKmjX00fG8sLI7m0ERwJW/AX3AAAACgAAAAAAAAABAAAAAAAAAAAAAAABAAAAAAAAAAAAAAAArqN6LeOagjxMaUP96Bzfs9e0corNZXzBWJkFoK7kvkwAAAAAO5rKAAAAAAAAAAABVvwF9wAAAEAKZ7IPj/46PuWU6ZOtyMosctNAkXRNX9WCAI5RnfRk+AyxDLoDZP/9l3NvsxQtWj9juQOuoBlFLnWu8intgxQA";

    let mut decoded = BASE64.decode(data.as_bytes()).unwrap();

    let te = xdr::TransactionEnvelope::unpack(&mut Cursor::new(&mut decoded))
        .unwrap()
        .0;

    assert_eq!(te.tx.fee, 10);
    assert_eq!(te.tx.seqNum, 1);
    match te.tx.operations[0].body {
        xdr::OperationBody::CREATE_ACCOUNT(ca) => assert_eq!(ca.startingBalance, 1000000000),
        _ => assert!(false, "Not a CreateAccountOp"),
    }
}

#[test]
fn test_convert_vec_to_uint256() {
    let expected: xdr::uint256 = xdr::uint256([
        0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
        25, 26, 27, 28, 29, 30, 31,
    ]);

    let input: Vec<u8> = vec![
        0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
        25, 26, 27, 28, 29, 30, 31,
    ];

    let output = xdr::uint256::try_from(&input[..]).unwrap();

    assert_eq!(expected, output);
}

#[test]
fn test_secret_key_conversion() {
    let encoded = "SBXJINOE4VUEH7QF7IIDDTPLBGBKZCQN6J4YMUM4JPSG7QW55NQRBIWX";
    let decoded = xdr::uint256([
        0x6e, 0x94, 0x35, 0xc4, 0xe5, 0x68, 0x43, 0xfe, 0x05, 0xfa, 0x10, 0x31, 0xcd, 0xeb, 0x09,
        0x82, 0xac, 0x8a, 0x0d, 0xf2, 0x79, 0x86, 0x51, 0x9c, 0x4b, 0xe4, 0x6f, 0xc2, 0xdd, 0xeb,
        0x61, 0x10,
    ]);

    let decode: xdr::StellarKey = encoded.parse().unwrap();

    match decode {
        xdr::StellarKey::SecretKey(sk) => assert_eq!(sk, decoded),
        _ => assert!(false, "Not decoded as secret key"),
    }
}

#[test]
fn test_public_key_conversion() {
    let encoded = "GBWCUJFSFMQSTXBILBKLLB4FJNZLTGLADUP3JGSZDXF2Q3MSIKCTID7R";
    let decoded = xdr::uint256([
        0x6c, 0x2a, 0x24, 0xb2, 0x2b, 0x21, 0x29, 0xdc, 0x28, 0x58, 0x54, 0xb5, 0x87, 0x85, 0x4b,
        0x72, 0xb9, 0x99, 0x60, 0x1d, 0x1f, 0xb4, 0x9a, 0x59, 0x1d, 0xcb, 0xa8, 0x6d, 0x92, 0x42,
        0x85, 0x34,
    ]);

    let decode: xdr::StellarKey = encoded.parse().unwrap();

    match decode {
        xdr::StellarKey::PublicKey(pk) => assert_eq!(pk, decoded),
        _ => assert!(false, "Not decoded as public key"),
    }
}

#[test]
fn test_string_to_signer_key() {
    let encoded = "SBXJINOE4VUEH7QF7IIDDTPLBGBKZCQN6J4YMUM4JPSG7QW55NQRBIWX";
    let decoded = xdr::uint256([
        0x6e, 0x94, 0x35, 0xc4, 0xe5, 0x68, 0x43, 0xfe, 0x05, 0xfa, 0x10, 0x31, 0xcd, 0xeb, 0x09,
        0x82, 0xac, 0x8a, 0x0d, 0xf2, 0x79, 0x86, 0x51, 0x9c, 0x4b, 0xe4, 0x6f, 0xc2, 0xdd, 0xeb,
        0x61, 0x10,
    ]);

    let sk: xdr::SignerKey = encoded.parse().unwrap();

    assert_eq!(
        sk,
        rust_stellar_xdr::SignerKey::SIGNER_KEY_TYPE_ED25519(decoded)
    );
}

#[test]
fn test_string_to_public_key() {
    let encoded = "GBWCUJFSFMQSTXBILBKLLB4FJNZLTGLADUP3JGSZDXF2Q3MSIKCTID7R";
    let decoded = xdr::uint256([
        0x6c, 0x2a, 0x24, 0xb2, 0x2b, 0x21, 0x29, 0xdc, 0x28, 0x58, 0x54, 0xb5, 0x87, 0x85, 0x4b,
        0x72, 0xb9, 0x99, 0x60, 0x1d, 0x1f, 0xb4, 0x9a, 0x59, 0x1d, 0xcb, 0xa8, 0x6d, 0x92, 0x42,
        0x85, 0x34,
    ]);

    let pk: xdr::PublicKey = encoded.parse().unwrap();

    assert_eq!(
        pk,
        rust_stellar_xdr::PublicKey::PUBLIC_KEY_TYPE_ED25519(decoded)
    );
}
