use rust_stellar_xdr as xdr;
use xdr_codec::Unpack;

use base64;
use std::io::Cursor;

#[test]
fn test_parse_transaction_envelope() {
    let data = "AAAAAGL8HQvQkbK2HA3WVjRrKmjX00fG8sLI7m0ERwJW/AX3AAAACgAAAAAAAAABAAAAAAAAAAAAAAABAAAAAAAAAAAAAAAArqN6LeOagjxMaUP96Bzfs9e0corNZXzBWJkFoK7kvkwAAAAAO5rKAAAAAAAAAAABVvwF9wAAAEAKZ7IPj/46PuWU6ZOtyMosctNAkXRNX9WCAI5RnfRk+AyxDLoDZP/9l3NvsxQtWj9juQOuoBlFLnWu8intgxQA";

    let mut decoded = base64::decode(data).unwrap();

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
