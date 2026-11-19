use super::align_of;

#[test]
fn the_classic_names_become_align_fractions() {
    assert_eq!(align_of("taLeftJustify", "tlTop"), (0.0, 0.0));
    assert_eq!(align_of("taCenter", "tlCenter"), (0.5, 0.5));
    assert_eq!(align_of("taRightJustify", "tlBottom"), (1.0, 1.0));
    assert_eq!(
        align_of("", ""),
        (0.0, 0.0),
        "an unknown name is the default corner"
    );
}
