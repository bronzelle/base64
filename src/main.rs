use base64::{decode, encode};

fn main() {
    let buffer = b"testing a long value";
    let base_64 = encode(buffer);
    let decoded = decode(base_64.as_ref());
    assert_eq!(decoded.as_ref(), buffer);
}
