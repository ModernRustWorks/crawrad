use datagrid::column::{Column, Value};
use datagrid::cursor::{Cursor, CursorLine};
use datagrid::cursor_contract;
use datagrid::group::Total;
use datagrid::query::Query;
use datagrid::source::{ColumnInfo, ColumnKind, DisplayHint, RowSource, VecSource};
use datagrid::store::Store;
use datagrid_sql::source::SqlSource;
use datagrid_sql::translate::Table;
use portadb::connection::Connection;
use portadb::drivers::RusqliteDriver;
use portadb::options::ConnectOptions;
use portadb::params::NamedParams;
use std::fmt::Write;
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};

static DB_COUNTER: AtomicU64 = AtomicU64::new(1);

fn make_sql_source(store: &Store) -> Arc<dyn RowSource> {
    let schema = cursor_contract::contract_schema();
    let db_id = DB_COUNTER.fetch_add(1, Ordering::Relaxed);
    let options = ConnectOptions::new(format!(
        "file:datagrid_sql_cursor_{db_id}?mode=memory&cache=shared"
    ));

    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("tokio runtime build");

    let mut conn = rt
        .block_on(async { Connection::open(RusqliteDriver::new(), &options).await })
        .expect("Connection::open");

    rt.block_on(async {
        conn.execute(
            "CREATE TABLE test_data (id INTEGER, name TEXT, price REAL, day INTEGER, flag INTEGER);",
            &NamedParams::new(),
        )
        .await
        .expect("CREATE TABLE");

        let num_rows = store.rows();
        for r in 0..num_rows {
            let mut params = NamedParams::new();
            for info in &schema {
                let col = store.column(&info.name).expect("column found");
                match col.get(r) {
                    Value::Null => params.insert(&info.name, portadb::value::Value::Null),
                    Value::Int(v) => params.insert(&info.name, v),
                    Value::Float(v) => params.insert(&info.name, v),
                    Value::Bool(b) => params.insert(&info.name, if b { 1i64 } else { 0i64 }),
                    Value::Text(s) => params.insert(&info.name, s.to_string()),
                }
            }
            conn.execute(
                "INSERT INTO test_data (id, name, price, day, flag) VALUES (:id, :name, :price, :day, :flag);",
                &params,
            )
            .await
            .expect("INSERT");
        }
    });

    let table = Table::sqlite("test_data", schema);
    let sql_source = SqlSource::from_connection(conn, table).expect("SqlSource::from_connection");
    Arc::new(sql_source)
}

#[test]
fn test_cursor_contract_sql_source() {
    cursor_contract::run_all(&make_sql_source);
}

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

fn assert_cursor_cells_equal(c1: &Cursor, c2: &Cursor) {
    assert_eq!(c1.rec_no(), c2.rec_no(), "rec_no mismatch");
    assert_eq!(c1.bof(), c2.bof(), "bof mismatch");
    assert_eq!(c1.eof(), c2.eof(), "eof mismatch");
    for f1 in c1.fields() {
        let f2 = c2.field(f1.name()).expect("field exists");
        match (f1.value(), f2.value()) {
            (Value::Null, Value::Null) => {}
            (Value::Int(a), Value::Int(b)) => assert_eq!(a, b, "col {} mismatch", f1.name()),
            (Value::Float(a), Value::Float(b)) => {
                let diff = (a - b).abs();
                assert!(
                    diff <= 1e-9 * a.abs().max(b.abs()).max(1.0),
                    "col {} float mismatch: {a} vs {b}",
                    f1.name()
                );
            }
            (Value::Bool(a), Value::Bool(b)) => assert_eq!(a, b, "col {} mismatch", f1.name()),
            (Value::Text(a), Value::Text(b)) => assert_eq!(a, b, "col {} mismatch", f1.name()),
            _ => panic!(
                "type mismatch for col {}: {:?} vs {:?}",
                f1.name(),
                f1.value(),
                f2.value()
            ),
        }
    }
}

#[test]
fn test_cursor_100k_rows() {
    let num_rows = 100_000;
    let mut col_id = Vec::with_capacity(num_rows);
    let mut col_name = Vec::with_capacity(num_rows);
    let mut col_price = Vec::with_capacity(num_rows);
    let mut col_day = Vec::with_capacity(num_rows);
    let mut col_flag = Vec::with_capacity(num_rows);

    let mut rng = XorShift64::new(0x1000_2026);
    let names = ["alpha", "bravo", "charlie", "delta", "echo", "foxtrot"];

    for r in 0..num_rows {
        col_id.push(Some(r as i64));
        col_name.push(Some(names[(rng.next_u64() % 6) as usize].to_string()));
        col_price.push(Some(((rng.next_u64() % 10_000) as f64) / 100.0));
        col_day.push(Some((rng.next_u64() % 30) as i64 + 1));
        col_flag.push(Some((rng.next_u64() & 1) != 0));
    }

    let mut store = Store::new();
    store.add("id", Column::ints(col_id.clone())).unwrap();
    store
        .add("name", Column::dict_from(col_name.clone()))
        .unwrap();
    store
        .add("price", Column::floats(col_price.clone()))
        .unwrap();
    store.add("day", Column::ints(col_day.clone())).unwrap();
    store.add("flag", Column::bools(col_flag.clone())).unwrap();

    let schema = vec![
        ColumnInfo {
            name: "id".into(),
            kind: ColumnKind::Int,
            nullable: true,
            hint: DisplayHint::Plain,
        },
        ColumnInfo {
            name: "name".into(),
            kind: ColumnKind::Text,
            nullable: true,
            hint: DisplayHint::Plain,
        },
        ColumnInfo {
            name: "price".into(),
            kind: ColumnKind::Float,
            nullable: true,
            hint: DisplayHint::Plain,
        },
        ColumnInfo {
            name: "day".into(),
            kind: ColumnKind::Int,
            nullable: true,
            hint: DisplayHint::Plain,
        },
        ColumnInfo {
            name: "flag".into(),
            kind: ColumnKind::Bool,
            nullable: true,
            hint: DisplayHint::Plain,
        },
    ];

    let vec_source: Arc<dyn RowSource> = Arc::new(VecSource::new(store, schema.clone()).unwrap());

    let db_id = DB_COUNTER.fetch_add(1, Ordering::Relaxed);
    let options = ConnectOptions::new(format!(
        "file:datagrid_sql_100k_{db_id}?mode=memory&cache=shared"
    ));
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();
    let mut conn = rt
        .block_on(async { Connection::open(RusqliteDriver::new(), &options).await })
        .unwrap();

    rt.block_on(async {
        conn.execute("PRAGMA synchronous = OFF;", &NamedParams::new())
            .await
            .unwrap();
        conn.execute(
            "CREATE TABLE test_data (id INTEGER, name TEXT, price REAL, day INTEGER, flag INTEGER);",
            &NamedParams::new(),
        )
        .await
        .unwrap();

        conn.execute("BEGIN TRANSACTION;", &NamedParams::new())
            .await
            .unwrap();

        let chunk_size = 1000;
        let mut row_idx = 0;
        while row_idx < num_rows {
            let current_chunk = (num_rows - row_idx).min(chunk_size);
            let mut sql = String::from("INSERT INTO test_data (id, name, price, day, flag) VALUES ");
            for i in 0..current_chunk {
                if i > 0 {
                    sql.push_str(", ");
                }
                let r = row_idx + i;
                let id = col_id[r].unwrap();
                let name = col_name[r].as_ref().unwrap();
                let price = col_price[r].unwrap();
                let day = col_day[r].unwrap();
                let flag = if col_flag[r].unwrap() { 1 } else { 0 };
                write!(sql, "({id}, '{name}', {price}, {day}, {flag})").unwrap();
            }
            sql.push(';');
            conn.execute(&sql, &NamedParams::new()).await.unwrap();
            row_idx += current_chunk;
        }

        conn.execute("COMMIT;", &NamedParams::new()).await.unwrap();
    });

    let table = Table::sqlite("test_data", schema);
    let sql_source = Arc::new(SqlSource::from_connection(conn, table).unwrap());

    let q = Query::new().sort_asc("price").sort_asc("id");

    let mut cursor_vec = Cursor::open(vec_source, q.clone()).unwrap();
    let mut cursor_sql = Cursor::open(sql_source.clone() as Arc<dyn RowSource>, q).unwrap();

    assert_cursor_cells_equal(&cursor_vec, &cursor_sql);

    // 1. last()
    cursor_vec.last().unwrap();
    cursor_sql.last().unwrap();
    assert_cursor_cells_equal(&cursor_vec, &cursor_sql);
    assert_eq!(
        sql_source.checkpoint_builds(),
        0,
        "last() reads backwards from the end, it must not scan the whole result"
    );
    assert!(sql_source.tail_fetches() > 0);

    // walking back from the end crosses from tail reads into checkpointed reads
    for step in 0..2_500 {
        cursor_vec.prior().unwrap();
        cursor_sql.prior().unwrap();
        if step % 97 == 0 {
            assert_cursor_cells_equal(&cursor_vec, &cursor_sql);
        }
    }
    assert_cursor_cells_equal(&cursor_vec, &cursor_sql);

    // 2. set_rec_no(50_000)
    cursor_vec.set_rec_no(50_000).unwrap();
    cursor_sql.set_rec_no(50_000).unwrap();
    assert_cursor_cells_equal(&cursor_vec, &cursor_sql);

    // 3. 50 random set_rec_no
    for _ in 0..50 {
        let rec = (rng.next_u64() % (num_rows as u64)) as usize;
        cursor_vec.set_rec_no(rec).unwrap();
        cursor_sql.set_rec_no(rec).unwrap();
        assert_cursor_cells_equal(&cursor_vec, &cursor_sql);
    }

    // 4. goto_bookmark
    let bm_vec = cursor_vec.bookmark().expect("bookmark vec");
    let bm_sql = cursor_sql.bookmark().expect("bookmark sql");

    // Move away
    cursor_vec.set_rec_no(1234).unwrap();
    cursor_sql.set_rec_no(1234).unwrap();

    assert!(cursor_vec.goto_bookmark(bm_vec).unwrap());
    assert!(cursor_sql.goto_bookmark(bm_sql).unwrap());
    assert_cursor_cells_equal(&cursor_vec, &cursor_sql);

    // 5. Assertions on SqlSource counters
    assert_eq!(
        sql_source.offset_fetches(),
        0,
        "offset_fetches should be 0, no OFFSET cliff"
    );
    assert_eq!(
        sql_source.checkpoint_builds(),
        1,
        "checkpoints must be built exactly once for the query and reused"
    );
}

#[test]
fn test_sql_and_vec_source_lines_equivalence() {
    let store = cursor_contract::build_grouping_store();
    let schema = cursor_contract::contract_schema();
    let vec_source: Arc<dyn RowSource> =
        Arc::new(VecSource::new(store.clone(), schema).expect("vec source"));
    let sql_source = make_sql_source(&store);

    let query = Query::new()
        .group("name")
        .with_total(Total::count("id"))
        .with_total(Total::sum("id"))
        .group("day")
        .with_total(Total::count("id"))
        .with_total(Total::sum("id"))
        .sort_desc("id");

    let mut cursor_vec = Cursor::open(vec_source, query.clone()).expect("open vec");
    let mut cursor_sql = Cursor::open(sql_source, query).expect("open sql");

    let mut it_vec = cursor_vec.lines();
    let mut it_sql = cursor_sql.lines();

    let mut line_count = 0;
    loop {
        let line_vec = it_vec.next_line().expect("vec next_line");
        let line_sql = it_sql.next_line().expect("sql next_line");

        match (line_vec, line_sql) {
            (None, None) => break,
            (
                Some(CursorLine::Header {
                    level: l_v,
                    group: g_v,
                }),
                Some(CursorLine::Header {
                    level: l_s,
                    group: g_s,
                }),
            ) => {
                line_count += 1;
                assert_eq!(l_v, l_s, "header level mismatch at line {line_count}");
                assert_eq!(g_v.key, g_s.key, "header key mismatch at line {line_count}");
                assert_eq!(
                    g_v.row_count, g_s.row_count,
                    "header row_count mismatch at line {line_count}"
                );
                assert_eq!(
                    g_v.totals.len(),
                    g_s.totals.len(),
                    "totals len mismatch at line {line_count}"
                );
                for (tv, ts) in g_v.totals.iter().zip(g_s.totals.iter()) {
                    match (tv, ts) {
                        (None, None) => {}
                        (Some(a), Some(b)) => {
                            let diff = (a - b).abs();
                            assert!(
                                diff <= 1e-9 * a.abs().max(b.abs()).max(1.0),
                                "total float mismatch at line {line_count}: {a} vs {b}"
                            );
                        }
                        _ => panic!(
                            "totals null mismatch at line {line_count}: {:?} vs {:?}",
                            tv, ts
                        ),
                    }
                }
            }
            (Some(CursorLine::Row(r_v)), Some(CursorLine::Row(r_s))) => {
                line_count += 1;
                assert_eq!(
                    r_v.rec_no(),
                    r_s.rec_no(),
                    "rec_no mismatch at line {line_count}"
                );
                for f_v in r_v.fields() {
                    let f_s = r_s.field(f_v.name()).expect("field in sql row");
                    match (f_v.value(), f_s.value()) {
                        (Value::Null, Value::Null) => {}
                        (Value::Int(a), Value::Int(b)) => {
                            assert_eq!(a, b, "col {} mismatch at line {line_count}", f_v.name());
                        }
                        (Value::Float(a), Value::Float(b)) => {
                            let diff = (a - b).abs();
                            assert!(
                                diff <= 1e-9 * a.abs().max(b.abs()).max(1.0),
                                "col {} float mismatch at line {line_count}: {a} vs {b}",
                                f_v.name()
                            );
                        }
                        (Value::Bool(a), Value::Bool(b)) => {
                            assert_eq!(a, b, "col {} mismatch at line {line_count}", f_v.name());
                        }
                        (Value::Text(a), Value::Text(b)) => {
                            assert_eq!(a, b, "col {} mismatch at line {line_count}", f_v.name());
                        }
                        _ => panic!(
                            "type mismatch at line {line_count} for {}: {:?} vs {:?}",
                            f_v.name(),
                            f_v.value(),
                            f_s.value()
                        ),
                    }
                }
            }
            (v, s) => panic!(
                "line kind mismatch at line {line_count}: vec has some = {}, sql has some = {}",
                v.is_some(),
                s.is_some()
            ),
        }
    }
    assert!(line_count > 0, "must have walked lines");
}
