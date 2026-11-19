//! #621: what a query costs through the portadb — the whole result
//! (`query`) against the row cursor (`query_each`, step 3), against typed
//! decoding (`query_each_as` / `FromRow`, step 4), and dyn vs typed connection
//! dispatch (`Connection::open_typed`, step 5).
//!
//! `cargo bench -p portadb --bench query_rows --features rusqlite,sqlx-sqlite`
//! runs the timings (criterion). `-- --report` prints only the allocation
//! table (allocations, bytes, peak memory, time, time to the first row) — the
//! numbers the PR quotes; CI runs that mode as a report, never as a gate.
//!
//! The table is 10 columns (5 INTEGER, 5 TEXT of 12 characters) in an
//! in-memory SQLite. `raw rusqlite` is the floor: the same rows read straight
//! into `Vec<Vec<rusqlite::types::Value>>`, with no abstraction on top.

use std::alloc::{GlobalAlloc, Layout, System};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::{Duration, Instant};

use criterion::{BenchmarkId, Criterion};
use portadb::Connection;
use portadb::drivers::rusqlite_driver::{Connection as RusqliteConn, RusqliteDriver};
use portadb::drivers::sqlx_driver::{Connection as SqlxSqliteConn, SqlxDriver};
use portadb::options::ConnectOptions;
use portadb::params::NamedParams;

struct Counting;

static ALLOCS: AtomicUsize = AtomicUsize::new(0);
static BYTES: AtomicUsize = AtomicUsize::new(0);
static LIVE: AtomicUsize = AtomicUsize::new(0);
static PEAK: AtomicUsize = AtomicUsize::new(0);

// SAFETY: every call is forwarded to `System`; the counters only observe.
unsafe impl GlobalAlloc for Counting {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        ALLOCS.fetch_add(1, Ordering::Relaxed);
        BYTES.fetch_add(layout.size(), Ordering::Relaxed);
        let live = LIVE.fetch_add(layout.size(), Ordering::Relaxed) + layout.size();
        PEAK.fetch_max(live, Ordering::Relaxed);
        // SAFETY: same layout, straight to the system allocator.
        unsafe { System.alloc(layout) }
    }
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        LIVE.fetch_sub(layout.size(), Ordering::Relaxed);
        // SAFETY: `ptr` came from `alloc` above with this layout.
        unsafe { System.dealloc(ptr, layout) }
    }
}

#[global_allocator]
static GLOBAL: Counting = Counting;

const SIZES: [usize; 2] = [1_000, 100_000];
const SELECT: &str = "SELECT c0, c1, c2, c3, c4, c5, c6, c7, c8, c9 FROM t";

fn seed_sql(rows: usize) -> [String; 2] {
    [
        "CREATE TABLE t (c0 INTEGER, c1 INTEGER, c2 INTEGER, c3 INTEGER, c4 INTEGER, \
         c5 TEXT, c6 TEXT, c7 TEXT, c8 TEXT, c9 TEXT)"
            .to_string(),
        format!(
            "WITH RECURSIVE n(i) AS (SELECT 1 UNION ALL SELECT i + 1 FROM n WHERE i < {rows}) \
             INSERT INTO t SELECT i, i * 2, i * 3, i * 4, i * 5, \
             printf('text-%07d', i), printf('text-%07d', i), printf('text-%07d', i), \
             printf('text-%07d', i), printf('text-%07d', i) FROM n"
        ),
    ]
}

struct Wide {
    c0: i64,
    c1: i64,
    c2: i64,
    c3: i64,
    c4: i64,
    c5: String,
    c6: String,
    c7: String,
    c8: String,
    c9: String,
}

impl Wide {
    fn touch(&self) -> usize {
        (self.c0 + self.c1 + self.c2 + self.c3 + self.c4) as usize
            + self.c5.len()
            + self.c6.len()
            + self.c7.len()
            + self.c8.len()
            + self.c9.len()
    }
}

struct WidePlan {
    idx: [usize; 10],
}

impl portadb::FromRow for Wide {
    type Plan = WidePlan;

    fn plan(columns: &[String]) -> Result<Self::Plan, portadb::Error> {
        let idx = [
            portadb::row_ref::column_index(columns, "c0")?,
            portadb::row_ref::column_index(columns, "c1")?,
            portadb::row_ref::column_index(columns, "c2")?,
            portadb::row_ref::column_index(columns, "c3")?,
            portadb::row_ref::column_index(columns, "c4")?,
            portadb::row_ref::column_index(columns, "c5")?,
            portadb::row_ref::column_index(columns, "c6")?,
            portadb::row_ref::column_index(columns, "c7")?,
            portadb::row_ref::column_index(columns, "c8")?,
            portadb::row_ref::column_index(columns, "c9")?,
        ];
        Ok(WidePlan { idx })
    }

    fn from_row<R: portadb::RowRef + ?Sized>(
        plan: &Self::Plan,
        row: &R,
    ) -> Result<Self, portadb::Error> {
        Ok(Wide {
            c0: portadb::row_ref::decode(row, plan.idx[0])?,
            c1: portadb::row_ref::decode(row, plan.idx[1])?,
            c2: portadb::row_ref::decode(row, plan.idx[2])?,
            c3: portadb::row_ref::decode(row, plan.idx[3])?,
            c4: portadb::row_ref::decode(row, plan.idx[4])?,
            c5: portadb::row_ref::decode(row, plan.idx[5])?,
            c6: portadb::row_ref::decode(row, plan.idx[6])?,
            c7: portadb::row_ref::decode(row, plan.idx[7])?,
            c8: portadb::row_ref::decode(row, plan.idx[8])?,
            c9: portadb::row_ref::decode(row, plan.idx[9])?,
        })
    }
}

async fn engine_conn(engine: &str, rows: usize) -> Connection {
    let mut conn = match engine {
        "rusqlite" => Connection::open(RusqliteDriver::new(), &ConnectOptions::new(":memory:"))
            .await
            .expect("rusqlite connects"),
        _ => Connection::open(SqlxDriver::new(), &ConnectOptions::new("sqlite::memory:"))
            .await
            .expect("sqlx connects"),
    };
    for sql in seed_sql(rows) {
        conn.execute(&sql, &NamedParams::new())
            .await
            .expect("seed runs");
    }
    conn
}

async fn typed_rusqlite_conn(rows: usize) -> Connection<RusqliteConn> {
    let driver = RusqliteDriver::new();
    let mut conn = Connection::open_typed(&driver, &ConnectOptions::new(":memory:"))
        .await
        .expect("typed rusqlite connects");
    for sql in seed_sql(rows) {
        conn.execute(&sql, &NamedParams::new())
            .await
            .expect("seed runs");
    }
    conn
}

async fn typed_sqlx_conn(rows: usize) -> Connection<SqlxSqliteConn> {
    let driver = SqlxDriver::new();
    let mut conn = Connection::open_typed(&driver, &ConnectOptions::new("sqlite::memory:"))
        .await
        .expect("typed sqlx connects");
    for sql in seed_sql(rows) {
        conn.execute(&sql, &NamedParams::new())
            .await
            .expect("seed runs");
    }
    conn
}

fn raw_conn(rows: usize) -> rusqlite::Connection {
    let conn = rusqlite::Connection::open_in_memory().expect("raw connects");
    for sql in seed_sql(rows) {
        conn.execute(&sql, []).expect("seed runs");
    }
    conn
}

/// Reads every row; returns how many, and when the first one was in hand.
fn raw_query(conn: &rusqlite::Connection, start: Instant) -> (usize, Duration) {
    let mut stmt = conn.prepare(SELECT).expect("prepares");
    let cols = stmt.column_count();
    let mut out: Vec<Vec<rusqlite::types::Value>> = Vec::new();
    let mut first = None;
    let mut rows = stmt.query([]).expect("queries");
    while let Some(r) = rows.next().expect("steps") {
        out.push((0..cols).map(|i| r.get(i).expect("reads")).collect());
        first.get_or_insert_with(|| start.elapsed());
    }
    (out.len(), first.unwrap_or_default())
}

struct Measure {
    allocs: usize,
    bytes: usize,
    peak: usize,
    time: Duration,
    first_row: Duration,
}

fn measure(run: impl FnOnce() -> Duration) -> Measure {
    let (a0, b0) = (
        ALLOCS.load(Ordering::Relaxed),
        BYTES.load(Ordering::Relaxed),
    );
    PEAK.store(LIVE.load(Ordering::Relaxed), Ordering::Relaxed);
    let base = LIVE.load(Ordering::Relaxed);
    let start = Instant::now();
    let first_row = run();
    Measure {
        time: start.elapsed(),
        allocs: ALLOCS.load(Ordering::Relaxed) - a0,
        bytes: BYTES.load(Ordering::Relaxed) - b0,
        peak: PEAK.load(Ordering::Relaxed).saturating_sub(base),
        first_row,
    }
}

fn measure_connection_ops<C: portadb::DriverConnection>(
    rt: &tokio::runtime::Runtime,
    mut conn: Connection<C>,
    rows: usize,
    name_base: &str,
    cases: &mut Vec<(String, Measure)>,
) {
    let m = measure(|| {
        let got = rt
            .block_on(conn.query(SELECT, &NamedParams::new()))
            .expect("query runs");
        assert_eq!(got.len(), rows);
        Duration::ZERO
    });
    cases.push((
        name_base.to_string(),
        Measure {
            first_row: m.time,
            ..m
        },
    ));

    let m = measure(|| {
        let start = Instant::now();
        let mut first = None;
        let mut count = 0usize;
        let mut on_row = |_row| {
            first.get_or_insert_with(|| start.elapsed());
            count += 1;
            std::ops::ControlFlow::Continue(())
        };
        let handed = rt
            .block_on(conn.query_each(SELECT, &NamedParams::new(), &mut on_row))
            .expect("query_each runs");
        assert_eq!(handed as usize, rows);
        assert_eq!(count, rows);
        first.unwrap_or_default()
    });
    cases.push((format!("{name_base} (each)"), m));

    let m = measure(|| {
        let start = Instant::now();
        let mut first = None;
        let mut count = 0usize;
        let on_row = |row: Wide| {
            first.get_or_insert_with(|| start.elapsed());
            count += 1;
            criterion::black_box(row.touch());
            std::ops::ControlFlow::Continue(())
        };
        let handed = rt
            .block_on(conn.query_each_as(SELECT, &NamedParams::new(), on_row))
            .expect("query_each_as runs");
        assert_eq!(handed as usize, rows);
        assert_eq!(count, rows);
        first.unwrap_or_default()
    });
    cases.push((format!("{name_base} (as)"), m));
}

fn report(rt: &tokio::runtime::Runtime) {
    println!(
        "| case | rows | allocations | allocs/row | bytes allocated | peak memory | time | first row |"
    );
    println!("|---|---:|---:|---:|---:|---:|---:|---:|");
    for rows in SIZES {
        let mut cases: Vec<(String, Measure)> = Vec::new();
        for engine in ["rusqlite", "sqlx"] {
            let conn = rt.block_on(engine_conn(engine, rows));
            let name_base = if engine == "rusqlite" {
                "portadb + rusqlite"
            } else {
                "portadb + sqlx"
            };
            measure_connection_ops(rt, conn, rows, name_base, &mut cases);

            // Typed Connection<C> via Connection::open_typed
            let typed_name_base = if engine == "rusqlite" {
                "portadb-typed+rusqlite"
            } else {
                "portadb-typed+sqlx"
            };
            if engine == "rusqlite" {
                let typed_conn = rt.block_on(typed_rusqlite_conn(rows));
                measure_connection_ops(rt, typed_conn, rows, typed_name_base, &mut cases);
            } else {
                let typed_conn = rt.block_on(typed_sqlx_conn(rows));
                measure_connection_ops(rt, typed_conn, rows, typed_name_base, &mut cases);
            }
        }
        let raw = raw_conn(rows);
        let start = Instant::now();
        let m = measure(|| raw_query(&raw, start).1);
        cases.push(("raw rusqlite (floor)".to_string(), m));
        for (name, m) in cases {
            println!(
                "| {name} | {rows} | {} | {:.1} | {:.1} MB | {:.1} MB | {:.1} ms | {:.3} ms |",
                m.allocs,
                m.allocs as f64 / rows as f64,
                m.bytes as f64 / 1e6,
                m.peak as f64 / 1e6,
                m.time.as_secs_f64() * 1e3,
                m.first_row.as_secs_f64() * 1e3,
            );
        }
    }
}

macro_rules! bench_connection_suite {
    ($group:ident, $rt:ident, $conn:ident, $engine_prefix:expr, $rows:ident) => {
        $group.bench_with_input(
            BenchmarkId::new($engine_prefix.clone(), $rows),
            &$rows,
            |b, _| {
                b.iter(|| {
                    $rt.block_on($conn.query(SELECT, &NamedParams::new()))
                        .expect("query runs")
                });
            },
        );

        $group.bench_with_input(
            BenchmarkId::new(format!("{}-each", $engine_prefix), $rows),
            &$rows,
            |b, _| {
                b.iter(|| {
                    let mut count = 0u64;
                    let mut on_row = |_row| {
                        count += 1;
                        std::ops::ControlFlow::Continue(())
                    };
                    $rt.block_on($conn.query_each(SELECT, &NamedParams::new(), &mut on_row))
                        .expect("query_each runs")
                });
            },
        );

        $group.bench_with_input(
            BenchmarkId::new(format!("{}-as", $engine_prefix), $rows),
            &$rows,
            |b, _| {
                b.iter(|| {
                    let mut count = 0u64;
                    let on_row = |row: Wide| {
                        count += 1;
                        criterion::black_box(row.touch());
                        std::ops::ControlFlow::Continue(())
                    };
                    $rt.block_on($conn.query_each_as(SELECT, &NamedParams::new(), on_row))
                        .expect("query_each_as runs")
                });
            },
        );
    };
}

fn timings(rt: &tokio::runtime::Runtime) {
    let mut c = Criterion::default().configure_from_args();
    let mut group = c.benchmark_group("query_all_rows");
    group.sample_size(10);
    for rows in SIZES {
        for engine in ["rusqlite", "sqlx"] {
            // The dyn connection is dropped before the typed one opens: each
            // `:memory:` connection gets its own private database.
            {
                let mut conn = rt.block_on(engine_conn(engine, rows));
                let dyn_prefix = format!("portadb+{engine}");
                bench_connection_suite!(group, rt, conn, dyn_prefix, rows);
            }

            let typed_prefix = format!("portadb-typed+{engine}");
            if engine == "rusqlite" {
                let mut typed_conn = rt.block_on(typed_rusqlite_conn(rows));
                bench_connection_suite!(group, rt, typed_conn, typed_prefix, rows);
            } else {
                let mut typed_conn = rt.block_on(typed_sqlx_conn(rows));
                bench_connection_suite!(group, rt, typed_conn, typed_prefix, rows);
            }
        }
        let raw = raw_conn(rows);
        group.bench_with_input(BenchmarkId::new("raw-rusqlite", rows), &rows, |b, _| {
            b.iter(|| raw_query(&raw, Instant::now()));
        });
    }
    group.finish();
    c.final_summary();
}

fn main() {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("runtime");
    if std::env::args().any(|a| a == "--report") {
        report(&rt);
    } else {
        timings(&rt);
    }
}
