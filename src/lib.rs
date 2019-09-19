#![allow(non_snake_case, non_camel_case_types, deprecated, unused_assignments)]
extern crate rand;

use xdr_codec;
use xdr_codec::Pack;

use std::convert::TryFrom;
use std::fmt;
use std::str::FromStr;

use crc16::{State, XMODEM};
use data_encoding;
extern crate byteorder;
use byteorder::ByteOrder;
use crypto::ed25519;
use sha2::{Digest, Sha256};

pub mod sep10;
mod util;

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

#[derive(Debug, PartialEq)]
pub enum NetworkType {
    PublicNetwork,
    TestNetwork,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KeyPair {
    pub pk: PublicKey,
    pub sk: SignerKey,
}

#[derive(Debug, PartialEq)]
pub enum ChallengeVerificationResult {
    ClientVerificationFailed,
    ServerVerificationFailed,
    HashFailure,
    ServerNotSource,
    SeqNumNotZero,
    NotExactlyOneOperation,
    SourceAccountInOperationMismatch,
    OperationNotManageData,
    OutsideTimeBounds,
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

impl fmt::Display for PublicKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut addr = vec![48u8];

        addr.extend_from_slice(match self {
            PublicKey::PUBLIC_KEY_TYPE_ED25519(key) => &key.0,
        });

        let crc = State::<XMODEM>::calculate(&addr);

        addr.extend_from_slice(&crc.to_le_bytes());

        write!(f, "{}", data_encoding::BASE32.encode(&addr))
    }
}

impl fmt::Display for SignerKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut addr = vec![];

        match self {
            SignerKey::SIGNER_KEY_TYPE_ED25519(key) => {
                addr.push(144u8);
                addr.extend_from_slice(&key.0)
            }
            SignerKey::SIGNER_KEY_TYPE_PRE_AUTH_TX(key) => {
                addr.push(152u8);
                addr.extend_from_slice(&key.0)
            }
            SignerKey::SIGNER_KEY_TYPE_HASH_X(key) => {
                addr.push(184u8);
                addr.extend_from_slice(&key.0)
            }
        };

        let crc = State::<XMODEM>::calculate(&addr);

        addr.extend_from_slice(&crc.to_le_bytes());

        write!(f, "{}", data_encoding::BASE32.encode(&addr))
    }
}

impl KeyPair {
    fn hint(&self) -> SignatureHint {
        let mut output = [0; 4];
        let PublicKey::PUBLIC_KEY_TYPE_ED25519(key) = self.pk;
        output.copy_from_slice(&key.0[28..32]);
        SignatureHint(output)
    }

    /*    fn join_keys(&self) -> [u8; 64] {
            let PublicKey::PUBLIC_KEY_TYPE_ED25519(pk) = self.pk;
            if let SignerKey::SIGNER_KEY_TYPE_ED25519(sk) = self.sk {
                let mut output = [0u8; 64];

                output[0..32].clone_from_slice(&pk.0);
                output[32..64].clone_from_slice(&sk.0);

                return output;
            }
            return [0u8; 64];
        }
    */
    fn join_keys(&self) -> [u8; 64] {
        if let SignerKey::SIGNER_KEY_TYPE_ED25519(sk) = self.sk {
            let (s, _) = ed25519::keypair(&sk.0);
            return s;
        }

        return [0u8; 64];
    }

    fn sign(&self, data: Hash) -> Signature {
        match self.sk {
            SignerKey::SIGNER_KEY_TYPE_ED25519(key) => {
                println!("KeyLengths: {}", &key.0.len());
                Signature(ed25519::signature(&data.0, &self.join_keys()).to_vec())
            }
            _ => Signature(vec![0; 4]),
        }
    }

    fn sign_decorated(&self, data: Hash) -> DecoratedSignature {
        let signature = self.sign(data);

        DecoratedSignature {
            hint: self.hint(),
            signature: signature,
        }
    }
}

impl NetworkType {
    pub fn get_network_id(&self) -> [u8; 32] {
        let mut hasher = Sha256::new();

        hasher.input(match self {
            NetworkType::PublicNetwork => "Public Global Stellar Network ; September 2015",
            NetworkType::TestNetwork => "Test SDF Network ; September 2015",
        });

        let mut output = [0u8; 32];
        output.copy_from_slice(&hasher.result());
        output
    }
}

impl Transaction {
    fn hash(&self, network: NetworkType) -> xdr_codec::Result<Hash> {
        let mut hasher = Sha256::new();

        hasher.input(&network.get_network_id());

        EnvelopeType::ENVELOPE_TYPE_TX.pack(&mut hasher)?;

        self.pack(&mut hasher)?;

        return Ok(Hash(hasher.result().into()));
    }
}

impl TransactionEnvelope {
    fn add_signatures(
        &mut self,
        signers: &[KeyPair],
        network: NetworkType,
    ) -> xdr_codec::Result<()> {
        let hash = self.tx.hash(network)?;

        let new_signatures: Vec<DecoratedSignature> =
            signers.iter().map(|s| s.sign_decorated(hash)).collect();
        self.signatures.extend_from_slice(&new_signatures);

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use xdr_codec::Unpack;

    use data_encoding::BASE64;
    use std::convert::TryFrom;
    use std::io::Cursor;

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
            0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23,
            24, 25, 26, 27, 28, 29, 30, 31,
        ]);

        let input: Vec<u8> = vec![
            0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23,
            24, 25, 26, 27, 28, 29, 30, 31,
        ];

        let output = uint256::try_from(&input[..]).unwrap();

        assert_eq!(expected, output);
    }

    #[test]
    fn test_secret_key_conversion() {
        let encoded = SSK;
        let decoded = uint256([
            65, 232, 215, 121, 236, 14, 103, 200, 230, 73, 60, 119, 200, 190, 166, 197, 213, 151,
            219, 254, 35, 116, 174, 104, 124, 196, 172, 8, 63, 211, 94, 122,
        ]);

        let decode: StellarKey = encoded.parse().unwrap();

        match decode {
            StellarKey::SecretKey(sk) => assert_eq!(sk, decoded),
            _ => assert!(false, "Not decoded as secret key"),
        }
    }

    #[test]
    fn test_public_key_conversion() {
        let encoded = SPK;
        let decoded = uint256([
            216, 137, 24, 69, 91, 229, 19, 62, 173, 227, 178, 45, 215, 144, 101, 154, 163, 253, 33,
            31, 108, 138, 64, 182, 177, 68, 174, 191, 48, 196, 121, 56,
        ]);

        let decode: StellarKey = encoded.parse().unwrap();

        match decode {
            StellarKey::PublicKey(pk) => assert_eq!(pk, decoded),
            _ => assert!(false, "Not decoded as public key"),
        }
    }

    #[test]
    fn test_string_to_signer_key() {
        let encoded = SSK;
        let decoded = uint256([
            65, 232, 215, 121, 236, 14, 103, 200, 230, 73, 60, 119, 200, 190, 166, 197, 213, 151,
            219, 254, 35, 116, 174, 104, 124, 196, 172, 8, 63, 211, 94, 122,
        ]);

        let sk: SignerKey = encoded.parse().unwrap();

        assert_eq!(sk, SignerKey::SIGNER_KEY_TYPE_ED25519(decoded));
    }

    #[test]
    fn test_string_to_public_key() {
        let encoded = SPK;
        let decoded = uint256([
            216, 137, 24, 69, 91, 229, 19, 62, 173, 227, 178, 45, 215, 144, 101, 154, 163, 253, 33,
            31, 108, 138, 64, 182, 177, 68, 174, 191, 48, 196, 121, 56,
        ]);

        let pk: PublicKey = encoded.parse().unwrap();

        assert_eq!(pk, PublicKey::PUBLIC_KEY_TYPE_ED25519(decoded));
    }

    #[test]
    fn test_public_key_to_string() {
        let encoded = CPK;
        let decoded = uint256([
            41, 230, 48, 57, 172, 66, 13, 134, 175, 172, 46, 207, 154, 204, 195, 100, 149, 45, 60,
            239, 178, 52, 140, 13, 190, 73, 40, 247, 163, 66, 135, 132,
        ]);

        let pk: PublicKey = PublicKey::PUBLIC_KEY_TYPE_ED25519(decoded);

        assert_eq!(pk.to_string(), encoded);
    }

    #[test]
    fn test_signer_key_to_string() {
        let encoded = CSK;
        let decoded = uint256([
            208, 48, 245, 207, 149, 194, 128, 131, 90, 7, 30, 15, 241, 249, 115, 201, 119, 247,
            208, 44, 174, 180, 154, 138, 209, 240, 105, 249, 218, 26, 51, 0,
        ]);

        let sk: SignerKey = SignerKey::SIGNER_KEY_TYPE_ED25519(decoded);

        assert_eq!(sk.to_string(), encoded);
    }
}
