use rust_stellar_xdr;

use std::io::Curosr;

#[test]
fn test_create_account_from_xdr() {
    let txe_b64 = "AAAAAMOrP0B2tL9IUn5QL8nn8q88kkFui1x3oW9omCj6hLhfAAAAZAAAAMcAAAAWAAAAAQAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAQAAAAEAAAAAEH3Rayw4M0iCLoEe96rPFNGYim8AVHJU0z4ebYZW4JwAAAAAAAAAAJ5yfHhgKAxylgecjAymWqNzLWRk";

    let decoded = base64::decode(txeB64);

    let ca: rust_stellar_xdr::CreateAccountOp.unpack(Cursor::new(decoded));
}
