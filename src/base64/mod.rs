use crate::base64::utils::{decode_chunk, encode_chunk};

pub mod encode_iterator;
pub mod utils;

pub fn encode(buffer: &[u8]) -> Box<[u8]> {
    let mut b64 = Vec::new();
    for group in buffer.chunks(3) {
        b64.extend_from_slice(&encode_chunk(group));
    }
    b64.into_boxed_slice()
}

pub fn decode(buffer: &[u8]) -> Box<[u8]> {
    let mut decoded = Vec::new();
    for chunk in buffer.chunks(4) {
        if let Some([b1, b2, b3]) = decode_chunk(chunk) {
            decoded.push(b1.unwrap());
            if let Some(b2) = b2 {
                decoded.push(b2);
            }
            if let Some(b3) = b3 {
                decoded.push(b3);
            }
        }
    }
    decoded.into_boxed_slice()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode() {
        let base_64 = encode(b"testing a long value");
        assert_eq!(base_64.as_ref(), b"dGVzdGluZyBhIGxvbmcgdmFsdWU=");
    }

    #[test]
    fn test_decode() {
        let base_64 = decode(b"dGVzdGluZyBhIGxvbmcgdmFsdWU=");
        assert_eq!(base_64.as_ref(), b"testing a long value");
    }
}
