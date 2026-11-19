//! Comprehensive benchmark suite comparing storage techniques side by side.
//!
//! Evaluates layout scanning, text encodings, null representations, single and multi-key
//! sorting algorithms, filter selection representations, group counting, and point lookup.

mod common;

use common::{
    BENCH_SEED, BenchRow, Cardinality, OrderKind, XorShift64, generate_floats, generate_ints,
    generate_layout_dataset, generate_strings,
};
use criterion::{Criterion, black_box, criterion_group, criterion_main};
use datagrid::{Column, Filter, Locator, Nulls, Order, Owned, SortKey, Store};
use std::collections::HashMap;

// ---------------------------------------------------------------------------
// 1. layout_scan: Row Structs vs Columnar Store
// ---------------------------------------------------------------------------

fn scan_row_structs(rows: &[BenchRow]) -> (f64, usize) {
    let mut sum = 0.0f64;
    let mut count = 0usize;
    for r in rows {
        sum += r.amount;
        if r.category == 1 {
            count += 1;
        }
    }
    (sum, count)
}

fn scan_columnar(store: &Store) -> (f64, usize) {
    let sum = match store.column("amount") {
        Some(Column::Float { values, .. }) => {
            let mut s = 0.0f64;
            for &v in values {
                s += v;
            }
            s
        }
        _ => 0.0,
    };
    let count = match store.column("category") {
        Some(Column::Int { values, .. }) => {
            let mut c = 0usize;
            for &v in values {
                if v == 1 {
                    c += 1;
                }
            }
            c
        }
        _ => 0,
    };
    (sum, count)
}

fn bench_layout_scan(c: &mut Criterion) {
    let mut g = c.benchmark_group("layout_scan");

    for (rows, label) in [(100_000, "100k"), (1_000_000, "1M")] {
        let (row_structs, store) = generate_layout_dataset(rows);

        g.bench_function(format!("row_struct/{}", label), |b| {
            b.iter(|| scan_row_structs(black_box(&row_structs)))
        });

        g.bench_function(format!("columnar/{}", label), |b| {
            b.iter(|| scan_columnar(black_box(&store)))
        });
    }

    g.finish();

    // 10M rows with reduced sample size
    let mut g10 = c.benchmark_group("layout_scan");
    g10.sample_size(10);
    let (row_structs, store) = generate_layout_dataset(10_000_000);

    g10.bench_function("row_struct/10M", |b| {
        b.iter(|| scan_row_structs(black_box(&row_structs)))
    });

    g10.bench_function("columnar/10M", |b| {
        b.iter(|| scan_columnar(black_box(&store)))
    });

    g10.finish();
}

// ---------------------------------------------------------------------------
// 2. text_storage: Plain Text vs Dictionary Encoding
// ---------------------------------------------------------------------------

fn bench_text_storage(c: &mut Criterion) {
    let mut g = c.benchmark_group("text_storage");

    let configs = [
        (100_000, Cardinality::Low, "100k_low", "Category_02"),
        (100_000, Cardinality::Medium, "100k_med", "City_0250"),
        (100_000, Cardinality::High, "100k_high", "Key_00050000"),
        (1_000_000, Cardinality::Low, "1M_low", "Category_02"),
        (1_000_000, Cardinality::Medium, "1M_med", "City_0250"),
        (1_000_000, Cardinality::High, "1M_high", "Key_00500000"),
    ];

    for (rows, card, shape, query_str) in configs {
        let strings = generate_strings(rows, card);
        let row_ids: Vec<u32> = (0..rows as u32).collect();

        let mut store_plain = Store::new();
        store_plain
            .add("text", Column::texts(strings.clone()))
            .expect("add plain text");

        let mut store_dict = Store::new();
        store_dict
            .add("text", Column::dict_from(strings))
            .expect("add dict text");

        let filter = Filter::Eq("text".into(), Owned::Text(query_str.to_string()));
        let sort_keys = vec![SortKey {
            column: "text".into(),
            order: Order::Asc,
            nulls: Nulls::Last,
        }];

        g.bench_function(format!("plain/{}", shape), |b| {
            b.iter(|| {
                let bm = store_plain.filter(black_box(&filter)).unwrap();
                let sorted = store_plain
                    .sort(black_box(&row_ids), black_box(&sort_keys))
                    .unwrap();
                (bm, sorted)
            })
        });

        g.bench_function(format!("dict/{}", shape), |b| {
            b.iter(|| {
                let bm = store_dict.filter(black_box(&filter)).unwrap();
                let sorted = store_dict
                    .sort(black_box(&row_ids), black_box(&sort_keys))
                    .unwrap();
                (bm, sorted)
            })
        });
    }

    g.finish();
}

// ---------------------------------------------------------------------------
// 3. nulls_repr: Bitmap Validity Mask vs Vec<Option<i64>>
// ---------------------------------------------------------------------------

fn sum_column_bitmap(col: &Column) -> i64 {
    match col {
        Column::Int { values, valid } => {
            let mut sum = 0i64;
            if let Some(mask) = valid {
                for idx in mask.ones() {
                    sum = sum.wrapping_add(values[idx as usize]);
                }
            } else {
                for &v in values {
                    sum = sum.wrapping_add(v);
                }
            }
            sum
        }
        _ => 0,
    }
}

fn sum_vec_option(items: &[Option<i64>]) -> i64 {
    let mut sum = 0i64;
    for v in items.iter().flatten() {
        sum = sum.wrapping_add(*v);
    }
    sum
}

fn bench_nulls_repr(c: &mut Criterion) {
    let mut g = c.benchmark_group("nulls_repr");

    for (rows, shape) in [(100_000, "100k_10p_nulls"), (1_000_000, "1M_10p_nulls")] {
        let items = generate_ints(rows, Cardinality::Medium, OrderKind::Random, 0.10);
        let col = Column::ints(items.clone());

        g.bench_function(format!("bitmap/{}", shape), |b| {
            b.iter(|| sum_column_bitmap(black_box(&col)))
        });

        g.bench_function(format!("vec_option/{}", shape), |b| {
            b.iter(|| sum_vec_option(black_box(&items)))
        });
    }

    g.finish();
}

// ---------------------------------------------------------------------------
// 4. sort_one_int and sort_one_float
// ---------------------------------------------------------------------------

fn sort_by_ordinal(row_ids: &[u32], ords: &[u64]) -> Vec<u32> {
    let mut p = row_ids.to_vec();
    p.sort_unstable_by_key(|&r| ords[r as usize]);
    p
}

fn sort_by_raw_int(row_ids: &[u32], values: &[i64]) -> Vec<u32> {
    let mut p = row_ids.to_vec();
    p.sort_unstable_by(|&a, &b| values[a as usize].cmp(&values[b as usize]));
    p
}

fn sort_by_raw_float(row_ids: &[u32], values: &[f64]) -> Vec<u32> {
    let mut p = row_ids.to_vec();
    p.sort_unstable_by(|&a, &b| values[a as usize].total_cmp(&values[b as usize]));
    p
}

fn bench_sort_one_int(c: &mut Criterion) {
    let mut g = c.benchmark_group("sort_one_int");

    let configs = [
        (100_000, OrderKind::Random, "100k_random"),
        (100_000, OrderKind::Sorted, "100k_sorted"),
        (100_000, OrderKind::Reverse, "100k_reverse"),
        (1_000_000, OrderKind::Random, "1M_random"),
        (1_000_000, OrderKind::Sorted, "1M_sorted"),
        (1_000_000, OrderKind::Reverse, "1M_reverse"),
    ];

    for (rows, order, shape) in configs {
        let raw = generate_ints(rows, Cardinality::High, order, 0.0);
        let values: Vec<i64> = raw.iter().map(|o| o.expect("no nulls")).collect();
        let ords: Vec<u64> = values.iter().map(|&v| (v as u64) ^ (1 << 63)).collect();
        let row_ids: Vec<u32> = (0..rows as u32).collect();

        let mut store = Store::new();
        store.add("val", Column::ints(raw)).expect("add int");

        let key = vec![SortKey {
            column: "val".into(),
            order: Order::Asc,
            nulls: Nulls::Last,
        }];

        g.bench_function(format!("radix/{}", shape), |b| {
            b.iter(|| {
                store
                    .sort(black_box(&row_ids), black_box(&key))
                    .expect("sort")
            })
        });

        g.bench_function(format!("std_ordinal/{}", shape), |b| {
            b.iter(|| sort_by_ordinal(black_box(&row_ids), black_box(&ords)))
        });

        g.bench_function(format!("std_raw/{}", shape), |b| {
            b.iter(|| sort_by_raw_int(black_box(&row_ids), black_box(&values)))
        });
    }

    g.finish();

    // 10M rows with reduced sample size
    let mut g10 = c.benchmark_group("sort_one_int");
    g10.sample_size(10);

    for (order, shape) in [
        (OrderKind::Random, "10M_random"),
        (OrderKind::Sorted, "10M_sorted"),
        (OrderKind::Reverse, "10M_reverse"),
    ] {
        let rows = 10_000_000;
        let raw = generate_ints(rows, Cardinality::High, order, 0.0);
        let values: Vec<i64> = raw.iter().map(|o| o.expect("no nulls")).collect();
        let ords: Vec<u64> = values.iter().map(|&v| (v as u64) ^ (1 << 63)).collect();
        let row_ids: Vec<u32> = (0..rows as u32).collect();

        let mut store = Store::new();
        store.add("val", Column::ints(raw)).expect("add int");

        let key = vec![SortKey {
            column: "val".into(),
            order: Order::Asc,
            nulls: Nulls::Last,
        }];

        g10.bench_function(format!("radix/{}", shape), |b| {
            b.iter(|| {
                store
                    .sort(black_box(&row_ids), black_box(&key))
                    .expect("sort")
            })
        });

        g10.bench_function(format!("std_ordinal/{}", shape), |b| {
            b.iter(|| sort_by_ordinal(black_box(&row_ids), black_box(&ords)))
        });

        g10.bench_function(format!("std_raw/{}", shape), |b| {
            b.iter(|| sort_by_raw_int(black_box(&row_ids), black_box(&values)))
        });
    }

    g10.finish();
}

fn bench_sort_one_float(c: &mut Criterion) {
    let mut g = c.benchmark_group("sort_one_float");

    let configs = [
        (100_000, OrderKind::Random, "100k_random"),
        (100_000, OrderKind::Sorted, "100k_sorted"),
        (100_000, OrderKind::Reverse, "100k_reverse"),
        (1_000_000, OrderKind::Random, "1M_random"),
        (1_000_000, OrderKind::Sorted, "1M_sorted"),
        (1_000_000, OrderKind::Reverse, "1M_reverse"),
    ];

    for (rows, order, shape) in configs {
        let raw = generate_floats(rows, order);
        let values: Vec<f64> = raw.iter().map(|o| o.expect("no nulls")).collect();
        let ords: Vec<u64> = values
            .iter()
            .map(|&v| {
                let b = v.to_bits();
                if (b >> 63) == 1 { !b } else { b | (1 << 63) }
            })
            .collect();
        let row_ids: Vec<u32> = (0..rows as u32).collect();

        let mut store = Store::new();
        store.add("val", Column::floats(raw)).expect("add float");

        let key = vec![SortKey {
            column: "val".into(),
            order: Order::Asc,
            nulls: Nulls::Last,
        }];

        g.bench_function(format!("radix/{}", shape), |b| {
            b.iter(|| {
                store
                    .sort(black_box(&row_ids), black_box(&key))
                    .expect("sort")
            })
        });

        g.bench_function(format!("std_ordinal/{}", shape), |b| {
            b.iter(|| sort_by_ordinal(black_box(&row_ids), black_box(&ords)))
        });

        g.bench_function(format!("std_raw/{}", shape), |b| {
            b.iter(|| sort_by_raw_float(black_box(&row_ids), black_box(&values)))
        });
    }

    g.finish();

    // 10M rows with reduced sample size
    let mut g10 = c.benchmark_group("sort_one_float");
    g10.sample_size(10);

    for (order, shape) in [
        (OrderKind::Random, "10M_random"),
        (OrderKind::Sorted, "10M_sorted"),
        (OrderKind::Reverse, "10M_reverse"),
    ] {
        let rows = 10_000_000;
        let raw = generate_floats(rows, order);
        let values: Vec<f64> = raw.iter().map(|o| o.expect("no nulls")).collect();
        let ords: Vec<u64> = values
            .iter()
            .map(|&v| {
                let b = v.to_bits();
                if (b >> 63) == 1 { !b } else { b | (1 << 63) }
            })
            .collect();
        let row_ids: Vec<u32> = (0..rows as u32).collect();

        let mut store = Store::new();
        store.add("val", Column::floats(raw)).expect("add float");

        let key = vec![SortKey {
            column: "val".into(),
            order: Order::Asc,
            nulls: Nulls::Last,
        }];

        g10.bench_function(format!("radix/{}", shape), |b| {
            b.iter(|| {
                store
                    .sort(black_box(&row_ids), black_box(&key))
                    .expect("sort")
            })
        });

        g10.bench_function(format!("std_ordinal/{}", shape), |b| {
            b.iter(|| sort_by_ordinal(black_box(&row_ids), black_box(&ords)))
        });

        g10.bench_function(format!("std_raw/{}", shape), |b| {
            b.iter(|| sort_by_raw_float(black_box(&row_ids), black_box(&values)))
        });
    }

    g10.finish();
}

// ---------------------------------------------------------------------------
// 5. sort_three_keys: LSD radix vs one comparator vs packed u128
// ---------------------------------------------------------------------------

fn sort_three_comparator(row_ids: &[u32], ords1: &[u64], ords2: &[u64], ords3: &[u64]) -> Vec<u32> {
    let mut p = row_ids.to_vec();
    p.sort_unstable_by(|&a, &b| {
        let ra = a as usize;
        let rb = b as usize;
        ords1[ra]
            .cmp(&ords1[rb])
            .then_with(|| ords2[ra].cmp(&ords2[rb]))
            .then_with(|| ords3[ra].cmp(&ords3[rb]))
    });
    p
}

fn sort_three_packed_u128(row_ids: &[u32], packed: &[u128]) -> Vec<u32> {
    let mut p = row_ids.to_vec();
    p.sort_unstable_by_key(|&r| packed[r as usize]);
    p
}

fn bench_sort_three_keys(c: &mut Criterion) {
    let mut g = c.benchmark_group("sort_three_keys");

    for (rows, shape) in [(100_000, "100k_random"), (1_000_000, "1M_random")] {
        let mut rng = XorShift64::new(BENCH_SEED);
        let strings_500 = generate_strings(rows, Cardinality::Medium);
        let strings_5 = generate_strings(rows, Cardinality::Low);

        let mut int32_vals = Vec::with_capacity(rows);
        for _ in 0..rows {
            let v = (rng.next_u64() % 1_000_000) as i64 - 500_000;
            int32_vals.push(Some(v));
        }

        let mut store = Store::new();
        store
            .add("k1_dict500", Column::dict_from(strings_500))
            .expect("add dict500");
        store
            .add("k2_dict5", Column::dict_from(strings_5))
            .expect("add dict5");
        store
            .add("k3_int32", Column::ints(int32_vals.clone()))
            .expect("add int32");

        let row_ids: Vec<u32> = (0..rows as u32).collect();
        let keys = vec![
            SortKey {
                column: "k1_dict500".into(),
                order: Order::Asc,
                nulls: Nulls::Last,
            },
            SortKey {
                column: "k2_dict5".into(),
                order: Order::Asc,
                nulls: Nulls::Last,
            },
            SortKey {
                column: "k3_int32".into(),
                order: Order::Asc,
                nulls: Nulls::Last,
            },
        ];

        let codes1: &[u32] = match store.column("k1_dict500").unwrap() {
            Column::Dict { codes, .. } => codes,
            _ => unreachable!(),
        };
        let codes2: &[u32] = match store.column("k2_dict5").unwrap() {
            Column::Dict { codes, .. } => codes,
            _ => unreachable!(),
        };
        let ints3: &[i64] = match store.column("k3_int32").unwrap() {
            Column::Int { values, .. } => values,
            _ => unreachable!(),
        };

        let ords1: Vec<u64> = codes1.iter().map(|&c| c as u64).collect();
        let ords2: Vec<u64> = codes2.iter().map(|&c| c as u64).collect();
        let ords3: Vec<u64> = ints3
            .iter()
            .map(|&v| ((v as i32 as u32) ^ (1 << 31)) as u64)
            .collect();

        let packed: Vec<u128> = (0..rows)
            .map(|i| ((codes1[i] as u128) << 64) | ((codes2[i] as u128) << 32) | (ords3[i] as u128))
            .collect();

        g.bench_function(format!("lsd/{}", shape), |b| {
            b.iter(|| {
                store
                    .sort(black_box(&row_ids), black_box(&keys))
                    .expect("sort")
            })
        });

        g.bench_function(format!("one_comparator/{}", shape), |b| {
            b.iter(|| {
                sort_three_comparator(
                    black_box(&row_ids),
                    black_box(&ords1),
                    black_box(&ords2),
                    black_box(&ords3),
                )
            })
        });

        g.bench_function(format!("packed_u128/{}", shape), |b| {
            b.iter(|| sort_three_packed_u128(black_box(&row_ids), black_box(&packed)))
        });
    }

    g.finish();

    // 10M rows with reduced sample size
    let mut g10 = c.benchmark_group("sort_three_keys");
    g10.sample_size(10);

    let rows = 10_000_000;
    let mut rng = XorShift64::new(BENCH_SEED);
    let strings_500 = generate_strings(rows, Cardinality::Medium);
    let strings_5 = generate_strings(rows, Cardinality::Low);

    let mut int32_vals = Vec::with_capacity(rows);
    for _ in 0..rows {
        let v = (rng.next_u64() % 1_000_000) as i64 - 500_000;
        int32_vals.push(Some(v));
    }

    let mut store = Store::new();
    store
        .add("k1_dict500", Column::dict_from(strings_500))
        .expect("add dict500");
    store
        .add("k2_dict5", Column::dict_from(strings_5))
        .expect("add dict5");
    store
        .add("k3_int32", Column::ints(int32_vals.clone()))
        .expect("add int32");

    let row_ids: Vec<u32> = (0..rows as u32).collect();
    let keys = vec![
        SortKey {
            column: "k1_dict500".into(),
            order: Order::Asc,
            nulls: Nulls::Last,
        },
        SortKey {
            column: "k2_dict5".into(),
            order: Order::Asc,
            nulls: Nulls::Last,
        },
        SortKey {
            column: "k3_int32".into(),
            order: Order::Asc,
            nulls: Nulls::Last,
        },
    ];

    let codes1: &[u32] = match store.column("k1_dict500").unwrap() {
        Column::Dict { codes, .. } => codes,
        _ => unreachable!(),
    };
    let codes2: &[u32] = match store.column("k2_dict5").unwrap() {
        Column::Dict { codes, .. } => codes,
        _ => unreachable!(),
    };
    let ints3: &[i64] = match store.column("k3_int32").unwrap() {
        Column::Int { values, .. } => values,
        _ => unreachable!(),
    };

    let ords1: Vec<u64> = codes1.iter().map(|&c| c as u64).collect();
    let ords2: Vec<u64> = codes2.iter().map(|&c| c as u64).collect();
    let ords3: Vec<u64> = ints3
        .iter()
        .map(|&v| ((v as i32 as u32) ^ (1 << 31)) as u64)
        .collect();

    let packed: Vec<u128> = (0..rows)
        .map(|i| ((codes1[i] as u128) << 64) | ((codes2[i] as u128) << 32) | (ords3[i] as u128))
        .collect();

    g10.bench_function("lsd/10M_random", |b| {
        b.iter(|| {
            store
                .sort(black_box(&row_ids), black_box(&keys))
                .expect("sort")
        })
    });

    g10.bench_function("one_comparator/10M_random", |b| {
        b.iter(|| {
            sort_three_comparator(
                black_box(&row_ids),
                black_box(&ords1),
                black_box(&ords2),
                black_box(&ords3),
            )
        })
    });

    g10.bench_function("packed_u128/10M_random", |b| {
        b.iter(|| sort_three_packed_u128(black_box(&row_ids), black_box(&packed)))
    });

    g10.finish();
}

// ---------------------------------------------------------------------------
// 6. filter_selection: Bitmap Result vs Vec<u32> Row IDs
// ---------------------------------------------------------------------------

fn filter_to_row_ids(values: &[i64], threshold: i64) -> u64 {
    let mut matching = Vec::new();
    for (i, &v) in values.iter().enumerate() {
        if v < threshold {
            matching.push(i as u32);
        }
    }
    let mut sum = 0u64;
    for id in matching {
        sum = sum.wrapping_add(id as u64);
    }
    sum
}

fn bench_filter_selection(c: &mut Criterion) {
    let mut g = c.benchmark_group("filter_selection");

    let configs = [
        (100_000, 1, "100k_1p"),
        (100_000, 50, "100k_50p"),
        (100_000, 99, "100k_99p"),
        (1_000_000, 1, "1M_1p"),
        (1_000_000, 50, "1M_50p"),
        (1_000_000, 99, "1M_99p"),
    ];

    for (rows, sel_pct, shape) in configs {
        let threshold = (rows * sel_pct / 100) as i64;
        let ints: Vec<Option<i64>> = (0..rows as i64).map(Some).collect();
        let raw_values: Vec<i64> = (0..rows as i64).collect();

        let mut store = Store::new();
        store.add("val", Column::ints(ints)).expect("add int");

        let filter = Filter::Lt("val".into(), Owned::Int(threshold));

        g.bench_function(format!("bitmap/{}", shape), |b| {
            b.iter(|| {
                let bm = store.filter(black_box(&filter)).unwrap();
                let mut sum = 0u64;
                for id in bm.ones() {
                    sum = sum.wrapping_add(id as u64);
                }
                black_box(sum)
            })
        });

        g.bench_function(format!("row_ids/{}", shape), |b| {
            b.iter(|| filter_to_row_ids(black_box(&raw_values), black_box(threshold)))
        });
    }

    g.finish();
}

// ---------------------------------------------------------------------------
// 7. group_one_key: Dense counting vs HashMap vs Sort-then-scan
// ---------------------------------------------------------------------------

fn group_by_hash_map(codes: &[u32]) -> HashMap<u64, usize> {
    let mut map: HashMap<u64, usize> = HashMap::new();
    for &c in codes {
        *map.entry(c as u64).or_insert(0) += 1;
    }
    map
}

fn group_by_sort_scan(codes: &[u32]) -> Vec<(u64, usize)> {
    let mut sorted = codes.to_vec();
    sorted.sort_unstable();
    let mut groups = Vec::new();
    if sorted.is_empty() {
        return groups;
    }
    let mut cur_key = sorted[0];
    let mut cur_count = 1usize;
    for &k in &sorted[1..] {
        if k == cur_key {
            cur_count += 1;
        } else {
            groups.push((cur_key as u64, cur_count));
            cur_key = k;
            cur_count = 1;
        }
    }
    groups.push((cur_key as u64, cur_count));
    groups
}

fn group_by_sort_scan_rows(codes: &[u32]) -> Vec<(u64, Vec<u32>)> {
    let mut pairs: Vec<(u32, u32)> = codes
        .iter()
        .enumerate()
        .map(|(i, &c)| (c, i as u32))
        .collect();
    pairs.sort_unstable_by_key(|&(c, _)| c);
    let mut groups = Vec::new();
    if pairs.is_empty() {
        return groups;
    }
    let mut cur_key = pairs[0].0;
    let mut cur_rows = vec![pairs[0].1];
    for &(k, r) in &pairs[1..] {
        if k == cur_key {
            cur_rows.push(r);
        } else {
            groups.push((cur_key as u64, cur_rows));
            cur_key = k;
            cur_rows = vec![r];
        }
    }
    groups.push((cur_key as u64, cur_rows));
    groups
}

fn bench_group_one_key(c: &mut Criterion) {
    let mut g = c.benchmark_group("group_one_key");

    let configs = [
        (100_000, Cardinality::Low, "100k_low"),
        (100_000, Cardinality::Medium, "100k_med"),
        (100_000, Cardinality::High, "100k_high"),
        (1_000_000, Cardinality::Low, "1M_low"),
        (1_000_000, Cardinality::Medium, "1M_med"),
        (1_000_000, Cardinality::High, "1M_high"),
    ];

    for (rows, card, shape) in configs {
        let strings = generate_strings(rows, card);
        let row_ids: Vec<u32> = (0..rows as u32).collect();

        let mut store = Store::new();
        store
            .add("key", Column::dict_from(strings))
            .expect("add dict");

        let codes: Vec<u32> = match store.column("key").unwrap() {
            Column::Dict { codes, .. } => codes.clone(),
            _ => unreachable!(),
        };

        let col_name = vec!["key".to_string()];

        g.bench_function(format!("dense/{}", shape), |b| {
            b.iter(|| {
                store
                    .group(black_box(&row_ids), black_box(&col_name), &[])
                    .unwrap()
            })
        });

        g.bench_function(format!("hash_map/{}", shape), |b| {
            b.iter(|| group_by_hash_map(black_box(&codes)))
        });

        g.bench_function(format!("sort_scan/{}", shape), |b| {
            b.iter(|| group_by_sort_scan(black_box(&codes)))
        });

        g.bench_function(format!("sort_scan_rows/{}", shape), |b| {
            b.iter(|| group_by_sort_scan_rows(black_box(&codes)))
        });
    }

    g.finish();
}

// ---------------------------------------------------------------------------
// 8. locate_eq: Prebuilt Hash vs Binary Search vs Linear Scan and Build Costs
// ---------------------------------------------------------------------------

fn locate_bin_search(sorted_rows: &[u32], values: &[i64], target: i64) -> Option<u32> {
    let idx = sorted_rows.partition_point(|&r| values[r as usize] < target);
    if idx < sorted_rows.len() && values[sorted_rows[idx] as usize] == target {
        Some(sorted_rows[idx])
    } else {
        None
    }
}

fn locate_linear_scan(values: &[i64], target: i64) -> Option<u32> {
    for (i, &v) in values.iter().enumerate() {
        if v == target {
            return Some(i as u32);
        }
    }
    None
}

fn build_sorted_permutation(values: &[i64]) -> Vec<u32> {
    let mut p: Vec<u32> = (0..values.len() as u32).collect();
    p.sort_unstable_by_key(|&r| values[r as usize]);
    p
}

fn bench_locate_eq(c: &mut Criterion) {
    let mut g = c.benchmark_group("locate_eq");

    for (rows, shape) in [(100_000, "100k"), (1_000_000, "1M")] {
        let ints = generate_ints(rows, Cardinality::High, OrderKind::Random, 0.0);
        let raw_values: Vec<i64> = ints.iter().map(|o| o.expect("no nulls")).collect();

        let mut store = Store::new();
        store.add("id", Column::ints(ints)).expect("add id");

        let target_val = raw_values[rows / 2];
        let query_owned = Owned::Int(target_val);

        let mut locator_prebuilt = Locator::new();
        locator_prebuilt
            .find_eq(&store, "id", &query_owned)
            .expect("build initial index");

        let sorted_rows = build_sorted_permutation(&raw_values);

        g.bench_function(format!("hash_query/{}", shape), |b| {
            b.iter(|| {
                locator_prebuilt
                    .find_eq(&store, "id", black_box(&query_owned))
                    .unwrap()
            })
        });

        g.bench_function(format!("bin_search_query/{}", shape), |b| {
            b.iter(|| {
                locate_bin_search(
                    black_box(&sorted_rows),
                    black_box(&raw_values),
                    black_box(target_val),
                )
            })
        });

        g.bench_function(format!("linear_scan/{}", shape), |b| {
            b.iter(|| locate_linear_scan(black_box(&raw_values), black_box(target_val)))
        });

        g.bench_function(format!("hash_build/{}", shape), |b| {
            b.iter(|| {
                let mut loc = Locator::new();
                loc.find_eq(&store, "id", black_box(&query_owned)).unwrap()
            })
        });

        g.bench_function(format!("bin_search_build/{}", shape), |b| {
            b.iter(|| build_sorted_permutation(black_box(&raw_values)))
        });
    }

    g.finish();
}

// ---------------------------------------------------------------------------
// Criterion Runner Configuration
// ---------------------------------------------------------------------------

criterion_group!(benches_layout, bench_layout_scan,);

criterion_group!(benches_text, bench_text_storage,);

criterion_group!(benches_nulls, bench_nulls_repr,);

criterion_group!(
    benches_sort,
    bench_sort_one_int,
    bench_sort_one_float,
    bench_sort_three_keys,
);

criterion_group!(
    benches_search,
    bench_filter_selection,
    bench_group_one_key,
    bench_locate_eq,
);

criterion_main!(
    benches_layout,
    benches_text,
    benches_nulls,
    benches_sort,
    benches_search,
);
