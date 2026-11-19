use crate::column::{Column, Value};
use crate::filter::{Filter, Owned};
use crate::group::{Aggregate, Total};
use crate::sort::{Nulls, Order, SortKey};
use crate::store::{Store, StoreError};
use crate::view::{GroupLevel, Query, ReuseStats, View};
use std::cmp::Ordering;
use std::collections::BTreeMap;

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

#[derive(Clone, Debug, PartialEq)]
enum NaiveVal {
    Null,
    Int(i64),
    Float(f64),
    Bool(bool),
    Text(String),
}

fn cmp_floats(a: f64, b: f64) -> Ordering {
    let fa = if a == 0.0 { 0.0 } else { a };
    let fb = if b == 0.0 { 0.0 } else { b };
    if fa.is_nan() && fb.is_nan() {
        Ordering::Equal
    } else if fa.is_nan() {
        Ordering::Greater
    } else if fb.is_nan() {
        Ordering::Less
    } else {
        let ba = fa.to_bits();
        let bb = fb.to_bits();
        let oa = if (ba >> 63) == 1 { !ba } else { ba | (1 << 63) };
        let ob = if (bb >> 63) == 1 { !bb } else { bb | (1 << 63) };
        oa.cmp(&ob)
    }
}

fn cmp_val(a: &NaiveVal, b: &NaiveVal, order: Order, nulls: Nulls) -> Ordering {
    match (a, b) {
        (NaiveVal::Null, NaiveVal::Null) => Ordering::Equal,
        (NaiveVal::Null, _) => match nulls {
            Nulls::First => Ordering::Less,
            Nulls::Last => Ordering::Greater,
        },
        (_, NaiveVal::Null) => match nulls {
            Nulls::First => Ordering::Greater,
            Nulls::Last => Ordering::Less,
        },
        _ => {
            let ord = match (a, b) {
                (NaiveVal::Int(x), NaiveVal::Int(y)) => x.cmp(y),
                (NaiveVal::Float(x), NaiveVal::Float(y)) => cmp_floats(*x, *y),
                (NaiveVal::Bool(x), NaiveVal::Bool(y)) => x.cmp(y),
                (NaiveVal::Text(x), NaiveVal::Text(y)) => x.as_bytes().cmp(y.as_bytes()),
                _ => Ordering::Equal,
            };
            match order {
                Order::Asc => ord,
                Order::Desc => ord.reverse(),
            }
        }
    }
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
enum NaiveValOrd {
    Bool(bool),
    Dict(String),
    Int(i64),
    Float(u64),
}

fn to_naive_ord(v: &NaiveVal) -> Option<NaiveValOrd> {
    match v {
        NaiveVal::Null => None,
        NaiveVal::Bool(b) => Some(NaiveValOrd::Bool(*b)),
        NaiveVal::Text(s) => Some(NaiveValOrd::Dict(s.clone())),
        NaiveVal::Int(i) => Some(NaiveValOrd::Int(*i)),
        NaiveVal::Float(f) => {
            let mut val = *f;
            if val == 0.0 {
                val = 0.0;
            }
            if val.is_nan() {
                Some(NaiveValOrd::Float(u64::MAX))
            } else {
                let b = val.to_bits();
                let ord = if (b >> 63) == 1 { !b } else { b | (1 << 63) };
                Some(NaiveValOrd::Float(ord))
            }
        }
    }
}

struct TestDataset {
    store: Store,
    naive_rows: Vec<Vec<NaiveVal>>,
    col_names: Vec<String>,
}

fn generate_dataset(seed: u64) -> TestDataset {
    let mut rng = XorShift64::new(seed);
    let num_rows = rng.next_range(10, 150) as usize;

    let int_edges = [i64::MIN, i64::MIN + 1, i64::MAX - 1, i64::MAX, 0, -1];
    let float_edges = [-0.0, 0.0, f64::INFINITY, f64::NEG_INFINITY, f64::NAN, 1.5];
    let dict_words = ["apple", "banana", "cherry", "date", "fig"];
    let text_edges = ["text_a", "text_b", "text_c"];

    let mut col_int = Vec::with_capacity(num_rows);
    let mut col_float = Vec::with_capacity(num_rows);
    let mut col_bool = Vec::with_capacity(num_rows);
    let mut col_dict = Vec::with_capacity(num_rows);
    let mut col_text = Vec::with_capacity(num_rows);

    let mut naive_rows = Vec::with_capacity(num_rows);

    for r in 0..num_rows {
        let v_int = match rng.next_range(0, 6) {
            0 => None,
            1..=2 => {
                let edge_idx = rng.next_range(0, int_edges.len() as u64 - 1) as usize;
                Some(int_edges[edge_idx])
            }
            _ => {
                let raw = rng.next_range(0, 2000) as i64 - 1000;
                Some(raw)
            }
        };
        col_int.push(v_int);

        let v_float = match rng.next_range(0, 7) {
            0 => None,
            1..=2 => {
                let edge_idx = rng.next_range(0, float_edges.len() as u64 - 1) as usize;
                Some(float_edges[edge_idx])
            }
            _ => {
                let raw = (rng.next_range(0, 4000) as f64 - 2000.0) / 10.0;
                Some(raw)
            }
        };
        col_float.push(v_float);

        let v_bool = if rng.next_range(0, 4) == 0 {
            None
        } else {
            Some(rng.next_bool())
        };
        col_bool.push(v_bool);

        let v_dict = if rng.next_range(0, 4) == 0 {
            None
        } else {
            let idx = rng.next_range(0, dict_words.len() as u64 - 1) as usize;
            Some(dict_words[idx].to_string())
        };
        col_dict.push(v_dict.clone());

        let v_text = if rng.next_range(0, 4) == 0 {
            None
        } else {
            let idx = rng.next_range(0, text_edges.len() as u64 - 1) as usize;
            Some(format!("{}_{}", text_edges[idx], r % 5))
        };
        col_text.push(v_text.clone());

        naive_rows.push(vec![
            v_int.map_or(NaiveVal::Null, NaiveVal::Int),
            v_float.map_or(NaiveVal::Null, NaiveVal::Float),
            v_bool.map_or(NaiveVal::Null, NaiveVal::Bool),
            v_dict.map_or(NaiveVal::Null, NaiveVal::Text),
            v_text.map_or(NaiveVal::Null, NaiveVal::Text),
        ]);
    }

    let mut store = Store::new();
    store
        .add("c_int", Column::ints(col_int))
        .expect("add c_int");
    store
        .add("c_float", Column::floats(col_float))
        .expect("add c_float");
    store
        .add("c_bool", Column::bools(col_bool))
        .expect("add c_bool");
    store
        .add("c_dict", Column::dict_from(col_dict))
        .expect("add c_dict");
    store
        .add("c_text", Column::texts(col_text))
        .expect("add c_text");

    TestDataset {
        store,
        naive_rows,
        col_names: vec![
            "c_int".into(),
            "c_float".into(),
            "c_bool".into(),
            "c_dict".into(),
            "c_text".into(),
        ],
    }
}

fn eval_naive_filter(f: &Filter, row: &[NaiveVal], col_names: &[String]) -> bool {
    match f {
        Filter::And(list) => list
            .iter()
            .all(|child| eval_naive_filter(child, row, col_names)),
        Filter::Or(list) => list
            .iter()
            .any(|child| eval_naive_filter(child, row, col_names)),
        Filter::Not(child) => !eval_naive_filter(child, row, col_names),
        Filter::IsNull(col) => {
            let idx = col_names.iter().position(|n| n == col).unwrap();
            row[idx] == NaiveVal::Null
        }
        Filter::Eq(col, target) => {
            let idx = col_names.iter().position(|n| n == col).unwrap();
            match (&row[idx], target) {
                (NaiveVal::Int(v), Owned::Int(t)) => v == t,
                (NaiveVal::Float(v), Owned::Float(t)) => v == t,
                (NaiveVal::Bool(v), Owned::Bool(t)) => v == t,
                (NaiveVal::Text(v), Owned::Text(t)) => v == t,
                _ => false,
            }
        }
        Filter::Lt(col, target) => {
            let idx = col_names.iter().position(|n| n == col).unwrap();
            match (&row[idx], target) {
                (NaiveVal::Int(v), Owned::Int(t)) => v < t,
                (NaiveVal::Float(v), Owned::Float(t)) => v < t,
                (NaiveVal::Bool(v), Owned::Bool(t)) => !v && *t,
                (NaiveVal::Text(v), Owned::Text(t)) => v < t,
                _ => false,
            }
        }
        Filter::Le(col, target) => {
            let idx = col_names.iter().position(|n| n == col).unwrap();
            match (&row[idx], target) {
                (NaiveVal::Int(v), Owned::Int(t)) => v <= t,
                (NaiveVal::Float(v), Owned::Float(t)) => v <= t,
                (NaiveVal::Bool(v), Owned::Bool(t)) => !v || *t,
                (NaiveVal::Text(v), Owned::Text(t)) => v <= t,
                _ => false,
            }
        }
        Filter::Gt(col, target) => {
            let idx = col_names.iter().position(|n| n == col).unwrap();
            match (&row[idx], target) {
                (NaiveVal::Int(v), Owned::Int(t)) => v > t,
                (NaiveVal::Float(v), Owned::Float(t)) => v > t,
                (NaiveVal::Bool(v), Owned::Bool(t)) => *v && !*t,
                (NaiveVal::Text(v), Owned::Text(t)) => v > t,
                _ => false,
            }
        }
        Filter::Ge(col, target) => {
            let idx = col_names.iter().position(|n| n == col).unwrap();
            match (&row[idx], target) {
                (NaiveVal::Int(v), Owned::Int(t)) => v >= t,
                (NaiveVal::Float(v), Owned::Float(t)) => v >= t,
                (NaiveVal::Bool(v), Owned::Bool(t)) => *v || !*t,
                (NaiveVal::Text(v), Owned::Text(t)) => v >= t,
                _ => false,
            }
        }
        Filter::Between(col, lo, hi) => {
            let idx = col_names.iter().position(|n| n == col).unwrap();
            match (&row[idx], lo, hi) {
                (NaiveVal::Int(v), Owned::Int(l), Owned::Int(h)) => v >= l && v <= h,
                (NaiveVal::Float(v), Owned::Float(l), Owned::Float(h)) => v >= l && v <= h,
                (NaiveVal::Bool(v), Owned::Bool(l), Owned::Bool(h)) => {
                    let ge_l = *v || !*l;
                    let le_h = !*v || *h;
                    ge_l && le_h
                }
                (NaiveVal::Text(v), Owned::Text(l), Owned::Text(h)) => v >= l && v <= h,
                _ => false,
            }
        }
        Filter::In(col, list) => {
            let idx = col_names.iter().position(|n| n == col).unwrap();
            match &row[idx] {
                NaiveVal::Int(v) => list.iter().any(|t| matches!(t, Owned::Int(x) if x == v)),
                NaiveVal::Float(v) => list.iter().any(|t| matches!(t, Owned::Float(x) if x == v)),
                NaiveVal::Bool(v) => list.iter().any(|t| matches!(t, Owned::Bool(x) if x == v)),
                NaiveVal::Text(v) => list.iter().any(|t| matches!(t, Owned::Text(x) if x == v)),
                NaiveVal::Null => false,
            }
        }
        Filter::Contains {
            column,
            text,
            ignore_case,
        } => {
            let idx = col_names.iter().position(|n| n == column).unwrap();
            if let NaiveVal::Text(s) = &row[idx] {
                if *ignore_case {
                    s.to_lowercase().contains(&text.to_lowercase())
                } else {
                    s.contains(text)
                }
            } else {
                false
            }
        }
        Filter::EqIgnoreCase { column, text } => {
            let idx = col_names.iter().position(|n| n == column).unwrap();
            if let NaiveVal::Text(s) = &row[idx] {
                s.to_lowercase() == text.to_lowercase()
            } else {
                false
            }
        }
        Filter::StartsWith {
            column,
            text,
            ignore_case,
        } => {
            let idx = col_names.iter().position(|n| n == column).unwrap();
            if let NaiveVal::Text(s) = &row[idx] {
                if *ignore_case {
                    s.to_lowercase().starts_with(&text.to_lowercase())
                } else {
                    s.starts_with(text)
                }
            } else {
                false
            }
        }
    }
}

fn compute_naive_totals(
    grp_rows: &[u32],
    all_rows: &[Vec<NaiveVal>],
    totals: &[Total],
    col_names: &[String],
) -> Vec<Option<f64>> {
    totals
        .iter()
        .map(|tot| {
            let col_idx = col_names.iter().position(|n| n == &tot.column).unwrap();
            if tot.aggregate == Aggregate::Count {
                return Some(grp_rows.len() as f64);
            }
            let mut sum = 0.0;
            let mut count = 0usize;
            let mut min = f64::INFINITY;
            let mut max = f64::NEG_INFINITY;

            for &r in grp_rows {
                let f = match all_rows[r as usize][col_idx] {
                    NaiveVal::Int(v) => Some(v as f64),
                    NaiveVal::Float(v) => Some(v),
                    _ => None,
                };
                if let Some(val) = f {
                    count += 1;
                    sum += val;
                    if val < min {
                        min = val;
                    }
                    if val > max {
                        max = val;
                    }
                }
            }

            if count == 0 {
                None
            } else {
                match tot.aggregate {
                    Aggregate::Count => Some(grp_rows.len() as f64),
                    Aggregate::Sum => Some(sum),
                    Aggregate::Avg => Some(sum / count as f64),
                    Aggregate::Min => Some(min),
                    Aggregate::Max => Some(max),
                }
            }
        })
        .collect()
}

fn assert_totals_match(actual: &[Option<f64>], expected: &[Option<f64>]) {
    assert_eq!(actual.len(), expected.len());
    for (a, e) in actual.iter().zip(expected.iter()) {
        match (a, e) {
            (None, None) => {}
            (Some(va), Some(ve)) => {
                let match_ok = va == ve
                    || (va.is_nan() && ve.is_nan())
                    || (va - ve).abs() <= 1e-9 * va.abs().max(ve.abs()).max(1.0);
                assert!(match_ok, "total mismatch: actual {} vs expected {}", va, ve);
            }
            _ => panic!(
                "total presence mismatch: actual {:?} vs expected {:?}",
                a, e
            ),
        }
    }
}

struct NaiveGroupNode {
    row_count: usize,
    totals: Vec<Option<f64>>,
    children: Vec<NaiveGroupNode>,
    rows: Vec<u32>,
}

#[test]
fn test_view_oracle_100_seeds() {
    for seed in 1..=100 {
        let dataset = generate_dataset(seed);
        let store = &dataset.store;
        let naive_rows = &dataset.naive_rows;
        let num_rows = store.rows();
        let mut rng = XorShift64::new(seed + 10000);

        // 1. Choose a random filter
        let filter = match rng.next_range(0, 5) {
            0 => None,
            1 => Some(Filter::Gt("c_int".into(), Owned::Int(0))),
            2 => Some(Filter::Lt("c_float".into(), Owned::Float(50.0))),
            3 => Some(Filter::Eq("c_bool".into(), Owned::Bool(true))),
            4 => Some(Filter::Between(
                "c_int".into(),
                Owned::Int(-500),
                Owned::Int(500),
            )),
            _ => Some(Filter::Not(Box::new(Filter::IsNull("c_dict".into())))),
        };

        // 2. Choose 1-2 sort keys
        let num_sort_keys = rng.next_range(1, 2) as usize;
        let mut sort_keys = Vec::with_capacity(num_sort_keys);
        for _ in 0..num_sort_keys {
            let col_idx = rng.next_range(0, dataset.col_names.len() as u64 - 1) as usize;
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
            sort_keys.push(SortKey {
                column: dataset.col_names[col_idx].clone(),
                order,
                nulls,
            });
        }

        // 3. Choose 1-2 grouping levels from groupable columns
        let groupable_cols = ["c_dict", "c_bool", "c_int", "c_float"];
        let num_group_levels = rng.next_range(1, 2) as usize;
        let mut group_by = Vec::with_capacity(num_group_levels);
        let first_col_idx = rng.next_range(0, groupable_cols.len() as u64 - 1) as usize;
        group_by.push(groupable_cols[first_col_idx].to_string());
        if num_group_levels == 2 {
            let second_col_idx = (first_col_idx + 1) % groupable_cols.len();
            group_by.push(groupable_cols[second_col_idx].to_string());
        }

        // 4. Totals
        let totals = vec![
            Total {
                column: "c_int".into(),
                aggregate: Aggregate::Count,
            },
            Total {
                column: "c_int".into(),
                aggregate: Aggregate::Sum,
            },
            Total {
                column: "c_float".into(),
                aggregate: Aggregate::Avg,
            },
            Total {
                column: "c_float".into(),
                aggregate: Aggregate::Min,
            },
            Total {
                column: "c_int".into(),
                aggregate: Aggregate::Max,
            },
        ];

        let query = Query {
            filter: filter.clone(),
            sort: sort_keys.clone(),
            group_by: group_by
                .iter()
                .map(|col| GroupLevel {
                    column: col.clone(),
                    totals: totals.clone(),
                })
                .collect(),
            summary: Vec::new(),
        };

        let view = View::new(store, query).expect("View::new must succeed");

        // --- Naive Computation ---
        // Filter
        let mut naive_filtered = Vec::new();
        for (r, row) in naive_rows.iter().enumerate().take(num_rows) {
            let passed = match &filter {
                Some(f) => eval_naive_filter(f, row, &dataset.col_names),
                None => true,
            };
            if passed {
                naive_filtered.push(r as u32);
            }
        }

        // Sort
        let mut naive_sorted = naive_filtered;
        naive_sorted.sort_by(|&ra, &rb| {
            for key in &sort_keys {
                let c_idx = dataset
                    .col_names
                    .iter()
                    .position(|n| n == &key.column)
                    .unwrap();
                let va = &naive_rows[ra as usize][c_idx];
                let vb = &naive_rows[rb as usize][c_idx];
                let cmp = cmp_val(va, vb, key.order, key.nulls);
                if cmp != Ordering::Equal {
                    return cmp;
                }
            }
            ra.cmp(&rb)
        });

        assert_eq!(
            view.row_count(),
            naive_sorted.len(),
            "row_count mismatch for seed {}",
            seed
        );
        assert_eq!(
            view.rows(),
            naive_sorted.as_slice(),
            "rows mismatch for seed {}",
            seed
        );

        // Group level by level
        let g0_col = &group_by[0];
        let g0_idx = dataset.col_names.iter().position(|n| n == g0_col).unwrap();
        let g0_desc = sort_keys
            .iter()
            .find(|k| k.column == *g0_col)
            .is_some_and(|k| k.order == Order::Desc);

        let mut level0_map: BTreeMap<Option<NaiveValOrd>, Vec<u32>> = BTreeMap::new();
        for &r in &naive_sorted {
            let ord = to_naive_ord(&naive_rows[r as usize][g0_idx]);
            level0_map.entry(ord).or_default().push(r);
        }

        let mut level0_entries: Vec<(Option<NaiveValOrd>, Vec<u32>)> =
            level0_map.into_iter().collect();
        if g0_desc {
            level0_entries.reverse();
        }

        let mut naive_groups = Vec::with_capacity(level0_entries.len());

        if num_group_levels == 1 {
            for (_key, g_rows) in level0_entries {
                let node_totals =
                    compute_naive_totals(&g_rows, naive_rows, &totals, &dataset.col_names);
                naive_groups.push(NaiveGroupNode {
                    row_count: g_rows.len(),
                    totals: node_totals,
                    children: Vec::new(),
                    rows: g_rows,
                });
            }
        } else {
            let g1_col = &group_by[1];
            let g1_idx = dataset.col_names.iter().position(|n| n == g1_col).unwrap();
            let g1_desc = sort_keys
                .iter()
                .find(|k| k.column == *g1_col)
                .is_some_and(|k| k.order == Order::Desc);

            for (_key0, g0_rows) in level0_entries {
                let g0_totals =
                    compute_naive_totals(&g0_rows, naive_rows, &totals, &dataset.col_names);

                let mut level1_map: BTreeMap<Option<NaiveValOrd>, Vec<u32>> = BTreeMap::new();
                for &r in &g0_rows {
                    let ord = to_naive_ord(&naive_rows[r as usize][g1_idx]);
                    level1_map.entry(ord).or_default().push(r);
                }

                let mut level1_entries: Vec<(Option<NaiveValOrd>, Vec<u32>)> =
                    level1_map.into_iter().collect();
                if g1_desc {
                    level1_entries.reverse();
                }

                let mut children = Vec::with_capacity(level1_entries.len());
                for (_key1, g1_rows) in level1_entries {
                    let g1_totals =
                        compute_naive_totals(&g1_rows, naive_rows, &totals, &dataset.col_names);
                    children.push(NaiveGroupNode {
                        row_count: g1_rows.len(),
                        totals: g1_totals,
                        children: Vec::new(),
                        rows: g1_rows,
                    });
                }

                naive_groups.push(NaiveGroupNode {
                    row_count: g0_rows.len(),
                    totals: g0_totals,
                    children,
                    rows: Vec::new(),
                });
            }
        }

        // Compare View groups with naive groups
        let view_groups = view.groups();
        assert_eq!(
            view_groups.len(),
            naive_groups.len(),
            "group count mismatch at top level for seed {}",
            seed
        );

        for (vg, ng) in view_groups.iter().zip(naive_groups.iter()) {
            assert_eq!(vg.level, 0);
            assert_eq!(vg.row_count, ng.row_count);
            assert_totals_match(&vg.totals, &ng.totals);

            if num_group_levels == 1 {
                assert!(vg.children.is_empty());
                assert_eq!(vg.rows, ng.rows);
            } else {
                assert!(vg.rows.is_empty());
                assert_eq!(vg.children.len(), ng.children.len());
                for (v_child, n_child) in vg.children.iter().zip(ng.children.iter()) {
                    assert_eq!(v_child.level, 1);
                    assert_eq!(v_child.row_count, n_child.row_count);
                    assert_eq!(v_child.rows, n_child.rows);
                    assert_totals_match(&v_child.totals, &n_child.totals);
                    assert!(v_child.children.is_empty());
                }
            }
        }
    }
}

#[test]
fn test_expanded_state_survives_requery() {
    let mut store = Store::new();
    store
        .add(
            "category",
            Column::dict_from(vec![
                Some("Alpha".into()),
                Some("Bravo".into()),
                Some("Charlie".into()),
                Some("Delta".into()),
                Some("Echo".into()),
            ]),
        )
        .unwrap();
    store
        .add(
            "value",
            Column::ints(vec![Some(50), Some(10), Some(40), Some(20), Some(30)]),
        )
        .unwrap();

    let q1 = Query {
        filter: None,
        sort: vec![SortKey {
            column: "value".into(),
            order: Order::Asc,
            nulls: Nulls::Last,
        }],
        group_by: vec![GroupLevel {
            column: "category".into(),
            totals: vec![],
        }],
        summary: Vec::new(),
    };

    let mut view = View::new(&store, q1).unwrap();
    assert_eq!(view.groups().len(), 5);

    // Expand two groups: path &[1] and &[3]
    assert!(view.set_expanded(&[1], true));
    assert!(view.set_expanded(&[3], true));

    assert!(!view.is_expanded(&[0]));
    assert!(view.is_expanded(&[1]));
    assert!(!view.is_expanded(&[2]));
    assert!(view.is_expanded(&[3]));
    assert!(!view.is_expanded(&[4]));

    let key1 = view.groups()[1].key.clone();
    let key3 = view.groups()[3].key.clone();

    // Requery with a different sort
    let q2 = Query {
        filter: None,
        sort: vec![SortKey {
            column: "value".into(),
            order: Order::Desc,
            nulls: Nulls::Last,
        }],
        group_by: vec![GroupLevel {
            column: "category".into(),
            totals: vec![],
        }],
        summary: Vec::new(),
    };

    view.requery(&store, q2).unwrap();

    // The same two keys must still be expanded, every other group collapsed
    for (idx, grp) in view.groups().iter().enumerate() {
        if grp.key == key1 || grp.key == key3 {
            assert!(
                view.is_expanded(&[idx]),
                "group with key {:?} should be expanded",
                grp.key
            );
        } else {
            assert!(
                !view.is_expanded(&[idx]),
                "group with key {:?} should be collapsed",
                grp.key
            );
        }
    }

    // Requery with a filter that removes one of them (filter removes key1)
    let val_to_keep = store.key_value("category", key3[0]).unwrap();
    let keep_text = match val_to_keep {
        Value::Text(s) => s.to_string(),
        _ => panic!("expected text"),
    };

    let q3 = Query {
        filter: Some(Filter::Eq("category".into(), Owned::Text(keep_text))),
        sort: vec![SortKey {
            column: "value".into(),
            order: Order::Desc,
            nulls: Nulls::Last,
        }],
        group_by: vec![GroupLevel {
            column: "category".into(),
            totals: vec![],
        }],
        summary: Vec::new(),
    };

    view.requery(&store, q3).unwrap();
    assert_eq!(view.groups().len(), 1);
    assert_eq!(view.groups()[0].key, key3);
    assert!(view.is_expanded(&[0]));
}

#[test]
fn test_reuse_stats() {
    let mut store = Store::new();
    store
        .add(
            "dept",
            Column::dict_from(vec![
                Some("HR".into()),
                Some("IT".into()),
                Some("HR".into()),
            ]),
        )
        .unwrap();
    store
        .add(
            "salary",
            Column::ints(vec![Some(5000), Some(7000), Some(6000)]),
        )
        .unwrap();
    store
        .add(
            "active",
            Column::bools(vec![Some(true), Some(false), Some(true)]),
        )
        .unwrap();

    let q1 = Query {
        filter: Some(Filter::Gt("salary".into(), Owned::Int(4000))),
        sort: vec![SortKey {
            column: "salary".into(),
            order: Order::Asc,
            nulls: Nulls::Last,
        }],
        group_by: vec![GroupLevel {
            column: "dept".into(),
            totals: vec![Total {
                column: "salary".into(),
                aggregate: Aggregate::Sum,
            }],
        }],
        summary: Vec::new(),
    };

    let mut view = View::new(&store, q1.clone()).unwrap();
    assert_eq!(
        view.reuse_stats(),
        ReuseStats {
            filter_runs: 1,
            sort_runs: 1,
            group_runs: 1,
        }
    );

    // 1. Same query -> nothing runs
    view.requery(&store, q1.clone()).unwrap();
    assert_eq!(
        view.reuse_stats(),
        ReuseStats {
            filter_runs: 1,
            sort_runs: 1,
            group_runs: 1,
        }
    );

    // 2. Changing only group_by -> filter_runs and sort_runs do not increase
    let mut q2 = q1.clone();
    q2.group_by[0].column = "active".into();
    view.requery(&store, q2.clone()).unwrap();
    assert_eq!(
        view.reuse_stats(),
        ReuseStats {
            filter_runs: 1,
            sort_runs: 1,
            group_runs: 2,
        }
    );

    // 3. Changing only totals -> filter_runs and sort_runs do not increase
    let mut q2_totals = q2.clone();
    q2_totals.group_by[0].totals = vec![Total {
        column: "salary".into(),
        aggregate: Aggregate::Avg,
    }];
    view.requery(&store, q2_totals.clone()).unwrap();
    assert_eq!(
        view.reuse_stats(),
        ReuseStats {
            filter_runs: 1,
            sort_runs: 1,
            group_runs: 3,
        }
    );

    // 4. Changing only sort -> filter_runs does not increase
    let mut q3 = q2_totals.clone();
    q3.sort = vec![SortKey {
        column: "salary".into(),
        order: Order::Desc,
        nulls: Nulls::Last,
    }];
    view.requery(&store, q3.clone()).unwrap();
    assert_eq!(
        view.reuse_stats(),
        ReuseStats {
            filter_runs: 1,
            sort_runs: 2,
            group_runs: 4,
        }
    );

    // 5. Changing filter -> filter_runs increases
    let mut q4 = q3;
    q4.filter = Some(Filter::Gt("salary".into(), Owned::Int(5500)));
    view.requery(&store, q4).unwrap();
    assert_eq!(
        view.reuse_stats(),
        ReuseStats {
            filter_runs: 2,
            sort_runs: 3,
            group_runs: 5,
        }
    );
}

#[test]
fn test_desc_on_grouping_column_reverses_groups() {
    let mut store = Store::new();
    store
        .add(
            "letter",
            Column::dict_from(vec![Some("A".into()), Some("B".into()), Some("C".into())]),
        )
        .unwrap();
    store
        .add("val", Column::ints(vec![Some(1), Some(2), Some(3)]))
        .unwrap();

    // Ascending sort on grouping column
    let q_asc = Query {
        filter: None,
        sort: vec![SortKey {
            column: "letter".into(),
            order: Order::Asc,
            nulls: Nulls::Last,
        }],
        group_by: vec![GroupLevel {
            column: "letter".into(),
            totals: vec![],
        }],
        summary: Vec::new(),
    };

    let view_asc = View::new(&store, q_asc).unwrap();
    let keys_asc: Vec<Option<u64>> = view_asc.groups().iter().map(|g| g.key[0]).collect();
    assert_eq!(keys_asc.len(), 3);

    // Descending sort on grouping column
    let q_desc = Query {
        filter: None,
        sort: vec![SortKey {
            column: "letter".into(),
            order: Order::Desc,
            nulls: Nulls::Last,
        }],
        group_by: vec![GroupLevel {
            column: "letter".into(),
            totals: vec![],
        }],
        summary: Vec::new(),
    };

    let view_desc = View::new(&store, q_desc).unwrap();
    let mut keys_desc: Vec<Option<u64>> = view_desc.groups().iter().map(|g| g.key[0]).collect();
    assert_eq!(keys_desc.len(), 3);

    keys_desc.reverse();
    assert_eq!(keys_asc, keys_desc);

    // Test with two grouping levels
    let mut store2 = Store::new();
    store2
        .add(
            "l1",
            Column::dict_from(vec![
                Some("A".into()),
                Some("A".into()),
                Some("B".into()),
                Some("B".into()),
            ]),
        )
        .unwrap();
    store2
        .add(
            "l2",
            Column::dict_from(vec![
                Some("X".into()),
                Some("Y".into()),
                Some("X".into()),
                Some("Y".into()),
            ]),
        )
        .unwrap();
    store2
        .add(
            "val",
            Column::ints(vec![Some(1), Some(2), Some(3), Some(4)]),
        )
        .unwrap();

    let q_l1_desc = Query {
        filter: None,
        sort: vec![
            SortKey {
                column: "l1".into(),
                order: Order::Desc,
                nulls: Nulls::Last,
            },
            SortKey {
                column: "l2".into(),
                order: Order::Asc,
                nulls: Nulls::Last,
            },
        ],
        group_by: vec![
            GroupLevel {
                column: "l1".into(),
                totals: vec![],
            },
            GroupLevel {
                column: "l2".into(),
                totals: vec![],
            },
        ],
        summary: Vec::new(),
    };

    let view2 = View::new(&store2, q_l1_desc).unwrap();
    assert_eq!(view2.groups().len(), 2);
    // Level 0 should be B, then A
    let val0 = store2.key_value("l1", view2.groups()[0].key[0]).unwrap();
    let val1 = store2.key_value("l1", view2.groups()[1].key[0]).unwrap();
    assert_eq!(val0, Value::Text("B"));
    assert_eq!(val1, Value::Text("A"));

    // Level 1 should be X, then Y (Asc)
    let child0_val0 = store2
        .key_value("l2", view2.groups()[0].children[0].key[1])
        .unwrap();
    let child0_val1 = store2
        .key_value("l2", view2.groups()[0].children[1].key[1])
        .unwrap();
    assert_eq!(child0_val0, Value::Text("X"));
    assert_eq!(child0_val1, Value::Text("Y"));
}

#[test]
fn test_set_expanded_on_missing_path() {
    let mut store = Store::new();
    store
        .add(
            "city",
            Column::dict_from(vec![Some("Paris".into()), Some("Tokyo".into())]),
        )
        .unwrap();
    store
        .add("id", Column::ints(vec![Some(1), Some(2)]))
        .unwrap();

    let query = Query {
        filter: None,
        sort: vec![],
        group_by: vec![GroupLevel {
            column: "city".into(),
            totals: vec![],
        }],
        summary: Vec::new(),
    };

    let mut view = View::new(&store, query).unwrap();

    assert!(!view.set_expanded(&[], true));
    assert!(!view.set_expanded(&[99], true));
    assert!(!view.set_expanded(&[0, 99], true));
    assert!(!view.set_expanded(&[1, 0, 0], false));

    assert!(!view.is_expanded(&[]));
    assert!(!view.is_expanded(&[99]));
    assert!(!view.is_expanded(&[0, 99]));
}

#[test]
fn test_expand_all_and_collapse_all() {
    let mut store = Store::new();
    store
        .add(
            "c1",
            Column::dict_from(vec![Some("A".into()), Some("B".into())]),
        )
        .unwrap();
    store
        .add(
            "c2",
            Column::dict_from(vec![Some("X".into()), Some("Y".into())]),
        )
        .unwrap();
    store
        .add("val", Column::ints(vec![Some(1), Some(2)]))
        .unwrap();

    let query = Query {
        filter: None,
        sort: vec![],
        group_by: vec![
            GroupLevel {
                column: "c1".into(),
                totals: vec![],
            },
            GroupLevel {
                column: "c2".into(),
                totals: vec![],
            },
        ],
        summary: Vec::new(),
    };

    let mut view = View::new(&store, query).unwrap();

    assert!(!view.is_expanded(&[0]));
    assert!(!view.is_expanded(&[0, 0]));
    assert!(!view.is_expanded(&[1]));
    assert!(!view.is_expanded(&[1, 0]));

    view.expand_all();
    assert!(view.is_expanded(&[0]));
    assert!(view.is_expanded(&[0, 0]));
    assert!(view.is_expanded(&[1]));
    assert!(view.is_expanded(&[1, 0]));

    view.collapse_all();
    assert!(!view.is_expanded(&[0]));
    assert!(!view.is_expanded(&[0, 0]));
    assert!(!view.is_expanded(&[1]));
    assert!(!view.is_expanded(&[1, 0]));
}

#[test]
fn test_view_validation_errors() {
    let mut store = Store::new();
    store
        .add("text_col", Column::texts(vec![Some("hello".into())]))
        .unwrap();
    store.add("int_col", Column::ints(vec![Some(42)])).unwrap();

    // Column not found in group_by
    let q_missing_group = Query {
        filter: None,
        sort: vec![],
        group_by: vec![GroupLevel {
            column: "unknown_col".into(),
            totals: vec![],
        }],
        summary: Vec::new(),
    };
    assert_eq!(
        View::new(&store, q_missing_group).err(),
        Some(StoreError::ColumnNotFound("unknown_col".into()))
    );

    // Text column in group_by -> NotGroupable
    let q_text_group = Query {
        filter: None,
        sort: vec![],
        group_by: vec![GroupLevel {
            column: "text_col".into(),
            totals: vec![],
        }],
        summary: Vec::new(),
    };
    assert_eq!(
        View::new(&store, q_text_group).err(),
        Some(StoreError::NotGroupable("text_col".into()))
    );

    // Column not found in sort
    let q_missing_sort = Query {
        filter: None,
        sort: vec![SortKey {
            column: "missing_sort".into(),
            order: Order::Asc,
            nulls: Nulls::Last,
        }],
        group_by: vec![],
        summary: Vec::new(),
    };
    assert_eq!(
        View::new(&store, q_missing_sort).err(),
        Some(StoreError::ColumnNotFound("missing_sort".into()))
    );

    // Sum aggregate on non-numeric column -> TypeMismatch
    let q_invalid_total = Query {
        filter: None,
        sort: vec![],
        group_by: vec![GroupLevel {
            column: "int_col".into(),
            totals: vec![Total {
                column: "text_col".into(),
                aggregate: Aggregate::Sum,
            }],
        }],
        summary: Vec::new(),
    };
    assert_eq!(
        View::new(&store, q_invalid_total).err(),
        Some(StoreError::TypeMismatch {
            column: "text_col".into(),
            expected: "Int or Float",
        })
    );
}

#[test]
fn changing_the_grouping_columns_collapses_everything() {
    let mut store = Store::new();
    store
        .add(
            "a",
            Column::dict_from(vec![Some("x".into()), Some("y".into())]),
        )
        .expect("column a");
    store
        .add(
            "b",
            Column::dict_from(vec![Some("x".into()), Some("y".into())]),
        )
        .expect("column b");
    let query = |col: &str| Query {
        filter: None,
        sort: Vec::new(),
        group_by: vec![GroupLevel {
            column: col.to_string(),
            totals: Vec::new(),
        }],
        summary: Vec::new(),
    };
    let mut view = View::new(&store, query("a")).expect("view");
    assert!(view.set_expanded(&[0], true));
    view.requery(&store, query("b")).expect("requery");
    // same ordinal, other column: it must not come back open
    assert!(!view.is_expanded(&[0]));
}

#[test]
fn test_expanded_state_survives_a_totals_only_change() {
    let mut store = Store::new();
    store
        .add(
            "category",
            Column::dict_from(vec![
                Some("Alpha".into()),
                Some("Bravo".into()),
                Some("Alpha".into()),
            ]),
        )
        .unwrap();
    store
        .add("value", Column::ints(vec![Some(5), Some(7), Some(9)]))
        .unwrap();

    let mut view = View::new(&store, Query::new().group("category")).unwrap();
    assert!(view.set_expanded(&[1], true));

    // Same level columns, new totals: the regroup must keep what the user opened
    let with_sum = Query::new()
        .group("category")
        .with_total(Total::sum("value"));
    view.requery(&store, with_sum).unwrap();

    assert!(view.is_expanded(&[1]));
    assert!(!view.is_expanded(&[0]));
    assert_eq!(view.groups()[0].totals, vec![Some(14.0)]);
}
