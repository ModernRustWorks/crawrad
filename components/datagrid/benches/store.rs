use criterion::{Criterion, black_box, criterion_group, criterion_main};
use datagrid::{
    Aggregate, Column, Filter, GroupLevel, Locator, Nulls, Order, Owned, Query, SortKey, Store,
    Total, View,
};
use std::collections::HashMap;

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
}

#[derive(Clone)]
struct RowStruct {
    _id: i64,
    amount: f64,
    city: String,
    _status: String,
    _name: String,
}

struct BenchRawData {
    amounts: Vec<f64>,
    cities: Vec<String>,
    statuses: Vec<String>,
}

fn generate_bench_raw(rows: usize) -> BenchRawData {
    let mut rng = XorShift64::new(0x123456789abcdef0);
    let city_names: Vec<String> = (0..500).map(|i| format!("City_{:03}", i)).collect();
    let status_names: Vec<String> = (0..5).map(|i| format!("Status_{}", i)).collect();

    let mut amounts = Vec::with_capacity(rows);
    let mut cities = Vec::with_capacity(rows);
    let mut statuses = Vec::with_capacity(rows);

    for _ in 0..rows {
        let amt = (((rng.next_u64() % 100_000) as f64) / 10.0) + 0.5;
        let c_idx = (rng.next_u64() % 500) as usize;
        let s_idx = (rng.next_u64() % 5) as usize;

        amounts.push(amt);
        cities.push(city_names[c_idx].clone());
        statuses.push(status_names[s_idx].clone());
    }

    BenchRawData {
        amounts,
        cities,
        statuses,
    }
}

fn build_bench_store(raw: &BenchRawData) -> Store {
    let rows = raw.amounts.len();
    let ids: Vec<Option<i64>> = (0..rows as i64).map(Some).collect();
    let amounts: Vec<Option<f64>> = raw.amounts.iter().copied().map(Some).collect();
    let cities: Vec<Option<String>> = raw.cities.iter().cloned().map(Some).collect();
    let statuses: Vec<Option<String>> = raw.statuses.iter().cloned().map(Some).collect();

    let names: Vec<Option<String>> = (0..rows)
        .map(|i| Some(format!("Customer_{}", i % 10000)))
        .collect();

    let mut store = Store::new();
    store.add("id", Column::ints(ids)).expect("add id");
    store
        .add("amount", Column::floats(amounts))
        .expect("add amount");
    store
        .add("city", Column::dict_from(cities))
        .expect("add city");
    store
        .add("status", Column::dict_from(statuses))
        .expect("add status");
    store.add("name", Column::texts(names)).expect("add name");
    store
}

fn build_naive_rows(raw: &BenchRawData) -> Vec<RowStruct> {
    let rows = raw.amounts.len();
    (0..rows)
        .map(|i| RowStruct {
            _id: i as i64,
            amount: raw.amounts[i],
            city: raw.cities[i].clone(),
            _status: raw.statuses[i].clone(),
            _name: format!("Customer_{}", i % 10000),
        })
        .collect()
}

fn bench_1m(c: &mut Criterion) {
    let mut g = c.benchmark_group("1M");
    let rows = 1_000_000;
    let raw = generate_bench_raw(rows);
    let store = build_bench_store(&raw);
    let row_ids: Vec<u32> = (0..rows as u32).collect();

    // 1. Sort by amount
    let key_amount = vec![SortKey {
        column: "amount".into(),
        order: Order::Asc,
        nulls: Nulls::Last,
    }];
    g.bench_function("sort_by_amount", |b| {
        b.iter(|| {
            store
                .sort(black_box(&row_ids), black_box(&key_amount))
                .unwrap()
        })
    });

    // Naive baseline: sort by amount
    let naive_rows = build_naive_rows(&raw);
    g.bench_function("naive_sort_by_amount", |b| {
        b.iter(|| {
            let mut v = naive_rows.clone();
            v.sort_by(|a, b| a.amount.partial_cmp(&b.amount).unwrap());
            v
        })
    });

    // 2. Sort by city, status, amount
    let keys_multi = vec![
        SortKey {
            column: "city".into(),
            order: Order::Asc,
            nulls: Nulls::Last,
        },
        SortKey {
            column: "status".into(),
            order: Order::Asc,
            nulls: Nulls::Last,
        },
        SortKey {
            column: "amount".into(),
            order: Order::Asc,
            nulls: Nulls::Last,
        },
    ];
    g.bench_function("sort_by_city_status_amount", |b| {
        b.iter(|| {
            store
                .sort(black_box(&row_ids), black_box(&keys_multi))
                .unwrap()
        })
    });

    // 3. Filter status == "Status_1" and amount > 100.0
    let filter = Filter::And(vec![
        Filter::Eq("status".into(), Owned::Text("Status_1".into())),
        Filter::Gt("amount".into(), Owned::Float(100.0)),
    ]);
    g.bench_function("filter_status_and_amount", |b| {
        b.iter(|| store.filter(black_box(&filter)).unwrap())
    });

    // 4. Group by city with Sum(amount) and Count
    let totals = vec![
        Total {
            column: "amount".into(),
            aggregate: Aggregate::Sum,
        },
        Total {
            column: "amount".into(),
            aggregate: Aggregate::Count,
        },
    ];
    g.bench_function("group_by_city_with_totals", |b| {
        b.iter(|| {
            store
                .group(
                    black_box(&row_ids),
                    black_box(&["city".into()]),
                    black_box(&totals),
                )
                .unwrap()
        })
    });

    // Naive baseline: group by city with sum & count
    g.bench_function("naive_group_by_city", |b| {
        b.iter(|| {
            let mut map: HashMap<String, (f64, usize)> = HashMap::new();
            for r in &naive_rows {
                let entry = map.entry(r.city.clone()).or_insert((0.0, 0));
                entry.0 += r.amount;
                entry.1 += 1;
            }
            map
        })
    });

    // 5. Group by city then status
    g.bench_function("group_by_city_then_status", |b| {
        b.iter(|| {
            let groups = store
                .group(black_box(&row_ids), black_box(&["city".into()]), &[])
                .unwrap();
            for grp in &groups {
                let _ = store
                    .group(&grp.rows, black_box(&["status".into()]), &[])
                    .unwrap();
            }
        })
    });

    // 6. Locate eq on id (pre-built vs including build)
    let mut locator_prebuilt = Locator::new();
    let query_id = Owned::Int(500_000);
    locator_prebuilt
        .find_eq(&store, "id", &query_id)
        .expect("prebuilt index");

    g.bench_function("locate_eq_id_prebuilt", |b| {
        b.iter(|| {
            locator_prebuilt
                .find_eq(&store, "id", black_box(&query_id))
                .unwrap()
        })
    });

    g.bench_function("locate_eq_id_including_build", |b| {
        b.iter(|| {
            let mut loc = Locator::new();
            loc.find_eq(&store, "id", black_box(&query_id)).unwrap()
        })
    });

    // 7. View::new with filter + sort by amount + group by city then status
    let q_view = Query {
        filter: Some(Filter::And(vec![
            Filter::Eq("status".into(), Owned::Text("Status_1".into())),
            Filter::Gt("amount".into(), Owned::Float(100.0)),
        ])),
        sort: vec![SortKey {
            column: "amount".into(),
            order: Order::Asc,
            nulls: Nulls::Last,
        }],
        group_by: vec![sum_level("city"), sum_level("status")],
        summary: Vec::new(),
    };

    g.bench_function("view_new", |b| {
        b.iter(|| View::new(black_box(&store), black_box(q_view.clone())).unwrap())
    });

    // 8. View::requery changing only the grouping
    let q_regroup_a = q_view.clone();
    let mut q_regroup_b = q_view.clone();
    q_regroup_b.group_by = vec![sum_level("status"), sum_level("city")];

    let mut view = View::new(&store, q_view.clone()).unwrap();
    let mut toggle = false;
    g.bench_function("view_requery_change_grouping", |b| {
        b.iter(|| {
            let q = if toggle { &q_regroup_a } else { &q_regroup_b };
            toggle = !toggle;
            view.requery(&store, black_box(q.clone())).unwrap()
        })
    });

    g.finish();
}

fn bench_10m(c: &mut Criterion) {
    let mut g = c.benchmark_group("10M");
    g.sample_size(10);

    let rows = 10_000_000;
    let raw = generate_bench_raw(rows);
    let store = build_bench_store(&raw);
    let row_ids: Vec<u32> = (0..rows as u32).collect();

    // 1. Sort by amount
    let key_amount = vec![SortKey {
        column: "amount".into(),
        order: Order::Asc,
        nulls: Nulls::Last,
    }];
    g.bench_function("sort_by_amount", |b| {
        b.iter(|| {
            store
                .sort(black_box(&row_ids), black_box(&key_amount))
                .unwrap()
        })
    });

    // 2. Sort by city, status, amount
    let keys_multi = vec![
        SortKey {
            column: "city".into(),
            order: Order::Asc,
            nulls: Nulls::Last,
        },
        SortKey {
            column: "status".into(),
            order: Order::Asc,
            nulls: Nulls::Last,
        },
        SortKey {
            column: "amount".into(),
            order: Order::Asc,
            nulls: Nulls::Last,
        },
    ];
    g.bench_function("sort_by_city_status_amount", |b| {
        b.iter(|| {
            store
                .sort(black_box(&row_ids), black_box(&keys_multi))
                .unwrap()
        })
    });

    // 3. Filter status == "Status_1" and amount > 100.0
    let filter = Filter::And(vec![
        Filter::Eq("status".into(), Owned::Text("Status_1".into())),
        Filter::Gt("amount".into(), Owned::Float(100.0)),
    ]);
    g.bench_function("filter_status_and_amount", |b| {
        b.iter(|| store.filter(black_box(&filter)).unwrap())
    });

    // 4. Group by city with Sum(amount) and Count
    let totals = vec![
        Total {
            column: "amount".into(),
            aggregate: Aggregate::Sum,
        },
        Total {
            column: "amount".into(),
            aggregate: Aggregate::Count,
        },
    ];
    g.bench_function("group_by_city_with_totals", |b| {
        b.iter(|| {
            store
                .group(
                    black_box(&row_ids),
                    black_box(&["city".into()]),
                    black_box(&totals),
                )
                .unwrap()
        })
    });

    // 5. Group by city then status
    g.bench_function("group_by_city_then_status", |b| {
        b.iter(|| {
            let groups = store
                .group(black_box(&row_ids), black_box(&["city".into()]), &[])
                .unwrap();
            for grp in &groups {
                let _ = store
                    .group(&grp.rows, black_box(&["status".into()]), &[])
                    .unwrap();
            }
        })
    });

    // 6. Locate eq on id (prebuilt vs including build)
    let mut locator_prebuilt = Locator::new();
    let query_id = Owned::Int(5_000_000);
    locator_prebuilt
        .find_eq(&store, "id", &query_id)
        .expect("prebuilt index");

    g.bench_function("locate_eq_id_prebuilt", |b| {
        b.iter(|| {
            locator_prebuilt
                .find_eq(&store, "id", black_box(&query_id))
                .unwrap()
        })
    });

    g.bench_function("locate_eq_id_including_build", |b| {
        b.iter(|| {
            let mut loc = Locator::new();
            loc.find_eq(&store, "id", black_box(&query_id)).unwrap()
        })
    });

    g.finish();
}

criterion_group!(benches, bench_1m, bench_10m);
criterion_main!(benches);

/// A grouping level carrying the amount sum, as every level did before per-level totals.
fn sum_level(column: &str) -> GroupLevel {
    GroupLevel {
        column: column.into(),
        totals: vec![Total {
            column: "amount".into(),
            aggregate: Aggregate::Sum,
        }],
    }
}
