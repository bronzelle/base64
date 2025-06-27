use crate::base64::utils::decode_chunk;

pub struct DecodeBase64Iter<'a, I: Iterator<Item = &'a u8>> {
    iterator: I,
}

impl<'a, I: Iterator<Item = &'a u8>> DecodeBase64Iter<'a, I> {
    pub fn new(iterator: I) -> Self {
        Self { iterator }
    }
}

pub trait DecodeBase64IterExt<'a>: Iterator<Item = &'a u8> {
    fn decode_base64_iter(self) -> DecodeBase64Iter<'a, Self>
    where
        Self: Sized,
    {
        DecodeBase64Iter::new(self)
    }
}

impl<'a, I: Iterator<Item = &'a u8>> DecodeBase64IterExt<'a> for I {}

impl<'a, I: Iterator<Item = &'a u8>> Iterator for DecodeBase64Iter<'a, I> {
    type Item = [u8; 3];

    fn next(&mut self) -> Option<Self::Item> {
        let mut buffer = [0u8; 4];
        let mut len = 0;
        loop {
            match self.iterator.next() {
                None => return None,
                Some(b) => {
                    buffer[len] = *b;
                    len += 1;
                    if len == 4 {
                        let response = decode_chunk(&buffer);
                        return response
                            .iter()
                            .filter_map(|v| match (v[0], v[1], v[2]) {
                                (Some(b1), Some(b2), Some(b3)) => Some([b1, b2, b3]),
                                _ => None,
                            })
                            .next();
                        // .flatten();
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::DecodeBase64IterExt;
    #[test]
    fn test_decode_iterator() {
        let buffer = b"dGVzdGluZyBhIGxvbmcgdmFsdWU=";
        let decoded: Vec<u8> = buffer.iter().decode_base64_iter().flatten().collect();
        assert_eq!(decoded.as_slice(), b"testing a long value");
    }
}
