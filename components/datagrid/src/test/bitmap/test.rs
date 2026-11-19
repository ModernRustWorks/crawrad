use super::*;

#[test]
fn test_bitmap_empty() {
    let bm = Bitmap::new(0, false);
    assert_eq!(bm.len(), 0);
    assert!(bm.is_empty());
    assert_eq!(bm.count_ones(), 0);
    assert_eq!(bm.ones().collect::<Vec<_>>(), Vec::<u32>::new());

    let bm_true = Bitmap::new(0, true);
    assert_eq!(bm_true.len(), 0);
    assert!(bm_true.is_empty());
    assert_eq!(bm_true.count_ones(), 0);
}

#[test]
fn test_bitmap_lengths_non_multiple_of_64() {
    let test_lengths = [1, 7, 63, 64, 65, 100, 127, 128, 129, 200];
    for &len in &test_lengths {
        let bm_zeros = Bitmap::new(len, false);
        assert_eq!(bm_zeros.len(), len);
        assert!(!bm_zeros.is_empty());
        assert_eq!(bm_zeros.count_ones(), 0);
        assert_eq!(bm_zeros.ones().count(), 0);

        let bm_ones = Bitmap::new(len, true);
        assert_eq!(bm_ones.len(), len);
        assert_eq!(bm_ones.count_ones(), len);
        let ones_vec = bm_ones.ones().collect::<Vec<_>>();
        assert_eq!(ones_vec.len(), len);
        let expected = (0..len as u32).collect::<Vec<_>>();
        assert_eq!(ones_vec, expected);

        let inverted = bm_zeros.not();
        assert_eq!(inverted, bm_ones);
        let double_inverted = inverted.not();
        assert_eq!(double_inverted, bm_zeros);
    }
}

#[test]
fn test_bitmap_set_and_get() {
    let mut bm = Bitmap::new(130, false);
    let indices = [0, 1, 63, 64, 65, 127, 128, 129];
    for &idx in &indices {
        assert!(!bm.get(idx));
        bm.set(idx, true);
        assert!(bm.get(idx));
    }
    assert_eq!(bm.count_ones(), indices.len());
    let collected = bm.ones().collect::<Vec<_>>();
    assert_eq!(
        collected,
        indices.iter().map(|&i| i as u32).collect::<Vec<_>>()
    );

    for &idx in &indices {
        bm.set(idx, false);
        assert!(!bm.get(idx));
    }
    assert_eq!(bm.count_ones(), 0);
}

#[test]
fn test_bitmap_and_or_not() {
    let mut b1 = Bitmap::new(100, false);
    let mut b2 = Bitmap::new(100, false);

    b1.set(10, true);
    b1.set(20, true);
    b1.set(70, true);

    b2.set(20, true);
    b2.set(30, true);
    b2.set(70, true);
    b2.set(80, true);

    let and_bm = b1.and(&b2);
    assert_eq!(and_bm.count_ones(), 2);
    assert!(and_bm.get(20));
    assert!(and_bm.get(70));
    assert!(!and_bm.get(10));
    assert!(!and_bm.get(30));

    let or_bm = b1.or(&b2);
    assert_eq!(or_bm.count_ones(), 5);
    assert!(or_bm.get(10));
    assert!(or_bm.get(20));
    assert!(or_bm.get(30));
    assert!(or_bm.get(70));
    assert!(or_bm.get(80));

    let not_b1 = b1.not();
    assert_eq!(not_b1.count_ones(), 100 - 3);
    assert!(!not_b1.get(10));
    assert!(!not_b1.get(20));
    assert!(!not_b1.get(70));
    assert!(not_b1.get(0));
    assert!(not_b1.get(99));
}
