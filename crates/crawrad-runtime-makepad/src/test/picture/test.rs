//! Which pictures an Image decodes, and when it decodes them again.

use super::{Kind, fingerprint, kind_of};

#[test]
fn png_and_jpeg_are_told_by_their_signature() {
    assert_eq!(
        kind_of(&[0x89, b'P', b'N', b'G', 0x0D, 0x0A]),
        Some(Kind::Png)
    );
    assert_eq!(kind_of(&[0xFF, 0xD8, 0xFF, 0xE0]), Some(Kind::Jpeg));
    assert_eq!(kind_of(b"<svg xmlns"), None);
    assert_eq!(kind_of(&[]), None);
}

#[test]
fn the_fingerprint_changes_with_the_picture_and_not_without() {
    let a: Vec<u8> = (0..10_000u32).map(|i| (i % 251) as u8).collect();
    let mut b = a.clone();
    assert_eq!(fingerprint(&a), fingerprint(&b));
    b.push(7);
    assert_ne!(fingerprint(&a), fingerprint(&b), "a different length");
    let mut c = a.clone();
    c[0] ^= 0xFF;
    assert_ne!(fingerprint(&a), fingerprint(&c), "a different first byte");
}
