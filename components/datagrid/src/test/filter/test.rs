use super::*;
use crate::column::Column;

#[test]
fn test_filter_comparisons_and_nulls() {
    let mut store = Store::new();
    store
        .add(
            "val",
            Column::ints(vec![Some(10), None, Some(20), Some(30), None, Some(40)]),
        )
        .expect("add val");

    // Eq
    let eq20 = store
        .filter(&Filter::Eq("val".into(), Owned::Int(20)))
        .unwrap();
    assert_eq!(eq20.ones().collect::<Vec<_>>(), vec![2]);

    // Gt: row 1 and 4 are null, should not match
    let gt20 = store
        .filter(&Filter::Gt("val".into(), Owned::Int(20)))
        .unwrap();
    assert_eq!(gt20.ones().collect::<Vec<_>>(), vec![3, 5]);

    // IsNull: matches rows 1 and 4
    let is_null = store.filter(&Filter::IsNull("val".into())).unwrap();
    assert_eq!(is_null.ones().collect::<Vec<_>>(), vec![1, 4]);

    // Between
    let between = store
        .filter(&Filter::Between(
            "val".into(),
            Owned::Int(15),
            Owned::Int(35),
        ))
        .unwrap();
    assert_eq!(between.ones().collect::<Vec<_>>(), vec![2, 3]);

    // In
    let in_filter = store
        .filter(&Filter::In(
            "val".into(),
            vec![Owned::Int(10), Owned::Int(40)],
        ))
        .unwrap();
    assert_eq!(in_filter.ones().collect::<Vec<_>>(), vec![0, 5]);
}

#[test]
fn test_filter_dict_strings() {
    let mut store = Store::new();
    store
        .add(
            "status",
            Column::dict_from(vec![
                Some("Pending".into()),
                Some("Completed".into()),
                None,
                Some("Cancelled".into()),
                Some("Processing".into()),
            ]),
        )
        .expect("add status");

    // StartsWith case-insensitive
    let starts = store
        .filter(&Filter::StartsWith {
            column: "status".into(),
            text: "p".into(),
            ignore_case: true,
        })
        .unwrap();
    // Pending (0) and Processing (4) match; null (2) does not
    assert_eq!(starts.ones().collect::<Vec<_>>(), vec![0, 4]);

    // Contains case-sensitive
    let contains = store
        .filter(&Filter::Contains {
            column: "status".into(),
            text: "ing".into(),
            ignore_case: false,
        })
        .unwrap();
    // Pending (0) and Processing (4) match
    assert_eq!(contains.ones().collect::<Vec<_>>(), vec![0, 4]);
}

#[test]
fn test_filter_and_or_not_combinations() {
    let mut store = Store::new();
    store
        .add(
            "num",
            Column::ints(vec![Some(1), Some(2), Some(3), Some(4), Some(5)]),
        )
        .expect("add num");

    let f = Filter::And(vec![
        Filter::Ge("num".into(), Owned::Int(2)),
        Filter::Not(Box::new(Filter::Eq("num".into(), Owned::Int(4)))),
    ]);
    let bm = store.filter(&f).unwrap();
    // 2 (idx 1), 3 (idx 2), 5 (idx 4)
    assert_eq!(bm.ones().collect::<Vec<_>>(), vec![1, 2, 4]);
}

#[test]
fn test_filter_errors() {
    let mut store = Store::new();
    store
        .add("num", Column::ints(vec![Some(1)]))
        .expect("add num");

    // Unknown column
    let err_col = store
        .filter(&Filter::Eq("unknown".into(), Owned::Int(1)))
        .expect_err("unknown column must error");
    assert_eq!(err_col, StoreError::ColumnNotFound("unknown".into()));

    // Type mismatch
    let err_type = store
        .filter(&Filter::Eq("num".into(), Owned::Text("abc".into())))
        .expect_err("type mismatch must error");
    assert_eq!(
        err_type,
        StoreError::TypeMismatch {
            column: "num".into(),
            expected: "Int"
        }
    );

    // Type mismatch on EqIgnoreCase for non-text column
    let err_eq_ic = store
        .filter(&Filter::eq_ignore_case("num", "1"))
        .expect_err("eq_ignore_case on int column must error");
    assert_eq!(
        err_eq_ic,
        StoreError::TypeMismatch {
            column: "num".into(),
            expected: "Text",
        }
    );
}

#[test]
fn test_filter_eq_ignore_case_unicode() {
    let mut store = Store::new();
    store
        .add(
            "word",
            Column::texts(vec![
                Some("ÁRVORE".into()),
                Some("árvore".into()),
                Some("FLORESTA".into()),
                None,
                Some("maçã".into()),
            ]),
        )
        .expect("add word");

    let f1 = Filter::eq_ignore_case("word", "árvore");
    let bm1 = store.filter(&f1).unwrap();
    assert_eq!(bm1.ones().collect::<Vec<_>>(), vec![0, 1]);

    let f2 = Filter::eq_ignore_case("word", "ÁRVORE");
    let bm2 = store.filter(&f2).unwrap();
    assert_eq!(bm2.ones().collect::<Vec<_>>(), vec![0, 1]);

    let f3 = Filter::eq_ignore_case("word", "MAÇÃ");
    let bm3 = store.filter(&f3).unwrap();
    assert_eq!(bm3.ones().collect::<Vec<_>>(), vec![4]);
}
