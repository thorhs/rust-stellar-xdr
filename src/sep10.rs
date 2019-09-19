use super::*;

use crate::util::generate_base64_nonce;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

pub fn generate_auth_challenge(
    server_keypair: &KeyPair,
    client_key: &PublicKey,
    anchor_name: &str,
    challenge_lifetime: Duration,
    network: NetworkType,
) -> TransactionEnvelope {
    let nonce_str = generate_base64_nonce();
    let sa = server_keypair.pk;
    let ca = client_key;

    let tb = TimeBounds {
        minTime: SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs(),
        maxTime: (SystemTime::now() + challenge_lifetime)
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs(),
    };

    let op = ManageDataOp {
        dataName: string64(format!("{}_auth", anchor_name)),
        dataValue: Some(DataValue(nonce_str.as_bytes().to_vec())),
    };
    let operations = vec![Operation {
        sourceAccount: Some(Box::new(*ca)),
        body: OperationBody::MANAGE_DATA(op),
    }];

    let tx = Transaction {
        sourceAccount: sa,
        fee: 0,
        seqNum: 0,
        timeBounds: Some(Box::new(tb)),
        memo: crate::Memo::MEMO_NONE,
        operations: operations,
        ext: crate::TransactionExt::Const0,
    };

    let mut te = TransactionEnvelope {
        tx: tx,
        signatures: Vec::new(),
    };

    te.add_signatures(&vec![*server_keypair][..], network)
        .unwrap();

    te
}

pub fn verify_challenge(
    challenge_envelope: TransactionEnvelope,
    client_key: PublicKey,
    server_key: PublicKey,
    network: NetworkType,
) -> Result<(), ChallengeVerificationResult> {
    let hash = match challenge_envelope.tx.hash(network) {
        Ok(hash) => hash,
        Err(_) => return Err(ChallengeVerificationResult::HashFailure),
    };

    let PublicKey::PUBLIC_KEY_TYPE_ED25519(ck) = client_key;
    let PublicKey::PUBLIC_KEY_TYPE_ED25519(sk) = server_key;

    if !challenge_envelope
        .signatures
        .iter()
        .find(|s| s.hint.0 == ck.0[28..32])
        .map_or(false, |s| ed25519::verify(&hash.0, &ck.0, &s.signature.0))
    {
        return Err(ChallengeVerificationResult::ClientVerificationFailed);
    }

    if !challenge_envelope
        .signatures
        .iter()
        .find(|s| s.hint.0 == sk.0[28..32])
        .map_or(false, |s| ed25519::verify(&hash.0, &sk.0, &s.signature.0))
    {
        return Err(ChallengeVerificationResult::ServerVerificationFailed);
    }

    if challenge_envelope.tx.sourceAccount != server_key {
        return Err(ChallengeVerificationResult::ServerNotSource);
    }

    if challenge_envelope.tx.seqNum != 0 {
        return Err(ChallengeVerificationResult::SeqNumNotZero);
    }

    if challenge_envelope.tx.operations.len() != 1 {
        return Err(ChallengeVerificationResult::NotExactlyOneOperation);
    }

    let op = &challenge_envelope.tx.operations[0];

    if let Some(sa) = &op.sourceAccount {
        let s: PublicKey = **sa;
        if s != PublicKey::PUBLIC_KEY_TYPE_ED25519(ck) {
            return Err(ChallengeVerificationResult::SourceAccountInOperationMismatch);
        }
    }

    match op.body {
        OperationBody::MANAGE_DATA(_) => {}
        _ => return Err(ChallengeVerificationResult::OperationNotManageData),
    }

    if let Some(tb) = challenge_envelope.tx.timeBounds {
        let minTime = SystemTime::UNIX_EPOCH + Duration::new(tb.minTime, 0);
        let maxTime = SystemTime::UNIX_EPOCH + Duration::new(tb.maxTime, 0);
        let now = SystemTime::now();

        println!("Min: {:?}, Now: {:?}, Max: {:?}", minTime, now, maxTime);

        if now < minTime || now > maxTime {
            return Err(ChallengeVerificationResult::OutsideTimeBounds);
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use xdr_codec::{Pack, Unpack};

    use data_encoding::BASE64;
    use std::str::FromStr;
    use std::time::Duration;

    static SPK: &'static str = "GDMISGCFLPSRGPVN4OZC3V4QMWNKH7JBD5WIUQFWWFCK5PZQYR4TQBCD";
    static SSK: &'static str = "SBA6RV3Z5QHGPSHGJE6HPSF6U3C5LF637YRXJLTIPTCKYCB72NPHVA4X";
    static CPK: &'static str = "GAU6MMBZVRBA3BVPVQXM7GWMYNSJKLJ456ZDJDANXZESR55DIKDYJY2L";
    static CSK: &'static str = "SDIDB5OPSXBIBA22A4PA74PZOPEXP56QFSXLJGUK2HYGT6O2DIZQB2ZH";

    fn make_auth_challenge(pk: &str, sk: &str, ck: &str, anchor: &str) -> TransactionEnvelope {
        let skp = KeyPair {
            sk: SignerKey::from_str(sk).unwrap(),
            pk: PublicKey::from_str(pk).unwrap(),
        };
        let cpk: PublicKey = PublicKey::from_str(ck).unwrap();

        generate_auth_challenge(
            &skp,
            &cpk,
            anchor,
            Duration::new(300, 0),
            NetworkType::TestNetwork,
        )
    }

    #[test]
    fn test_generate_challenge() {
        let ac = make_auth_challenge(SPK, SSK, CPK, "https://isk.toti.is");

        let mut buf = Vec::new();
        ac.pack(&mut buf).unwrap();
        println!("Auth challenge: {}", BASE64.encode(&buf));
    }

    #[test]
    fn test_sign_challenge() {
        let challenge = b"AAAAANiJGEVb5RM+reOyLdeQZZqj/SEfbIpAtrFErr8wxHk4AAAAAAAAAAAAAAAAAAAAAQAAAABdg5T8AAAAAF2DligAAAAAAAAAAQAAAAEAAAAAKeYwOaxCDYavrC7PmszDZJUtPO+yNIwNvkko96NCh4QAAAAKAAAAGGh0dHBzOi8vaXNrLnRvdGkuaXNfYXV0aAAAAAEAAABAM0pacTRzOWc3Z0QrUVd2dFRCUURDQVlpaVdZQy9QYW1abG5WTWlZRkkwUlZ6YkUxVTF0eEN0UzRQZE44QWNCdQAAAAAAAAABMMR5OAAAAECY/WNZgnc70WPQ1frdP9K9D3NvW7mwweqjX1+KTHcsD8POd6CrYMMQybOBcGzOV/1F9jmYJjs4iom942zMnYYK";
        let csk = CSK;
        let cpk = CPK;

        let buf = BASE64.decode(challenge).unwrap();
        let mut challenge_envelope = TransactionEnvelope::unpack(&mut &buf[..]).unwrap().0;

        let ckp = KeyPair {
            sk: SignerKey::from_str(csk).unwrap(),
            pk: PublicKey::from_str(cpk).unwrap(),
        };

        challenge_envelope
            .add_signatures(&[ckp], NetworkType::TestNetwork)
            .unwrap();

        let mut buf = Vec::new();
        challenge_envelope.pack(&mut buf).unwrap();

        println!("Signed Challenge: {}", BASE64.encode(&buf));
    }

    #[test]
    fn test_verify_challenge() {
        let ckp = KeyPair {
            sk: SignerKey::from_str(CSK).unwrap(),
            pk: PublicKey::from_str(CPK).unwrap(),
        };

        let mut ac = make_auth_challenge(SPK, SSK, CPK, "https://isk.toti.is");
        ac.add_signatures(&[ckp], NetworkType::TestNetwork).unwrap();

        let ck = PublicKey::from_str(CPK).unwrap();
        let sk = PublicKey::from_str(SPK).unwrap();

        match verify_challenge(ac, ck, sk, NetworkType::TestNetwork) {
            Err(err) => assert!(false, "Error validating challenge: {:?}", err),
            _ => (),
        }
    }
}
