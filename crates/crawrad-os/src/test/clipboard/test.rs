//! Tests for clipboard image encoding and decoding.

use super::*;

#[test]
fn rgba_to_png_round_trips_2x1_image() {
    let rgba = [255, 0, 0, 255, 0, 255, 0, 255];
    let png_bytes = rgba_to_png(2, 1, &rgba).expect("png encoding should succeed");

    let decoder = png::Decoder::new(&png_bytes[..]);
    let mut reader = decoder.read_info().expect("png header read should succeed");
    assert_eq!(reader.info().width, 2);
    assert_eq!(reader.info().height, 1);

    let mut buf = vec![0u8; reader.output_buffer_size()];
    let output_info = reader
        .next_frame(&mut buf)
        .expect("reading frame should succeed");
    assert_eq!(&buf[..output_info.buffer_size()], &rgba);
}

#[test]
fn rgba_to_png_rejects_empty_or_mismatched_dimensions() {
    assert!(rgba_to_png(0, 0, &[]).is_none());
    assert!(rgba_to_png(0, 1, &[]).is_none());
    assert!(rgba_to_png(1, 0, &[]).is_none());
    assert!(rgba_to_png(2, 1, &[255, 0, 0, 255]).is_none());
    assert!(rgba_to_png(1, 1, &[255, 0, 0]).is_none());
}
