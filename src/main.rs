const BASE64_MAP: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

const INVALID: u8 = 255;

const REVERSE_MAP: [u8; 256] = {
    let mut map = [INVALID; 256];
    let mut i = 0;
    while i < BASE64_MAP.len() {
        map[BASE64_MAP[i] as usize] = i as u8;
        i += 1;
    }
    map
};

fn unmap(c: u8) -> u8 {
    // let value = REVERSE_MAP[c as usize];
    // if value == INVALID { None } else { Some(value) }
    REVERSE_MAP[c as usize]
}

fn map(c: u8) -> u8 {
    if c < 64 { BASE64_MAP[c as usize] } else { c }
}

fn decode_chunk(chunk: &[u8]) -> Option<[Option<u8>; 3]> {
    if chunk.len() != 4 {
        return None;
    }
    const PAD: u8 = '=' as u8;

    let c1 = unmap(chunk[0]);
    let c2 = unmap(chunk[1]);

    Some(match (chunk[2], chunk[3]) {
        (PAD, PAD) => {
            let b1 = (c1 << 2) | (c2 >> 4);
            [Some(b1), None, None]
        }
        (c3, PAD) => {
            let c3 = unmap(c3);
            let b1 = (c1 << 2) | (c2 >> 4);
            let b2 = ((c2 & 0b0000_1111) << 4) | (c3 >> 2);
            [Some(b1), Some(b2), None]
        }
        (c3, c4) => {
            let c3 = unmap(c3);
            let c4 = unmap(c4);
            let b1 = (c1 << 2) | (c2 >> 4);
            let b2 = ((c2 & 0b0000_1111) << 4) | (c3 >> 2);
            let b3 = ((c3 & 0b0000_0011) << 6) | c4;
            [Some(b1), Some(b2), Some(b3)]
        }
    })
}

fn encode_chunk(chunk: &[u8]) -> [u8; 4] {
    let p = '=' as u8;

    let b1 = (chunk[0] & 0b1111_1100) >> 2;
    let (b2, b3, b4) = match chunk.len() {
        1 => {
            let b2 = (chunk[0] & 0b0000_0011) << 4;
            (b2, p, p)
        }
        2 => {
            let b2 = ((chunk[0] & 0b0000_0011) << 4) | ((chunk[1] & 0b1111_0000) >> 4);
            let b3 = (chunk[1] & 0b0000_1111) << 2;
            (b2, b3, p)
        }
        3 => {
            let b2 = ((chunk[0] & 0b0000_0011) << 4) | ((chunk[1] & 0b1111_0000) >> 4);
            let b3 = ((chunk[1] & 0b0000_1111) << 2) | ((chunk[2] & 0b1100_0000) >> 6);
            let b4 = chunk[2] & 0b0011_1111;
            (b2, b3, b4)
        }
        _ => (0, 0, 0),
    };

    let b1 = map(b1);
    let b2 = map(b2);
    let b3 = if b3 == p { p } else { map(b3) };
    let b4 = if b4 == p { p } else { map(b4) };
    [b1, b2, b3, b4]
}

fn encode(buffer: &[u8]) -> Box<[u8]> {
    let mut b64 = Vec::new();
    for group in buffer.chunks(3) {
        b64.extend_from_slice(&encode_chunk(group));
    }
    b64.into_boxed_slice()
}

fn decode(buffer: &[u8]) -> Box<[u8]> {
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

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map() {
        assert_eq!(map(0), b'A');
        assert_eq!(map(26), b'a');
        assert_eq!(map(52), b'0');
        assert_eq!(map(62), b'+');
        assert_eq!(map(63), b'/');
    }

    #[test]
    fn test_encode_chunk() {
        let chunk_1 = encode_chunk(b"A");
        assert_eq!(&chunk_1, b"QQ==");
        let chunk_2 = encode_chunk(b"AB");
        assert_eq!(&chunk_2, b"QUI=");
        let chunk_3 = encode_chunk(b"ABC");
        assert_eq!(&chunk_3, b"QUJD");
    }

    #[test]
    fn test_encode() {
        let base_64 = encode(b"testing a long value");
        assert_eq!(base_64.as_ref(), b"dGVzdGluZyBhIGxvbmcgdmFsdWU=");
    }

    #[test]
    fn test_unmap() {
        assert_eq!(unmap(b'A'), 0);
        assert_eq!(unmap(b'a'), 26);
        assert_eq!(unmap(b'0'), 52);
        assert_eq!(unmap(b'+'), 62);
        assert_eq!(unmap(b'/'), 63);
        assert_eq!(unmap(b'='), INVALID);
    }

    #[test]
    fn test_decode_chunk() {
        let chunk_1 = decode_chunk(b"QQ==");
        assert_eq!(chunk_1, Some([Some(b'A'), None, None]));
        let chunk_2 = decode_chunk(b"QUI=");
        assert_eq!(chunk_2, Some([Some(b'A'), Some(b'B'), None]));
        let chunk_3 = decode_chunk(b"QUJD");
        assert_eq!(chunk_3, Some([Some(b'A'), Some(b'B'), Some(b'C')]));
    }

    #[test]
    fn test_decode() {
        let base_64 = decode(b"dGVzdGluZyBhIGxvbmcgdmFsdWU=");
        assert_eq!(base_64.as_ref(), b"testing a long value");
    }
}
