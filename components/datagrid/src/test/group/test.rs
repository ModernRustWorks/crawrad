use super::*;
use crate::column::Column;

#[test]
fn test_group_single_column_with_totals() {
    let mut store = Store::new();
    store
        .add(
            "dept",
            Column::dict_from(vec![
                Some("Sales".into()),
                Some("Eng".into()),
                Some("Sales".into()),
                Some("Eng".into()),
                Some("Eng".into()),
            ]),
        )
        .expect("add dept");
    store
        .add(
            "salary",
            Column::floats(vec![
                Some(100.0),
                Some(200.0),
                Some(150.0),
                Some(300.0),
                None,
            ]),
        )
        .expect("add salary");

    let rows: Vec<u32> = (0..5).collect();
    let totals = vec![
        Total {
            column: "salary".into(),
            aggregate: Aggregate::Count,
        },
        Total {
            column: "salary".into(),
            aggregate: Aggregate::Sum,
        },
        Total {
            column: "salary".into(),
            aggregate: Aggregate::Avg,
        },
        Total {
            column: "salary".into(),
            aggregate: Aggregate::Min,
        },
        Total {
            column: "salary".into(),
            aggregate: Aggregate::Max,
        },
    ];

    let groups = store
        .group(&rows, &["dept".into()], &totals)
        .expect("group");

    // "Eng" is code 0, "Sales" is code 1.
    // In key order: Eng (ord 0) comes before Sales (ord 1).
    assert_eq!(groups.len(), 2);

    let eng = &groups[0];
    assert_eq!(
        store.key_value("dept", eng.key[0]).unwrap(),
        Value::Text("Eng")
    );
    assert_eq!(eng.rows, vec![1, 3, 4]);
    assert_eq!(eng.totals[0], Some(3.0)); // Count: 3 rows
    assert_eq!(eng.totals[1], Some(500.0)); // Sum: 200 + 300
    assert_eq!(eng.totals[2], Some(250.0)); // Avg: 500 / 2
    assert_eq!(eng.totals[3], Some(200.0)); // Min: 200
    assert_eq!(eng.totals[4], Some(300.0)); // Max: 300

    let sales = &groups[1];
    assert_eq!(
        store.key_value("dept", sales.key[0]).unwrap(),
        Value::Text("Sales")
    );
    assert_eq!(sales.rows, vec![0, 2]);
    assert_eq!(sales.totals[0], Some(2.0)); // Count: 2 rows
    assert_eq!(sales.totals[1], Some(250.0)); // Sum: 100 + 150
    assert_eq!(sales.totals[2], Some(125.0)); // Avg: 125
    assert_eq!(sales.totals[3], Some(100.0)); // Min: 100
    assert_eq!(sales.totals[4], Some(150.0)); // Max: 150
}

#[test]
fn test_focused_int_min_groups_distinct() {
    let input = vec![
        Some(i64::MIN),
        Some(i64::MIN + 1),
        Some(i64::MIN),
        Some(i64::MIN + 1),
    ];
    let mut store = Store::new();
    store.add("val", Column::ints(input)).expect("add val");

    let rows: Vec<u32> = (0..4).collect();
    let groups = store
        .group(&rows, &["val".into()], &[])
        .expect("group must succeed");

    assert_eq!(
        groups.len(),
        2,
        "i64::MIN and i64::MIN+1 must form 2 distinct groups"
    );
    assert_eq!(
        store.key_value("val", groups[0].key[0]).unwrap(),
        Value::Int(i64::MIN)
    );
    assert_eq!(
        store.key_value("val", groups[1].key[0]).unwrap(),
        Value::Int(i64::MIN + 1)
    );
    assert_eq!(groups[0].rows, vec![0, 2]);
    assert_eq!(groups[1].rows, vec![1, 3]);
}

#[test]
fn test_group_two_columns_and_subgroup() {
    let mut store = Store::new();
    store
        .add(
            "country",
            Column::dict_from(vec![
                Some("US".into()),
                Some("US".into()),
                Some("BR".into()),
                Some("BR".into()),
            ]),
        )
        .expect("add country");
    store
        .add(
            "active",
            Column::bools(vec![Some(true), Some(false), Some(true), Some(true)]),
        )
        .expect("add active");
    store
        .add(
            "score",
            Column::ints(vec![Some(10), Some(20), Some(30), Some(40)]),
        )
        .expect("add score");

    let rows: Vec<u32> = (0..4).collect();
    let totals = vec![Total {
        column: "score".into(),
        aggregate: Aggregate::Sum,
    }];

    // Level 1: group by country
    let level1 = store
        .group(&rows, &["country".into()], &totals)
        .expect("level 1");
    assert_eq!(level1.len(), 2);

    // Subgroup level 2 on level1's first group (BR)
    let br_subgroups = store
        .group(&level1[0].rows, &["active".into()], &totals)
        .expect("subgroup");
    assert_eq!(br_subgroups.len(), 1);
    assert_eq!(br_subgroups[0].rows, vec![2, 3]);
    assert_eq!(br_subgroups[0].totals[0], Some(70.0));
}

#[test]
fn test_group_not_groupable_plain_text() {
    let mut store = Store::new();
    store
        .add("txt", Column::texts(vec![Some("a".into())]))
        .expect("add txt");

    let err = store
        .group(&[0], &["txt".into()], &[])
        .expect_err("plain text must fail");
    assert_eq!(err, StoreError::NotGroupable("txt".into()));

    let kv_err = store
        .key_value("txt", Some(0))
        .expect_err("key_value on plain text must fail");
    assert_eq!(kv_err, StoreError::NotGroupable("txt".into()));
}

#[test]
fn test_group_column_not_found() {
    let store = Store::new();
    let err = store
        .key_value("missing", Some(0))
        .expect_err("missing column must fail");
    assert_eq!(err, StoreError::ColumnNotFound("missing".into()));
}

#[test]
fn test_group_all_null_totals() {
    let mut store = Store::new();
    store
        .add("grp", Column::ints(vec![Some(1), Some(1)]))
        .expect("add grp");
    store
        .add("val", Column::floats(vec![None, None]))
        .expect("add val");

    let totals = vec![
        Total {
            column: "val".into(),
            aggregate: Aggregate::Sum,
        },
        Total {
            column: "val".into(),
            aggregate: Aggregate::Avg,
        },
        Total {
            column: "val".into(),
            aggregate: Aggregate::Min,
        },
        Total {
            column: "val".into(),
            aggregate: Aggregate::Max,
        },
    ];

    let groups = store.group(&[0, 1], &["grp".into()], &totals).unwrap();
    assert_eq!(groups.len(), 1);
    assert_eq!(groups[0].totals[0], None);
    assert_eq!(groups[0].totals[1], None);
    assert_eq!(groups[0].totals[2], None);
    assert_eq!(groups[0].totals[3], None);
}

#[test]
fn test_dense_and_hash_paths_equal() {
    let mut store = Store::new();
    store
        .add(
            "dept",
            Column::dict_from(vec![
                Some("Sales".into()),
                Some("Eng".into()),
                None,
                Some("Sales".into()),
                Some("Eng".into()),
                None,
                Some("Marketing".into()),
                Some("Eng".into()),
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
                None,
                Some(true),
                Some(true),
            ]),
        )
        .expect("add active");
    store
        .add(
            "salary",
            Column::floats(vec![
                Some(100.0),
                Some(200.0),
                Some(150.0),
                Some(300.0),
                None,
                Some(400.0),
                Some(500.0),
                Some(600.0),
            ]),
        )
        .expect("add salary");

    let rows: Vec<u32> = (0..8).collect();
    let totals = vec![
        Total {
            column: "salary".into(),
            aggregate: Aggregate::Count,
        },
        Total {
            column: "salary".into(),
            aggregate: Aggregate::Sum,
        },
        Total {
            column: "salary".into(),
            aggregate: Aggregate::Avg,
        },
        Total {
            column: "salary".into(),
            aggregate: Aggregate::Min,
        },
        Total {
            column: "salary".into(),
            aggregate: Aggregate::Max,
        },
    ];

    // Single key: dept (forces dense vs hash)
    let dense_1 = store
        .group_with_method(&rows, &["dept".into()], &totals, GroupMethod::Dense)
        .unwrap();
    let hash_1 = store
        .group_with_method(&rows, &["dept".into()], &totals, GroupMethod::Hash)
        .unwrap();
    assert_eq!(dense_1, hash_1, "single-key dense and hash must match");

    // Two keys: dept, active (forces dense vs hash)
    let dense_2 = store
        .group_with_method(
            &rows,
            &["dept".into(), "active".into()],
            &totals,
            GroupMethod::Dense,
        )
        .unwrap();
    let hash_2 = store
        .group_with_method(
            &rows,
            &["dept".into(), "active".into()],
            &totals,
            GroupMethod::Hash,
        )
        .unwrap();
    assert_eq!(dense_2, hash_2, "two-key dense and hash must match");
}

#[test]
fn test_group_dense_sort_hash_identical() {
    let mut store = Store::new();
    store
        .add(
            "dept",
            Column::dict_from(vec![
                Some("Sales".into()),
                Some("Eng".into()),
                None,
                Some("Sales".into()),
                Some("Eng".into()),
                None,
                Some("Marketing".into()),
                Some("Eng".into()),
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
                None,
                Some(true),
                Some(true),
            ]),
        )
        .expect("add active");
    store
        .add(
            "salary",
            Column::floats(vec![
                Some(100.0),
                Some(200.0),
                Some(150.0),
                Some(300.0),
                None,
                Some(400.0),
                Some(500.0),
                Some(600.0),
            ]),
        )
        .expect("add salary");

    let rows: Vec<u32> = (0..8).collect();
    let totals = vec![
        Total {
            column: "salary".into(),
            aggregate: Aggregate::Count,
        },
        Total {
            column: "salary".into(),
            aggregate: Aggregate::Sum,
        },
        Total {
            column: "salary".into(),
            aggregate: Aggregate::Avg,
        },
        Total {
            column: "salary".into(),
            aggregate: Aggregate::Min,
        },
        Total {
            column: "salary".into(),
            aggregate: Aggregate::Max,
        },
    ];

    // Single key: dept
    let dense_1 = store
        .group_with_method(&rows, &["dept".into()], &totals, GroupMethod::Dense)
        .unwrap();
    let sort_1 = store
        .group_with_method(&rows, &["dept".into()], &totals, GroupMethod::Sort)
        .unwrap();
    let hash_1 = store
        .group_with_method(&rows, &["dept".into()], &totals, GroupMethod::Hash)
        .unwrap();
    assert_eq!(dense_1, sort_1, "single-key dense and sort must match");
    assert_eq!(sort_1, hash_1, "single-key sort and hash must match");

    // Two keys: dept, active
    let dense_2 = store
        .group_with_method(
            &rows,
            &["dept".into(), "active".into()],
            &totals,
            GroupMethod::Dense,
        )
        .unwrap();
    let sort_2 = store
        .group_with_method(
            &rows,
            &["dept".into(), "active".into()],
            &totals,
            GroupMethod::Sort,
        )
        .unwrap();
    let hash_2 = store
        .group_with_method(
            &rows,
            &["dept".into(), "active".into()],
            &totals,
            GroupMethod::Hash,
        )
        .unwrap();
    assert_eq!(dense_2, sort_2, "two-key dense and sort must match");
    assert_eq!(sort_2, hash_2, "two-key sort and hash must match");
}
