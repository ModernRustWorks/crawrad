use super::*;
use crate::column::Column;

#[test]
fn test_locator_find_eq() {
    let mut store = Store::new();
    store
        .add(
            "id",
            Column::ints(vec![
                Some(100),
                Some(200),
                None,
                Some(300),
                Some(200), // duplicate of 200
            ]),
        )
        .expect("add id");

    let mut locator = Locator::new();

    // First occurrence of 200 is row 1
    let found = locator.find_eq(&store, "id", &Owned::Int(200)).unwrap();
    assert_eq!(found, Some(1));

    // First occurrence of 100 is row 0
    let found_first = locator.find_eq(&store, "id", &Owned::Int(100)).unwrap();
    assert_eq!(found_first, Some(0));

    // Non-existent value
    let not_found = locator.find_eq(&store, "id", &Owned::Int(999)).unwrap();
    assert_eq!(not_found, None);

    // Repeated query uses cached index
    let cached = locator.find_eq(&store, "id", &Owned::Int(200)).unwrap();
    assert_eq!(cached, Some(1));
}

#[test]
fn test_locator_find_eq_float_zeros_and_nan() {
    let mut store = Store::new();
    store
        .add(
            "f",
            Column::floats(vec![
                Some(1.0),
                Some(-0.0),     // row 1 has -0.0
                Some(0.0),      // row 2 has 0.0
                Some(f64::NAN), // row 3 has NaN
            ]),
        )
        .expect("add f");

    let mut locator = Locator::new();

    // Looking for 0.0 should find row 1 (-0.0 and 0.0 are canonicalized)
    let found_zero = locator.find_eq(&store, "f", &Owned::Float(0.0)).unwrap();
    assert_eq!(found_zero, Some(1));

    let found_neg_zero = locator.find_eq(&store, "f", &Owned::Float(-0.0)).unwrap();
    assert_eq!(found_neg_zero, Some(1));

    // Looking for NaN
    let found_nan = locator
        .find_eq(&store, "f", &Owned::Float(f64::NAN))
        .unwrap();
    assert_eq!(found_nan, Some(3));
}

#[test]
fn test_locator_range_binary_search() {
    let mut store = Store::new();
    store
        .add(
            "score",
            Column::floats(vec![
                Some(10.5),
                None,
                Some(45.0),
                Some(20.0),
                Some(35.2),
                Some(20.0),
            ]),
        )
        .expect("add score");

    let mut locator = Locator::new();

    // Range [20.0, 40.0]
    let rows = locator
        .range(&store, "score", &Owned::Float(20.0), &Owned::Float(40.0))
        .unwrap();

    // Matching rows in sorted order:
    // 20.0: rows 3 and 5
    // 35.2: row 4
    assert_eq!(rows.len(), 3);
    for &r in &rows {
        let val = store.value(r as usize, 0);
        if let crate::column::Value::Float(f) = val {
            assert!((20.0..=40.0).contains(&f));
        } else {
            panic!("expected float");
        }
    }

    // Out of bounds range
    let empty_range = locator
        .range(&store, "score", &Owned::Float(100.0), &Owned::Float(200.0))
        .unwrap();
    assert!(empty_range.is_empty());
}

#[test]
fn test_locator_clear() {
    let mut store = Store::new();
    store
        .add("id", Column::ints(vec![Some(10), Some(20)]))
        .expect("add id");

    let mut locator = Locator::new();
    let _ = locator.find_eq(&store, "id", &Owned::Int(10));
    assert!(!locator.sorted_indexes.is_empty());

    locator.clear();
    assert!(locator.sorted_indexes.is_empty());
}

#[test]
fn test_locator_find_eq_duplicates_first_row() {
    let mut store = Store::new();
    store
        .add(
            "val",
            Column::ints(vec![
                Some(50),
                Some(10), // row 1: first 10
                Some(20),
                Some(10), // row 3: second 10
                Some(10), // row 4: third 10
                Some(30),
            ]),
        )
        .expect("add val");

    let mut locator = Locator::new();
    let found = locator.find_eq(&store, "val", &Owned::Int(10)).unwrap();
    assert_eq!(found, Some(1));
}
