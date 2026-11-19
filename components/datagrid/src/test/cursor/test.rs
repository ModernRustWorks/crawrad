use crate::column::Column;
use crate::column::Value;
use crate::cursor::{Cursor, CursorLine, Date, Field};
use crate::cursor_contract;
use crate::flat::{GroupedFlat, LineKey, LineKind, WindowedRows};
use crate::group::Total;
use crate::query::Query;
use crate::source::{ColumnInfo, ColumnKind, DisplayHint, VecSource};
use crate::store::Store;
use crate::view::View;
use std::sync::Arc;

#[test]
fn test_cursor_contract_vec_source() {
    cursor_contract::run_all(&|store| {
        Arc::new(
            VecSource::new(store.clone(), cursor_contract::contract_schema())
                .expect("VecSource::new"),
        )
    });
}

#[test]
fn test_bounded_memory_one_million_rows() {
    const NUM_ROWS: usize = 1_000_000;
    const PAGE_SIZE: usize = 1_000;
    const MAX_PAGES: usize = 4;

    let mut store = Store::new();
    let values: Vec<Option<i64>> = (0..NUM_ROWS as i64).map(Some).collect();
    store.add("val", Column::ints(values)).expect("add val");

    let schema = vec![ColumnInfo {
        name: "val".into(),
        kind: ColumnKind::Int,
        nullable: false,
        hint: DisplayHint::Plain,
    }];

    let source = Arc::new(VecSource::new(store, schema).expect("VecSource"));
    let expected_sum: i64 = ((NUM_ROWS as i64 - 1) * (NUM_ROWS as i64)) / 2; // 499_999_500_000

    // 1. Walk with first() + next() until eof
    {
        let mut cursor =
            Cursor::with_window(source.clone(), Query::default(), PAGE_SIZE, MAX_PAGES);
        cursor.first().expect("first");

        let mut sum: i64 = 0;
        let mut count: usize = 0;

        while !cursor.eof() {
            sum += cursor
                .field("val")
                .expect("field val")
                .as_i64()
                .expect("as_i64");
            count += 1;

            if count.is_multiple_of(100_000) {
                assert!(
                    cursor.cached_pages() <= MAX_PAGES,
                    "cached_pages ({}) must be <= MAX_PAGES ({MAX_PAGES}) at row {count}",
                    cursor.cached_pages()
                );
            }

            cursor.next().expect("next");
        }

        assert_eq!(count, NUM_ROWS);
        assert_eq!(sum, expected_sum);
        // WindowedRows fetches pages of 1000 lines. On the forward sequential scan, each page
        // boundary triggers WindowedRows::ensure which prefetches the immediate next page.
        // Because the next page is already in the cache when the cursor advances, exactly 1000
        // fetch calls occur (one per 1000-line page across 1,000,000 rows).
        assert_eq!(cursor.fetch_calls(), 1000);
        assert!(cursor.cached_pages() <= MAX_PAGES);
    }

    // 2. Same walk using rows()
    {
        let mut cursor = Cursor::with_window(source, Query::default(), PAGE_SIZE, MAX_PAGES);
        let mut it = cursor.rows();

        let mut sum: i64 = 0;
        let mut count: usize = 0;

        while let Some(row) = it.next_row().expect("next_row") {
            sum += row
                .field("val")
                .expect("field val")
                .as_i64()
                .expect("as_i64");
            count += 1;

            if count.is_multiple_of(100_000) {
                assert!(
                    it.cached_pages() <= MAX_PAGES,
                    "cached_pages ({}) must be <= MAX_PAGES ({MAX_PAGES}) at row {count}",
                    it.cached_pages()
                );
            }
        }

        assert_eq!(count, NUM_ROWS);
        assert_eq!(sum, expected_sum);
        assert_eq!(it.fetch_calls(), 1000);
        assert!(it.cached_pages() <= MAX_PAGES);
    }
}

/// Every day from year -800 to 3000 matches a calendar walked one day at a time —
/// century leap rules (1900 no, 2000 yes, 2100 no) and negative years included.
#[test]
fn date_from_days_matches_a_day_by_day_calendar() {
    fn is_leap(y: i32) -> bool {
        (y % 4 == 0 && y % 100 != 0) || y % 400 == 0
    }
    fn month_len(y: i32, m: u8) -> u8 {
        match m {
            2 if is_leap(y) => 29,
            2 => 28,
            4 | 6 | 9 | 11 => 30,
            _ => 31,
        }
    }
    // walk backwards from 1970-01-01 to find the start day, then forwards checking every day
    let (mut y, mut m, mut d) = (1970_i32, 1_u8, 1_u8);
    let mut days: i64 = 0;
    while y > -800 {
        if d > 1 {
            d -= 1;
        } else if m > 1 {
            m -= 1;
            d = month_len(y, m);
        } else {
            y -= 1;
            m = 12;
            d = 31;
        }
        days -= 1;
    }
    while y < 3000 {
        assert_eq!(Date::from_days(days), Date::new(y, m, d), "day {days}");
        days += 1;
        if d < month_len(y, m) {
            d += 1;
        } else if m < 12 {
            m += 1;
            d = 1;
        } else {
            y += 1;
            m = 1;
            d = 1;
        }
    }
}

#[test]
fn date_time_takes_the_calendar_day_also_before_the_epoch() {
    let info = ColumnInfo {
        name: "at".into(),
        kind: ColumnKind::Int,
        nullable: false,
        hint: DisplayHint::DateTime,
    };
    let at = |ms: i64| {
        Field {
            info: &info,
            value: Value::Int(ms),
        }
        .as_date()
    };
    assert_eq!(at(0), Some(Date::new(1970, 1, 1)));
    assert_eq!(at(86_399_999), Some(Date::new(1970, 1, 1)));
    assert_eq!(at(-1), Some(Date::new(1969, 12, 31)));
    assert_eq!(at(-86_400_000), Some(Date::new(1969, 12, 31)));
    assert_eq!(at(-86_400_001), Some(Date::new(1969, 12, 30)));
}

#[test]
fn a_window_that_never_counted_needs_a_fetch() {
    let mut store = Store::new();
    store
        .add("v", Column::ints(vec![Some(1), Some(2)]))
        .expect("add");
    let window = WindowedRows::new(
        Arc::new(VecSource::from_store(store)),
        Query::default(),
        1,
        2,
    );
    assert!(window.needs_fetch(0..1));
    assert_eq!(window.fetch_calls(), 0, "asking must never fetch");
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

#[test]
fn test_lines_walk_matches_grouped_flat_oracle() {
    let mut rng = XorShift64::new(0x2026_0674);
    let num_rows = 2000;
    let mut g1_vals = Vec::with_capacity(num_rows);
    let mut g2_vals = Vec::with_capacity(num_rows);
    let mut val_vals = Vec::with_capacity(num_rows);

    let g1_options = [
        None,
        Some("US".to_string()),
        Some("CA".to_string()),
        Some("UK".to_string()),
        Some("DE".to_string()),
    ];

    for _ in 0..num_rows {
        let g1_idx = (rng.next_u64() % 6) as usize;
        g1_vals.push(if g1_idx < 5 {
            g1_options[g1_idx].clone()
        } else {
            None
        });

        let g2_rand = rng.next_u64() % 10;
        g2_vals.push(if g2_rand == 0 {
            None
        } else {
            Some((g2_rand * 10) as i64)
        });

        let v_rand = rng.next_u64() % 100;
        val_vals.push(if v_rand == 0 {
            None
        } else {
            Some(v_rand as i64)
        });
    }

    let mut store = Store::new();
    store.add("g1", Column::dict_from(g1_vals)).expect("add g1");
    store.add("g2", Column::ints(g2_vals)).expect("add g2");
    store.add("val", Column::ints(val_vals)).expect("add val");

    let schema = vec![
        ColumnInfo {
            name: "g1".into(),
            kind: ColumnKind::Text,
            nullable: true,
            hint: DisplayHint::Plain,
        },
        ColumnInfo {
            name: "g2".into(),
            kind: ColumnKind::Int,
            nullable: true,
            hint: DisplayHint::Plain,
        },
        ColumnInfo {
            name: "val".into(),
            kind: ColumnKind::Int,
            nullable: true,
            hint: DisplayHint::Plain,
        },
    ];

    let query = Query::new()
        .group("g1")
        .with_total(Total::count("val"))
        .with_total(Total::sum("val"))
        .group("g2")
        .with_total(Total::count("val"))
        .with_total(Total::sum("val"))
        .sort_asc("val");

    let view = View::new(&store, query.clone()).expect("view");
    let mut flat = GroupedFlat::new(view, false);
    flat.expand_all();

    let source = Arc::new(VecSource::new(store, schema).expect("source"));
    let mut cursor = Cursor::open(source, query).expect("cursor open");
    let mut it = cursor.lines();

    for i in 0..flat.len() {
        let flat_line = flat.line(i).expect("flat line");
        let cursor_line = it
            .next_line()
            .expect("next_line")
            .expect("cursor line present");

        match (flat_line.kind, cursor_line) {
            (LineKind::Header, CursorLine::Header { level, group }) => {
                assert_eq!(level, flat_line.depth, "header level mismatch at line {i}");
                assert_eq!(
                    group.row_count,
                    flat.group_row_count(i).expect("row count"),
                    "header row_count mismatch at line {i}"
                );
                let exp_totals = flat.group_totals(i).expect("totals");
                assert_eq!(group.totals.len(), exp_totals.len());
                for (a, b) in group.totals.iter().zip(exp_totals.iter()) {
                    match (a, b) {
                        (None, None) => {}
                        (Some(x), Some(y)) => {
                            assert!(
                                (x - y).abs() < 1e-9,
                                "total mismatch at line {i}: {x} vs {y}"
                            );
                        }
                        _ => panic!("total nullness mismatch at line {i}: {:?} vs {:?}", a, b),
                    }
                }
            }
            (LineKind::Row, CursorLine::Row(row_ref)) => {
                let exp_id = match flat_line.key {
                    LineKey::Row(id) => id,
                    _ => panic!("expected Row key at line {i}"),
                };
                assert_eq!(
                    row_ref.row_id(),
                    Some(exp_id),
                    "row id mismatch at line {i}"
                );
            }
            _ => panic!("kind mismatch at line {i}: {:?}", flat_line.kind),
        }
    }

    assert!(it.next_line().expect("next_line").is_none());
}

#[test]
fn test_lines_one_million_rows_bounded_memory() {
    const NUM_ROWS: usize = 1_000_000;
    const PAGE_SIZE: usize = 1_000;
    const MAX_PAGES: usize = 4;
    const DISTINCT_GROUPS: usize = 1_000;

    let mut store = Store::new();
    let grp_values: Vec<Option<i64>> = (0..NUM_ROWS as i64)
        .map(|r| Some(r / (NUM_ROWS / DISTINCT_GROUPS) as i64))
        .collect();
    let id_values: Vec<Option<i64>> = (0..NUM_ROWS as i64).map(Some).collect();

    store.add("grp", Column::ints(grp_values)).expect("add grp");
    store.add("id", Column::ints(id_values)).expect("add id");

    let schema = vec![
        ColumnInfo {
            name: "grp".into(),
            kind: ColumnKind::Int,
            nullable: false,
            hint: DisplayHint::Plain,
        },
        ColumnInfo {
            name: "id".into(),
            kind: ColumnKind::Int,
            nullable: false,
            hint: DisplayHint::Plain,
        },
    ];

    let source = Arc::new(VecSource::new(store, schema).expect("source"));
    let query = Query::new().group("grp");

    let mut cursor = Cursor::with_window(source, query, PAGE_SIZE, MAX_PAGES);
    let mut it = cursor.lines();

    let mut header_count: usize = 0;
    let mut row_count: usize = 0;

    while let Some(line) = it.next_line().expect("next_line") {
        match line {
            CursorLine::Header { .. } => {
                header_count += 1;
            }
            CursorLine::Row(_) => {
                row_count += 1;
                if row_count.is_multiple_of(100_000) {
                    assert!(
                        it.cached_pages() <= MAX_PAGES,
                        "cached_pages ({}) must be <= MAX_PAGES ({MAX_PAGES}) at row {row_count}",
                        it.cached_pages()
                    );
                }
            }
        }
    }

    assert_eq!(row_count, NUM_ROWS);
    assert_eq!(header_count, DISTINCT_GROUPS);
    assert_eq!(it.fetch_calls(), 1000);
    assert!(it.cached_pages() <= MAX_PAGES);
}
