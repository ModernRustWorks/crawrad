use crate::column::Column;
use crate::group::Total;
use crate::live::{LiveRows, Screen, ScreenLine};
use crate::query::Query;
use crate::source::{
    Capabilities, ColumnInfo, ColumnKind, DisplayHint, FakeSlowSource, RowSource, Rows,
    SourceError, VecSource,
};
use crate::store::{Store, StoreError};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::{Duration, Instant};

fn make_slow_source(num_rows: usize, delay: Duration) -> (Arc<FakeSlowSource>, Arc<dyn RowSource>) {
    let mut store = Store::new();
    let values: Vec<Option<i64>> = (0..num_rows as i64).map(Some).collect();
    store.add("val", Column::ints(values)).expect("add val");

    let schema = vec![ColumnInfo {
        name: "val".into(),
        kind: ColumnKind::Int,
        nullable: false,
        hint: DisplayHint::Plain,
    }];

    let vec_source = Box::new(VecSource::new(store, schema).expect("VecSource"));
    let slow = Arc::new(FakeSlowSource::new(vec_source, delay));
    let source: Arc<dyn RowSource> = slow.clone();
    (slow, source)
}

fn poll_until(live: &mut LiveRows, f: impl Fn(&Screen) -> bool) {
    let start = Instant::now();
    let timeout = Duration::from_secs(5);
    while start.elapsed() < timeout {
        live.poll();
        if f(live.screen()) {
            return;
        }
        std::thread::sleep(Duration::from_millis(5));
    }
    panic!(
        "poll_until timed out after 5s; current screen: {:?}",
        live.screen()
    );
}

#[test]
fn test_show_and_set_query_non_blocking() {
    let (_slow, source) = make_slow_source(10_000, Duration::from_millis(300));
    let mut live = LiveRows::spawn(source, Query::default(), 256, 4);

    let t0 = Instant::now();
    live.show(500, 10);
    let d_show = t0.elapsed();
    assert!(
        d_show < Duration::from_millis(20),
        "show took {:?}, expected < 20 ms",
        d_show
    );

    let t1 = Instant::now();
    live.set_query(Query::default());
    let d_query = t1.elapsed();
    assert!(
        d_query < Duration::from_millis(20),
        "set_query took {:?}, expected < 20 ms",
        d_query
    );
}

#[test]
fn test_show_fetches_rows() {
    const N: usize = 1_000;
    let (_slow, source) = make_slow_source(N, Duration::from_millis(5));
    let mut live = LiveRows::spawn(source, Query::default(), 256, 4);

    live.show(500, 10);
    poll_until(&mut live, |s| {
        s.first == 500 && s.rows().count() == 10 && s.pending.is_none()
    });

    let screen = live.screen();
    assert_eq!(screen.first, 500);
    assert_eq!(screen.total, Some(N));
    assert_eq!(screen.pending, None);
    assert_eq!(screen.rows().count(), 10);
    for (i, row) in screen.rows().enumerate() {
        assert_eq!(row[0], Some((500 + i).to_string()));
    }
}

#[test]
fn test_pending_while_in_flight() {
    let (_slow, source) = make_slow_source(10_000, Duration::from_millis(300));
    let mut live = LiveRows::spawn(source, Query::default(), 256, 4);

    live.show(0, 10);
    poll_until(&mut live, |s| {
        s.first == 0 && s.rows().count() == 10 && s.pending.is_none()
    });
    let initial_lines = live.screen().lines.clone();
    assert_eq!(
        live.screen().rows().next().unwrap()[0],
        Some("0".to_string())
    );

    live.show(500, 10);

    let screen = live.screen();
    assert_eq!(screen.pending, Some((500, 10)));
    assert_eq!(screen.lines, initial_lines);

    std::thread::sleep(Duration::from_millis(50));
    live.poll();
    let screen = live.screen();
    assert_eq!(screen.pending, Some((500, 10)));
    assert_eq!(screen.lines, initial_lines);

    poll_until(&mut live, |s| s.first == 500 && s.pending.is_none());
    assert_ne!(live.screen().lines, initial_lines);
    assert_eq!(
        live.screen().rows().next().unwrap()[0],
        Some("500".to_string())
    );
}

#[test]
fn test_latest_request_wins() {
    let (slow, source) = make_slow_source(50_000, Duration::from_millis(100));
    let mut live = LiveRows::spawn(source, Query::default(), 256, 4);

    for i in 0..30 {
        live.show(i * 1000, 10);
    }

    poll_until(&mut live, |s| {
        s.first == 29_000 && s.rows().count() == 10 && s.pending.is_none()
    });
    let screen = live.screen();
    assert_eq!(screen.first, 29_000);
    assert_eq!(screen.rows().next().unwrap()[0], Some("29000".to_string()));

    let stats = slow.stats();
    assert!(
        stats.fetch_calls < 10,
        "fetch_calls was {}, expected < 10",
        stats.fetch_calls
    );
}

#[test]
fn test_busy_telemetry_and_cached() {
    let (slow, source) = make_slow_source(10_000, Duration::from_millis(200));
    let mut live = LiveRows::spawn(source, Query::default(), 256, 4);

    let asked = Instant::now();
    live.show(500, 10);
    poll_until(&mut live, |s| {
        s.first == 500 && s.pending.is_none() && s.busy.and_then(|b| b.ended).is_some()
    });

    // A cold show is several source calls (the count, then the page and its
    // prefetched neighbours): ONE wait spanning them all, so the spinner's
    // clock never restarts in the middle.
    let stats = slow.stats();
    assert_eq!(stats.count_calls, 1);
    assert!(stats.fetch_calls >= 1);
    let screen = live.screen();
    let busy1 = screen.busy.expect("busy must be Some");
    let ended1 = busy1.ended.expect("ended must be Some");
    let duration = ended1.duration_since(busy1.started);
    assert!(
        duration >= Duration::from_millis(380),
        "busy duration was {:?}, expected one wait over both calls (>= 380 ms)",
        duration
    );
    // ...and it starts at the FIRST call, right after the request, not at a later one.
    let late = busy1.started.duration_since(asked);
    assert!(
        late < Duration::from_millis(100),
        "the wait started {late:?} after the request"
    );

    live.show(500, 10);
    poll_until(&mut live, |s| s.pending.is_none());

    let screen2 = live.screen();
    let busy2 = screen2.busy.expect("busy must still be Some");
    assert_eq!(
        busy2.started, busy1.started,
        "cached show must leave busy unchanged (same started)"
    );
    assert_eq!(
        busy2.ended, busy1.ended,
        "cached show must leave busy unchanged (same ended)"
    );
}

#[test]
fn test_set_query_sort_descending() {
    const N: usize = 1_000;
    let (_slow, source) = make_slow_source(N, Duration::from_millis(5));
    let mut live = LiveRows::spawn(source, Query::default(), 256, 4);

    live.show(500, 10);
    poll_until(&mut live, |s| s.first == 500 && s.pending.is_none());

    let query = Query::default().sort_desc("val");
    live.set_query(query);

    poll_until(&mut live, |s| {
        s.first == 0 && s.rows().next().is_some() && s.pending.is_none()
    });

    let screen = live.screen();
    assert_eq!(screen.first, 0);
    assert_eq!(screen.rows().next().unwrap()[0].as_deref(), Some("999"));
}

#[test]
fn test_drop_with_fetch_in_flight() {
    let (_slow, source) = make_slow_source(10_000, Duration::from_millis(300));
    let mut live = LiveRows::spawn(source, Query::default(), 256, 4);

    live.show(500, 10);
    std::thread::sleep(Duration::from_millis(50));

    let t0 = Instant::now();
    drop(live);
    let elapsed = t0.elapsed();

    assert!(
        elapsed < Duration::from_secs(1),
        "drop took {:?}, expected < 1 s",
        elapsed
    );
}

struct FailableSource {
    inner: VecSource,
    fail: AtomicBool,
}

impl FailableSource {
    fn new(inner: VecSource) -> Self {
        Self {
            inner,
            fail: AtomicBool::new(true),
        }
    }
}

impl RowSource for FailableSource {
    fn schema(&self) -> &[ColumnInfo] {
        self.inner.schema()
    }

    fn capabilities(&self) -> Capabilities {
        self.inner.capabilities()
    }

    fn count(&self, query: &Query) -> Result<usize, SourceError> {
        self.inner.count(query)
    }

    fn fetch(&self, query: &Query, range: std::ops::Range<usize>) -> Result<Rows, SourceError> {
        if self.fail.load(Ordering::SeqCst) {
            Err(SourceError::Store(StoreError::TypeMismatch {
                column: "val".into(),
                expected: "int",
            }))
        } else {
            self.inner.fetch(query, range)
        }
    }
}

#[test]
fn test_source_error_resilience() {
    let mut store = Store::new();
    let values: Vec<Option<i64>> = (0..100).map(Some).collect();
    store.add("val", Column::ints(values)).expect("add val");

    let schema = vec![ColumnInfo {
        name: "val".into(),
        kind: ColumnKind::Int,
        nullable: false,
        hint: DisplayHint::Plain,
    }];

    let vec_source = VecSource::new(store, schema).expect("VecSource");
    let failable = Arc::new(FailableSource::new(vec_source));
    let source: Arc<dyn RowSource> = failable.clone();

    let mut live = LiveRows::spawn(source, Query::default(), 256, 4);

    // Initial show with failing fetch
    live.show(0, 10);
    poll_until(&mut live, |s| s.error.is_some() && s.pending.is_none());

    assert!(live.screen().error.is_some());
    assert_eq!(live.screen().rows().count(), 0);

    // Fix the source so fetch succeeds
    failable.fail.store(false, Ordering::SeqCst);

    // Later show still answers
    live.show(0, 10);
    poll_until(&mut live, |s| {
        s.error.is_none() && s.rows().count() == 10 && s.pending.is_none()
    });

    let screen = live.screen();
    assert_eq!(screen.first, 0);
    assert_eq!(screen.rows().count(), 10);
    assert_eq!(screen.rows().next().unwrap()[0].as_deref(), Some("0"));
}

/// Past the end before the total is known: the worker clamps to the last rows.
#[test]
fn test_show_past_the_end_before_the_total_is_known() {
    let (_slow, source) = make_slow_source(1_000, Duration::from_millis(5));
    let mut live = LiveRows::spawn(source, Query::default(), 256, 4);

    live.show(5_000, 10);
    poll_until(&mut live, |s| {
        s.pending.is_none() && s.rows().next().is_some()
    });

    let screen = live.screen();
    assert_eq!(screen.first, 990);
    assert_eq!(screen.rows().count(), 10);
    assert_eq!(screen.rows().nth(9).unwrap()[0].as_deref(), Some("999"));
}

#[test]
fn test_grouped_single_level() {
    const NUM_ROWS: usize = 1_000;
    let mut store = Store::new();
    let g_values: Vec<Option<i64>> = (0..NUM_ROWS as i64).map(|i| Some(i / 500)).collect();
    let val_values: Vec<Option<i64>> = (0..NUM_ROWS as i64).map(Some).collect();
    store.add("g", Column::ints(g_values)).expect("add g");
    store.add("val", Column::ints(val_values)).expect("add val");

    let schema = vec![
        ColumnInfo {
            name: "g".into(),
            kind: ColumnKind::Int,
            nullable: false,
            hint: DisplayHint::Plain,
        },
        ColumnInfo {
            name: "val".into(),
            kind: ColumnKind::Int,
            nullable: false,
            hint: DisplayHint::Plain,
        },
    ];

    let source = Arc::new(VecSource::new(store, schema).expect("VecSource"));
    let query = Query::new().group("g");
    let mut live = LiveRows::spawn(source, query, 256, 4);

    live.show(495, 10);
    poll_until(&mut live, |s| {
        s.first == 495 && s.rows().count() == 10 && s.pending.is_none()
    });

    let screen = live.screen();
    assert_eq!(screen.lines.len(), 12); // 2 headers + 10 rows

    // Line 0: Header level 0, key "0", row_count 500
    match &screen.lines[0] {
        ScreenLine::Header {
            level,
            key,
            row_count,
            column,
            ..
        } => {
            assert_eq!(*level, 0);
            assert_eq!(key, "0");
            assert_eq!(*row_count, 500);
            assert_eq!(column, "g");
        }
        other => panic!("expected Header, got {:?}", other),
    }

    // Lines 1..6: 5 rows (495..499)
    for (i, row_idx) in (495..500).enumerate() {
        match &screen.lines[1 + i] {
            ScreenLine::Row { index, cells, .. } => {
                assert_eq!(*index, row_idx);
                assert_eq!(cells[0].as_deref(), Some("0"));
                assert_eq!(cells[1].as_deref(), Some(row_idx.to_string().as_str()));
            }
            other => panic!("expected Row, got {:?}", other),
        }
    }

    // Line 6: Header level 0, key "1", row_count 500
    match &screen.lines[6] {
        ScreenLine::Header {
            level,
            key,
            row_count,
            column,
            ..
        } => {
            assert_eq!(*level, 0);
            assert_eq!(key, "1");
            assert_eq!(*row_count, 500);
            assert_eq!(column, "g");
        }
        other => panic!("expected Header, got {:?}", other),
    }

    // Lines 7..12: 5 rows (500..504)
    for (i, row_idx) in (500..505).enumerate() {
        match &screen.lines[7 + i] {
            ScreenLine::Row { index, cells, .. } => {
                assert_eq!(*index, row_idx);
                assert_eq!(cells[0].as_deref(), Some("1"));
                assert_eq!(cells[1].as_deref(), Some(row_idx.to_string().as_str()));
            }
            other => panic!("expected Row, got {:?}", other),
        }
    }
}

#[test]
fn test_grouped_two_levels() {
    const NUM_ROWS: usize = 1_000;
    let mut store = Store::new();
    let a_values: Vec<Option<i64>> = (0..NUM_ROWS as i64).map(|i| Some(i / 500)).collect();
    let b_values: Vec<Option<i64>> = (0..NUM_ROWS as i64).map(|i| Some((i / 250) % 2)).collect();
    let val_values: Vec<Option<i64>> = (0..NUM_ROWS as i64).map(Some).collect();
    store.add("a", Column::ints(a_values)).expect("add a");
    store.add("b", Column::ints(b_values)).expect("add b");
    store.add("val", Column::ints(val_values)).expect("add val");

    let schema = vec![
        ColumnInfo {
            name: "a".into(),
            kind: ColumnKind::Int,
            nullable: false,
            hint: DisplayHint::Plain,
        },
        ColumnInfo {
            name: "b".into(),
            kind: ColumnKind::Int,
            nullable: false,
            hint: DisplayHint::Plain,
        },
        ColumnInfo {
            name: "val".into(),
            kind: ColumnKind::Int,
            nullable: false,
            hint: DisplayHint::Plain,
        },
    ];

    let source = Arc::new(VecSource::new(store, schema).expect("VecSource"));
    let query = Query::new().group("a").group("b");
    let mut live = LiveRows::spawn(source, query, 256, 4);

    // Window 245..255: crosses b boundary (at 250) within a=0
    live.show(245, 10);
    poll_until(&mut live, |s| {
        s.first == 245 && s.rows().count() == 10 && s.pending.is_none()
    });

    let screen = live.screen();
    // Top of window must have both levels:
    // Line 0: Header level 0 (a = "0")
    // Line 1: Header level 1 (b = "0")
    // Lines 2..7: Rows 245..249 (5 rows)
    // Line 7: Header level 1 ONLY (b = "1")
    // Lines 8..13: Rows 250..254 (5 rows)
    assert_eq!(screen.lines.len(), 13); // 3 headers + 10 rows

    match &screen.lines[0] {
        ScreenLine::Header {
            level, column, key, ..
        } => {
            assert_eq!(*level, 0);
            assert_eq!(column, "a");
            assert_eq!(key, "0");
        }
        other => panic!("expected Header level 0, got {:?}", other),
    }

    match &screen.lines[1] {
        ScreenLine::Header {
            level, column, key, ..
        } => {
            assert_eq!(*level, 1);
            assert_eq!(column, "b");
            assert_eq!(key, "0");
        }
        other => panic!("expected Header level 1, got {:?}", other),
    }

    for (i, row_idx) in (245..250).enumerate() {
        match &screen.lines[2 + i] {
            ScreenLine::Row { index, .. } => assert_eq!(*index, row_idx),
            other => panic!("expected Row, got {:?}", other),
        }
    }

    match &screen.lines[7] {
        ScreenLine::Header {
            level, column, key, ..
        } => {
            assert_eq!(*level, 1);
            assert_eq!(column, "b");
            assert_eq!(key, "1");
        }
        other => panic!("expected Header level 1, got {:?}", other),
    }

    for (i, row_idx) in (250..255).enumerate() {
        match &screen.lines[8 + i] {
            ScreenLine::Row { index, .. } => assert_eq!(*index, row_idx),
            other => panic!("expected Row, got {:?}", other),
        }
    }
}

#[test]
fn test_ungrouped_has_no_headers() {
    let (_slow, source) = make_slow_source(100, Duration::from_millis(5));
    let mut live = LiveRows::spawn(source, Query::default(), 256, 4);

    live.show(10, 20);
    poll_until(&mut live, |s| {
        s.first == 10 && s.rows().count() == 20 && s.pending.is_none()
    });

    let screen = live.screen();
    assert_eq!(screen.lines.len(), 20);
    for line in &screen.lines {
        assert!(matches!(line, ScreenLine::Row { .. }));
    }
}

#[test]
fn test_grouped_totals() {
    const NUM_ROWS: usize = 100;
    let mut store = Store::new();
    let g_values: Vec<Option<i64>> = (0..NUM_ROWS as i64).map(|i| Some(i / 50)).collect();
    let val_values: Vec<Option<i64>> = (0..NUM_ROWS as i64).map(Some).collect();
    store.add("g", Column::ints(g_values)).expect("add g");
    store.add("val", Column::ints(val_values)).expect("add val");

    let schema = vec![
        ColumnInfo {
            name: "g".into(),
            kind: ColumnKind::Int,
            nullable: false,
            hint: DisplayHint::Plain,
        },
        ColumnInfo {
            name: "val".into(),
            kind: ColumnKind::Int,
            nullable: false,
            hint: DisplayHint::Plain,
        },
    ];

    let source = Arc::new(VecSource::new(store, schema).expect("VecSource"));
    let query = Query::new().group("g").with_total(Total::sum("val"));
    let mut live = LiveRows::spawn(source, query, 256, 4);

    live.show(0, 10);
    poll_until(&mut live, |s| {
        s.first == 0 && s.rows().count() == 10 && s.pending.is_none()
    });

    let screen = live.screen();
    match &screen.lines[0] {
        ScreenLine::Header {
            level,
            column,
            key,
            row_count,
            totals,
        } => {
            assert_eq!(*level, 0);
            assert_eq!(column, "g");
            assert_eq!(key, "0");
            assert_eq!(*row_count, 50);
            assert_eq!(totals.len(), 1);
            assert_eq!(totals[0], Some(1225.0));
        }
        other => panic!("expected Header, got {:?}", other),
    }
}

/// A group whose key is null is labelled "(empty)", not a blank line.
#[test]
fn test_null_group_key_reads_empty() {
    let mut store = Store::new();
    let g_values: Vec<Option<i64>> = (0..20i64).map(|i| (i >= 10).then_some(1)).collect();
    store.add("g", Column::ints(g_values)).expect("add g");
    let schema = vec![ColumnInfo {
        name: "g".into(),
        kind: ColumnKind::Int,
        nullable: true,
        hint: DisplayHint::Plain,
    }];
    let source = Arc::new(VecSource::new(store, schema).expect("VecSource"));
    let mut live = LiveRows::spawn(source, Query::new().group("g"), 256, 4);

    live.show(0, 20);
    poll_until(&mut live, |s| s.rows().count() == 20 && s.pending.is_none());

    let keys: Vec<&str> = live
        .screen()
        .lines
        .iter()
        .filter_map(|line| match line {
            ScreenLine::Header { key, .. } => Some(key.as_str()),
            ScreenLine::Row { .. } => None,
        })
        .collect();
    assert_eq!(keys.len(), 2);
    assert!(keys.contains(&"(empty)") && keys.contains(&"1"), "{keys:?}");
}

#[test]
fn test_null_cells_produce_none() {
    let mut store = Store::new();
    let val_values: Vec<Option<i64>> = vec![Some(10), None, Some(30)];
    store.add("val", Column::ints(val_values)).expect("add val");
    let schema = vec![ColumnInfo {
        name: "val".into(),
        kind: ColumnKind::Int,
        nullable: true,
        hint: DisplayHint::Plain,
    }];
    let source = Arc::new(VecSource::new(store, schema).expect("VecSource"));
    let mut live = LiveRows::spawn(source, Query::default(), 256, 4);

    live.show(0, 3);
    poll_until(&mut live, |s| s.rows().count() == 3 && s.pending.is_none());

    let screen = live.screen();
    let rows: Vec<_> = screen.rows().collect();
    assert_eq!(rows[0][0].as_deref(), Some("10"));
    assert_eq!(rows[1][0], None);
    assert_eq!(rows[2][0].as_deref(), Some("30"));
}

#[test]
fn test_live_grid_summary_published() {
    use crate::group::Aggregate;

    let mut store = Store::new();
    let val_values: Vec<Option<i64>> = vec![Some(10), Some(20), Some(30)];
    store.add("val", Column::ints(val_values)).expect("add val");
    let schema = vec![ColumnInfo {
        name: "val".into(),
        kind: ColumnKind::Int,
        nullable: false,
        hint: DisplayHint::Plain,
    }];
    let source = Arc::new(VecSource::new(store, schema).expect("VecSource"));

    let query = Query {
        summary: vec![
            Total {
                column: "val".into(),
                aggregate: Aggregate::Sum,
            },
            Total {
                column: "val".into(),
                aggregate: Aggregate::Count,
            },
        ],
        ..Default::default()
    };

    let mut live = LiveRows::spawn(source, query, 256, 4);
    live.show(0, 3);
    poll_until(&mut live, |s| s.summary == vec![Some(60.0), Some(3.0)]);

    let q2 = Query {
        summary: vec![Total {
            column: "val".into(),
            aggregate: Aggregate::Max,
        }],
        ..Default::default()
    };
    live.set_query(q2);
    poll_until(&mut live, |s| s.summary == vec![Some(30.0)]);
}

#[test]
fn test_live_rows_answers_distinct() {
    let mut store = Store::new();
    let values = vec![
        Some("Engineering".into()),
        Some("Sales".into()),
        Some("Engineering".into()),
        None,
    ];
    store.add("dept", Column::texts(values)).expect("add dept");
    let schema = vec![ColumnInfo {
        name: "dept".into(),
        kind: ColumnKind::Text,
        nullable: true,
        hint: DisplayHint::Plain,
    }];
    let source = Arc::new(VecSource::new(store, schema).expect("VecSource"));

    let mut live = LiveRows::spawn(source, Query::default(), 256, 4);
    live.show(0, 4);
    poll_until(&mut live, |s| s.rows().count() == 4 && s.pending.is_none());

    live.distinct("dept", Query::default());
    poll_until(&mut live, |s| s.distinct.is_some());

    let screen = live.screen();
    let (col, vals) = screen.distinct.as_ref().unwrap();
    assert_eq!(col, "dept");
    assert_eq!(
        vals,
        &vec![None, Some("Engineering".into()), Some("Sales".into()),]
    );
    // checking a value re-queries; the open list must keep its values
    live.set_query(Query::new().filter(crate::Filter::In(
        "dept".into(),
        vec![crate::filter::Owned::Text("Sales".into())],
    )));
    assert!(live.screen().distinct.is_some());
}
