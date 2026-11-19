//! Reusable test contract suite asserting dataset-style `Cursor` behavior across row sources.

use crate::column::{Column, Value};
use crate::cursor::{Cursor, CursorEvent, CursorLine, Date, LocateOptions};
use crate::filter::{Filter, Owned};
use crate::group::Total;
use crate::query::Query;
use crate::sort::{Nulls, Order, SortKey};
use crate::source::{ColumnInfo, ColumnKind, DisplayHint, RowSource, SourceError, SourceGroup};
use crate::store::Store;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};

/// Returns the standardized schema for the cursor contract suite.
pub fn contract_schema() -> Vec<ColumnInfo> {
    vec![
        ColumnInfo {
            name: "id".into(),
            kind: ColumnKind::Int,
            nullable: false,
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
            hint: DisplayHint::Date,
        },
        ColumnInfo {
            name: "flag".into(),
            kind: ColumnKind::Bool,
            nullable: true,
            hint: DisplayHint::Plain,
        },
    ]
}

/// Builds a fixed [`Store`] with `num_rows` matching the contract schema.
pub fn build_fixed_store(num_rows: usize) -> Store {
    let mut store = Store::new();
    let mut ids = Vec::with_capacity(num_rows);
    let mut names = Vec::with_capacity(num_rows);
    let mut prices = Vec::with_capacity(num_rows);
    let mut days = Vec::with_capacity(num_rows);
    let mut flags = Vec::with_capacity(num_rows);

    for i in 0..num_rows {
        ids.push(Some(i as i64));
        match i {
            0 => {
                names.push(Some("alpha".to_string()));
                prices.push(Some(10.5));
                days.push(Some(0)); // 1970-01-01
                flags.push(Some(true));
            }
            1 => {
                names.push(None); // null name
                prices.push(Some(20.0));
                days.push(Some(19723)); // 2024-01-01
                flags.push(Some(false));
            }
            2 => {
                names.push(Some("bravo".to_string()));
                prices.push(None); // null price
                days.push(Some(-1)); // 1969-12-31
                flags.push(Some(true));
            }
            3 => {
                names.push(Some("charlie".to_string()));
                prices.push(Some(30.25));
                days.push(None); // null day
                flags.push(None); // null flag
            }
            _ => {
                names.push(Some(format!("item_{i}")));
                prices.push(Some((i as f64) * 10.0));
                days.push(Some((i as i64) * 100));
                flags.push(Some(i % 2 == 0));
            }
        }
    }

    store.add("id", Column::ints(ids)).expect("add id");
    store.add("name", Column::texts(names)).expect("add name");
    store
        .add("price", Column::floats(prices))
        .expect("add price");
    store.add("day", Column::ints(days)).expect("add day");
    store.add("flag", Column::bools(flags)).expect("add flag");
    store
}

/// Builds a [`Store`] matching `contract_schema` designed for multi-level grouping tests.
///
/// Has 64 rows, two group columns (`name` and `day`) with repeated values and nulls in both,
/// forming a 4x4 group grid (16 groups >= 3x3), with an Int value column (`id`) for totals.
pub fn build_grouping_store() -> Store {
    let mut store = Store::new();
    let num_rows = 64;
    let mut ids = Vec::with_capacity(num_rows);
    let mut names = Vec::with_capacity(num_rows);
    let mut prices = Vec::with_capacity(num_rows);
    let mut days = Vec::with_capacity(num_rows);
    let mut flags = Vec::with_capacity(num_rows);

    let name_options = [
        None,
        Some("grp_a".to_string()),
        Some("grp_b".to_string()),
        Some("grp_c".to_string()),
    ];
    let day_options: [Option<i64>; 4] = [None, Some(10), Some(20), Some(30)];

    // 4 names * 4 days = 16 groups, 4 rows each = 64 rows
    for i in 0..num_rows {
        ids.push(Some(i as i64));
        let g_name = (i / 16) % 4;
        let g_day = (i / 4) % 4;
        names.push(name_options[g_name].clone());
        days.push(day_options[g_day]);
        prices.push(Some((i as f64) * 1.5));
        flags.push(Some(i % 2 == 0));
    }

    store.add("id", Column::ints(ids)).expect("add id");
    store
        .add("name", Column::dict_from(names))
        .expect("add name");
    store
        .add("price", Column::floats(prices))
        .expect("add price");
    store.add("day", Column::ints(days)).expect("add day");
    store.add("flag", Column::bools(flags)).expect("add flag");
    store
}

/// Runs the complete cursor contract suite over a [`RowSource`] factory.
///
/// Panics with a descriptive message on assertion failure.
pub fn run_all(make: &dyn Fn(&Store) -> Arc<dyn RowSource>) {
    test_empty_result(make);
    test_single_row_result(make);
    test_navigation_and_edges(make);
    test_fields_nulls_and_dates(make);
    test_filtered_and_sorted_query(make);
    test_rows_iteration(make);
    test_listeners(make);
    test_open(make);
    test_busy_idle(make);
    test_locate(make);
    test_lookup(make);
    test_bookmark(make);
    test_grouped_rows_order(make);
    test_lines_iteration(make);
    test_group_path_and_group(make);
    test_group_busy_idle(make);
}

fn test_open(make: &dyn Fn(&Store) -> Arc<dyn RowSource>) {
    let store = build_fixed_store(10);
    let cursor = Cursor::open(make(&store), Query::default()).expect("open");
    assert_eq!(cursor.rec_no(), Some(0));
    assert!(cursor.bof());
    assert!(!cursor.eof());
    assert_eq!(cursor.field("id").expect("id after open").as_i64(), Some(0));

    let empty = build_fixed_store(0);
    let cursor = Cursor::open(make(&empty), Query::default()).expect("open empty");
    assert!(cursor.bof() && cursor.eof());
    assert_eq!(
        cursor.field("id").err(),
        Some(SourceError::OutOfRange { index: 0, len: 0 })
    );
}

/// `Busy`/`Idle` frame every real call to the source and nothing else.
fn test_busy_idle(make: &dyn Fn(&Store) -> Arc<dyn RowSource>) {
    let store = build_fixed_store(10);
    let mut cursor = Cursor::with_window(make(&store), Query::default(), 2, 2);
    let log = Arc::new(Mutex::new(Vec::new()));
    let sink = log.clone();
    cursor.on_event(move |ev| sink.lock().expect("lock").push(ev.clone()));
    let take = |log: &Arc<Mutex<Vec<CursorEvent>>>| std::mem::take(&mut *log.lock().expect("lock"));

    // count + first page (rows 0-1, prefetch 2-3): two framed calls, no scroll (already on 0)
    cursor.first().expect("first");
    assert_eq!(
        take(&log),
        vec![
            CursorEvent::Busy,
            CursorEvent::Idle,
            CursorEvent::Busy,
            CursorEvent::Idle
        ]
    );

    // row 1 is cached: a plain scroll
    cursor.next().expect("next");
    assert_eq!(take(&log), vec![CursorEvent::Scrolled { rec_no: 1 }]);

    // row 2 sits in a cached page, but the page after it (4-5) is not: one framed fetch
    cursor.next().expect("next");
    assert_eq!(
        take(&log),
        vec![
            CursorEvent::Busy,
            CursorEvent::Idle,
            CursorEvent::Scrolled { rec_no: 2 }
        ]
    );

    // a no-op move announces nothing
    cursor.first().expect("first");
    take(&log);
    cursor.prior().expect("prior at 0");
    assert!(take(&log).is_empty());
}

fn test_empty_result(make: &dyn Fn(&Store) -> Arc<dyn RowSource>) {
    let store = build_fixed_store(0);
    let source = make(&store);
    let mut cursor = Cursor::new(source, Query::default());

    assert_eq!(cursor.record_count().expect("count"), 0);
    assert!(cursor.is_empty().expect("is_empty"));
    assert_eq!(cursor.rec_no(), None);
    assert!(cursor.bof(), "empty cursor must be bof");
    assert!(cursor.eof(), "empty cursor must be eof");

    cursor.first().expect("first on empty");
    assert_eq!(cursor.rec_no(), None);
    assert!(cursor.bof());
    assert!(cursor.eof());

    cursor.last().expect("last on empty");
    assert_eq!(cursor.rec_no(), None);
    assert!(cursor.bof());
    assert!(cursor.eof());

    cursor.next().expect("next on empty");
    assert_eq!(cursor.rec_no(), None);
    assert!(cursor.bof());
    assert!(cursor.eof());

    cursor.prior().expect("prior on empty");
    assert_eq!(cursor.rec_no(), None);
    assert!(cursor.bof());
    assert!(cursor.eof());

    assert_eq!(cursor.move_by(5).expect("move_by on empty"), 0);
    assert_eq!(cursor.rec_no(), None);

    assert_eq!(
        cursor.set_rec_no(0),
        Err(SourceError::OutOfRange { index: 0, len: 0 })
    );

    assert_eq!(
        cursor.field("id").err(),
        Some(SourceError::OutOfRange { index: 0, len: 0 })
    );
    assert_eq!(cursor.fields().count(), 0);
}

fn test_single_row_result(make: &dyn Fn(&Store) -> Arc<dyn RowSource>) {
    let store = build_fixed_store(1);
    let source = make(&store);
    let mut cursor = Cursor::new(source, Query::default());

    assert_eq!(cursor.record_count().expect("count"), 1);
    assert!(!cursor.is_empty().expect("is_empty"));

    cursor.first().expect("first on single");
    assert_eq!(cursor.rec_no(), Some(0));
    assert!(cursor.bof());
    assert!(!cursor.eof(), "on 1-row result, first() leaves eof false");

    cursor.prior().expect("prior on single");
    assert_eq!(cursor.rec_no(), Some(0));
    assert!(cursor.bof());
    assert!(!cursor.eof());

    cursor.next().expect("next on single");
    assert_eq!(cursor.rec_no(), Some(0), "stay on last row");
    assert!(cursor.bof(), "bof stays true on 1-row result");
    assert!(cursor.eof(), "eof becomes true after next on last row");

    cursor.last().expect("last on single");
    assert_eq!(cursor.rec_no(), Some(0));
    assert!(!cursor.bof(), "last() sets bof to is_empty (false)");
    assert!(cursor.eof());
}

fn test_navigation_and_edges(make: &dyn Fn(&Store) -> Arc<dyn RowSource>) {
    let store = build_fixed_store(10);
    let source = make(&store);
    let mut cursor = Cursor::new(source, Query::default());

    assert_eq!(cursor.record_count().expect("count"), 10);
    assert!(!cursor.is_empty().expect("is_empty"));
    assert_eq!(cursor.rec_no(), Some(0));
    assert!(cursor.bof());
    assert!(!cursor.eof());

    cursor.first().expect("first");
    assert_eq!(cursor.rec_no(), Some(0));
    assert!(cursor.bof());
    assert!(!cursor.eof());

    cursor.prior().expect("prior at 0");
    assert_eq!(cursor.rec_no(), Some(0));
    assert!(cursor.bof());
    assert!(!cursor.eof());

    cursor.next().expect("next from 0");
    assert_eq!(cursor.rec_no(), Some(1));
    assert!(!cursor.bof());
    assert!(!cursor.eof());

    cursor.last().expect("last");
    assert_eq!(cursor.rec_no(), Some(9));
    assert!(!cursor.bof());
    assert!(cursor.eof());

    cursor.next().expect("next at last row");
    assert_eq!(cursor.rec_no(), Some(9));
    assert!(!cursor.bof());
    assert!(cursor.eof());

    cursor.prior().expect("prior from 9");
    assert_eq!(cursor.rec_no(), Some(8));
    assert!(!cursor.bof());
    assert!(!cursor.eof());

    assert_eq!(cursor.move_by(0).expect("move_by 0"), 0);
    assert_eq!(cursor.rec_no(), Some(8));

    assert_eq!(cursor.move_by(5).expect("move_by 5 clamps at 9"), 1);
    assert_eq!(cursor.rec_no(), Some(9));
    assert!(cursor.eof());
    assert!(!cursor.bof());

    assert_eq!(cursor.move_by(-5).expect("move_by -5 to 4"), -5);
    assert_eq!(cursor.rec_no(), Some(4));
    assert!(!cursor.bof());
    assert!(!cursor.eof());

    assert_eq!(cursor.move_by(-20).expect("move_by -20 clamps at 0"), -4);
    assert_eq!(cursor.rec_no(), Some(0));
    assert!(cursor.bof());
    assert!(!cursor.eof());

    cursor.set_rec_no(5).expect("set_rec_no 5");
    assert_eq!(cursor.rec_no(), Some(5));
    assert!(!cursor.bof());
    assert!(!cursor.eof());

    assert_eq!(
        cursor.set_rec_no(10),
        Err(SourceError::OutOfRange { index: 10, len: 10 })
    );
    assert_eq!(cursor.rec_no(), Some(5), "cursor must not move on error");

    cursor.set_rec_no(0).expect("set_rec_no 0");
    assert_eq!(cursor.rec_no(), Some(0));
    assert!(cursor.bof());
    assert!(!cursor.eof());

    cursor.set_rec_no(9).expect("set_rec_no 9");
    assert_eq!(cursor.rec_no(), Some(9));
    assert!(!cursor.bof());
    assert!(cursor.eof());
}

fn test_fields_nulls_and_dates(make: &dyn Fn(&Store) -> Arc<dyn RowSource>) {
    let store = build_fixed_store(10);
    let source = make(&store);
    let mut cursor = Cursor::new(source, Query::default());

    assert_eq!(
        cursor.field("nonexistent").err(),
        Some(SourceError::UnknownField("nonexistent".to_string()))
    );
    assert_eq!(
        cursor.field("id").err(),
        Some(SourceError::NotPositioned),
        "a field read before any move must say the cursor is not positioned"
    );
    assert_eq!(cursor.fields().count(), 0);

    // Row 0
    cursor.first().expect("first");
    let f_id = cursor.field("id").expect("id");
    assert_eq!(f_id.as_i64(), Some(0));
    assert_eq!(f_id.as_f64(), Some(0.0), "Int must convert to f64");
    assert!(!f_id.is_null());

    let f_name = cursor.field("name").expect("name");
    assert_eq!(f_name.as_str(), Some("alpha"));
    assert_eq!(f_name.to_string_lossy(), "alpha");

    let f_price = cursor.field("price").expect("price");
    assert_eq!(f_price.as_f64(), Some(10.5));

    let f_day = cursor.field("day").expect("day");
    assert_eq!(f_day.as_date(), Some(Date::new(1970, 1, 1)));
    assert_eq!(f_day.to_string_lossy(), "1970-01-01");

    let f_flag = cursor.field("flag").expect("flag");
    assert_eq!(f_flag.as_bool(), Some(true));

    // Row 1: null name, day = 19723 (2024-01-01)
    cursor.next().expect("next");
    let f_name1 = cursor.field("name").expect("name");
    assert!(f_name1.is_null());
    assert_eq!(f_name1.as_str(), None);
    assert_eq!(f_name1.value(), Value::Null);

    let f_day1 = cursor.field("day").expect("day");
    assert_eq!(f_day1.as_date(), Some(Date::new(2024, 1, 1)));
    assert_eq!(f_day1.to_string_lossy(), "2024-01-01");

    // Row 2: null price, day = -1 (1969-12-31)
    cursor.next().expect("next");
    let f_price2 = cursor.field("price").expect("price");
    assert!(f_price2.is_null());
    assert_eq!(f_price2.as_f64(), None);

    let f_day2 = cursor.field("day").expect("day");
    assert_eq!(f_day2.as_date(), Some(Date::new(1969, 12, 31)));
    assert_eq!(f_day2.to_string_lossy(), "1969-12-31");

    // Row 3: null day, null flag
    cursor.next().expect("next");
    let f_day3 = cursor.field("day").expect("day");
    assert!(f_day3.is_null());
    assert_eq!(f_day3.as_date(), None);

    let f_flag3 = cursor.field("flag").expect("flag");
    assert!(f_flag3.is_null());
    assert_eq!(f_flag3.as_bool(), None);

    // Schema order in fields()
    let field_names: Vec<String> = cursor.fields().map(|f| f.name().to_string()).collect();
    assert_eq!(field_names, vec!["id", "name", "price", "day", "flag"]);
}

fn test_filtered_and_sorted_query(make: &dyn Fn(&Store) -> Arc<dyn RowSource>) {
    let store = build_fixed_store(10);
    let source = make(&store);

    let mut q = Query::new();
    q.filter = Some(Filter::gt("price", Owned::Float(20.0)));
    q.sort.push(SortKey {
        column: "price".into(),
        order: Order::Desc,
        nulls: Nulls::Last,
    });

    let count = source.count(&q).expect("source count");
    let direct_rows = source.fetch(&q, 0..count).expect("source fetch");

    let mut cursor = Cursor::new(source, q);
    assert_eq!(cursor.record_count().expect("cursor count"), count);

    cursor.first().expect("first");
    let mut cursor_ids = Vec::with_capacity(count);
    while !cursor.eof() {
        let id = cursor.field("id").expect("id").as_i64().expect("i64");
        cursor_ids.push(id);
        cursor.next().expect("next");
    }

    assert_eq!(cursor_ids.len(), count);
    for (i, &cid) in cursor_ids.iter().enumerate() {
        assert_eq!(direct_rows.columns[0].get(i), Value::Int(cid));
    }
}

fn test_rows_iteration(make: &dyn Fn(&Store) -> Arc<dyn RowSource>) {
    let store = build_fixed_store(10);
    let source = make(&store);
    let mut cursor = Cursor::new(source, Query::default());

    let mut it = cursor.rows();
    let mut visited = 0;
    while let Some(row) = it.next_row().expect("next_row") {
        assert_eq!(row.rec_no(), visited);
        let id = row.field("id").expect("id").as_i64().expect("as_i64");
        assert_eq!(id, visited as i64);
        visited += 1;
    }
    assert_eq!(visited, 10);
}

fn test_listeners(make: &dyn Fn(&Store) -> Arc<dyn RowSource>) {
    let store = build_fixed_store(10);
    let source = make(&store);
    let mut cursor = Cursor::new(source, Query::default());

    let scrolled_log = Arc::new(Mutex::new(Vec::new()));
    let query_changed = Arc::new(AtomicUsize::new(0));

    let sc_clone = scrolled_log.clone();
    let qc_clone = query_changed.clone();

    let lid = cursor.on_event(move |ev| match ev {
        CursorEvent::Scrolled { rec_no } => {
            sc_clone.lock().expect("lock").push(*rec_no);
        }
        CursorEvent::QueryChanged => {
            qc_clone.fetch_add(1, Ordering::Relaxed);
        }
        CursorEvent::Busy | CursorEvent::Idle => {}
    });

    // first() when already at row 0 is a no-op move -> must not fire
    cursor.first().expect("first");
    assert!(scrolled_log.lock().expect("lock").is_empty());

    // next() moves from 0 to 1 -> fires 1
    cursor.next().expect("next to 1");
    assert_eq!(*scrolled_log.lock().expect("lock"), vec![1]);

    // next() moves from 1 to 2 -> fires 2
    cursor.next().expect("next to 2");
    assert_eq!(*scrolled_log.lock().expect("lock"), vec![1, 2]);

    // set_rec_no to current position 2 is a no-op -> must not fire
    cursor.set_rec_no(2).expect("set_rec_no 2");
    assert_eq!(*scrolled_log.lock().expect("lock"), vec![1, 2]);

    // last() moves to 9 -> fires 9
    cursor.last().expect("last to 9");
    assert_eq!(*scrolled_log.lock().expect("lock"), vec![1, 2, 9]);

    // next() on last row is a no-op -> must not fire
    cursor.next().expect("next on 9");
    assert_eq!(*scrolled_log.lock().expect("lock"), vec![1, 2, 9]);

    // move_by(0) is a no-op -> must not fire
    cursor.move_by(0).expect("move_by 0");
    assert_eq!(*scrolled_log.lock().expect("lock"), vec![1, 2, 9]);

    // set_query fires QueryChanged, not Scrolled
    cursor.set_query(Query::default());
    assert_eq!(query_changed.load(Ordering::Relaxed), 1);
    assert_eq!(*scrolled_log.lock().expect("lock"), vec![1, 2, 9]);

    // remove_listener unregisters
    cursor.remove_listener(lid);
    cursor.next().expect("next after remove");
    assert_eq!(*scrolled_log.lock().expect("lock"), vec![1, 2, 9]);
}

fn test_locate(make: &dyn Fn(&Store) -> Arc<dyn RowSource>) {
    let store = build_fixed_store(10);
    let mut cursor = Cursor::open(make(&store), Query::default()).expect("open");

    // Exact match
    let found = cursor
        .locate(
            &[("name", Owned::Text("bravo".into()))],
            LocateOptions::default(),
        )
        .expect("locate exact");
    assert!(found);
    assert_eq!(cursor.rec_no(), Some(2));
    assert_eq!(cursor.field("id").expect("id").as_i64(), Some(2));

    // Partial match
    let found = cursor
        .locate(
            &[("name", Owned::Text("char".into()))],
            LocateOptions {
                case_insensitive: false,
                partial_key: true,
            },
        )
        .expect("locate partial");
    assert!(found);
    assert_eq!(cursor.rec_no(), Some(3));
    assert_eq!(cursor.field("id").expect("id").as_i64(), Some(3));

    // Case-insensitive match
    let found = cursor
        .locate(
            &[("name", Owned::Text("ALPHA".into()))],
            LocateOptions {
                case_insensitive: true,
                partial_key: false,
            },
        )
        .expect("locate case-insensitive");
    assert!(found);
    assert_eq!(cursor.rec_no(), Some(0));
    assert_eq!(cursor.field("id").expect("id").as_i64(), Some(0));

    // Two keys match
    let found = cursor
        .locate(
            &[("price", Owned::Float(10.5)), ("flag", Owned::Bool(true))],
            LocateOptions::default(),
        )
        .expect("locate two keys");
    assert!(found);
    assert_eq!(cursor.rec_no(), Some(0));

    // Not found: cursor must not move and no Scrolled event emitted
    cursor.set_rec_no(1).expect("set_rec_no 1");
    let scrolled_log = Arc::new(Mutex::new(Vec::new()));
    let sc_sink = scrolled_log.clone();
    let events_log = Arc::new(Mutex::new(Vec::new()));
    let ev_sink = events_log.clone();
    cursor.on_event(move |ev| {
        ev_sink.lock().expect("lock").push(ev.clone());
        if let CursorEvent::Scrolled { rec_no } = ev {
            sc_sink.lock().expect("lock").push(*rec_no);
        }
    });

    let found = cursor
        .locate(
            &[("name", Owned::Text("nonexistent".into()))],
            LocateOptions::default(),
        )
        .expect("locate nonexistent");
    assert!(!found);
    assert_eq!(cursor.rec_no(), Some(1));
    assert!(
        scrolled_log.lock().expect("lock").is_empty(),
        "no Scrolled on not found"
    );

    // Busy/Idle frame locate
    let events = events_log.lock().expect("lock").clone();
    assert!(
        events.contains(&CursorEvent::Busy) && events.contains(&CursorEvent::Idle),
        "Busy and Idle must frame locate"
    );

    // Unknown key
    let err = cursor.locate(
        &[("nonexistent_column", Owned::Int(1))],
        LocateOptions::default(),
    );
    assert_eq!(
        err,
        Err(SourceError::UnknownField("nonexistent_column".into()))
    );
}

fn test_lookup(make: &dyn Fn(&Store) -> Arc<dyn RowSource>) {
    let store = build_fixed_store(10);
    let mut cursor = Cursor::open(make(&store), Query::default()).expect("open");
    cursor.set_rec_no(2).expect("set_rec_no 2");

    // Lookup existing value without moving cursor
    let val = cursor
        .lookup(
            &[("name", Owned::Text("alpha".into()))],
            LocateOptions::default(),
            "price",
        )
        .expect("lookup");
    assert_eq!(val, Some(Owned::Float(10.5)));
    assert_eq!(cursor.rec_no(), Some(2), "lookup must not move cursor");

    // Lookup on null field
    let val_null = cursor
        .lookup(
            &[("name", Owned::Text("bravo".into()))],
            LocateOptions::default(),
            "price",
        )
        .expect("lookup null");
    assert_eq!(val_null, None);
    assert_eq!(cursor.rec_no(), Some(2));

    // Lookup not found
    let val_none = cursor
        .lookup(
            &[("name", Owned::Text("nonexistent".into()))],
            LocateOptions::default(),
            "price",
        )
        .expect("lookup nonexistent");
    assert_eq!(val_none, None);
    assert_eq!(cursor.rec_no(), Some(2));

    // Lookup unknown result field
    let err = cursor.lookup(
        &[("name", Owned::Text("alpha".into()))],
        LocateOptions::default(),
        "unknown_field",
    );
    assert_eq!(err, Err(SourceError::UnknownField("unknown_field".into())));
}

fn test_bookmark(make: &dyn Fn(&Store) -> Arc<dyn RowSource>) {
    let store = build_fixed_store(10);
    let mut cursor = Cursor::open(make(&store), Query::default()).expect("open");

    // Position on row 2 (id = 2)
    cursor.set_rec_no(2).expect("set_rec_no 2");
    let bm = cursor.bookmark().expect("bookmark on row 2");
    // the bookmark is the source's row identity (store row in memory, rowid in SQL): opaque here
    assert_eq!(cursor.row_id(), Some(bm.id()));

    // Change query with another sort order
    let mut q = Query::default();
    q.sort.push(SortKey {
        column: "price".into(),
        order: Order::Desc,
        nulls: Nulls::Last,
    });
    cursor.set_query(q);
    cursor.first().expect("first on new query");

    // goto_bookmark lands on the same row id
    let moved = cursor.goto_bookmark(bm).expect("goto_bookmark");
    assert!(moved);
    assert_eq!(cursor.field("id").expect("id").as_i64(), Some(2));

    // Exclude row 2 with filter
    let q_exclude = Query::new().filter(Filter::gt("id", Owned::Int(5)));
    cursor.set_query(q_exclude);
    cursor.first().expect("first on excluded query");
    let rec_before = cursor.rec_no();

    let moved_ex = cursor.goto_bookmark(bm).expect("goto_bookmark excluded");
    assert!(!moved_ex);
    assert_eq!(
        cursor.rec_no(),
        rec_before,
        "cursor must not move if excluded"
    );
}

fn compare_opt_owned(a: &Option<Owned>, b: &Option<Owned>) -> std::cmp::Ordering {
    match (a, b) {
        (None, None) => std::cmp::Ordering::Equal,
        (None, Some(_)) => std::cmp::Ordering::Less,
        (Some(_), None) => std::cmp::Ordering::Greater,
        (Some(oa), Some(ob)) => oa.canonical_cmp(ob),
    }
}

fn test_grouped_rows_order(make: &dyn Fn(&Store) -> Arc<dyn RowSource>) {
    let store = build_grouping_store();

    // 1-level grouping with extra user sort on id desc
    let q1 = Query::new()
        .group("name")
        .with_total(Total::count("id"))
        .with_total(Total::sum("id"))
        .sort_desc("id");
    let mut cursor = Cursor::open(make(&store), q1).expect("open q1");
    let mut rows_count = 0;
    let mut prev_path: Option<Vec<Option<Owned>>> = None;
    let mut prev_id: Option<i64> = None;

    while !cursor.eof() {
        let path = cursor.group_path().expect("group_path exists");
        let id = cursor
            .field("id")
            .expect("id exists")
            .as_i64()
            .expect("as_i64");
        rows_count += 1;

        if let Some(prev) = prev_path {
            let ord = compare_opt_owned(&prev[0], &path[0]);
            assert!(
                ord != std::cmp::Ordering::Greater,
                "group path must be non-decreasing: {:?} vs {:?}",
                prev,
                path
            );
            if ord == std::cmp::Ordering::Equal {
                assert!(
                    id <= prev_id.expect("prev_id"),
                    "user sort on id desc must hold within group: {id} vs {:?}",
                    prev_id
                );
            }
        }
        prev_path = Some(path);
        prev_id = Some(id);
        cursor.next().expect("next");
    }
    assert_eq!(rows_count, 64);

    // 2-level grouping with extra user sort on id desc
    let q2 = Query::new()
        .group("name")
        .with_total(Total::count("id"))
        .with_total(Total::sum("id"))
        .group("day")
        .with_total(Total::count("id"))
        .with_total(Total::sum("id"))
        .sort_desc("id");
    let mut cursor = Cursor::open(make(&store), q2).expect("open q2");
    let mut rows_count = 0;
    let mut prev_path: Option<Vec<Option<Owned>>> = None;
    let mut prev_id: Option<i64> = None;

    while !cursor.eof() {
        let path = cursor.group_path().expect("group_path exists");
        let id = cursor
            .field("id")
            .expect("id exists")
            .as_i64()
            .expect("as_i64");
        rows_count += 1;

        if let Some(prev) = prev_path {
            let ord0 = compare_opt_owned(&prev[0], &path[0]);
            let ord = if ord0 == std::cmp::Ordering::Equal {
                compare_opt_owned(&prev[1], &path[1])
            } else {
                ord0
            };
            assert!(
                ord != std::cmp::Ordering::Greater,
                "group path must be non-decreasing: {:?} vs {:?}",
                prev,
                path
            );
            if ord == std::cmp::Ordering::Equal {
                assert!(
                    id <= prev_id.expect("prev_id"),
                    "user sort on id desc must hold within group: {id} vs {:?}",
                    prev_id
                );
            }
        }
        prev_path = Some(path);
        prev_id = Some(id);
        cursor.next().expect("next");
    }
    assert_eq!(rows_count, 64);
}

fn test_lines_iteration(make: &dyn Fn(&Store) -> Arc<dyn RowSource>) {
    let store = build_grouping_store();
    let q = Query::new()
        .group("name")
        .with_total(Total::count("id"))
        .with_total(Total::sum("id"))
        .group("day")
        .with_total(Total::count("id"))
        .with_total(Total::sum("id"))
        .sort_desc("id");
    let source = make(&store);
    let mut cursor = Cursor::open(source.clone(), q.clone()).expect("open");
    let mut it = cursor.lines();

    let mut active_headers: Vec<Option<SourceGroup>> = vec![None, None];
    let mut row_accum: Vec<usize> = vec![0, 0];
    let mut current_path: Vec<Option<Owned>> = Vec::new();
    let mut total_rows = 0;

    while let Some(line) = it.next_line().expect("next_line") {
        match line {
            CursorLine::Header { level, group } => {
                let expected_groups = source
                    .groups(&q, &current_path[..level])
                    .expect("source.groups");
                let matched = expected_groups
                    .iter()
                    .find(|g| g.key == group.key)
                    .expect("group found in source.groups");
                assert_eq!(matched.row_count, group.row_count);
                assert_eq!(matched.totals, group.totals);

                for l in level..2 {
                    if let Some(prev_grp) = active_headers[l].take() {
                        assert_eq!(
                            row_accum[l], prev_grp.row_count,
                            "header count at level {l} must match rows that followed"
                        );
                        row_accum[l] = 0;
                    }
                }

                active_headers[level] = Some(group.clone());
                row_accum[level] = 0;

                if current_path.len() <= level {
                    current_path.resize(level + 1, None);
                }
                current_path[level] = group.key[0].clone();
                current_path.truncate(level + 1);
            }
            CursorLine::Row(_) => {
                total_rows += 1;
                for l in 0..2 {
                    if active_headers[l].is_some() {
                        row_accum[l] += 1;
                    }
                }
            }
        }
    }

    assert_eq!(total_rows, 64);
    for l in 0..2 {
        if let Some(prev_grp) = active_headers[l].take() {
            assert_eq!(
                row_accum[l], prev_grp.row_count,
                "trailing header count at level {l} must match rows"
            );
        }
    }

    assert!(it.cached_pages() > 0);
    assert!(it.fetch_calls() > 0);

    // Ungrouped query yields only rows
    let q_ungrouped = Query::default();
    let mut cursor_ungrouped = Cursor::open(source, q_ungrouped).expect("open ungrouped");
    let mut it_ungrouped = cursor_ungrouped.lines();
    let mut ung_rows = 0;
    while let Some(line) = it_ungrouped.next_line().expect("next_line") {
        match line {
            CursorLine::Header { .. } => panic!("ungrouped query must not emit headers"),
            CursorLine::Row(_) => ung_rows += 1,
        }
    }
    assert_eq!(ung_rows, 64);
}

fn test_group_path_and_group(make: &dyn Fn(&Store) -> Arc<dyn RowSource>) {
    let store = build_grouping_store();
    let q = Query::new()
        .group("name")
        .with_total(Total::count("id"))
        .group("day")
        .with_total(Total::count("id"))
        .sort_asc("id");
    let mut cursor = Cursor::open(make(&store), q).expect("open");

    // Row 0 is in null-key group for name and null-key group for day (nulls first)
    assert_eq!(cursor.rec_no(), Some(0));
    let path = cursor.group_path().expect("group_path");
    assert_eq!(path, vec![None, None]);

    let g0 = cursor.group(0).expect("group 0").expect("some g0");
    assert_eq!(g0.key, vec![None]);
    assert_eq!(g0.row_count, 16);

    let g1 = cursor.group(1).expect("group 1").expect("some g1");
    assert_eq!(g1.key, vec![None]);
    assert_eq!(g1.row_count, 4);

    let g2 = cursor.group(2).expect("group 2 out of range");
    assert!(g2.is_none());
    let g99 = cursor.group(99).expect("group 99 out of range");
    assert!(g99.is_none());

    // Advance to a non-null group row
    while !cursor.eof() {
        if let Some(p) = cursor.group_path()
            && p[0].is_some()
            && p[1].is_some()
        {
            break;
        }
        cursor.next().expect("next");
    }
    assert!(!cursor.eof());
    let path_nn = cursor.group_path().expect("group_path non-null");
    let g0_nn = cursor.group(0).expect("group 0").expect("some g0");
    assert_eq!(g0_nn.key, vec![path_nn[0].clone()]);
    assert_eq!(g0_nn.row_count, 16);

    let g1_nn = cursor.group(1).expect("group 1").expect("some g1");
    assert_eq!(g1_nn.key, vec![path_nn[1].clone()]);
    assert_eq!(g1_nn.row_count, 4);
    assert!(cursor.group(2).expect("group 2").is_none());
}

fn test_group_busy_idle(make: &dyn Fn(&Store) -> Arc<dyn RowSource>) {
    let store = build_grouping_store();
    let q = Query::new().group("name").group("day");
    let mut cursor = Cursor::open(make(&store), q).expect("open");

    let events = Arc::new(Mutex::new(Vec::new()));
    let sink = events.clone();
    cursor.on_event(move |ev| sink.lock().expect("lock").push(ev.clone()));

    // Clear events from open/first
    events.lock().expect("lock").clear();

    // First group(0) call: parent path is &[] -> cache miss -> Busy and Idle emitted
    let _ = cursor.group(0).expect("group 0");
    {
        let evs = events.lock().expect("lock");
        assert!(
            evs.contains(&CursorEvent::Busy) && evs.contains(&CursorEvent::Idle),
            "Busy/Idle must frame cache miss"
        );
    }

    // Clear events
    events.lock().expect("lock").clear();

    // Second group(0) call on same parent: cache hit -> emits nothing!
    let _ = cursor.group(0).expect("group 0 second time");
    {
        let evs = events.lock().expect("lock");
        assert!(
            evs.is_empty(),
            "second group() call on same parent must emit nothing, got: {:?}",
            *evs
        );
    }
}
