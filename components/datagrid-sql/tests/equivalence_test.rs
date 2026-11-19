use datagrid::column::{Column, Value};
use datagrid::filter::{Filter, Owned};
use datagrid::group::Total;
use datagrid::query::Query;
use datagrid::sort::{Nulls, Order, SortKey};
use datagrid::source::{ColumnInfo, ColumnKind, DisplayHint, RowSource, SourceError, VecSource};
use datagrid::store::{Store, StoreError};
use datagrid_sql::source::SqlSource;
use datagrid_sql::translate::Table;
use portadb::connection::Connection;
use portadb::drivers::RusqliteDriver;
use portadb::options::ConnectOptions;
use portadb::params::NamedParams;

struct XorShift64(u64);

impl XorShift64 {
    fn new(seed: u64) -> Self {
        Self(if seed == 0 { 0x853c49e6748fea9b } else { seed })
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

struct Dataset {
    vec_source: VecSource,
    sql_source: SqlSource,
    num_rows: usize,
}

// Generates a store with Int, Float, Bool, Dict-text columns and ~10% nulls.
// Note: IEEE NaN is intentionally excluded because SQLite converts NaN to NULL;
// tests therefore compare non-NaN real values.
// ASCII-only strings are used to match SQLite's default ASCII-folding lower() function.
fn build_dataset(seed: u64) -> Dataset {
    let mut rng = XorShift64::new(seed);
    let num_rows = rng.next_range(80, 250) as usize;

    let ascii_words = [
        // mixed case on purpose: case-sensitive and case-insensitive text filters must differ
        "alpha", "Alpine", "BRAVO", "charlie", "Delta", "ECHO", "foxtrot", "Golf", "HOTEL", "india",
        "Juliet", "kilo", "lima",
    ];

    let mut col_int = Vec::with_capacity(num_rows);
    let mut col_float = Vec::with_capacity(num_rows);
    let mut col_bool = Vec::with_capacity(num_rows);
    let mut col_dict = Vec::with_capacity(num_rows);

    for _ in 0..num_rows {
        // ~10% nulls
        let v_int = if rng.next_range(0, 9) == 0 {
            None
        } else {
            Some(rng.next_range(0, 1000) as i64 - 500)
        };

        let v_float = if rng.next_range(0, 9) == 0 {
            None
        } else {
            let raw = (rng.next_range(0, 20_000) as f64 - 10_000.0) / 10.0;
            Some(raw)
        };

        let v_bool = if rng.next_range(0, 9) == 0 {
            None
        } else {
            Some(rng.next_bool())
        };

        let v_dict = if rng.next_range(0, 9) == 0 {
            None
        } else {
            let idx = rng.next_range(0, ascii_words.len() as u64 - 1) as usize;
            Some(ascii_words[idx].to_string())
        };

        col_int.push(v_int);
        col_float.push(v_float);
        col_bool.push(v_bool);
        col_dict.push(v_dict);
    }

    let mut store = Store::new();
    store.add("c_int", Column::ints(col_int.clone())).unwrap();
    store
        .add("c_float", Column::floats(col_float.clone()))
        .unwrap();
    store
        .add("c_bool", Column::bools(col_bool.clone()))
        .unwrap();
    store
        .add("c_dict", Column::dict_from(col_dict.clone()))
        .unwrap();

    let schema = vec![
        ColumnInfo {
            name: "c_int".into(),
            kind: ColumnKind::Int,
            nullable: true,
            hint: DisplayHint::Plain,
        },
        ColumnInfo {
            name: "c_float".into(),
            kind: ColumnKind::Float,
            nullable: true,
            hint: DisplayHint::Plain,
        },
        ColumnInfo {
            name: "c_bool".into(),
            kind: ColumnKind::Bool,
            nullable: true,
            hint: DisplayHint::Plain,
        },
        ColumnInfo {
            name: "c_dict".into(),
            kind: ColumnKind::Text,
            nullable: true,
            hint: DisplayHint::Plain,
        },
    ];

    let vec_source = VecSource::new(store, schema.clone()).unwrap();

    // Populate in-memory SQLite in store order
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();

    // Each dataset gets its own named shared in-memory database so parallel
    // tests do not collide.
    let options = ConnectOptions::new(format!(
        "file:datagrid_sql_eq_{seed:x}?mode=memory&cache=shared"
    ));
    let mut conn = rt
        .block_on(async { Connection::open(RusqliteDriver::new(), &options).await })
        .unwrap();

    rt.block_on(async {
        conn.execute(
            "CREATE TABLE test_data (c_int INTEGER, c_float REAL, c_bool INTEGER, c_dict TEXT);",
            &NamedParams::new(),
        )
        .await
        .unwrap();

        for r in 0..num_rows {
            let mut params = NamedParams::new();
            match col_int[r] {
                Some(v) => params.insert("c_int", v),
                None => params.insert("c_int", portadb::value::Value::Null),
            }
            match col_float[r] {
                Some(v) => params.insert("c_float", v),
                None => params.insert("c_float", portadb::value::Value::Null),
            }
            match col_bool[r] {
                Some(v) => params.insert("c_bool", if v { 1i64 } else { 0i64 }),
                None => params.insert("c_bool", portadb::value::Value::Null),
            }
            match &col_dict[r] {
                Some(v) => params.insert("c_dict", v.clone()),
                None => params.insert("c_dict", portadb::value::Value::Null),
            }
            conn.execute(
                "INSERT INTO test_data (c_int, c_float, c_bool, c_dict) VALUES (:c_int, :c_float, :c_bool, :c_dict);",
                &params,
            )
            .await
            .unwrap();
        }
    });

    let table = Table::sqlite("test_data", schema);
    let sql_source = SqlSource::from_connection(conn, table).unwrap();

    Dataset {
        vec_source,
        sql_source,
        num_rows,
    }
}

#[test]
fn test_equivalence_200_random_queries() {
    let dataset = build_dataset(0x5EED_2026);
    let vec_src = &dataset.vec_source;
    let sql_src = &dataset.sql_source;
    let num_rows = dataset.num_rows;

    let mut rng = XorShift64::new(0xABCD_EF01);

    for query_idx in 0..200 {
        let mut q = Query::new();

        // 1. Random sort (0 to 3 keys)
        let num_sort_keys = rng.next_range(0, 3) as usize;
        let col_choices = ["c_int", "c_float", "c_bool", "c_dict"];
        for _ in 0..num_sort_keys {
            let c_name = col_choices[rng.next_range(0, 3) as usize];
            let order = if rng.next_bool() {
                Order::Asc
            } else {
                Order::Desc
            };
            let nulls = if rng.next_bool() {
                Nulls::First
            } else {
                Nulls::Last
            };
            q.sort.push(SortKey {
                column: c_name.into(),
                order,
                nulls,
            });
        }

        // 2. Random filter
        let filter_kind = rng.next_range(0, 15);
        let filter = match filter_kind {
            0 => None,
            1 => Some(Filter::Eq(
                "c_int".into(),
                Owned::Int(rng.next_range(0, 100) as i64 - 50),
            )),
            2 => Some(Filter::Gt("c_int".into(), Owned::Int(0))),
            3 => Some(Filter::Lt("c_float".into(), Owned::Float(50.0))),
            4 => Some(Filter::Between(
                "c_int".into(),
                Owned::Int(-200),
                Owned::Int(200),
            )),
            5 => Some(Filter::IsNull("c_int".into())),
            6 => Some(Filter::IsNull("c_float".into())),
            7 => Some(Filter::Eq("c_bool".into(), Owned::Bool(rng.next_bool()))),
            8 => Some(if rng.next_bool() {
                Filter::StartsWith {
                    column: "c_dict".into(),
                    text: "al".into(),
                    ignore_case: false,
                }
            } else {
                Filter::StartsWith {
                    column: "c_dict".into(),
                    text: "AL".into(),
                    ignore_case: true,
                }
            }),
            9 => Some(Filter::Contains {
                column: "c_dict".into(),
                text: "ho".into(),
                ignore_case: true,
            }),
            10 => Some(Filter::Not(Box::new(Filter::Eq(
                "c_int".into(),
                Owned::Int(10),
            )))),
            11 => Some(Filter::Not(Box::new(Filter::IsNull("c_bool".into())))),
            12 => Some(Filter::In(
                "c_dict".into(),
                vec![Owned::Text("alpha".into()), Owned::Text("delta".into())],
            )),
            13 => Some(Filter::And(vec![
                Filter::Gt("c_int".into(), Owned::Int(-100)),
                Filter::Lt("c_float".into(), Owned::Float(200.0)),
            ])),
            14 => Some(Filter::eq_ignore_case(
                "c_dict",
                if rng.next_bool() { "ALPHA" } else { "hotel" },
            )),
            _ => None,
        };
        q.filter = filter;

        // Grouping: level 0 and level 1
        q = q
            .group("c_dict")
            .with_total(Total::count("c_int"))
            .with_total(Total::sum("c_float"))
            .with_total(Total::avg("c_float"))
            .with_total(Total::min("c_int"))
            .with_total(Total::max("c_int"))
            .group("c_bool")
            .with_total(Total::count("c_int"))
            .with_total(Total::sum("c_float"));

        // A. Count equivalence
        let count_vec = vec_src.count(&q).unwrap();
        let count_sql = sql_src.count(&q).unwrap();
        assert_eq!(
            count_vec, count_sql,
            "count mismatch on query #{query_idx}: {count_vec} vs {count_sql}"
        );

        // B. Fetch several windows: start, middle, end, past end, and random jumps
        let rand_p1 = rng.next_range(0, count_vec as u64 + 10) as usize;
        let rand_len1 = rng.next_range(1, 25) as usize;
        let rand_p2 = rng.next_range(0, count_vec as u64 + 10) as usize;
        let rand_len2 = rng.next_range(1, 25) as usize;
        let rand_p3 = rng.next_range(0, count_vec as u64 + 10) as usize;
        let rand_len3 = rng.next_range(1, 25) as usize;

        let windows = [
            0..20,
            (num_rows / 3)..(num_rows / 3 + 20),
            (num_rows.saturating_sub(25))..num_rows,
            (num_rows + 50)..(num_rows + 100),
            rand_p1..(rand_p1 + rand_len1),
            rand_p2..(rand_p2 + rand_len2),
            rand_p3..(rand_p3 + rand_len3),
        ];

        for w in windows {
            let vec_rows = vec_src.fetch(&q, w.clone()).unwrap();
            let sql_rows = sql_src.fetch(&q, w.clone()).unwrap();

            assert_eq!(vec_rows.start, sql_rows.start);
            assert_eq!(vec_rows.len, sql_rows.len);

            // Rowid mapping: SQLite rowid starts at 1, store ids are 0-based
            let mapped_sql_ids: Vec<u64> = sql_rows
                .ids
                .iter()
                .map(|&id| id.saturating_sub(1))
                .collect();
            assert_eq!(
                mapped_sql_ids, vec_rows.ids,
                "ids mismatch on query #{query_idx} for window {w:?}"
            );

            for c_idx in 0..vec_src.schema().len() {
                let col_vec = &vec_rows.columns[c_idx];
                let col_sql = &sql_rows.columns[c_idx];
                assert_eq!(col_vec.len(), col_sql.len());

                for r in 0..vec_rows.len {
                    let val_vec = col_vec.get(r);
                    let val_sql = col_sql.get(r);
                    match (val_vec, val_sql) {
                        (Value::Null, Value::Null) => {}
                        (Value::Int(a), Value::Int(b)) => assert_eq!(a, b),
                        (Value::Float(a), Value::Float(b)) => {
                            let diff = (a - b).abs();
                            assert!(
                                diff <= 1e-9 * a.abs().max(b.abs()).max(1.0),
                                "float mismatch {a} vs {b}"
                            );
                        }
                        (Value::Bool(a), Value::Bool(b)) => assert_eq!(a, b),
                        (Value::Text(a), Value::Text(b)) => assert_eq!(a, b),
                        _ => panic!(
                            "cell mismatch at col {c_idx}, row {r}: {val_vec:?} vs {val_sql:?}"
                        ),
                    }
                }
            }
        }

        // C. Scrolling consecutive windows uses keyset after the first and gives identical rows
        sql_src.reset_counters();
        let chunk_size = 15;
        let w0 = 0..chunk_size;
        let w1 = chunk_size..(chunk_size * 2);

        let rows_w0 = sql_src.fetch(&q, w0.clone()).unwrap();
        if rows_w0.len == chunk_size {
            let offset_calls_before = sql_src.offset_fetches();
            let keyset_calls_before = sql_src.keyset_fetches();

            let rows_w1_keyset = sql_src.fetch(&q, w1.clone()).unwrap();
            let keyset_calls_after = sql_src.keyset_fetches();

            assert!(
                keyset_calls_after > keyset_calls_before,
                "consecutive scrolling must trigger keyset fetch after first window"
            );

            // Compare rows_w1_keyset with VecSource's rows
            let rows_w1_vec = vec_src.fetch(&q, w1.clone()).unwrap();
            assert_eq!(rows_w1_keyset.len, rows_w1_vec.len);
            let mapped_keyset_ids: Vec<u64> = rows_w1_keyset
                .ids
                .iter()
                .map(|&id| id.saturating_sub(1))
                .collect();
            assert_eq!(mapped_keyset_ids, rows_w1_vec.ids);
            let _ = offset_calls_before;
        }

        // D. Groups level 0 and level 1 equal
        let vec_g0 = vec_src.groups(&q, &[]).unwrap();
        let sql_g0 = sql_src.groups(&q, &[]).unwrap();

        assert_eq!(
            vec_g0.len(),
            sql_g0.len(),
            "level 0 groups count mismatch on query #{query_idx}"
        );

        for (g_idx, (vg, sg)) in vec_g0.iter().zip(sql_g0.iter()).enumerate() {
            assert_eq!(
                vg.key, sg.key,
                "group key mismatch at level 0, item #{g_idx}"
            );
            assert_eq!(
                vg.row_count, sg.row_count,
                "group row_count mismatch at level 0, item #{g_idx}"
            );
            assert_eq!(vg.totals.len(), sg.totals.len());

            for (t_idx, (vt, st)) in vg.totals.iter().zip(sg.totals.iter()).enumerate() {
                match (vt, st) {
                    (None, None) => {}
                    (Some(a), Some(b)) => {
                        let diff = (a - b).abs();
                        assert!(
                            diff <= 1e-9 * a.abs().max(b.abs()).max(1.0),
                            "total mismatch on level 0, group #{g_idx}, total #{t_idx}: {a} vs {b}"
                        );
                    }
                    _ => panic!(
                        "total nullability mismatch at level 0, group #{g_idx}, total #{t_idx}: {vt:?} vs {st:?}"
                    ),
                }
            }

            // Level 1 groups under this level 0 parent
            let parent_path = &[vg.key[0].clone()];
            let vec_g1 = vec_src.groups(&q, parent_path).unwrap();
            let sql_g1 = sql_src.groups(&q, parent_path).unwrap();

            assert_eq!(
                vec_g1.len(),
                sql_g1.len(),
                "level 1 groups count mismatch for parent {:?}",
                parent_path
            );

            for (vg1, sg1) in vec_g1.iter().zip(sql_g1.iter()) {
                assert_eq!(vg1.key, sg1.key);
                assert_eq!(vg1.row_count, sg1.row_count);
                assert_eq!(vg1.totals.len(), sg1.totals.len());
                for (vt, st) in vg1.totals.iter().zip(sg1.totals.iter()) {
                    match (vt, st) {
                        (None, None) => {}
                        (Some(a), Some(b)) => {
                            let diff = (a - b).abs();
                            assert!(diff <= 1e-9 * a.abs().max(b.abs()).max(1.0));
                        }
                        _ => panic!("total mismatch at level 1"),
                    }
                }
            }
        }

        // E. position_of of random id equals VecSource
        let rand_r = rng.next_range(0, num_rows as u64 - 1);
        let pos_vec = vec_src.position_of(&q, rand_r).unwrap();
        let pos_sql = sql_src.position_of(&q, rand_r + 1).unwrap();
        assert_eq!(
            pos_vec, pos_sql,
            "position_of mismatch on query #{query_idx} for row {rand_r}"
        );
        let out_of_bounds_r = num_rows as u64 + 999;
        assert_eq!(vec_src.position_of(&q, out_of_bounds_r).unwrap(), None);
        assert_eq!(sql_src.position_of(&q, out_of_bounds_r + 1).unwrap(), None);

        // F. locate of random filter equals VecSource
        let loc_kind = rng.next_range(1, 15);
        let loc_filter = match loc_kind {
            1 => Filter::Eq(
                "c_int".into(),
                Owned::Int(rng.next_range(0, 100) as i64 - 50),
            ),
            2 => Filter::Gt("c_int".into(), Owned::Int(0)),
            3 => Filter::Lt("c_float".into(), Owned::Float(50.0)),
            4 => Filter::Between("c_int".into(), Owned::Int(-200), Owned::Int(200)),
            5 => Filter::IsNull("c_int".into()),
            6 => Filter::IsNull("c_float".into()),
            7 => Filter::Eq("c_bool".into(), Owned::Bool(rng.next_bool())),
            8 => Filter::StartsWith {
                column: "c_dict".into(),
                text: "al".into(),
                ignore_case: false,
            },
            9 => Filter::Contains {
                column: "c_dict".into(),
                text: "ho".into(),
                ignore_case: true,
            },
            10 => Filter::Not(Box::new(Filter::Eq("c_int".into(), Owned::Int(10)))),
            11 => Filter::Not(Box::new(Filter::IsNull("c_bool".into()))),
            12 => Filter::In(
                "c_dict".into(),
                vec![Owned::Text("alpha".into()), Owned::Text("delta".into())],
            ),
            13 => Filter::And(vec![
                Filter::Gt("c_int".into(), Owned::Int(-100)),
                Filter::Lt("c_float".into(), Owned::Float(200.0)),
            ]),
            14 => Filter::eq_ignore_case("c_dict", if rng.next_bool() { "ALPHA" } else { "hotel" }),
            _ => Filter::IsNull("c_int".into()),
        };
        let loc_vec = vec_src.locate(&q, &loc_filter).unwrap();
        let loc_sql = sql_src.locate(&q, &loc_filter).unwrap();
        match (loc_vec, loc_sql) {
            (None, None) => {}
            (Some((p_vec, id_vec)), Some((p_sql, id_sql))) => {
                assert_eq!(
                    p_vec, p_sql,
                    "locate position mismatch on query #{query_idx}"
                );
                assert_eq!(
                    id_vec,
                    id_sql.saturating_sub(1),
                    "locate id mismatch on query #{query_idx}"
                );
            }
            (v, s) => panic!("locate mismatch on query #{query_idx}: vec={v:?}, sql={s:?}"),
        }
        let missing_filter = Filter::Eq("c_int".into(), Owned::Int(999_999_999));
        assert_eq!(vec_src.locate(&q, &missing_filter).unwrap(), None);
        assert_eq!(sql_src.locate(&q, &missing_filter).unwrap(), None);
    }
}

#[test]
fn test_query_rejection_equivalence() {
    let dataset = build_dataset(0x1337);
    let vec_src = &dataset.vec_source;
    let sql_src = &dataset.sql_source;

    // 1. Column not found in sort
    let bad_sort_query = Query::new().sort_asc("non_existent_column");
    let vec_res = vec_src.fetch(&bad_sort_query, 0..10);
    let sql_res = sql_src.fetch(&bad_sort_query, 0..10);
    assert_eq!(
        vec_res.unwrap_err(),
        SourceError::Store(StoreError::ColumnNotFound("non_existent_column".into()))
    );
    assert_eq!(
        sql_res.unwrap_err(),
        SourceError::Store(StoreError::ColumnNotFound("non_existent_column".into()))
    );

    // 2. Column not found in filter
    let bad_filter_query = Query::new().filter(Filter::Eq("missing_column".into(), Owned::Int(1)));
    let vec_res = vec_src.count(&bad_filter_query);
    let sql_res = sql_src.count(&bad_filter_query);
    assert_eq!(
        vec_res.unwrap_err(),
        SourceError::Store(StoreError::ColumnNotFound("missing_column".into()))
    );
    assert_eq!(
        sql_res.unwrap_err(),
        SourceError::Store(StoreError::ColumnNotFound("missing_column".into()))
    );

    // 3. Type mismatch in total aggregation (Sum over text column)
    // (grouped by an Int column: the in-memory engine cannot group a plain-text
    // column at all, so grouping by `c_dict` would fail there for another reason)
    let bad_total_query = Query::new().group("c_int").with_total(Total::sum("c_dict"));
    let vec_res = vec_src.groups(&bad_total_query, &[]);
    let sql_res = sql_src.groups(&bad_total_query, &[]);
    assert_eq!(
        vec_res.unwrap_err(),
        SourceError::Store(StoreError::TypeMismatch {
            column: "c_dict".into(),
            expected: "Int or Float",
        })
    );
    assert_eq!(
        sql_res.unwrap_err(),
        SourceError::Store(StoreError::TypeMismatch {
            column: "c_dict".into(),
            expected: "Int or Float",
        })
    );
}
