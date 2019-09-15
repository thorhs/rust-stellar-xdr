#![allow(non_snake_case, non_camel_case_types, deprecated, unused_assignments)]
use xdr_codec;

use std::convert::TryFrom;
use std::str::FromStr;

use crc16::{State, XMODEM};
use data_encoding;
extern crate byteorder;
use byteorder::ByteOrder;

#[allow(dead_code)]
include!(concat!(env!("OUT_DIR"), "/Stellar.rs"));

impl TryFrom<&[u8]> for uint256 {
    type Error = ();

    fn try_from(src: &[u8]) -> Result<Self, ()> {
        if src.len() != 32 {
            return Err(());
        }

        let mut output = [0u8; 32];
        for (i, val) in src.iter().enumerate() {
            output[i] = *val;
        }

        Ok(uint256(output))
    }
}

#[derive(Debug, PartialEq)]
pub enum StellarKey {
    PublicKey(uint256),
    SecretKey(uint256),
    PreAuthTx(uint256),
    Sha256Hash(uint256),
}

#[derive(Debug, PartialEq)]
pub enum StellarKeyParseError {
    Base64DecodeError,
    ChecksumError,
    KeyLengthError,
    UnknownKeyTypeError,
}

impl FromStr for StellarKey {
    type Err = StellarKeyParseError;

    fn from_str(s: &str) -> Result<StellarKey, Self::Err> {
        let decoded = match data_encoding::BASE32.decode(s.as_bytes()) {
            Ok(d) => d,
            Err(_) => return Err(StellarKeyParseError::Base64DecodeError),
        };

        let decoded_length = decoded.len();
        let version_byte = &decoded[0];
        let payload = &decoded[0..decoded_length - 2];
        let data = &payload[1..];
        let checksum = byteorder::LittleEndian::read_u16(&decoded[decoded_length - 2..]);

        let calculated_checksum = State::<XMODEM>::calculate(payload);

        if checksum != calculated_checksum {
            return Err(StellarKeyParseError::ChecksumError);
        }

        let key = match uint256::try_from(data) {
            Ok(k) => k,
            Err(_) => return Err(StellarKeyParseError::KeyLengthError),
        };

        match version_byte {
            48u8 => Ok(StellarKey::PublicKey(key)),
            144u8 => Ok(StellarKey::SecretKey(key)),
            152u8 => Ok(StellarKey::PreAuthTx(key)),
            184u8 => Ok(StellarKey::Sha256Hash(key)),
            _ => Err(StellarKeyParseError::UnknownKeyTypeError),
        }
    }
}

impl FromStr for PublicKey {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, ()> {
        match StellarKey::from_str(s) {
            Ok(StellarKey::PublicKey(key)) => Ok(PublicKey::PUBLIC_KEY_TYPE_ED25519(key)),
            _ => Err(()),
        }
    }
}

impl FromStr for SignerKey {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, ()> {
        match StellarKey::from_str(s) {
            Ok(StellarKey::SecretKey(key)) => Ok(SignerKey::SIGNER_KEY_TYPE_ED25519(key)),
            _ => Err(()),
        }
    }
}
