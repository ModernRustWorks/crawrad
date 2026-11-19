use criterion::{BatchSize, Criterion, black_box, criterion_group, criterion_main};
use datagrid::group::Total;
use datagrid::query::Query;
use datagrid::source::{ColumnInfo, ColumnKind, DisplayHint, RowSource};
use datagrid::{Cursor, LocateOptions, Owned};
use datagrid_sql::source::SqlSource;
use datagrid_sql::translate::Table;
use portadb::connection::Connection;
use portadb::drivers::RusqliteDriver;
use portadb::options::ConnectOptions;
use portadb::params::NamedParams;
use std::path::{Path, PathBuf};
use std::sync::Arc;

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

struct TempDbGuard {
    path: PathBuf,
}

impl Drop for TempDbGuard {
    fn drop(&mut self) {
        let _ = std::fs::remove_file(&self.path);
        let _ = std::fs::remove_file(format!("{}-wal", self.path.display()));
        let _ = std::fs::remove_file(format!("{}-shm", self.path.display()));
    }
}

fn setup_bench_database(path: &Path, rows: usize) {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();

    let path_str = path.to_str().expect("valid path");
    let options = ConnectOptions::new(path_str);
    let mut conn = rt
        .block_on(async { Connection::open(RusqliteDriver::new(), &options).await })
        .unwrap();

    rt.block_on(async {
        conn.execute("PRAGMA synchronous = OFF;", &NamedParams::new())
            .await
            .unwrap();
        conn.execute("PRAGMA journal_mode = MEMORY;", &NamedParams::new())
            .await
            .unwrap();
        conn.execute(
            "CREATE TABLE bench_server (id INTEGER PRIMARY KEY, amount REAL NOT NULL, category TEXT, score INTEGER);",
            &NamedParams::new(),
        )
        .await
        .unwrap();

        let mut tx = conn.begin_transaction().await.unwrap();

        let mut rng = XorShift64::new(0x1337_2026);
        let categories = ["Category_A", "Category_B", "Category_C", "Category_D", "Category_E"];

        // Batch inserts in chunks
        let chunk_size = 5000;
        let mut row_idx = 0;

        while row_idx < rows {
            let current_chunk = (rows - row_idx).min(chunk_size);
            let mut sql = String::from("INSERT INTO bench_server (id, amount, category, score) VALUES ");
            let mut params = NamedParams::new();

            for i in 0..current_chunk {
                if i > 0 {
                    sql.push_str(", ");
                }
                let id = row_idx + i + 1;
                let amt = ((rng.next_u64() % 1_000_000) as f64) / 10.0;
                let cat = categories[(rng.next_u64() % 5) as usize];
                let score = (rng.next_u64() % 100) as i64;

                let p_id = format!("i{i}");
                let p_amt = format!("a{i}");
                let p_cat = format!("c{i}");
                let p_sc = format!("s{i}");

                params.insert(&p_id, id as i64);
                params.insert(&p_amt, amt);
                params.insert(&p_cat, cat);
                params.insert(&p_sc, score);

                sql.push_str(&format!("(:{p_id}, :{p_amt}, :{p_cat}, :{p_sc})"));
            }

            tx.execute(&sql, &params).await.unwrap();
            row_idx += current_chunk;
        }

        tx.commit().await.unwrap();

        // Index on the first sort column
        conn.execute(
            "CREATE INDEX idx_bench_amount ON bench_server(amount);",
            &NamedParams::new(),
        )
        .await
        .unwrap();
    });
}

fn bench_server(c: &mut Criterion) {
    let mut g = c.benchmark_group("server_mode_1m");

    let temp_db_path =
        std::env::temp_dir().join(format!("crawrad_bench_server_{}.db", std::process::id()));
    let _guard = TempDbGuard {
        path: temp_db_path.clone(),
    };

    let total_rows = 1_000_000;
    setup_bench_database(&temp_db_path, total_rows);

    let schema = vec![
        ColumnInfo {
            name: "id".into(),
            kind: ColumnKind::Int,
            nullable: false,
            hint: DisplayHint::Plain,
        },
        ColumnInfo {
            name: "amount".into(),
            kind: ColumnKind::Float,
            nullable: false,
            hint: DisplayHint::Plain,
        },
        ColumnInfo {
            name: "category".into(),
            kind: ColumnKind::Text,
            nullable: true,
            hint: DisplayHint::Plain,
        },
        ColumnInfo {
            name: "score".into(),
            kind: ColumnKind::Int,
            nullable: true,
            hint: DisplayHint::Plain,
        },
    ];

    let table = Table::sqlite("bench_server", schema);
    let path_str = temp_db_path.to_str().expect("valid path");
    let source = Arc::new(SqlSource::open_sqlite(path_str, table).unwrap());

    let q_sort = Query::new().sort_asc("amount");

    // 1. Time count
    g.bench_function("count", |b| {
        b.iter(|| source.count(black_box(&q_sort)).unwrap())
    });

    // 2. First window (100 rows) sorted
    g.bench_function("first_window_100", |b| {
        b.iter(|| source.fetch(black_box(&q_sort), black_box(0..100)).unwrap())
    });

    // 3. Deep window at row 900,000 by OFFSET (new query instance with no bookmarks)
    let q_offset = Query::new().sort_asc("amount");
    g.bench_function("deep_window_900k_offset", |b| {
        b.iter(|| {
            source
                .fetch(black_box(&q_offset), black_box(900_000..900_100))
                .unwrap()
        })
    });

    // 4. Same deep window by keyset after scrolling a bookmark there
    let q_keyset = Query::new().sort_asc("amount");
    // Scroll consecutive windows up to 900_000 in steps to populate bookmark at 900_000
    let step = 1000;
    let mut cur = 0;
    while cur < 900_000 {
        let next = (cur + step).min(900_000);
        source.fetch(&q_keyset, cur..next).unwrap();
        cur = next;
    }

    g.bench_function("deep_window_900k_keyset", |b| {
        b.iter(|| {
            source
                .fetch(black_box(&q_keyset), black_box(900_000..900_100))
                .unwrap()
        })
    });

    // 5. Groups level 0 with a sum
    let q_group = Query::new()
        .group("category")
        .with_total(Total::sum("amount"));

    g.bench_function("groups_level_0_sum", |b| {
        b.iter(|| source.groups(black_box(&q_group), black_box(&[])).unwrap())
    });

    // 6. Cursor::last() cold
    let q_last = Query::new().sort_asc("amount");
    g.bench_function("last_cold", |b| {
        b.iter_batched(
            || {
                source.clear_cache();
                Cursor::open(source.clone() as Arc<dyn RowSource>, q_last.clone()).unwrap()
            },
            |mut cursor| {
                cursor.last().unwrap();
                black_box(cursor)
            },
            BatchSize::SmallInput,
        )
    });

    // 7. Cursor::set_rec_no(500_000) cold (includes the checkpoint build)
    let q_500k = Query::new().sort_asc("amount");
    g.bench_function("set_rec_no_500k_cold", |b| {
        b.iter_batched(
            || {
                source.clear_cache();
                Cursor::open(source.clone() as Arc<dyn RowSource>, q_500k.clone()).unwrap()
            },
            |mut cursor| {
                cursor.set_rec_no(500_000).unwrap();
                black_box(cursor)
            },
            BatchSize::SmallInput,
        )
    });

    // 8. Cursor::set_rec_no random after the build
    let q_random = Query::new().sort_asc("amount");
    let mut cursor_random =
        Cursor::open(source.clone() as Arc<dyn RowSource>, q_random.clone()).unwrap();
    cursor_random.set_rec_no(500_000).unwrap();
    let mut rng_random = XorShift64::new(0xCAFE_BABE);
    g.bench_function("set_rec_no_random", |b| {
        b.iter(|| {
            let target = (rng_random.next_u64() % 1_000_000) as usize;
            cursor_random.set_rec_no(black_box(target)).unwrap();
            black_box(cursor_random.rec_no())
        })
    });

    // 9. locate on an indexed column
    let q_locate = Query::new().sort_asc("id");
    let mut cursor_locate = Cursor::open(source.clone() as Arc<dyn RowSource>, q_locate).unwrap();
    let opts = LocateOptions::default();
    g.bench_function("locate_indexed", |b| {
        b.iter(|| {
            cursor_locate
                .locate(black_box(&[("amount", Owned::Float(50000.0))]), opts)
                .unwrap()
        })
    });

    // 10. goto_bookmark after a re-sort
    let q_sort1 = Query::new().sort_asc("amount");
    let q_sort2 = Query::new().sort_desc("amount");
    let mut cursor_bm = Cursor::open(source.clone() as Arc<dyn RowSource>, q_sort1).unwrap();
    cursor_bm.set_rec_no(500_000).unwrap();
    let bm = cursor_bm.bookmark().expect("bookmark present");
    g.bench_function("goto_bookmark_resort", |b| {
        b.iter(|| {
            cursor_bm.set_query(black_box(q_sort2.clone()));
            let found = cursor_bm.goto_bookmark(black_box(bm)).unwrap();
            black_box(found)
        })
    });

    // 11. position_of
    let target_id = 500_000u64;
    g.bench_function("position_of", |b| {
        b.iter(|| {
            source
                .position_of(black_box(&q_sort), black_box(target_id))
                .unwrap()
        })
    });

    g.finish();
}

criterion_group!(benches, bench_server);
criterion_main!(benches);
