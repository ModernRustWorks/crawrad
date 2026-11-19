use super::*;
use crate::Value;
use std::cmp::Ordering;

#[test]
fn test_float_ordering_and_zeros() {
    let floats = vec![
        Some(f64::NAN),
        Some(f64::INFINITY),
        Some(2.0),
        Some(0.0),
        Some(-0.0), // -0.0 ties with 0.0, ordered by row id
        Some(-1.5),
        Some(f64::NEG_INFINITY),
        None,
    ];

    let mut store = Store::new();
    store.add("f", Column::floats(floats)).expect("add f");

    let rows: Vec<u32> = (0..8).collect();

    // Ascending with Nulls Last
    let sorted_asc_last = store
        .sort(
            &rows,
            &[SortKey {
                column: "f".into(),
                order: Order::Asc,
                nulls: Nulls::Last,
            }],
        )
        .expect("sort");
    // Expected order:
    // -inf (idx 6), -1.5 (idx 5), 0.0 (idx 3 then -0.0 idx 4 by row id tie-break),
    // 2.0 (idx 2), inf (idx 1), NaN (idx 0), Null (idx 7)
    assert_eq!(sorted_asc_last, vec![6, 5, 3, 4, 2, 1, 0, 7]);

    // Ascending with Nulls First
    let sorted_asc_first = store
        .sort(
            &rows,
            &[SortKey {
                column: "f".into(),
                order: Order::Asc,
                nulls: Nulls::First,
            }],
        )
        .expect("sort");
    assert_eq!(sorted_asc_first, vec![7, 6, 5, 3, 4, 2, 1, 0]);

    // Descending with Nulls Last
    let sorted_desc_last = store
        .sort(
            &rows,
            &[SortKey {
                column: "f".into(),
                order: Order::Desc,
                nulls: Nulls::Last,
            }],
        )
        .expect("sort");
    // In Descending, NaN is last among non-nulls (smallest after !ord inversion):
    // NaN (idx 0), inf (idx 1), 2.0 (idx 2), 0.0 (idx 3), -0.0 (idx 4), -1.5 (idx 5), -inf (idx 6), Null (idx 7)
    assert_eq!(sorted_desc_last, vec![0, 1, 2, 3, 4, 5, 6, 7]);
}

#[test]
fn test_focused_int_extremes_order() {
    let input = vec![
        Some(i64::MAX),
        Some(i64::MIN + 1),
        Some(i64::MIN),
        Some(i64::MAX - 1),
    ];
    let mut store = Store::new();
    store.add("val", Column::ints(input)).expect("add val");

    let rows: Vec<u32> = (0..4).collect();

    // Ascending
    let asc = store
        .sort(
            &rows,
            &[SortKey {
                column: "val".into(),
                order: Order::Asc,
                nulls: Nulls::Last,
            }],
        )
        .expect("sort asc");
    // i64::MIN (idx 2), i64::MIN + 1 (idx 1), i64::MAX - 1 (idx 3), i64::MAX (idx 0)
    assert_eq!(asc, vec![2, 1, 3, 0]);

    // Descending
    let desc = store
        .sort(
            &rows,
            &[SortKey {
                column: "val".into(),
                order: Order::Desc,
                nulls: Nulls::Last,
            }],
        )
        .expect("sort desc");
    // i64::MAX (idx 0), i64::MAX - 1 (idx 3), i64::MIN + 1 (idx 1), i64::MIN (idx 2)
    assert_eq!(desc, vec![0, 3, 1, 2]);
}

#[test]
fn test_single_key_radix_sort_ints() {
    let ints = vec![Some(100), Some(-50), None, Some(0), Some(42), Some(-50)];
    let mut store = Store::new();
    store.add("val", Column::ints(ints)).expect("add val");

    let rows: Vec<u32> = (0..6).collect();
    let sorted = store
        .sort(
            &rows,
            &[SortKey {
                column: "val".into(),
                order: Order::Asc,
                nulls: Nulls::First,
            }],
        )
        .expect("sort");

    // -50 has two occurrences: rows 1 and 5. Since radix sort is stable, row 1 must precede row 5.
    // Null is row 2.
    assert_eq!(sorted, vec![2, 1, 5, 3, 4, 0]);
}

#[test]
fn test_multi_key_sort() {
    let mut store = Store::new();
    store
        .add(
            "dept",
            Column::dict_from(vec![
                Some("ENG".into()),
                Some("HR".into()),
                Some("ENG".into()),
                Some("HR".into()),
                Some("ENG".into()),
            ]),
        )
        .expect("add dept");
    store
        .add(
            "salary",
            Column::ints(vec![
                Some(5000),
                Some(3000),
                Some(7000),
                Some(3000),
                Some(5000),
            ]),
        )
        .expect("add salary");

    let rows: Vec<u32> = (0..5).collect();
    let keys = vec![
        SortKey {
            column: "dept".into(),
            order: Order::Asc,
            nulls: Nulls::Last,
        },
        SortKey {
            column: "salary".into(),
            order: Order::Desc,
            nulls: Nulls::Last,
        },
    ];

    let sorted = store.sort(&rows, &keys).expect("sort");
    assert_eq!(sorted, vec![2, 0, 4, 1, 3]);
}

#[test]
fn test_plain_text_prefix_and_full_tie_break() {
    let mut store = Store::new();
    store
        .add(
            "name",
            Column::texts(vec![
                Some("identical_prefix_1".into()),
                Some("identical_prefix_2".into()),
                Some("a".into()),
                Some("identical_prefix_0".into()),
            ]),
        )
        .expect("add name");

    let rows: Vec<u32> = (0..4).collect();
    let keys = vec![SortKey {
        column: "name".into(),
        order: Order::Asc,
        nulls: Nulls::Last,
    }];

    let sorted = store.sort(&rows, &keys).expect("sort");
    assert_eq!(sorted, vec![2, 3, 0, 1]);
}

#[test]
fn test_three_keys_dict_int_nulls_text_prefix_ties() {
    let mut store = Store::new();
    store
        .add(
            "dept",
            Column::dict_from(vec![
                Some("ENG".into()),
                Some("ENG".into()),
                Some("HR".into()),
                Some("ENG".into()),
                Some("HR".into()),
                Some("ENG".into()),
                Some("ENG".into()),
            ]),
        )
        .expect("add dept");
    store
        .add(
            "prio",
            Column::ints(vec![
                Some(10),
                None, // null with Nulls::First
                Some(20),
                Some(10),
                Some(20),
                None,
                Some(10),
            ]),
        )
        .expect("add prio");
    store
        .add(
            "desc",
            Column::texts(vec![
                Some("prefix_1234_B".into()),
                Some("prefix_1234_X".into()),
                Some("prefix_1234_A".into()),
                Some("prefix_1234_A".into()),
                Some("prefix_1234_Z".into()),
                Some("prefix_1234_Y".into()),
                Some("prefix_1234_A".into()), // identical full string to row 3 -> tie-break by row id
            ]),
        )
        .expect("add desc");

    let rows: Vec<u32> = (0..7).collect();
    let keys = vec![
        SortKey {
            column: "dept".into(),
            order: Order::Asc,
            nulls: Nulls::Last,
        },
        SortKey {
            column: "prio".into(),
            order: Order::Asc,
            nulls: Nulls::First,
        },
        SortKey {
            column: "desc".into(),
            order: Order::Asc,
            nulls: Nulls::Last,
        },
    ];

    let sorted = store.sort(&rows, &keys).expect("sort");

    // Plain comparator sort written in the test:
    let mut naive = rows.clone();
    naive.sort_by(|&ra, &rb| {
        // Key 0: dept
        let da = store.value(ra as usize, 0);
        let db = store.value(rb as usize, 0);
        let cd = match (da, db) {
            (Value::Text(a), Value::Text(b)) => a.cmp(b),
            _ => Ordering::Equal,
        };
        if cd != Ordering::Equal {
            return cd;
        }

        // Key 1: prio (Nulls::First)
        let pa = store.value(ra as usize, 1);
        let pb = store.value(rb as usize, 1);
        let cp = match (pa, pb) {
            (Value::Null, Value::Null) => Ordering::Equal,
            (Value::Null, _) => Ordering::Less,
            (_, Value::Null) => Ordering::Greater,
            (Value::Int(a), Value::Int(b)) => a.cmp(&b),
            _ => Ordering::Equal,
        };
        if cp != Ordering::Equal {
            return cp;
        }

        // Key 2: desc
        let ta = store.value(ra as usize, 2);
        let tb = store.value(rb as usize, 2);
        let ct = match (ta, tb) {
            (Value::Text(a), Value::Text(b)) => a.cmp(b),
            _ => Ordering::Equal,
        };
        if ct != Ordering::Equal {
            return ct;
        }

        ra.cmp(&rb)
    });

    assert_eq!(sorted, naive);
}

#[test]
fn test_sort_column_not_found() {
    let store = Store::new();
    let err = store
        .sort(
            &[0, 1],
            &[SortKey {
                column: "missing".into(),
                order: Order::Asc,
                nulls: Nulls::Last,
            }],
        )
        .expect_err("must error on unknown column");
    assert_eq!(err, StoreError::ColumnNotFound("missing".into()));
}

#[test]
fn test_adaptive_sort_shapes() {
    // 1. Sorted, reverse, all-equal, and small inputs
    let test_cases: Vec<(&str, Vec<Option<i64>>)> = vec![
        (
            "sorted",
            vec![Some(10), Some(20), Some(30), Some(40), Some(50), None],
        ),
        (
            "reverse",
            vec![Some(50), Some(40), Some(30), Some(20), Some(10), None],
        ),
        (
            "all_equal",
            vec![Some(42), Some(42), Some(42), Some(42), Some(42), None],
        ),
        ("small_2", vec![Some(20), Some(10)]),
        ("small_3", vec![Some(30), None, Some(10)]),
    ];

    for (_name, input) in test_cases {
        let n = input.len();
        let rows: Vec<u32> = (0..n as u32).collect();

        let mut store = Store::new();
        store
            .add("val", Column::ints(input.clone()))
            .expect("add val");

        for order in [Order::Asc, Order::Desc] {
            for nulls in [Nulls::First, Nulls::Last] {
                let keys = vec![SortKey {
                    column: "val".into(),
                    order,
                    nulls,
                }];

                let sorted = store.sort(&rows, &keys).expect("sort");

                // Oracle comparator
                let mut naive = rows.clone();
                naive.sort_by(|&ra, &rb| {
                    let va = input[ra as usize];
                    let vb = input[rb as usize];
                    let c = match (va, vb) {
                        (None, None) => Ordering::Equal,
                        (None, Some(_)) => match nulls {
                            Nulls::First => Ordering::Less,
                            Nulls::Last => Ordering::Greater,
                        },
                        (Some(_), None) => match nulls {
                            Nulls::First => Ordering::Greater,
                            Nulls::Last => Ordering::Less,
                        },
                        (Some(a), Some(b)) => {
                            let ord = a.cmp(&b);
                            match order {
                                Order::Asc => ord,
                                Order::Desc => ord.reverse(),
                            }
                        }
                    };
                    if c != Ordering::Equal { c } else { ra.cmp(&rb) }
                });

                assert_eq!(
                    sorted, naive,
                    "failed on order {:?}, nulls {:?}",
                    order, nulls
                );
            }
        }
    }
}

#[test]
fn test_packed_vs_lsd_identical() {
    let mut store = Store::new();
    store
        .add(
            "dept",
            Column::dict_from(vec![
                Some("ENG".into()),
                Some("HR".into()),
                Some("ENG".into()),
                Some("HR".into()),
                None,
                Some("ENG".into()),
                Some("HR".into()),
                Some("ENG".into()),
            ]),
        )
        .expect("add dept");
    store
        .add(
            "active",
            Column::bools(vec![
                Some(true),
                Some(false),
                Some(true),
                None,
                Some(false),
                Some(true),
                Some(false),
                None,
            ]),
        )
        .expect("add active");
    store
        .add(
            "salary",
            Column::ints(vec![
                Some(5000),
                Some(3000),
                Some(5000), // tie with row 0
                Some(3000),
                Some(7000),
                None,
                Some(3000),
                Some(5000),
            ]),
        )
        .expect("add salary");

    let rows: Vec<u32> = (0..8).collect();

    for order1 in [Order::Asc, Order::Desc] {
        for order2 in [Order::Asc, Order::Desc] {
            for nulls1 in [Nulls::First, Nulls::Last] {
                for nulls2 in [Nulls::First, Nulls::Last] {
                    let keys = vec![
                        SortKey {
                            column: "dept".into(),
                            order: order1,
                            nulls: nulls1,
                        },
                        SortKey {
                            column: "salary".into(),
                            order: order2,
                            nulls: nulls2,
                        },
                    ];

                    let packed = store
                        .sort_packed(&rows, &keys)
                        .expect("sort_packed")
                        .expect("packed eligible");
                    let lsd = store.sort_lsd(&rows, &keys).expect("sort_lsd");

                    assert_eq!(
                        packed, lsd,
                        "packed vs lsd mismatch with order1={:?}, order2={:?}, nulls1={:?}, nulls2={:?}",
                        order1, order2, nulls1, nulls2
                    );
                }
            }
        }
    }
}

#[test]
fn test_radix_matches_stable_sort_above_small_sort() {
    // `Store::sort` only reaches the radix pass above SMALL_SORT rows, so it is
    // checked here directly against the std stable sort, wide keys and ties included.
    let mut state = 0x9E37_79B9_7F4A_7C15u64;
    let mut pairs = Vec::with_capacity(5_000);
    for row in 0..5_000u32 {
        state ^= state << 13;
        state ^= state >> 7;
        state ^= state << 17;
        let ord = if row % 3 == 0 { state % 7 } else { state };
        pairs.push((ord, row));
    }
    let mut expected = pairs.clone();
    expected.sort_by_key(|&(ord, _)| ord);
    assert_eq!(lsd_radix_sort(pairs), expected);
}
