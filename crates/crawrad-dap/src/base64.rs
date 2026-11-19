//! Standard base64 decoding — the encoding of a `readMemory` body.
//!
//! One function, so written here instead of adding a dependency for it.

use crawrad_note::note;
#[note("base64-decode")]
pub fn decode(text: &str) -> Option<Vec<u8>> {
    fn sextet(c: u8) -> Option<u32> {
        match c {
            b'A'..=b'Z' => Some(u32::from(c - b'A')),
            b'a'..=b'z' => Some(u32::from(c - b'a') + 26),
            b'0'..=b'9' => Some(u32::from(c - b'0') + 52),
            b'+' => Some(62),
            b'/' => Some(63),
            _ => None,
        }
    }
    let bytes: Vec<u8> = text.bytes().filter(|b| !b.is_ascii_whitespace()).collect();
    if !bytes.len().is_multiple_of(4) {
        return None;
    }
    let mut out = Vec::with_capacity(bytes.len() / 4 * 3);
    for chunk in bytes.chunks(4) {
        let pad = chunk.iter().rev().take_while(|&&c| c == b'=').count();
        if pad > 2 {
            return None;
        }
        let mut n = 0u32;
        for &c in &chunk[..4 - pad] {
            n = (n << 6) | sextet(c)?;
        }
        n <<= 6 * pad as u32;
        let decoded = [(n >> 16) as u8, (n >> 8) as u8, n as u8];
        out.extend_from_slice(&decoded[..3 - pad]);
    }
    Some(out)
}

#[cfg(test)]
mod tests {
    use super::decode;

    #[test]
    fn decodes_the_three_padding_shapes() {
        assert_eq!(decode("TWFu").as_deref(), Some(&b"Man"[..]));
        assert_eq!(decode("TWE=").as_deref(), Some(&b"Ma"[..]));
        assert_eq!(decode("TQ==").as_deref(), Some(&b"M"[..]));
        assert_eq!(decode("").as_deref(), Some(&b""[..]));
    }

    #[test]
    fn decodes_what_lldb_dap_sent_on_30_09() {
        // The 16 bytes behind a `String` of 13 bytes, measured on the owner's
        // machine: an 8-byte pointer, then the length 13 (0x0D).
        let bytes = decode("wNlGT40CAAANAAAAAAAAAA==").expect("valid base64");
        assert_eq!(bytes.len(), 16);
        assert_eq!(u64::from_le_bytes(bytes[8..16].try_into().unwrap()), 13);
    }

    #[test]
    fn rejects_what_is_not_base64() {
        assert_eq!(decode("abc"), None);
        assert_eq!(decode("ab!="), None);
        assert_eq!(decode("a==="), None);
    }
}
