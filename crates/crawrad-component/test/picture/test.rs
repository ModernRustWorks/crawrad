use crate::Picture;

#[test]
fn picture_default_is_empty() {
    let pic = Picture::default();
    assert!(pic.is_empty());
    assert_eq!(pic.bytes().len(), 0);
    assert_eq!(pic.to_string(), "(None)");
}

#[test]
fn picture_with_bytes() {
    let png_header = vec![0x89, b'P', b'N', b'G', 0x0D, 0x0A, 0x1A, 0x0A, 0, 1, 2, 3];
    let pic = Picture {
        bytes: png_header.clone(),
        source: None,
    };
    assert!(!pic.is_empty());
    assert_eq!(pic.bytes(), png_header.as_slice());
    assert_eq!(pic.to_string(), "(Picture: 12 bytes)");
}

#[test]
fn picture_with_source() {
    let pic = Picture {
        bytes: Vec::new(),
        source: Some("assets/logo.png".to_string()),
    };
    assert_eq!(pic.source.as_deref(), Some("assets/logo.png"));
    assert_eq!(pic.to_string(), "(Picture: assets/logo.png)");
}
