//! `SqlSource::update` / `insert`: the Grid's write-back (#936 P2).

use std::sync::atomic::{AtomicU64, Ordering};

use datagrid::column::Value;
use datagrid::filter::{Filter, Owned};
use datagrid::query::Query;
use datagrid::source::{RowSource, SourceError};
use datagrid_sql::SqlSource;
use portadb::params::NamedParams;

static DB_COUNTER: AtomicU64 = AtomicU64::new(1);

/// `people(name TEXT NOT NULL, city TEXT, balance REAL)` with two rows, opened by name.
fn people() -> (SqlSource, SqlSource) {
    let id = DB_COUNTER.fetch_add(1, Ordering::Relaxed);
    let url = format!("file:datagrid_sql_write_{id}?mode=memory&cache=shared");
    let keeper = SqlSource::open_sqlite(&url, datagrid_sql::Table::sqlite("none", Vec::new()))
        .expect("open the database");
    for sql in [
        "CREATE TABLE people (name TEXT NOT NULL, city TEXT, balance REAL DEFAULT 0)",
        "INSERT INTO people (name, city, balance) VALUES ('Ana', 'Natal', 10.5), ('Bruno', 'Recife', 20)",
    ] {
        keeper.execute(sql, &NamedParams::new()).expect("seed");
    }
    let source = SqlSource::open_sqlite_table(&url, "people").expect("open people");
    (keeper, source)
}

fn texts(source: &SqlSource, column: usize) -> Vec<Option<String>> {
    let query = Query::default();
    let n = source.count(&query).expect("count");
    let rows = source.fetch(&query, 0..n).expect("fetch");
    (0..rows.len)
        .map(|i| match rows.columns[column].get(i) {
            Value::Text(t) => Some(t.to_string()),
            Value::Null => None,
            other => panic!("not text: {other:?}"),
        })
        .collect()
}

fn id_of(source: &SqlSource, name: &str) -> u64 {
    let filter = Filter::eq("name", Owned::Text(name.to_string()));
    source
        .locate(&Query::default(), &filter)
        .expect("locate")
        .expect("found")
        .1
}

#[test]
fn an_update_writes_every_changed_value_of_the_row() {
    let (_keeper, source) = people();
    assert!(source.can_write());
    let bruno = id_of(&source, "Bruno");
    source
        .update(
            bruno,
            &[
                ("city".into(), Some(Owned::Text("Olinda".into()))),
                ("balance".into(), Some(Owned::Int(25))),
            ],
        )
        .expect("update");
    assert_eq!(
        texts(&source, 1),
        [Some("Natal".into()), Some("Olinda".into())]
    );
    let sum = source
        .summary(&Query {
            summary: vec![datagrid::group::Total {
                column: "balance".into(),
                aggregate: datagrid::group::Aggregate::Sum,
            }],
            ..Default::default()
        })
        .expect("summary");
    assert_eq!(sum, [Some(35.5)], "the cached totals see the write");
}

#[test]
fn null_goes_in_a_nullable_column_and_is_refused_in_a_required_one() {
    let (_keeper, source) = people();
    let ana = id_of(&source, "Ana");
    source
        .update(ana, &[("city".into(), None)])
        .expect("city may be NULL");
    assert_eq!(texts(&source, 1)[0], None);
    let err = source.update(ana, &[("name".into(), None)]).unwrap_err();
    assert_eq!(err, SourceError::Backend("name is required".into()));
    assert_eq!(
        texts(&source, 0)[0].as_deref(),
        Some("Ana"),
        "nothing written"
    );
}

#[test]
fn a_wrong_kind_or_unknown_column_is_refused_before_the_database() {
    let (_keeper, source) = people();
    let ana = id_of(&source, "Ana");
    assert!(matches!(
        source.update(ana, &[("balance".into(), Some(Owned::Text("x".into())))]),
        Err(SourceError::Store(_))
    ));
    assert!(matches!(
        source.update(ana, &[("nope".into(), Some(Owned::Int(1)))]),
        Err(SourceError::Store(_))
    ));
    assert!(matches!(
        source.update(ana, &[("rowid".into(), Some(Owned::Int(9)))]),
        Err(SourceError::Unsupported(_))
    ));
}

#[test]
fn updating_a_row_that_is_gone_says_so() {
    let (_keeper, source) = people();
    let err = source
        .update(999, &[("city".into(), Some(Owned::Text("X".into())))])
        .unwrap_err();
    assert_eq!(err, SourceError::Backend("row 999 no longer exists".into()));
}

#[test]
fn an_insert_returns_the_new_key_and_left_out_columns_take_their_default() {
    let (_keeper, source) = people();
    let id = source
        .insert(&[("name".into(), Some(Owned::Text("Carla".into())))])
        .expect("insert");
    assert_eq!(id_of(&source, "Carla"), id);
    assert_eq!(
        source.count(&Query::default()).expect("count"),
        3,
        "the cached count sees it"
    );
    let err = source
        .insert(&[("city".into(), Some(Owned::Text("Natal".into())))])
        .unwrap_err();
    assert!(
        matches!(err, SourceError::Backend(_)),
        "NOT NULL name: {err:?}"
    );
}

/// Polls `live` until `done` holds (5 s at most).
fn wait_for(live: &mut datagrid::LiveRows, done: impl Fn(&datagrid::Screen) -> bool) {
    let start = std::time::Instant::now();
    while start.elapsed() < std::time::Duration::from_secs(5) {
        live.poll();
        if done(live.screen()) {
            return;
        }
        std::thread::sleep(std::time::Duration::from_millis(5));
    }
    panic!("timed out: {:?}", live.screen());
}

fn row_cells(screen: &datagrid::Screen) -> Vec<(u64, Vec<Option<String>>)> {
    screen
        .lines
        .iter()
        .filter_map(|l| match l {
            datagrid::ScreenLine::Row { id, cells, .. } => Some((*id, cells.clone())),
            _ => None,
        })
        .collect()
}

#[test]
fn a_write_through_live_rows_comes_back_on_the_screen_with_its_totals() {
    let (_keeper, source) = people();
    let query = Query {
        summary: vec![datagrid::group::Total {
            column: "balance".into(),
            aggregate: datagrid::group::Aggregate::Sum,
        }],
        ..Default::default()
    };
    let mut live = datagrid::LiveRows::spawn(std::sync::Arc::new(source), query, 50, 4);
    live.show(0, 10);
    wait_for(&mut live, |s| s.total == Some(2) && !s.summary.is_empty());
    let (bruno, _) = row_cells(live.screen())[1].clone();

    let ticket = live.write(datagrid::Write::Update {
        id: bruno,
        changes: vec![("balance".into(), Some(Owned::Float(30.0)))],
    });
    wait_for(&mut live, |s| {
        s.written.as_ref().is_some_and(|w| w.ticket == ticket) && s.summary == [Some(40.5)]
    });
    assert_eq!(live.screen().written.as_ref().unwrap().result, Ok(bruno));
    wait_for(&mut live, |s| {
        row_cells(s)
            .get(1)
            .is_some_and(|(_, c)| c[2].as_deref() == Some("30"))
    });

    let ticket = live.write(datagrid::Write::Insert(vec![(
        "name".into(),
        Some(Owned::Text("Carla".into())),
    )]));
    wait_for(&mut live, |s| {
        s.written.as_ref().is_some_and(|w| w.ticket == ticket) && s.total == Some(3)
    });

    let ticket = live.write(datagrid::Write::Update {
        id: bruno,
        changes: vec![("name".into(), None)],
    });
    wait_for(&mut live, |s| {
        s.written.as_ref().is_some_and(|w| w.ticket == ticket)
    });
    let refused = &live.screen().written.as_ref().unwrap().result;
    assert!(
        matches!(refused, Err(e) if e.contains("name is required")),
        "{refused:?}"
    );
}

#[test]
fn the_key_column_is_the_rowid_alias_when_the_table_has_one() {
    let (_keeper, source) = people();
    assert_eq!(source.key_column(), Some("rowid"), "no INTEGER PRIMARY KEY");

    let url = "file:datagrid_sql_write_keys?mode=memory&cache=shared";
    let keeper = SqlSource::open_sqlite(url, datagrid_sql::Table::sqlite("none", Vec::new()))
        .expect("open the database");
    for sql in [
        "CREATE TABLE items (id INTEGER PRIMARY KEY, name TEXT)",
        "CREATE TABLE pairs (a INTEGER, b INTEGER, PRIMARY KEY (a, b))",
        "CREATE TABLE codes (code TEXT PRIMARY KEY)",
    ] {
        keeper.execute(sql, &NamedParams::new()).expect("create");
    }
    let key = |table: &str| {
        SqlSource::open_sqlite_table(url, table)
            .expect("open")
            .key_column()
            .map(str::to_string)
    };
    assert_eq!(key("items").as_deref(), Some("id"));
    assert_eq!(
        key("pairs").as_deref(),
        Some("rowid"),
        "a composite key is no alias"
    );
    assert_eq!(
        key("codes").as_deref(),
        Some("rowid"),
        "only INTEGER aliases rowid"
    );
}
