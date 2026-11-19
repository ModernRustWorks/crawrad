use crate::column::{Column, Value};
use crate::filter::{Filter, Owned};
use crate::group::{Aggregate, Total};
use crate::sort::{Nulls, Order, SortKey};
use crate::source::{
    Capabilities, ColumnKind, DisplayHint, FakeSlowSource, RowSource, SourceError, VecSource,
};
use crate::store::{Store, StoreError};
use crate::view::{GroupLevel, Query, View};
use std::time::Duration;

struct XorShift64(u64);

impl XorShift64 {
    fn new(seed: u64) -> Self {
        let s = if seed == 0 { 0x853c49e6748fea9b } else { seed };
        Self(s)
    }

    fn next_u64(&mut self) -> u64 {
        let mut x = self.0;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.0 = x;
        x
    }

    fn next_range(&mut self, min: u64, max: u64) -> u64 {
        assert!(min <= max);
        let range = max - min + 1;
        min + (self.next_u64() % range)
    }

    fn next_bool(&mut self) -> bool {
        (self.next_u64() & 1) == 1
    }
}

#[test]
fn test_dict_gather_copies_only_the_entries_the_window_uses() {
    let col = Column::dict_from(
        ["pear", "apple", "kiwi", "fig", "apple", "pear"]
            .iter()
            .map(|s| Some(s.to_string()))
            .collect(),
    );
    let window = col.gather(&[5, 1, 4]);
    match &window {
        Column::Dict { dict, codes, .. } => {
            assert_eq!(dict, &vec!["apple".to_string(), "pear".to_string()]);
            assert_eq!(codes, &vec![1, 0, 0]);
        }
        _ => panic!("expected dict column variant"),
    }
    assert_eq!(window.get(0), Value::Text("pear"));
    assert_eq!(window.get(1), Value::Text("apple"));
}

#[test]
fn test_column_gather_all_kinds_with_and_without_nulls() {
    // 1. Column::Int without nulls
    let int_col = Column::ints(vec![Some(10), Some(20), Some(30), Some(40)]);
    let gathered = int_col.gather(&[1, 3]);
    assert_eq!(gathered.len(), 2);
    assert_eq!(gathered.get(0), Value::Int(20));
    assert_eq!(gathered.get(1), Value::Int(40));
    assert!(gathered.validity().is_none());

    // Column::Int with nulls
    let int_nulls = Column::ints(vec![Some(10), None, Some(30), None]);
    let gathered_nulls = int_nulls.gather(&[0, 1, 2, 3]);
    assert_eq!(gathered_nulls.len(), 4);
    assert_eq!(gathered_nulls.get(0), Value::Int(10));
    assert_eq!(gathered_nulls.get(1), Value::Null);
    assert_eq!(gathered_nulls.get(2), Value::Int(30));
    assert_eq!(gathered_nulls.get(3), Value::Null);
    assert!(gathered_nulls.validity().is_some());

    let gathered_all_null = int_nulls.gather(&[1, 3]);
    assert_eq!(gathered_all_null.len(), 2);
    assert_eq!(gathered_all_null.get(0), Value::Null);
    assert_eq!(gathered_all_null.get(1), Value::Null);

    // 2. Column::Float without and with nulls
    let float_col = Column::floats(vec![Some(1.5), Some(2.5), Some(3.5)]);
    let gathered_float = float_col.gather(&[2, 0]);
    assert_eq!(gathered_float.len(), 2);
    assert_eq!(gathered_float.get(0), Value::Float(3.5));
    assert_eq!(gathered_float.get(1), Value::Float(1.5));
    assert!(gathered_float.validity().is_none());

    let float_nulls = Column::floats(vec![Some(1.5), None, Some(3.5)]);
    let gathered_float_nulls = float_nulls.gather(&[1, 2]);
    assert_eq!(gathered_float_nulls.len(), 2);
    assert_eq!(gathered_float_nulls.get(0), Value::Null);
    assert_eq!(gathered_float_nulls.get(1), Value::Float(3.5));

    // 3. Column::Bool without and with nulls
    let bool_col = Column::bools(vec![Some(true), Some(false), Some(true)]);
    let gathered_bool = bool_col.gather(&[0, 1]);
    assert_eq!(gathered_bool.len(), 2);
    assert_eq!(gathered_bool.get(0), Value::Bool(true));
    assert_eq!(gathered_bool.get(1), Value::Bool(false));
    assert!(gathered_bool.validity().is_none());

    let bool_nulls = Column::bools(vec![Some(true), None, Some(false)]);
    let gathered_bool_nulls = bool_nulls.gather(&[1, 2]);
    assert_eq!(gathered_bool_nulls.len(), 2);
    assert_eq!(gathered_bool_nulls.get(0), Value::Null);
    assert_eq!(gathered_bool_nulls.get(1), Value::Bool(false));

    // 4. Column::Text without and with nulls
    let text_col = Column::texts(vec![
        Some("alpha".into()),
        Some("beta".into()),
        Some("gamma".into()),
    ]);
    let gathered_text = text_col.gather(&[2, 0]);
    assert_eq!(gathered_text.len(), 2);
    assert_eq!(gathered_text.get(0), Value::Text("gamma"));
    assert_eq!(gathered_text.get(1), Value::Text("alpha"));
    assert!(gathered_text.validity().is_none());

    let text_nulls = Column::texts(vec![Some("alpha".into()), None, Some("gamma".into())]);
    let gathered_text_nulls = text_nulls.gather(&[1, 2]);
    assert_eq!(gathered_text_nulls.len(), 2);
    assert_eq!(gathered_text_nulls.get(0), Value::Null);
    assert_eq!(gathered_text_nulls.get(1), Value::Text("gamma"));

    // 5. Column::Dict keeps the dictionary
    let dict_col = Column::dict_from(vec![
        Some("zebra".into()),
        Some("apple".into()),
        Some("mango".into()),
    ]);
    let gathered_dict = dict_col.gather(&[1, 2, 0]);
    assert_eq!(gathered_dict.len(), 3);
    assert_eq!(gathered_dict.get(0), Value::Text("apple"));
    assert_eq!(gathered_dict.get(1), Value::Text("mango"));
    assert_eq!(gathered_dict.get(2), Value::Text("zebra"));
    if let (Column::Dict { dict: d1, .. }, Column::Dict { dict: d2, .. }) =
        (&dict_col, &gathered_dict)
    {
        assert_eq!(d1, d2);
    } else {
        panic!("expected dict column variant");
    }

    let dict_nulls = Column::dict_from(vec![Some("zebra".into()), None, Some("apple".into())]);
    let gathered_dict_nulls = dict_nulls.gather(&[0, 1, 2]);
    assert_eq!(gathered_dict_nulls.len(), 3);
    assert_eq!(gathered_dict_nulls.get(0), Value::Text("zebra"));
    assert_eq!(gathered_dict_nulls.get(1), Value::Null);
    assert_eq!(gathered_dict_nulls.get(2), Value::Text("apple"));
    if let (Column::Dict { dict: d1, .. }, Column::Dict { dict: d2, .. }) =
        (&dict_nulls, &gathered_dict_nulls)
    {
        assert_eq!(d1, d2);
    } else {
        panic!("expected dict column variant");
    }

    // Empty gather
    let empty_gathered = int_col.gather(&[]);
    assert_eq!(empty_gathered.len(), 0);
    assert!(empty_gathered.is_empty());
}

#[test]
fn test_schema_derivation_and_validation() {
    let mut store = Store::new();
    store
        .add("id", Column::ints(vec![Some(1), Some(2)]))
        .unwrap();
    store
        .add("score", Column::floats(vec![Some(10.5), None]))
        .unwrap();
    store
        .add("active", Column::bools(vec![Some(true), Some(false)]))
        .unwrap();
    store
        .add(
            "name",
            Column::texts(vec![Some("alice".into()), Some("bob".into())]),
        )
        .unwrap();
    store
        .add("tag", Column::dict_from(vec![Some("cat_a".into()), None]))
        .unwrap();

    let source = VecSource::from_store(store.clone());
    let schema = source.schema();
    assert_eq!(schema.len(), 5);

    assert_eq!(schema[0].name, "id");
    assert_eq!(schema[0].kind, ColumnKind::Int);
    assert!(!schema[0].nullable);
    assert_eq!(schema[0].hint, DisplayHint::Plain);

    assert_eq!(schema[1].name, "score");
    assert_eq!(schema[1].kind, ColumnKind::Float);
    assert!(schema[1].nullable);

    assert_eq!(schema[2].name, "active");
    assert_eq!(schema[2].kind, ColumnKind::Bool);
    assert!(!schema[2].nullable);

    assert_eq!(schema[3].name, "name");
    assert_eq!(schema[3].kind, ColumnKind::Text);
    assert!(!schema[3].nullable);

    // Dict derives as Text
    assert_eq!(schema[4].name, "tag");
    assert_eq!(schema[4].kind, ColumnKind::Text);
    assert!(schema[4].nullable);

    // VecSource::new with valid schema succeeds
    let manual_valid = VecSource::new(store.clone(), schema.to_vec());
    assert!(manual_valid.is_ok());

    // Mismatched schema: column not found
    let mut bad_schema = schema.to_vec();
    bad_schema[0].name = "non_existent".into();
    let res_not_found = VecSource::new(store.clone(), bad_schema);
    assert!(matches!(
        res_not_found,
        Err(SourceError::Store(StoreError::ColumnNotFound(_)))
    ));

    // Mismatched schema: wrong type
    let mut bad_type = schema.to_vec();
    bad_type[0].kind = ColumnKind::Float;
    let res_type = VecSource::new(store.clone(), bad_type);
    assert!(matches!(
        res_type,
        Err(SourceError::Store(StoreError::TypeMismatch { .. }))
    ));

    // Mismatched schema: duplicate column
    let mut dup_schema = schema.to_vec();
    dup_schema[1].name = dup_schema[0].name.clone();
    dup_schema[1].kind = dup_schema[0].kind;
    let res_dup = VecSource::new(store.clone(), dup_schema);
    assert!(matches!(
        res_dup,
        Err(SourceError::Store(StoreError::DuplicateColumn(_)))
    ));

    // Mismatched schema: length mismatch
    let truncated_schema = schema[0..4].to_vec();
    let res_len = VecSource::new(store, truncated_schema);
    assert!(matches!(
        res_len,
        Err(SourceError::Store(StoreError::LengthMismatch { .. }))
    ));
}

#[test]
fn test_fetch_windows_and_clamping() {
    let mut store = Store::new();
    let n = 100;
    let ids: Vec<Option<i64>> = (0..n).map(|i| Some(i as i64)).collect();
    let scores: Vec<Option<f64>> = (0..n).map(|i| Some((n - i) as f64)).collect();
    let tags: Vec<Option<String>> = (0..n).map(|i| Some(format!("tag_{}", i % 4))).collect();

    store.add("id", Column::ints(ids)).unwrap();
    store.add("score", Column::floats(scores)).unwrap();
    store.add("tag", Column::dict_from(tags)).unwrap();

    let query = Query {
        filter: Some(Filter::Gt("score".into(), Owned::Float(30.0))),
        sort: vec![SortKey {
            column: "score".into(),
            order: Order::Asc,
            nulls: Nulls::Last,
        }],
        group_by: Vec::new(),
        summary: Vec::new(),
    };

    let view = View::new(&store, query.clone()).unwrap();
    let expected_rows = view.rows();
    let total_matches = expected_rows.len();
    assert_eq!(total_matches, 70); // scores 31..=100

    let source = VecSource::from_store(store.clone());

    // count() matches
    assert_eq!(source.count(&query).unwrap(), total_matches);

    // 1. Start window
    let r_start = source.fetch(&query, 0..10).unwrap();
    assert_eq!(r_start.start, 0);
    assert_eq!(r_start.len, 10);
    assert_eq!(
        r_start.ids,
        expected_rows[0..10]
            .iter()
            .map(|&x| x as u64)
            .collect::<Vec<_>>()
    );
    for (i, &row) in expected_rows.iter().take(10).enumerate() {
        assert_eq!(r_start.columns[0].get(i), store.value(row as usize, 0));
        assert_eq!(r_start.columns[1].get(i), store.value(row as usize, 1));
    }

    // 2. Middle window
    let r_mid = source.fetch(&query, 20..35).unwrap();
    assert_eq!(r_mid.start, 20);
    assert_eq!(r_mid.len, 15);
    assert_eq!(
        r_mid.ids,
        expected_rows[20..35]
            .iter()
            .map(|&x| x as u64)
            .collect::<Vec<_>>()
    );
    for i in 0..15 {
        assert_eq!(
            r_mid.columns[0].get(i),
            store.value(expected_rows[20 + i] as usize, 0)
        );
    }

    // 3. End window
    let r_end = source
        .fetch(&query, (total_matches - 5)..total_matches)
        .unwrap();
    assert_eq!(r_end.start, total_matches - 5);
    assert_eq!(r_end.len, 5);
    assert_eq!(
        r_end.ids,
        expected_rows[(total_matches - 5)..total_matches]
            .iter()
            .map(|&x| x as u64)
            .collect::<Vec<_>>()
    );
    for i in 0..5 {
        assert_eq!(
            r_end.columns[0].get(i),
            store.value(expected_rows[total_matches - 5 + i] as usize, 0)
        );
    }

    // 4. Clamped beyond the end
    let r_clamped = source
        .fetch(&query, (total_matches - 3)..(total_matches + 20))
        .unwrap();
    assert_eq!(r_clamped.start, total_matches - 3);
    assert_eq!(r_clamped.len, 3);
    for i in 0..3 {
        assert_eq!(
            r_clamped.columns[0].get(i),
            store.value(expected_rows[total_matches - 3 + i] as usize, 0)
        );
    }

    // 5. Completely beyond the end
    let r_past = source
        .fetch(&query, (total_matches + 5)..(total_matches + 20))
        .unwrap();
    assert_eq!(r_past.len, 0);

    // 6. Empty range
    let r_empty = source.fetch(&query, 10..10).unwrap();
    assert_eq!(r_empty.start, 10);
    assert_eq!(r_empty.len, 0);

    // 7. Inverted range
    let (inv_start, inv_end) = (25, 10);
    let r_inverted = source.fetch(&query, inv_start..inv_end).unwrap();
    assert_eq!(r_inverted.len, 0);
}

#[test]
fn test_scrolling_cache_filter_sort_once() {
    let mut store = Store::new();
    let n = 250;
    store
        .add(
            "val",
            Column::ints((0..n).map(|i| Some(i as i64)).collect()),
        )
        .unwrap();

    let query = Query {
        filter: Some(Filter::Gt("val".into(), Owned::Int(50))),
        sort: vec![SortKey {
            column: "val".into(),
            order: Order::Desc,
            nulls: Nulls::Last,
        }],
        group_by: Vec::new(),
        summary: Vec::new(),
    };

    let source = VecSource::from_store(store);
    assert_eq!(source.execution_count(), 0);

    // Scroll 50 consecutive windows
    for i in 0..50 {
        let start = i * 4;
        let rows = source.fetch(&query, start..(start + 4)).unwrap();
        assert_eq!(rows.start, start);
    }

    // Only 1 execution was run because queries matched
    assert_eq!(source.execution_count(), 1);

    // Changing query triggers a new execution
    let mut query2 = query.clone();
    query2.sort[0].order = Order::Asc;
    let _ = source.fetch(&query2, 0..10).unwrap();
    assert_eq!(source.execution_count(), 2);

    // Subsequent fetches of query2 hit the cache again
    let _ = source.fetch(&query2, 10..20).unwrap();
    assert_eq!(source.execution_count(), 2);

    // Totals do not change the row order, so they reuse the cached permutation
    let mut query3 = query2.clone();
    query3.group_by.push(GroupLevel {
        column: "val".into(),
        totals: vec![Total {
            column: "val".into(),
            aggregate: Aggregate::Sum,
        }],
    });
    let _ = source.fetch(&query3, 20..30).unwrap();
    assert_eq!(source.execution_count(), 2);
}

#[test]
fn test_property_random_windows_and_fake_slow_source() {
    let seeds = [0x853c49e6748fea9b, 0x12345678abcdef01, 0xdeadbeefcafebabe];

    for seed in seeds {
        let mut rng = XorShift64::new(seed);
        let rows_count = 150;

        let mut store = Store::new();
        let ids: Vec<Option<i64>> = (0..rows_count).map(|i| Some(i as i64)).collect();
        let floats: Vec<Option<f64>> = (0..rows_count)
            .map(|_| {
                if rng.next_bool() {
                    Some(rng.next_range(1, 1000) as f64)
                } else {
                    None
                }
            })
            .collect();
        let dicts: Vec<Option<String>> = (0..rows_count)
            .map(|_| {
                let code = rng.next_range(0, 4);
                Some(format!("cat_{}", code))
            })
            .collect();

        store.add("id", Column::ints(ids)).unwrap();
        store.add("amount", Column::floats(floats)).unwrap();
        store.add("category", Column::dict_from(dicts)).unwrap();

        let query = Query {
            filter: Some(Filter::Gt("amount".into(), Owned::Float(200.0))),
            sort: vec![SortKey {
                column: "amount".into(),
                order: Order::Asc,
                nulls: Nulls::Last,
            }],
            group_by: Vec::new(),
            summary: Vec::new(),
        };

        let view = View::new(&store, query.clone()).unwrap();
        let expected_row_ids = view.rows();
        let expected_count = expected_row_ids.len();

        let base_source = VecSource::from_store(store.clone());
        let slow_source = FakeSlowSource::new(Box::new(base_source), Duration::from_millis(0));

        let count = slow_source.count(&query).unwrap();
        assert_eq!(count, expected_count);

        let mut offset = 0;
        let mut gathered_ids = Vec::new();
        let max_req = 20;

        while offset < expected_count {
            let win = rng.next_range(5, max_req as u64) as usize;
            let rows = slow_source.fetch(&query, offset..(offset + win)).unwrap();
            if rows.len == 0 {
                break;
            }
            if let Column::Int { values, .. } = &rows.columns[0] {
                gathered_ids.extend_from_slice(values);
            }
            offset += rows.len;
        }

        assert_eq!(gathered_ids.len(), expected_count);
        for (i, &expected_rid) in expected_row_ids.iter().enumerate() {
            assert_eq!(
                Value::Int(gathered_ids[i]),
                store.value(expected_rid as usize, 0)
            );
        }

        let stats = slow_source.stats();
        assert_eq!(stats.count_calls, 1);
        assert!(stats.fetch_calls > 0);
        assert_eq!(stats.total_rows_returned, expected_count);
        assert!(stats.max_window_requested <= max_req);
    }
}

#[test]
fn test_unsupported_capabilities() {
    let mut store = Store::new();
    store
        .add("id", Column::ints(vec![Some(1), Some(2), Some(3)]))
        .unwrap();

    let full_query = Query {
        filter: Some(Filter::Gt("id".into(), Owned::Int(1))),
        sort: vec![SortKey {
            column: "id".into(),
            order: Order::Asc,
            nulls: Nulls::Last,
        }],
        group_by: vec![GroupLevel {
            column: "id".into(),
            totals: vec![Total {
                column: "id".into(),
                aggregate: Aggregate::Count,
            }],
        }],
        summary: Vec::new(),
    };

    // 1. Reduced filter capability
    let no_filter = FakeSlowSource::with_capabilities(
        Box::new(VecSource::from_store(store.clone())),
        Duration::from_millis(0),
        Capabilities {
            filter: false,
            sort: true,
            group: true,
            totals: true,
            keyset: false,
        },
    );
    assert!(matches!(
        no_filter.count(&full_query),
        Err(SourceError::Unsupported("filter"))
    ));
    assert!(matches!(
        no_filter.fetch(&full_query, 0..10),
        Err(SourceError::Unsupported("filter"))
    ));

    // 2. Reduced sort capability
    let no_sort = FakeSlowSource::with_capabilities(
        Box::new(VecSource::from_store(store.clone())),
        Duration::from_millis(0),
        Capabilities {
            filter: true,
            sort: false,
            group: true,
            totals: true,
            keyset: false,
        },
    );
    let mut sort_query = Query::default();
    sort_query.sort.push(SortKey {
        column: "id".into(),
        order: Order::Asc,
        nulls: Nulls::Last,
    });
    assert!(matches!(
        no_sort.fetch(&sort_query, 0..10),
        Err(SourceError::Unsupported("sort"))
    ));

    // 3. Reduced group capability
    let no_group = FakeSlowSource::with_capabilities(
        Box::new(VecSource::from_store(store.clone())),
        Duration::from_millis(0),
        Capabilities {
            filter: true,
            sort: true,
            group: false,
            totals: true,
            keyset: false,
        },
    );
    let mut group_query = Query::default();
    group_query.group_by.push(GroupLevel::new("id"));
    assert!(matches!(
        no_group.fetch(&group_query, 0..10),
        Err(SourceError::Unsupported("group"))
    ));

    // 4. Reduced totals capability
    let no_totals = FakeSlowSource::with_capabilities(
        Box::new(VecSource::from_store(store)),
        Duration::from_millis(0),
        Capabilities {
            filter: true,
            sort: true,
            group: true,
            totals: false,
            keyset: false,
        },
    );
    let mut totals_query = Query::default();
    totals_query.group_by.push(GroupLevel {
        column: "id".into(),
        totals: vec![Total {
            column: "id".into(),
            aggregate: Aggregate::Count,
        }],
    });
    assert!(matches!(
        no_totals.fetch(&totals_query, 0..10),
        Err(SourceError::Unsupported("totals"))
    ));
}

#[test]
fn test_vec_source_summary() {
    let mut store = Store::new();
    store
        .add(
            "id",
            Column::ints(vec![Some(1), Some(2), Some(3), Some(4), Some(5)]),
        )
        .unwrap();
    store
        .add(
            "val",
            Column::floats(vec![Some(10.0), None, Some(30.0), Some(40.0), None]),
        )
        .unwrap();

    let source = VecSource::from_store(store);

    let query = Query {
        summary: vec![
            Total {
                column: "id".into(),
                aggregate: Aggregate::Count,
            },
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
        ],
        ..Default::default()
    };

    let totals = source.summary(&query).unwrap();
    assert_eq!(totals[0], Some(5.0));
    assert_eq!(totals[1], Some(80.0));
    assert_eq!(totals[2], Some(80.0 / 3.0));
    assert_eq!(totals[3], Some(10.0));
    assert_eq!(totals[4], Some(40.0));

    let mut filtered_query = query.clone();
    filtered_query.filter = Some(Filter::Le("id".into(), Owned::Int(3)));

    let filtered_totals = source.summary(&filtered_query).unwrap();
    assert_eq!(filtered_totals[0], Some(3.0));
    assert_eq!(filtered_totals[1], Some(40.0));
    assert_eq!(filtered_totals[2], Some(20.0));
    assert_eq!(filtered_totals[3], Some(10.0));
    assert_eq!(filtered_totals[4], Some(30.0));
}

#[test]
fn test_vec_source_distinct() {
    let mut store = Store::new();
    store
        .add(
            "id",
            Column::ints(vec![Some(1), Some(2), Some(3), Some(4), Some(5), Some(6)]),
        )
        .unwrap();
    store
        .add(
            "city",
            Column::texts(vec![
                Some("Paris".into()),
                None,
                Some("Berlin".into()),
                Some("Paris".into()),
                None,
                Some("Amsterdam".into()),
            ]),
        )
        .unwrap();

    let source = VecSource::from_store(store);

    // 1. Without filter: distinct values sorted, NULL first
    let q = Query::default();
    let distinct_all = source.distinct(&q, "city", 10).unwrap();
    assert_eq!(
        distinct_all,
        vec![
            None,
            Some("Amsterdam".into()),
            Some("Berlin".into()),
            Some("Paris".into()),
        ]
    );

    // 2. Limit respected
    let distinct_limited = source.distinct(&q, "city", 2).unwrap();
    assert_eq!(distinct_limited, vec![None, Some("Amsterdam".into())]);

    // 3. Filter respected (and accepts Owned::Float on Int id)
    let filtered_q = Query {
        filter: Some(Filter::Le("id".into(), Owned::Float(3.0))),
        ..Default::default()
    };
    let distinct_filtered = source.distinct(&filtered_q, "city", 10).unwrap();
    assert_eq!(
        distinct_filtered,
        vec![None, Some("Berlin".into()), Some("Paris".into())]
    );
}
