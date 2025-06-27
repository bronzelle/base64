use crate::base64::decode_iterator::DecodeBase64IterExt;
use crate::base64::encode_iterator::EncodeBase64IterExt;

pub mod decode_iterator;
pub mod encode_iterator;
pub mod utils;

pub fn encode(buffer: &[u8]) -> Box<[u8]> {
    buffer.iter().encode_base64_iter().flatten().collect()
}

pub fn decode(buffer: &[u8]) -> Box<[u8]> {
    buffer.iter().decode_base64_iter().flatten().collect()
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
