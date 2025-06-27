use crate::base64::utils::encode_chunk;

struct EncodeBase64Iter<I: Iterator<Item = u8>> {
    iterator: I,
}

impl<I: Iterator<Item = u8>> EncodeBase64Iter<I> {
    pub fn new(iterator: I) -> Self {
        Self { iterator }
    }
}

trait EncodeBase64IterExt: Iterator<Item = u8> {
    fn encode_base64_iter(self) -> EncodeBase64Iter<Self>
    where
        Self: Sized,
    {
        EncodeBase64Iter::new(self)
    }
}

impl<I: Iterator<Item = u8>> EncodeBase64IterExt for I {}

impl<I: Iterator<Item = u8>> Iterator for EncodeBase64Iter<I> {
    type Item = [u8; 4];

    fn next(&mut self) -> Option<Self::Item> {
        let mut buffer = [0u8; 3];
        let mut len = 0;
        loop {
            match self.iterator.next() {
                None => {
                    if len == 0 {
                        return None;
                    } else {
                        return Some(encode_chunk(&buffer[0..len]));
                    }
                }
                Some(b) => {
                    buffer[len] = b;
                    len += 1;
                    if len == 3 {
                        return Some(encode_chunk(&buffer));
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::EncodeBase64IterExt;

    #[test]
    fn test_encode_iterator() {
        let buffer = b"testing a long value";
        let base_64: Vec<u8> = buffer
            .iter()
            .copied()
            .encode_base64_iter()
            .flatten()
            .collect();
        assert_eq!(base_64.as_slice(), b"dGVzdGluZyBhIGxvbmcgdmFsdWU=");
    }
}
