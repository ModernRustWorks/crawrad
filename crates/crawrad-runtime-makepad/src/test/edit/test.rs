use super::align_x_of;

#[test]
fn alignment_maps_to_horizontal_fractions() {
    assert_eq!(align_x_of("taLeftJustify"), 0.0);
    assert_eq!(align_x_of("taCenter"), 0.5);
    assert_eq!(align_x_of("taRightJustify"), 1.0);
    assert_eq!(align_x_of("unknown"), 0.0);
}
