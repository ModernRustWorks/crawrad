use crate::column::{Column, Value};
use crate::filter::{Filter, Owned};
use crate::group::{Aggregate, Total};
use crate::locate::Locator;
use crate::sort::{Nulls, Order, SortKey};
use crate::store::Store;
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

#[derive(Clone, PartialEq, Eq)]
struct NaiveKey(Vec<Option<NaiveValOrd>>);

impl PartialOrd for NaiveKey {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for NaiveKey {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0)
    }
}

#[derive(Clone, PartialEq)]
enum NaiveValOrd {
    Bool(bool),
    Dict(String),
    Int(i64),
    Float(u64), // total-order representation
}

impl Eq for NaiveValOrd {}

impl PartialOrd for NaiveValOrd {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for NaiveValOrd {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (NaiveValOrd::Bool(a), NaiveValOrd::Bool(b)) => a.cmp(b),
            (NaiveValOrd::Dict(a), NaiveValOrd::Dict(b)) => a.cmp(b),
            (NaiveValOrd::Int(a), NaiveValOrd::Int(b)) => a.cmp(b),
            (NaiveValOrd::Float(a), NaiveValOrd::Float(b)) => a.cmp(b),
            _ => Ordering::Equal,
        }
    }
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
    let num_rows = rng.next_range(1, 200) as usize;

    let int_edges = [i64::MIN, i64::MIN + 1, i64::MAX - 1, i64::MAX, 0, -1];

    let float_edges = [
        -0.0,
        0.0,
        f64::INFINITY,
        f64::NEG_INFINITY,
        f64::NAN,
        f64::MIN_POSITIVE,
    ];

    let text_edges = [
        "",
        "\0",
        "abcdefgh",
        "abcdefgh1",
        "abcdefgh2",
        "ABCdefgh",
        "ação",
        "日本",
    ];

    let dict_words = [
        "",
        "\0",
        "abcdefgh",
        "abcdefgh1",
        "abcdefgh2",
        "ABCdefgh",
        "ação",
        "日本",
        "apple",
        "banana",
    ];

    let mut col_int = Vec::with_capacity(num_rows);
    let mut col_float = Vec::with_capacity(num_rows);
    let mut col_bool = Vec::with_capacity(num_rows);
    let mut col_dict = Vec::with_capacity(num_rows);
    let mut col_text = Vec::with_capacity(num_rows);

    let mut naive_rows = Vec::with_capacity(num_rows);

    for r in 0..num_rows {
        // c_int with edges or sorted/reverse/all-equal patterns
        let v_int = match seed % 6 {
            1 => Some(r as i64 * 10),
            2 => Some((num_rows - r) as i64 * 10),
            3 => Some(42),
            _ => match rng.next_range(0, 7) {
                0 => None,
                1..=3 => {
                    let edge_idx = rng.next_range(0, int_edges.len() as u64 - 1) as usize;
                    Some(int_edges[edge_idx])
                }
                _ => {
                    let raw = rng.next_range(0, 2000) as i64 - 1000;
                    Some(raw)
                }
            },
        };
        col_int.push(v_int);

        // c_float with edges or sorted/reverse patterns
        let v_float = match seed % 6 {
            4 => Some(r as f64 * 1.5),
            5 => Some((num_rows - r) as f64 * 1.5),
            _ => match rng.next_range(0, 9) {
                0 => None,
                1..=4 => {
                    let edge_idx = rng.next_range(0, float_edges.len() as u64 - 1) as usize;
                    Some(float_edges[edge_idx])
                }
                _ => {
                    let raw = (rng.next_range(0, 4000) as f64 - 2000.0) / 10.0;
                    Some(raw)
                }
            },
        };
        col_float.push(v_float);

        // c_bool
        let v_bool = if rng.next_range(0, 4) == 0 {
            None
        } else {
            Some(rng.next_bool())
        };
        col_bool.push(v_bool);

        // c_dict with edges
        let v_dict = if rng.next_range(0, 4) == 0 {
            None
        } else {
            let idx = rng.next_range(0, dict_words.len() as u64 - 1) as usize;
            Some(dict_words[idx].to_string())
        };
        col_dict.push(v_dict.clone());

        // c_text with edges
        let v_text = match rng.next_range(0, 6) {
            0 => None,
            1..=2 => {
                let edge_idx = rng.next_range(0, text_edges.len() as u64 - 1) as usize;
                Some(text_edges[edge_idx].to_string())
            }
            _ => Some(format!("text_{}_{}", r % 15, rng.next_range(100, 999))),
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

#[test]
fn test_oracle_suite_200_seeds() {
    for seed in 1..=200 {
        let dataset = generate_dataset(seed);
        let store = &dataset.store;
        let naive_rows = &dataset.naive_rows;
        let num_rows = store.rows();
        let rows: Vec<u32> = (0..num_rows as u32).collect();

        let mut rng = XorShift64::new(seed + 999);

        // 1. Sort with 1, 2, and 3 keys, asc/desc, nulls first/last
        for num_keys in 1..=3 {
            let mut keys = Vec::with_capacity(num_keys);
            for _ in 0..num_keys {
                let col_idx = rng.next_range(0, 4) as usize;
                let col_name = dataset.col_names[col_idx].clone();
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
                keys.push(SortKey {
                    column: col_name,
                    order,
                    nulls,
                });
            }

            let sorted_dg = store.sort(&rows, &keys).expect("sort must succeed");

            // Naive sort
            let mut naive_sorted = rows.clone();
            naive_sorted.sort_by(|&ra, &rb| {
                for key in &keys {
                    let c_idx = dataset
                        .col_names
                        .iter()
                        .position(|n| n == &key.column)
                        .expect("col exists");
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
                sorted_dg, naive_sorted,
                "sort mismatch for seed {} with {} keys",
                seed, num_keys
            );
        }

        // 2. Filters
        let filters_to_test = vec![
            Filter::Eq("c_int".into(), Owned::Int(0)),
            Filter::Eq("c_int".into(), Owned::Int(i64::MIN)),
            Filter::Gt("c_int".into(), Owned::Int(0)),
            Filter::Between("c_int".into(), Owned::Int(-200), Owned::Int(200)),
            Filter::In(
                "c_int".into(),
                vec![Owned::Int(i64::MIN), Owned::Int(0), Owned::Int(i64::MAX)],
            ),
            Filter::IsNull("c_int".into()),
            Filter::Lt("c_float".into(), Owned::Float(10.0)),
            Filter::Eq("c_float".into(), Owned::Float(0.0)), // -0.0 and 0.0 match
            Filter::IsNull("c_float".into()),
            Filter::Eq("c_bool".into(), Owned::Bool(true)),
            Filter::Eq("c_dict".into(), Owned::Text("abcdefgh".into())),
            Filter::StartsWith {
                column: "c_dict".into(),
                text: "abc".into(),
                ignore_case: false,
            },
            Filter::Contains {
                column: "c_text".into(),
                text: "ção".into(),
                ignore_case: false,
            },
            Filter::eq_ignore_case("c_dict", "ABC"),
            Filter::eq_ignore_case("c_text", "ÇÃO"),
            Filter::And(vec![
                Filter::Gt("c_int".into(), Owned::Int(-500)),
                Filter::Lt("c_int".into(), Owned::Int(500)),
                Filter::Not(Box::new(Filter::IsNull("c_bool".into()))),
            ]),
            Filter::Or(vec![
                Filter::Eq("c_dict".into(), Owned::Text("".into())),
                Filter::Eq("c_dict".into(), Owned::Text("日本".into())),
            ]),
        ];

        for f in &filters_to_test {
            let bm = store.filter(f).expect("filter must succeed");
            let dg_matching = bm.ones().collect::<Vec<_>>();

            let mut naive_matching = Vec::new();
            for (r, row) in naive_rows.iter().enumerate().take(num_rows) {
                if eval_naive_filter(f, row, &dataset.col_names) {
                    naive_matching.push(r as u32);
                }
            }

            assert_eq!(
                dg_matching, naive_matching,
                "filter mismatch for seed {:?} on filter {:?}",
                seed, f
            );
        }

        // 3. Group by 1 and 2 columns with all aggregates
        for num_group_cols in 1..=2 {
            let group_cols: Vec<String> = if num_group_cols == 1 {
                vec!["c_dict".into()]
            } else {
                vec!["c_dict".into(), "c_bool".into()]
            };

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

            let dg_groups = store.group(&rows, &group_cols, &totals).expect("group");

            // Naive group with BTreeMap
            let mut btree_groups: BTreeMap<NaiveKey, Vec<u32>> = BTreeMap::new();
            for &r in &rows {
                let mut k_vals = Vec::new();
                for g_col in &group_cols {
                    let c_idx = dataset
                        .col_names
                        .iter()
                        .position(|n| n == g_col)
                        .expect("col exists");
                    k_vals.push(to_naive_ord(&naive_rows[r as usize][c_idx]));
                }
                btree_groups.entry(NaiveKey(k_vals)).or_default().push(r);
            }

            assert_eq!(dg_groups.len(), btree_groups.len());

            for (dg_grp, (naive_k, naive_grp_rows)) in dg_groups.iter().zip(btree_groups.iter()) {
                assert_eq!(&dg_grp.rows, naive_grp_rows);

                // Verify headers via key_value
                for (k_idx, col_name) in group_cols.iter().enumerate() {
                    let ord = dg_grp.key[k_idx];
                    let val = store.key_value(col_name, ord).expect("key_value");
                    match &naive_k.0[k_idx] {
                        None => assert_eq!(val, Value::Null),
                        Some(NaiveValOrd::Bool(b)) => assert_eq!(val, Value::Bool(*b)),
                        Some(NaiveValOrd::Dict(s)) => assert_eq!(val, Value::Text(s)),
                        Some(NaiveValOrd::Int(i)) => assert_eq!(val, Value::Int(*i)),
                        Some(NaiveValOrd::Float(bits)) => {
                            if *bits == u64::MAX {
                                match val {
                                    Value::Float(f) => assert!(f.is_nan()),
                                    _ => panic!("expected float nan"),
                                }
                            } else {
                                match val {
                                    Value::Float(f) => assert_eq!(f.to_bits(), *bits),
                                    _ => panic!("expected float"),
                                }
                            }
                        }
                    }
                }

                // Verify totals
                let naive_totals =
                    compute_naive_totals(naive_grp_rows, naive_rows, &totals, &dataset.col_names);
                assert_eq!(dg_grp.totals.len(), naive_totals.len());
                for (dg_t, naive_t) in dg_grp.totals.iter().zip(naive_totals.iter()) {
                    match (dg_t, naive_t) {
                        (None, None) => {}
                        (Some(a), Some(b)) => {
                            let same = a == b
                                || (a.is_nan() && b.is_nan())
                                || (a - b).abs() <= 1e-9 * a.abs().max(b.abs()).max(1.0);
                            assert!(same, "totals difference too high: {} vs {}", a, b);
                        }
                        _ => panic!("total mismatch: {:?} vs {:?}", dg_t, naive_t),
                    }
                }
            }
        }

        // 4. Locate: find_eq and range
        let mut locator = Locator::new();
        // find_eq
        let query_int = Owned::Int(rng.next_range(0, 100) as i64 - 50);
        let dg_find = locator
            .find_eq(store, "c_int", &query_int)
            .expect("locate eq");
        let mut naive_find = None;
        for (r, row) in naive_rows.iter().enumerate().take(num_rows) {
            if row[0]
                == NaiveVal::Int(match query_int {
                    Owned::Int(v) => v,
                    _ => 0,
                })
            {
                naive_find = Some(r as u32);
                break;
            }
        }
        assert_eq!(dg_find, naive_find);

        // range
        let lo = Owned::Int(-200);
        let hi = Owned::Int(200);
        let dg_range = locator
            .range(store, "c_int", &lo, &hi)
            .expect("locate range");

        let mut naive_range = Vec::new();
        for (r, row) in naive_rows.iter().enumerate().take(num_rows) {
            if let NaiveVal::Int(v) = row[0]
                && (-200..=200).contains(&v)
            {
                naive_range.push(r as u32);
            }
        }
        naive_range.sort_by(|&ra, &rb| {
            let va = &naive_rows[ra as usize][0];
            let vb = &naive_rows[rb as usize][0];
            let c = cmp_val(va, vb, Order::Asc, Nulls::Last);
            if c != Ordering::Equal { c } else { ra.cmp(&rb) }
        });
        assert_eq!(dg_range, naive_range);
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
