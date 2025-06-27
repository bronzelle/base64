use crate::base64::utils::encode_chunk;

pub struct EncodeBase64Iter<'a, I: Iterator<Item = &'a u8>> {
    iterator: I,
}

impl<'a, I: Iterator<Item = &'a u8>> EncodeBase64Iter<'a, I> {
    pub fn new(iterator: I) -> Self {
        Self { iterator }
    }
}

pub trait EncodeBase64IterExt<'a>: Iterator<Item = &'a u8> {
    fn encode_base64_iter(self) -> EncodeBase64Iter<'a, Self>
    where
        Self: Sized,
    {
        EncodeBase64Iter::new(self)
    }
}

impl<'a, I: Iterator<Item = &'a u8>> EncodeBase64IterExt<'a> for I {}

impl<'a, I: Iterator<Item = &'a u8>> Iterator for EncodeBase64Iter<'a, I> {
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
                    buffer[len] = *b;
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
        let base_64: Vec<u8> = buffer.iter().encode_base64_iter().flatten().collect();
        assert_eq!(base_64.as_slice(), b"dGVzdGluZyBhIGxvbmcgdmFsdWU=");
    }
}
