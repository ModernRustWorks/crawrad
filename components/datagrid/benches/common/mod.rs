//! Shared benchmark fixtures and deterministic data generators.

use datagrid::{Column, Store};

/// Constant seed for deterministic and reproducible benchmarks across runs.
pub const BENCH_SEED: u64 = 0x853c_49e6_748f_ea9b;

/// Fast, deterministic 64-bit pseudo-random number generator (XorShift64).
#[derive(Clone, Copy, Debug)]
pub struct XorShift64(u64);

impl XorShift64 {
    /// Creates a new generator from a seed, falling back to a fixed non-zero constant if 0.
    pub fn new(seed: u64) -> Self {
        Self(if seed == 0 { BENCH_SEED } else { seed })
    }

    /// Generates the next pseudo-random `u64`.
    pub fn next_u64(&mut self) -> u64 {
        let mut x = self.0;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.0 = x;
        x
    }
}

/// Data cardinality (distinct value count).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cardinality {
    /// 5 distinct values.
    Low,
    /// 500 distinct values.
    Medium,
    /// All values distinct.
    High,
}

/// Presortedness order of input data.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OrderKind {
    Random,
    Sorted,
    Reverse,
}

/// Row structure representing a row-oriented database record for comparison.
#[derive(Clone, Debug, PartialEq)]
pub struct BenchRow {
    pub id: i64,
    pub amount: f64,
    pub category: i64,
    pub extra: u64,
}

/// Generates a dataset in both row-oriented (`Vec<BenchRow>`) and columnar (`Store`) format.
pub fn generate_layout_dataset(rows: usize) -> (Vec<BenchRow>, Store) {
    let mut rng = XorShift64::new(BENCH_SEED);
    let mut rows_vec = Vec::with_capacity(rows);
    let mut ids = Vec::with_capacity(rows);
    let mut amounts = Vec::with_capacity(rows);
    let mut categories = Vec::with_capacity(rows);
    let mut extras = Vec::with_capacity(rows);

    for i in 0..rows {
        let id = i as i64;
        let amt = (((rng.next_u64() % 100_000) as f64) / 10.0) + 0.5;
        let cat = (rng.next_u64() % 50) as i64;
        let ext = rng.next_u64();

        rows_vec.push(BenchRow {
            id,
            amount: amt,
            category: cat,
            extra: ext,
        });

        ids.push(Some(id));
        amounts.push(Some(amt));
        categories.push(Some(cat));
        extras.push(Some(ext as i64));
    }

    let mut store = Store::new();
    store.add("id", Column::ints(ids)).expect("add id");
    store
        .add("amount", Column::floats(amounts))
        .expect("add amount");
    store
        .add("category", Column::ints(categories))
        .expect("add category");
    store.add("extra", Column::ints(extras)).expect("add extra");

    (rows_vec, store)
}

/// Generates an integer column vector matching the specified cardinality, order, and null ratio.
pub fn generate_ints(
    rows: usize,
    card: Cardinality,
    order: OrderKind,
    null_ratio: f64,
) -> Vec<Option<i64>> {
    let mut rng = XorShift64::new(BENCH_SEED);
    let mut raw = Vec::with_capacity(rows);

    match card {
        Cardinality::Low => {
            for _ in 0..rows {
                raw.push((rng.next_u64() % 5) as i64);
            }
        }
        Cardinality::Medium => {
            for _ in 0..rows {
                raw.push((rng.next_u64() % 500) as i64);
            }
        }
        Cardinality::High => {
            for i in 0..rows {
                raw.push(i as i64);
            }
            if order == OrderKind::Random {
                // Fisher-Yates shuffle
                for i in (1..rows).rev() {
                    let j = (rng.next_u64() as usize) % (i + 1);
                    raw.swap(i, j);
                }
            }
        }
    }

    match order {
        OrderKind::Random => {}
        OrderKind::Sorted => raw.sort_unstable(),
        OrderKind::Reverse => {
            raw.sort_unstable();
            raw.reverse();
        }
    }

    if null_ratio > 0.0 {
        let period = (1.0 / null_ratio).round() as usize;
        let effective_period = period.max(1);
        raw.into_iter()
            .enumerate()
            .map(|(i, v)| {
                if i % effective_period == 0 {
                    None
                } else {
                    Some(v)
                }
            })
            .collect()
    } else {
        raw.into_iter().map(Some).collect()
    }
}

/// Generates a float column vector with the specified order.
pub fn generate_floats(rows: usize, order: OrderKind) -> Vec<Option<f64>> {
    let mut rng = XorShift64::new(BENCH_SEED);
    let mut raw = Vec::with_capacity(rows);

    for _ in 0..rows {
        let v = (((rng.next_u64() % 1_000_000) as f64) / 10.0) + 0.5;
        raw.push(v);
    }

    match order {
        OrderKind::Random => {}
        OrderKind::Sorted => raw.sort_by(|a, b| a.total_cmp(b)),
        OrderKind::Reverse => {
            raw.sort_by(|a, b| b.total_cmp(a));
        }
    }

    raw.into_iter().map(Some).collect()
}

/// Generates a string vector with low, medium, or high cardinality.
pub fn generate_strings(rows: usize, card: Cardinality) -> Vec<Option<String>> {
    let mut rng = XorShift64::new(BENCH_SEED);
    match card {
        Cardinality::Low => {
            let pool: Vec<String> = (0..5).map(|i| format!("Category_{:02}", i)).collect();
            let mut out = Vec::with_capacity(rows);
            for _ in 0..rows {
                let idx = (rng.next_u64() % 5) as usize;
                out.push(Some(pool[idx].clone()));
            }
            out
        }
        Cardinality::Medium => {
            let pool: Vec<String> = (0..500).map(|i| format!("City_{:04}", i)).collect();
            let mut out = Vec::with_capacity(rows);
            for _ in 0..rows {
                let idx = (rng.next_u64() % 500) as usize;
                out.push(Some(pool[idx].clone()));
            }
            out
        }
        Cardinality::High => {
            let mut out = Vec::with_capacity(rows);
            for i in 0..rows {
                out.push(Some(format!("Key_{:08}", i)));
            }
            // Shuffle
            for i in (1..rows).rev() {
                let j = (rng.next_u64() as usize) % (i + 1);
                out.swap(i, j);
            }
            out
        }
    }
}
