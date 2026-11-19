//! Tests for the headless grid screen model.

use super::*;
use datagrid::filter::Owned;
use std::time::{Duration, Instant};

/// Compares floating-point values with a tiny tolerance for arithmetic imprecision.
fn close(a: f64, b: f64) -> bool {
    (a - b).abs() < 1e-9
}

#[test]
fn header_arrows_and_click_cycling_and_switching() {
    let cols = vec![
        GridColumn::new("id", "Id", ColumnAlign::Right),
        GridColumn::new("customer", "Customer", ColumnAlign::Left),
        GridColumn::new("total", "Total", ColumnAlign::Right),
    ];
    let mut state = GridState::new(cols);
    let screen = datagrid::Screen::default();
    let t0 = Instant::now();

    // Initial state: not sorted
    let view0 = state.view(&screen, t0);
    assert_eq!(view0.header[0].arrow, "");
    assert_eq!(view0.header[1].arrow, "");
    assert_eq!(view0.header[2].arrow, "");

    // Click Total (column 2): not sorted -> Asc
    let q1 = state.click_header(2);
    assert_eq!(q1.sort.len(), 1);
    assert_eq!(q1.sort[0].column, "total");
    assert_eq!(q1.sort[0].order, datagrid::Order::Asc);
    let view1 = state.view(&screen, t0);
    assert_eq!(view1.header[0].arrow, "");
    assert_eq!(view1.header[1].arrow, "");
    assert_eq!(view1.header[2].arrow, " ▲");

    // Click Total again: Asc -> Desc
    let q2 = state.click_header(2);
    assert_eq!(q2.sort.len(), 1);
    assert_eq!(q2.sort[0].column, "total");
    assert_eq!(q2.sort[0].order, datagrid::Order::Desc);
    let view2 = state.view(&screen, t0);
    assert_eq!(view2.header[2].arrow, " ▼");

    // Click Total again: Desc -> Asc
    let q3 = state.click_header(2);
    assert_eq!(q3.sort.len(), 1);
    assert_eq!(q3.sort[0].column, "total");
    assert_eq!(q3.sort[0].order, datagrid::Order::Asc);
    let view3 = state.view(&screen, t0);
    assert_eq!(view3.header[2].arrow, " ▲");

    // Click Customer (column 1): switches column, not sorted -> Asc
    let q4 = state.click_header(1);
    assert_eq!(q4.sort.len(), 1);
    assert_eq!(q4.sort[0].column, "customer");
    assert_eq!(q4.sort[0].order, datagrid::Order::Asc);
    let view4 = state.view(&screen, t0);
    assert_eq!(view4.header[0].arrow, "");
    assert_eq!(view4.header[1].arrow, " ▲");
    assert_eq!(view4.header[2].arrow, "");
}

#[test]
fn group_by_deduplication_ungroup_and_chips() {
    let cols = vec![
        GridColumn::new("city", "City", ColumnAlign::Left),
        GridColumn::new("customer", "Customer", ColumnAlign::Left),
        GridColumn::new("total", "Total", ColumnAlign::Right),
    ];
    let mut state = GridState::new(cols);
    let screen = datagrid::Screen::default();
    let t0 = Instant::now();

    assert!(state.view(&screen, t0).chips.is_empty());

    // Group by City
    let q1 = state.group_by_column(0);
    assert_eq!(q1.group_by.len(), 1);
    assert_eq!(q1.group_by[0].column, "city");
    assert_eq!(state.view(&screen, t0).chips, vec!["City"]);

    // Group by City again (deduplication)
    let q2 = state.group_by_column(0);
    assert_eq!(q2.group_by.len(), 1);
    assert_eq!(state.view(&screen, t0).chips, vec!["City"]);

    // Group by Customer (level 1)
    let q3 = state.group_by_column(1);
    assert_eq!(q3.group_by.len(), 2);
    assert_eq!(q3.group_by[0].column, "city");
    assert_eq!(q3.group_by[1].column, "customer");
    assert_eq!(state.view(&screen, t0).chips, vec!["City", "Customer"]);

    // Ungroup level 0 (City)
    let q4 = state.ungroup(0);
    assert_eq!(q4.group_by.len(), 1);
    assert_eq!(q4.group_by[0].column, "customer");
    assert_eq!(state.view(&screen, t0).chips, vec!["Customer"]);

    // Ungroup remaining level 0 (Customer)
    let q5 = state.ungroup(0);
    assert!(q5.group_by.is_empty());
    assert!(state.view(&screen, t0).chips.is_empty());
}

#[test]
fn grouped_screen_headers() {
    let cols = vec![
        GridColumn::new("city", "City", ColumnAlign::Left),
        GridColumn::new("total", "Total", ColumnAlign::Right),
        GridColumn::new("price", "Price", ColumnAlign::Right),
    ];
    let mut state = GridState::new(cols);
    state.group_by = vec![datagrid::GroupLevel {
        column: "city".into(),
        totals: vec![
            datagrid::Total::sum("total"),
            datagrid::Total::count("total"),
            datagrid::Total::min("price"),
            datagrid::Total::max("price"),
            datagrid::Total::avg("price"),
        ],
    }];

    let screen = datagrid::Screen {
        first: 0,
        lines: vec![
            datagrid::ScreenLine::Header {
                level: 0,
                column: "city".into(),
                key: "Curitiba".into(),
                row_count: 12408,
                totals: vec![
                    Some(1903220.40),
                    Some(12408.0),
                    Some(10.50),
                    Some(9999.90),
                    Some(153.39),
                ],
            },
            datagrid::ScreenLine::Header {
                level: 0,
                column: "city".into(),
                key: "Ponta Grossa".into(),
                row_count: 1,
                totals: vec![None, None, None, None, None],
            },
        ],
        total: Some(12409),
        pending: None,
        busy: None,
        error: None,
        summary: Vec::new(),
        distinct: None,
        written: None,
    };

    let view = state.view(&screen, Instant::now());
    assert_eq!(view.lines.len(), 2);

    match &view.lines[0] {
        ViewLine::Header {
            level,
            title,
            count,
            totals,
        } => {
            assert_eq!(*level, 0);
            assert_eq!(title, "▼ City: Curitiba");
            assert_eq!(count, "12,408 rows");
            assert_eq!(
                totals,
                "Σ Total 1,903,220.40 · Total count 12,408.00 · min Price 10.50 · max Price 9,999.90 · avg Price 153.39"
            );
        }
        _ => panic!("expected header line"),
    }

    match &view.lines[1] {
        ViewLine::Header {
            level,
            title,
            count,
            totals,
        } => {
            assert_eq!(*level, 0);
            assert_eq!(title, "▼ City: Ponta Grossa");
            assert_eq!(count, "1 row");
            assert_eq!(
                totals,
                "Σ Total — · Total count — · min Price — · max Price — · avg Price —"
            );
        }
        _ => panic!("expected header line"),
    }
}

#[test]
fn footer_variations() {
    let cols = vec![
        GridColumn::new("id", "Id", ColumnAlign::Right),
        GridColumn::new("city", "City", ColumnAlign::Left),
        GridColumn::new("total", "Total", ColumnAlign::Right),
    ];
    let mut state = GridState::new(cols);
    state.rec_no = 482_112; // 0-based -> row 482,113
    let t0 = Instant::now();

    // 1. Position with total
    let screen_with_total = datagrid::Screen {
        total: Some(1_000_000),
        ..Default::default()
    };
    let view = state.view(&screen_with_total, t0);
    assert_eq!(view.footer.position, "Row 482,113 of 1,000,000");

    // 2. Position without total
    let screen_no_total = datagrid::Screen {
        total: None,
        ..Default::default()
    };
    let view = state.view(&screen_no_total, t0);
    assert_eq!(view.footer.position, "Row 482,113");

    // 3. State: fetching only while pending
    let screen_pending = datagrid::Screen {
        pending: Some((0, 50)),
        ..Default::default()
    };
    assert_eq!(state.view(&screen_pending, t0).footer.state, "fetching…");
    let screen_idle = datagrid::Screen {
        pending: None,
        ..Default::default()
    };
    assert_eq!(state.view(&screen_idle, t0).footer.state, "");

    // 4. Order text in all 4 combinations
    // 4a. Neither sorted nor grouped
    assert_eq!(state.view(&screen_idle, t0).footer.order, "");

    // 4b. Sorted only
    state.sort = vec![datagrid::SortKey {
        column: "total".into(),
        order: datagrid::Order::Desc,
        nulls: datagrid::Nulls::Last,
    }];
    assert_eq!(
        state.view(&screen_idle, t0).footer.order,
        "sorted by Total ▼"
    );

    // 4c. Both sorted and grouped
    state.group_by = vec![datagrid::GroupLevel::new("city")];
    assert_eq!(
        state.view(&screen_idle, t0).footer.order,
        "sorted by Total ▼ · grouped by City"
    );

    // 4d. Grouped only
    state.sort.clear();
    assert_eq!(state.view(&screen_idle, t0).footer.order, "grouped by City");

    // 5. Timing from last ended wait
    let screen_ended = datagrid::Screen {
        busy: Some(datagrid::Busy {
            started: t0,
            ended: Some(t0 + Duration::from_micros(3100)),
        }),
        ..Default::default()
    };
    assert_eq!(
        state.view(&screen_ended, t0).footer.timing,
        "last page 3.1 ms"
    );

    let screen_running = datagrid::Screen {
        busy: Some(datagrid::Busy {
            started: t0,
            ended: None,
        }),
        ..Default::default()
    };
    assert_eq!(state.view(&screen_running, t0).footer.timing, "");

    let screen_no_busy = datagrid::Screen::default();
    assert_eq!(state.view(&screen_no_busy, t0).footer.timing, "");
}

#[test]
fn overlay_timeline() {
    let cols = vec![GridColumn::new("id", "Id", ColumnAlign::Left)];
    let state = GridState::new(cols);
    let t0 = Instant::now();

    // Default timing: show_after = 150ms, min_visible = 400ms, fade_out = 150ms.
    let make_screen = |started, ended| datagrid::Screen {
        pending: Some((731_199, 201)),
        busy: Some(datagrid::Busy { started, ended }),
        ..Default::default()
    };

    // 1. At 100 ms running: none, rows_alpha == 1.0, animating
    let s_running = make_screen(t0, None);
    let v_100 = state.view(&s_running, t0 + Duration::from_millis(100));
    assert!(v_100.overlay.is_none());
    assert_eq!(v_100.rows_alpha, 1.0);
    assert!(v_100.animating);

    // 2. At 200 ms running: opacity 1, rows_alpha == 0.45, title and detail texts
    let v_200 = state.view(&s_running, t0 + Duration::from_millis(200));
    let ov_200 = v_200.overlay.expect("overlay visible at 200ms");
    assert!(close(ov_200.opacity, 1.0));
    assert!(close(v_200.rows_alpha, 0.45));
    assert_eq!(ov_200.title, "Loading rows 731,200 – 731,400");
    assert_eq!(ov_200.detail, "0.20 s · you can keep scrolling");
    assert!(v_200.animating);

    // 3. Ended at 260 ms, viewed at 500 ms: still 1 (min visible)
    let s_ended_260 = make_screen(t0, Some(t0 + Duration::from_millis(260)));
    let v_500 = state.view(&s_ended_260, t0 + Duration::from_millis(500));
    let ov_500 = v_500.overlay.expect("overlay visible at 500ms");
    assert!(close(ov_500.opacity, 1.0));
    assert_eq!(ov_500.detail, "0.26 s · you can keep scrolling");
    assert!(close(v_500.rows_alpha, 0.45));
    assert!(v_500.animating);

    // 4. Viewed at 625 ms: between 0 and 1
    // hold_until = max(260, 150 + 400) = 550ms. fade ends at 550 + 150 = 700ms.
    // At 625ms: elapsed in fade = 75ms. opacity = 1.0 - (75 / 150) = 0.5.
    let v_625 = state.view(&s_ended_260, t0 + Duration::from_millis(625));
    let ov_625 = v_625.overlay.expect("overlay in fade at 625ms");
    assert!(ov_625.opacity > 0.0 && ov_625.opacity < 1.0);
    assert!(close(ov_625.opacity, 0.5));
    assert!(v_625.rows_alpha > 0.45 && v_625.rows_alpha < 1.0);
    assert!(close(v_625.rows_alpha, 1.0 - (1.0 - 0.45) * 0.5));
    assert!(v_625.animating);

    // 5. Viewed at 700 ms: none and animating == false
    let v_700 = state.view(&s_ended_260, t0 + Duration::from_millis(700));
    assert!(v_700.overlay.is_none());
    assert_eq!(v_700.rows_alpha, 1.0);
    assert!(!v_700.animating);

    // 6. Ended at 40 ms: never an overlay
    let s_fast = make_screen(t0, Some(t0 + Duration::from_millis(40)));
    for ms in [40, 100, 200, 500, 700] {
        let v = state.view(&s_fast, t0 + Duration::from_millis(ms));
        assert!(v.overlay.is_none());
        assert_eq!(v.rows_alpha, 1.0);
        assert!(!v.animating);
    }
}

#[test]
fn selected_and_odd_flags() {
    let cols = vec![GridColumn::new("id", "Id", ColumnAlign::Left)];
    let mut state = GridState::new(cols);
    state.rec_no = 2;

    let screen = datagrid::Screen {
        lines: vec![
            datagrid::ScreenLine::Row {
                index: 0,
                id: 0,
                cells: vec![Some("row0".into())],
            },
            datagrid::ScreenLine::Row {
                index: 1,
                id: 1,
                cells: vec![Some("row1".into())],
            },
            datagrid::ScreenLine::Row {
                index: 2,
                id: 2,
                cells: vec![Some("row2".into())],
            },
            datagrid::ScreenLine::Row {
                index: 3,
                id: 3,
                cells: vec![Some("row3".into())],
            },
        ],
        ..Default::default()
    };

    let view = state.view(&screen, Instant::now());
    assert_eq!(view.lines.len(), 4);

    match &view.lines[0] {
        ViewLine::Row {
            index,
            odd,
            selected,
            ..
        } => {
            assert_eq!(*index, 0);
            assert!(!odd);
            assert!(!selected);
        }
        _ => panic!("expected row"),
    }
    match &view.lines[1] {
        ViewLine::Row {
            index,
            odd,
            selected,
            ..
        } => {
            assert_eq!(*index, 1);
            assert!(odd);
            assert!(!selected);
        }
        _ => panic!("expected row"),
    }
    match &view.lines[2] {
        ViewLine::Row {
            index,
            odd,
            selected,
            ..
        } => {
            assert_eq!(*index, 2);
            assert!(!odd);
            assert!(selected);
        }
        _ => panic!("expected row"),
    }
    match &view.lines[3] {
        ViewLine::Row {
            index,
            odd,
            selected,
            ..
        } => {
            assert_eq!(*index, 3);
            assert!(odd);
            assert!(!selected);
        }
        _ => panic!("expected row"),
    }
}

#[test]
fn thousands_separator() {
    assert_eq!(format_thousands_int(0), "0");
    assert_eq!(format_thousands_int(999), "999");
    assert_eq!(format_thousands_int(1_000), "1,000");
    assert_eq!(format_thousands_int(1_000_000), "1,000,000");

    assert_eq!(format_float_thousands(-1_903_220.40), "-1,903,220.40");
    assert_eq!(format_float_thousands(-1_000.0), "-1,000.00");
    assert_eq!(format_float_thousands(-999.5), "-999.50");
    assert_eq!(format_float_thousands(0.0), "0.00");
    assert_eq!(format_float_thousands(999.0), "999.00");
    assert_eq!(format_float_thousands(1_000.0), "1,000.00");
    assert_eq!(format_float_thousands(1_000_000.0), "1,000,000.00");
}

#[test]
fn line_for_item_row_present() {
    let screen = datagrid::Screen {
        first: 10,
        lines: vec![
            datagrid::ScreenLine::Row {
                index: 10,
                id: 10,
                cells: vec![Some("a".into())],
            },
            datagrid::ScreenLine::Row {
                index: 11,
                id: 11,
                cells: vec![Some("b".into())],
            },
        ],
        ..Default::default()
    };
    let state = GridState::new(vec![GridColumn::new("col", "Col", ColumnAlign::Left)]);
    let view = state.view(&screen, Instant::now());

    let res = line_for_item(&view, 10, 11);
    assert!(!res.is_stale);
    assert!(res.headers.is_empty());
    match res.row {
        Some(ViewLine::Row { index, cells, .. }) => {
            assert_eq!(*index, 11);
            assert_eq!(cells, &["b"]);
        }
        _ => panic!("expected row 11"),
    }
}

#[test]
fn line_for_item_row_absent_offset_from_old_screen() {
    // Old screen has rows 100..102 (first = 100).
    // Requested first = 500.
    // Item = 501 -> offset = 1 -> target in old screen = 100 + 1 = 101.
    let screen = datagrid::Screen {
        first: 100,
        lines: vec![
            datagrid::ScreenLine::Row {
                index: 100,
                id: 100,
                cells: vec![Some("r100".into())],
            },
            datagrid::ScreenLine::Row {
                index: 101,
                id: 101,
                cells: vec![Some("r101".into())],
            },
        ],
        ..Default::default()
    };
    let state = GridState::new(vec![GridColumn::new("col", "Col", ColumnAlign::Left)]);
    let view = state.view(&screen, Instant::now());

    let res = line_for_item(&view, 500, 501);
    assert!(res.is_stale);
    assert!(res.headers.is_empty());
    match res.row {
        Some(ViewLine::Row { index, cells, .. }) => {
            assert_eq!(*index, 101);
            assert_eq!(cells, &["r101"]);
        }
        _ => panic!("expected row 101 from old screen"),
    }

    // The first requested item stands on the old screen's first row.
    let res_first = line_for_item(&view, 500, 500);
    assert!(res_first.is_stale);
    match res_first.row {
        Some(ViewLine::Row { index, .. }) => assert_eq!(*index, 100),
        _ => panic!("expected row 100 from old screen"),
    }
}

#[test]
fn line_for_item_absent_with_no_old_row() {
    // Old screen has rows 100..101.
    // Requested first = 500.
    // Item = 510 -> offset = 10 -> target in old screen = 110 (does not exist in old screen).
    let screen = datagrid::Screen {
        first: 100,
        lines: vec![
            datagrid::ScreenLine::Row {
                index: 100,
                id: 100,
                cells: vec![Some("r100".into())],
            },
            datagrid::ScreenLine::Row {
                index: 101,
                id: 101,
                cells: vec![Some("r101".into())],
            },
        ],
        ..Default::default()
    };
    let state = GridState::new(vec![GridColumn::new("col", "Col", ColumnAlign::Left)]);
    let view = state.view(&screen, Instant::now());

    let res = line_for_item(&view, 500, 510);
    assert!(res.is_stale);
    assert!(res.headers.is_empty());
    assert!(res.row.is_none());

    // Also when item < requested_first:
    let res2 = line_for_item(&view, 500, 499);
    assert!(res2.is_stale);
    assert!(res2.headers.is_empty());
    assert!(res2.row.is_none());

    // Also with an empty screen:
    let empty_view = state.view(&datagrid::Screen::default(), Instant::now());
    let res3 = line_for_item(&empty_view, 0, 0);
    assert!(res3.is_stale);
    assert!(res3.headers.is_empty());
    assert!(res3.row.is_none());
}

#[test]
fn line_for_item_headers_attached_to_right_item() {
    let screen = datagrid::Screen {
        first: 0,
        lines: vec![
            datagrid::ScreenLine::Header {
                level: 0,
                column: "city".into(),
                key: "Curitiba".into(),
                row_count: 2,
                totals: vec![],
            },
            datagrid::ScreenLine::Header {
                level: 1,
                column: "customer".into(),
                key: "Acme".into(),
                row_count: 1,
                totals: vec![],
            },
            datagrid::ScreenLine::Row {
                index: 0,
                id: 0,
                cells: vec![Some("row0".into()), Some("cust0".into())],
            },
            datagrid::ScreenLine::Row {
                index: 1,
                id: 1,
                cells: vec![Some("row1".into()), Some("cust1".into())],
            },
            datagrid::ScreenLine::Header {
                level: 1,
                column: "customer".into(),
                key: "Beta".into(),
                row_count: 1,
                totals: vec![],
            },
            datagrid::ScreenLine::Row {
                index: 2,
                id: 2,
                cells: vec![Some("row2".into()), Some("cust2".into())],
            },
        ],
        ..Default::default()
    };
    let mut state = GridState::new(vec![
        GridColumn {
            name: "city".into(),
            title: "City".into(),
            align: ColumnAlign::Left,
            source: Some(0),
            width: None,
            display_format: String::new(),
            title_align: ColumnAlign::Left,
            min_width: 0.0,
            max_width: None,
            null_text: String::new(),
            boolean_texts: String::new(),
            value_map: String::new(),
            renderer: CellRenderer::Text,
            color: Color::transparent(),
            text_color: Color::transparent(),
            title_color: Color::transparent(),
            valign: ColumnVAlign::Center,
            padding: None,
            kind: None,
            ellipsis: false,
            word_wrap: false,
            title_word_wrap: false,
            title_hint: String::new(),
            cell_hint: false,
            title_font: crawrad_ir::Font::default(),
            font: crawrad_ir::Font::default(),
            auto_size: false,
            resizable: true,
            movable: true,
            sortable: true,
            sort_order: "soNone".into(),
            sort_priority: 0,
            nulls_position: "npLast".into(),
            groupable: true,
            group_index: 0,
            summary: "smNone".into(),
            summary_format: String::new(),
            group_summary: "smNone".into(),
            filterable: false,
            filter_kind: "fkAuto".into(),
            read_only: false,
            required: false,
            default_value: String::new(),
            max_length: 0,
            min_value: 0,
            max_value: 0,
            tab_stop: true,
            editor: "edAuto".into(),
            pick_list: Vec::new(),
            edit_mask: String::new(),
            get_text_hook: false,
            set_text_hook: false,
            validate_hook: false,
            lookup_key: String::new(),
            lookup_display: Vec::new(),
            button_hook: false,
            drop_down_count: 8,
            style_rules: Vec::new(),
            draw_cell_hook: false,
            lookup: LookupHandle::default(),
        },
        GridColumn {
            name: "customer".into(),
            title: "Customer".into(),
            align: ColumnAlign::Left,
            source: Some(1),
            width: None,
            display_format: String::new(),
            title_align: ColumnAlign::Left,
            min_width: 0.0,
            max_width: None,
            null_text: String::new(),
            boolean_texts: String::new(),
            value_map: String::new(),
            renderer: CellRenderer::Text,
            color: Color::transparent(),
            text_color: Color::transparent(),
            title_color: Color::transparent(),
            valign: ColumnVAlign::Center,
            padding: None,
            kind: None,
            ellipsis: false,
            word_wrap: false,
            title_word_wrap: false,
            title_hint: String::new(),
            cell_hint: false,
            title_font: crawrad_ir::Font::default(),
            font: crawrad_ir::Font::default(),
            auto_size: false,
            resizable: true,
            movable: true,
            sortable: true,
            sort_order: "soNone".into(),
            sort_priority: 0,
            nulls_position: "npLast".into(),
            groupable: true,
            group_index: 0,
            summary: "smNone".into(),
            summary_format: String::new(),
            group_summary: "smNone".into(),
            filterable: false,
            filter_kind: "fkAuto".into(),
            read_only: false,
            required: false,
            default_value: String::new(),
            max_length: 0,
            min_value: 0,
            max_value: 0,
            tab_stop: true,
            editor: "edAuto".into(),
            pick_list: Vec::new(),
            edit_mask: String::new(),
            get_text_hook: false,
            set_text_hook: false,
            validate_hook: false,
            lookup_key: String::new(),
            lookup_display: Vec::new(),
            button_hook: false,
            drop_down_count: 8,
            style_rules: Vec::new(),
            draw_cell_hook: false,
            lookup: LookupHandle::default(),
        },
    ]);
    state.group_by = vec![
        datagrid::GroupLevel::new("city"),
        datagrid::GroupLevel::new("customer"),
    ];
    let view = state.view(&screen, Instant::now());

    // Item 0: has 2 headers (Curitiba and Acme)
    let res0 = line_for_item(&view, 0, 0);
    assert!(!res0.is_stale);
    assert_eq!(res0.headers.len(), 2);
    match res0.headers[0] {
        ViewLine::Header { title, level, .. } => {
            assert_eq!(*level, 0);
            assert_eq!(title, "▼ City: Curitiba");
        }
        _ => panic!("expected header 0"),
    }
    match res0.headers[1] {
        ViewLine::Header { title, level, .. } => {
            assert_eq!(*level, 1);
            assert_eq!(title, "▼ Customer: Acme");
        }
        _ => panic!("expected header 1"),
    }
    assert_eq!(
        res0.row,
        Some(&ViewLine::Row {
            index: 0,
            id: 0,
            cells: vec!["row0".into(), "cust0".into()],
            raw_cells: vec![Some("row0".into()), Some("cust0".into())],
            odd: false,
            selected: true,
            marker: "",
        })
    );

    // Item 1: has 0 headers
    let res1 = line_for_item(&view, 0, 1);
    assert!(!res1.is_stale);
    assert!(res1.headers.is_empty());
    assert_eq!(
        res1.row,
        Some(&ViewLine::Row {
            index: 1,
            id: 1,
            cells: vec!["row1".into(), "cust1".into()],
            raw_cells: vec![Some("row1".into()), Some("cust1".into())],
            odd: true,
            selected: false,
            marker: "",
        })
    );

    // Item 2: has 1 header (Beta)
    let res2 = line_for_item(&view, 0, 2);
    assert!(!res2.is_stale);
    assert_eq!(res2.headers.len(), 1);
    match res2.headers[0] {
        ViewLine::Header { title, level, .. } => {
            assert_eq!(*level, 1);
            assert_eq!(title, "▼ Customer: Beta");
        }
        _ => panic!("expected header Beta"),
    }
    assert_eq!(
        res2.row,
        Some(&ViewLine::Row {
            index: 2,
            id: 2,
            cells: vec!["row2".into(), "cust2".into()],
            raw_cells: vec![Some("row2".into()), Some("cust2".into())],
            odd: false,
            selected: false,
            marker: "",
        })
    );
}

#[test]
fn the_runtime_marks_reach_header_footer_and_group_lines() {
    let mut state = GridState::new(vec![
        GridColumn::new("city", "City", ColumnAlign::Left),
        GridColumn::new("total", "Total", ColumnAlign::Right),
    ]);
    state.marks = Marks {
        ascending: " up",
        descending: " down",
        group_open: "open ",
        ..Marks::default()
    };
    state.click_header(1);
    state.group_by = vec![datagrid::GroupLevel::new("city")];
    let screen = datagrid::Screen {
        lines: vec![datagrid::ScreenLine::Header {
            level: 0,
            column: "city".into(),
            key: "Curitiba".into(),
            row_count: 2,
            totals: vec![],
        }],
        ..Default::default()
    };
    let view = state.view(&screen, Instant::now());
    assert_eq!(view.header[1].arrow, " up");
    assert_eq!(view.footer.order, "sorted by Total up · grouped by City");
    match &view.lines[0] {
        ViewLine::Header { title, .. } => assert_eq!(title, "open City: Curitiba"),
        other => panic!("expected a group line, got {other:?}"),
    }

    state.click_header(1);
    assert_eq!(state.view(&screen, Instant::now()).header[1].arrow, " down");
}

#[test]
fn columns_from_a_schema_take_their_title_and_alignment_from_the_field() {
    let info = |name: &str, kind| datagrid::ColumnInfo {
        name: name.into(),
        kind,
        nullable: true,
        hint: datagrid::DisplayHint::Plain,
    };
    let cols = GridColumn::from_schema(&[
        info("id", datagrid::ColumnKind::Int),
        info("customer_name", datagrid::ColumnKind::Text),
        info("total", datagrid::ColumnKind::Float),
        info("paid", datagrid::ColumnKind::Bool),
    ]);
    assert_eq!(
        cols,
        vec![
            GridColumn {
                name: "id".into(),
                title: "Id".into(),
                align: ColumnAlign::Right,
                source: Some(0),
                width: None,
                display_format: String::new(),
                title_align: ColumnAlign::Right,
                min_width: 0.0,
                max_width: None,
                null_text: String::new(),
                boolean_texts: String::new(),
                color: Color::transparent(),
                text_color: Color::transparent(),
                title_color: Color::transparent(),
                valign: ColumnVAlign::Center,
                padding: None,
                kind: Some("INTEGER".into()),
                ellipsis: false,
                word_wrap: false,
                title_word_wrap: false,
                title_hint: String::new(),
                cell_hint: false,
                title_font: Font {
                    name: String::new(),
                    size: 0.0,
                    ..Default::default()
                },
                font: Font {
                    name: String::new(),
                    size: 0.0,
                    ..Default::default()
                },
                auto_size: false,
                resizable: true,
                movable: true,
                sortable: true,
                sort_order: "soNone".into(),
                sort_priority: 0,
                nulls_position: "npLast".into(),
                groupable: true,
                group_index: 0,
                value_map: String::new(),
                renderer: CellRenderer::Text,
                summary: "smNone".into(),
                summary_format: String::new(),
                group_summary: "smNone".into(),
                filterable: false,
                filter_kind: "fkAuto".into(),
                read_only: false,
                required: false,
                default_value: String::new(),
                max_length: 0,
                min_value: 0,
                max_value: 0,
                tab_stop: true,
                editor: "edAuto".into(),
                pick_list: Vec::new(),
                edit_mask: String::new(),
                get_text_hook: false,
                set_text_hook: false,
                validate_hook: false,
                lookup_key: String::new(),
                lookup_display: Vec::new(),
                button_hook: false,
                drop_down_count: 8,
                style_rules: Vec::new(),
                draw_cell_hook: false,
                lookup: LookupHandle::default(),
            },
            GridColumn {
                name: "customer_name".into(),
                title: "Customer name".into(),
                align: ColumnAlign::Left,
                source: Some(1),
                width: None,
                display_format: String::new(),
                title_align: ColumnAlign::Left,
                min_width: 0.0,
                max_width: None,
                null_text: String::new(),
                boolean_texts: String::new(),
                color: Color::transparent(),
                text_color: Color::transparent(),
                title_color: Color::transparent(),
                valign: ColumnVAlign::Center,
                padding: None,
                kind: Some("TEXT".into()),
                ellipsis: false,
                word_wrap: false,
                title_word_wrap: false,
                title_hint: String::new(),
                cell_hint: false,
                title_font: Font {
                    name: String::new(),
                    size: 0.0,
                    ..Default::default()
                },
                font: Font {
                    name: String::new(),
                    size: 0.0,
                    ..Default::default()
                },
                auto_size: false,
                resizable: true,
                movable: true,
                sortable: true,
                sort_order: "soNone".into(),
                sort_priority: 0,
                nulls_position: "npLast".into(),
                groupable: true,
                group_index: 0,
                value_map: String::new(),
                renderer: CellRenderer::Text,
                summary: "smNone".into(),
                summary_format: String::new(),
                group_summary: "smNone".into(),
                filterable: false,
                filter_kind: "fkAuto".into(),
                read_only: false,
                required: false,
                default_value: String::new(),
                max_length: 0,
                min_value: 0,
                max_value: 0,
                tab_stop: true,
                editor: "edAuto".into(),
                pick_list: Vec::new(),
                edit_mask: String::new(),
                get_text_hook: false,
                set_text_hook: false,
                validate_hook: false,
                lookup_key: String::new(),
                lookup_display: Vec::new(),
                button_hook: false,
                drop_down_count: 8,
                style_rules: Vec::new(),
                draw_cell_hook: false,
                lookup: LookupHandle::default(),
            },
            GridColumn {
                name: "total".into(),
                title: "Total".into(),
                align: ColumnAlign::Right,
                source: Some(2),
                width: None,
                display_format: String::new(),
                title_align: ColumnAlign::Right,
                min_width: 0.0,
                max_width: None,
                null_text: String::new(),
                boolean_texts: String::new(),
                color: Color::transparent(),
                text_color: Color::transparent(),
                title_color: Color::transparent(),
                valign: ColumnVAlign::Center,
                padding: None,
                kind: Some("FLOAT".into()),
                ellipsis: false,
                word_wrap: false,
                title_word_wrap: false,
                title_hint: String::new(),
                cell_hint: false,
                title_font: Font {
                    name: String::new(),
                    size: 0.0,
                    ..Default::default()
                },
                font: Font {
                    name: String::new(),
                    size: 0.0,
                    ..Default::default()
                },
                auto_size: false,
                resizable: true,
                movable: true,
                sortable: true,
                sort_order: "soNone".into(),
                sort_priority: 0,
                nulls_position: "npLast".into(),
                groupable: true,
                group_index: 0,
                value_map: String::new(),
                renderer: CellRenderer::Text,
                summary: "smNone".into(),
                summary_format: String::new(),
                group_summary: "smNone".into(),
                filterable: false,
                filter_kind: "fkAuto".into(),
                read_only: false,
                required: false,
                default_value: String::new(),
                max_length: 0,
                min_value: 0,
                max_value: 0,
                tab_stop: true,
                editor: "edAuto".into(),
                pick_list: Vec::new(),
                edit_mask: String::new(),
                get_text_hook: false,
                set_text_hook: false,
                validate_hook: false,
                lookup_key: String::new(),
                lookup_display: Vec::new(),
                button_hook: false,
                drop_down_count: 8,
                style_rules: Vec::new(),
                draw_cell_hook: false,
                lookup: LookupHandle::default(),
            },
            GridColumn {
                name: "paid".into(),
                title: "Paid".into(),
                align: ColumnAlign::Left,
                source: Some(3),
                width: None,
                display_format: String::new(),
                title_align: ColumnAlign::Left,
                min_width: 0.0,
                max_width: None,
                null_text: String::new(),
                boolean_texts: String::new(),
                color: Color::transparent(),
                text_color: Color::transparent(),
                title_color: Color::transparent(),
                valign: ColumnVAlign::Center,
                padding: None,
                kind: Some("BOOLEAN".into()),
                ellipsis: false,
                word_wrap: false,
                title_word_wrap: false,
                title_hint: String::new(),
                cell_hint: false,
                title_font: Font {
                    name: String::new(),
                    size: 0.0,
                    ..Default::default()
                },
                font: Font {
                    name: String::new(),
                    size: 0.0,
                    ..Default::default()
                },
                auto_size: false,
                resizable: true,
                movable: true,
                sortable: true,
                sort_order: "soNone".into(),
                sort_priority: 0,
                nulls_position: "npLast".into(),
                groupable: true,
                group_index: 0,
                value_map: String::new(),
                renderer: CellRenderer::Text,
                summary: "smNone".into(),
                summary_format: String::new(),
                group_summary: "smNone".into(),
                filterable: false,
                filter_kind: "fkAuto".into(),
                read_only: false,
                required: false,
                default_value: String::new(),
                max_length: 0,
                min_value: 0,
                max_value: 0,
                tab_stop: true,
                editor: "edAuto".into(),
                pick_list: Vec::new(),
                edit_mask: String::new(),
                get_text_hook: false,
                set_text_hook: false,
                validate_hook: false,
                lookup_key: String::new(),
                lookup_display: Vec::new(),
                button_hook: false,
                drop_down_count: 8,
                style_rules: Vec::new(),
                draw_cell_hook: false,
                lookup: LookupHandle::default(),
            },
        ]
    );
    assert_eq!(title_from_field(""), "");
    assert_eq!(title_from_field("_x"), " x");
    assert_eq!(title_from_field("ção"), "Ção");
}

#[test]
fn an_unavailable_source_answers_every_query_with_its_reason() {
    use datagrid::RowSource;
    let source = UnavailableSource::new("table not found: orders");
    assert!(source.schema().is_empty());
    let query = datagrid::Query::default();
    match source.count(&query) {
        Err(datagrid::SourceError::Backend(m)) => assert_eq!(m, "table not found: orders"),
        other => panic!("expected the reason, got {other:?}"),
    }
    match source.fetch(&query, 0..10) {
        Err(datagrid::SourceError::Backend(m)) => assert_eq!(m, "table not found: orders"),
        Err(other) => panic!("expected the reason, got {other:?}"),
        Ok(_) => panic!("an unavailable source has no rows"),
    }
}

#[test]
fn format_display_handles_various_masks_and_inputs() {
    // "#,##0.00" on "1250" -> "1,250.00"
    assert_eq!(format_display("1250", "#,##0.00"), "1,250.00");
    // "#,##0.00" on "-3.5" -> "-3.50"
    assert_eq!(format_display("-3.5", "#,##0.00"), "-3.50");
    // "0" mask on "7.6" -> "8"
    assert_eq!(format_display("7.6", "0"), "8");
    // "000" on "5" -> "005"
    assert_eq!(format_display("5", "000"), "005");
    // Prefix and suffix
    assert_eq!(format_display("12.5", "R$ #,##0.00"), "R$ 12.50");
    assert_eq!(format_display("5.67", "0.0 %"), "5.7 %");
    // Non-numeric text unchanged
    assert_eq!(format_display("hello world", "#,##0.00"), "hello world");
    // Empty mask leaves text unchanged
    assert_eq!(format_display("1250", ""), "1250");
}

#[test]
fn from_design_with_empty_specs_defaults_to_schema_fields() {
    let names = vec!["id".into(), "name".into(), "amount".into()];
    let numeric = vec![true, false, true];
    let cols = GridColumn::from_design(&names, &numeric, &[]);
    assert_eq!(cols.len(), 3);
    assert_eq!(cols[0].name, "id");
    assert_eq!(cols[0].title, "Id");
    assert_eq!(cols[0].align, ColumnAlign::Right);
    assert_eq!(cols[0].source, Some(0));
    assert_eq!(cols[0].width, None);
    assert_eq!(cols[0].display_format, "");
    assert!(cols[0].sortable);
    assert_eq!(cols[0].sort_order, "soNone");
    assert_eq!(cols[0].sort_priority, 0);
    assert_eq!(cols[0].nulls_position, "npLast");
    assert!(cols[0].groupable);
    assert_eq!(cols[0].group_index, 0);

    assert_eq!(cols[1].name, "name");
    assert_eq!(cols[1].title, "Name");
    assert_eq!(cols[1].align, ColumnAlign::Left);
    assert_eq!(cols[1].source, Some(1));

    assert_eq!(cols[2].name, "amount");
    assert_eq!(cols[2].title, "Amount");
    assert_eq!(cols[2].align, ColumnAlign::Right);
    assert_eq!(cols[2].source, Some(2));
}

#[test]
fn from_design_filters_reorders_and_applies_column_specs() {
    let names = vec!["id".into(), "name".into(), "amount".into()];
    let numeric = vec![true, false, true];
    let specs = vec![
        ColumnSpec {
            field: "amount".into(),
            title: "".into(), // derives from field -> "Amount"
            width: 150,
            align: "taRightJustify".into(),
            display_format: "$ #,##0.00".into(),
            visible: true,
            title_alignment: "taLeftJustify".into(),
            min_width: 0,
            max_width: 0,
            null_text: String::new(),
            boolean_texts: String::new(),
            value_map: String::new(),
            renderer: "crText".into(),
            color: Color::transparent(),
            text_color: Color::transparent(),
            title_color: Color::transparent(),
            vertical_alignment: "tlCenter".into(),
            padding: Edges::uniform(0.0),
            ellipsis: false,
            word_wrap: false,
            title_word_wrap: false,
            title_hint: String::new(),
            cell_hint: false,
            title_font: Font {
                name: String::new(),
                size: 0.0,
                ..Default::default()
            },
            font: Font {
                name: String::new(),
                size: 0.0,
                ..Default::default()
            },
            auto_size: false,
            resizable: true,
            movable: true,
            sortable: true,
            sort_order: "soNone".into(),
            sort_priority: 0,
            nulls_position: "npLast".into(),
            groupable: true,
            group_index: 0,
            summary: "smNone".into(),
            summary_format: String::new(),
            group_summary: "smNone".into(),
            filterable: false,
            filter_kind: "fkAuto".into(),
            read_only: false,
            required: false,
            default_value: String::new(),
            max_length: 0,
            min_value: 0,
            max_value: 0,
            tab_stop: true,
            editor: "edAuto".into(),
            pick_list: Vec::new(),
            edit_mask: String::new(),
            get_text_hook: false,
            set_text_hook: false,
            validate_hook: false,
            lookup_key: String::new(),
            lookup_display: Vec::new(),
            button_hook: false,
            drop_down_count: 8,
            field_type: "UNKNOWN".into(),
            draw_cell_hook: false,
            style_rules: Vec::new(),
        },
        ColumnSpec {
            field: "unknown".into(), // not in schema -> skipped
            title: "Unknown".into(),
            width: 100,
            align: "taLeftJustify".into(),
            display_format: String::new(),
            visible: true,
            title_alignment: "taLeftJustify".into(),
            min_width: 0,
            max_width: 0,
            null_text: String::new(),
            boolean_texts: String::new(),
            value_map: String::new(),
            renderer: "crText".into(),
            color: Color::transparent(),
            text_color: Color::transparent(),
            title_color: Color::transparent(),
            vertical_alignment: "tlCenter".into(),
            padding: Edges::uniform(0.0),
            ellipsis: false,
            word_wrap: false,
            title_word_wrap: false,
            title_hint: String::new(),
            cell_hint: false,
            title_font: Font {
                name: String::new(),
                size: 0.0,
                ..Default::default()
            },
            font: Font {
                name: String::new(),
                size: 0.0,
                ..Default::default()
            },
            auto_size: false,
            resizable: true,
            movable: true,
            sortable: true,
            sort_order: "soNone".into(),
            sort_priority: 0,
            nulls_position: "npLast".into(),
            groupable: true,
            group_index: 0,
            summary: "smNone".into(),
            summary_format: String::new(),
            group_summary: "smNone".into(),
            filterable: false,
            filter_kind: "fkAuto".into(),
            read_only: false,
            required: false,
            default_value: String::new(),
            max_length: 0,
            min_value: 0,
            max_value: 0,
            tab_stop: true,
            editor: "edAuto".into(),
            pick_list: Vec::new(),
            edit_mask: String::new(),
            get_text_hook: false,
            set_text_hook: false,
            validate_hook: false,
            lookup_key: String::new(),
            lookup_display: Vec::new(),
            button_hook: false,
            drop_down_count: 8,
            field_type: "UNKNOWN".into(),
            draw_cell_hook: false,
            style_rules: Vec::new(),
        },
        ColumnSpec {
            field: "name".into(),
            title: "Customer Name".into(),
            width: 0, // 0 -> None
            align: "taCenter".into(),
            display_format: String::new(),
            visible: true,
            title_alignment: "taLeftJustify".into(),
            min_width: 0,
            max_width: 0,
            null_text: String::new(),
            boolean_texts: String::new(),
            value_map: String::new(),
            renderer: "crText".into(),
            color: Color::transparent(),
            text_color: Color::transparent(),
            title_color: Color::transparent(),
            vertical_alignment: "tlCenter".into(),
            padding: Edges::uniform(0.0),
            ellipsis: false,
            word_wrap: false,
            title_word_wrap: false,
            title_hint: String::new(),
            cell_hint: false,
            title_font: Font {
                name: String::new(),
                size: 0.0,
                ..Default::default()
            },
            font: Font {
                name: String::new(),
                size: 0.0,
                ..Default::default()
            },
            auto_size: false,
            resizable: true,
            movable: true,
            sortable: true,
            sort_order: "soNone".into(),
            sort_priority: 0,
            nulls_position: "npLast".into(),
            groupable: true,
            group_index: 0,
            summary: "smNone".into(),
            summary_format: String::new(),
            group_summary: "smNone".into(),
            filterable: false,
            filter_kind: "fkAuto".into(),
            read_only: false,
            required: false,
            default_value: String::new(),
            max_length: 0,
            min_value: 0,
            max_value: 0,
            tab_stop: true,
            editor: "edAuto".into(),
            pick_list: Vec::new(),
            edit_mask: String::new(),
            get_text_hook: false,
            set_text_hook: false,
            validate_hook: false,
            lookup_key: String::new(),
            lookup_display: Vec::new(),
            button_hook: false,
            drop_down_count: 8,
            field_type: "UNKNOWN".into(),
            draw_cell_hook: false,
            style_rules: Vec::new(),
        },
        ColumnSpec {
            field: "id".into(),
            title: "ID".into(),
            width: 50,
            align: "taRightJustify".into(),
            display_format: String::new(),
            visible: false, // hidden -> skipped
            title_alignment: "taLeftJustify".into(),
            min_width: 0,
            max_width: 0,
            null_text: String::new(),
            boolean_texts: String::new(),
            value_map: String::new(),
            renderer: "crText".into(),
            color: Color::transparent(),
            text_color: Color::transparent(),
            title_color: Color::transparent(),
            vertical_alignment: "tlCenter".into(),
            padding: Edges::uniform(0.0),
            ellipsis: false,
            word_wrap: false,
            title_word_wrap: false,
            title_hint: String::new(),
            cell_hint: false,
            title_font: Font {
                name: String::new(),
                size: 0.0,
                ..Default::default()
            },
            font: Font {
                name: String::new(),
                size: 0.0,
                ..Default::default()
            },
            auto_size: false,
            resizable: true,
            movable: true,
            sortable: true,
            sort_order: "soNone".into(),
            sort_priority: 0,
            nulls_position: "npLast".into(),
            groupable: true,
            group_index: 0,
            summary: "smNone".into(),
            summary_format: String::new(),
            group_summary: "smNone".into(),
            filterable: false,
            filter_kind: "fkAuto".into(),
            read_only: false,
            required: false,
            default_value: String::new(),
            max_length: 0,
            min_value: 0,
            max_value: 0,
            tab_stop: true,
            editor: "edAuto".into(),
            pick_list: Vec::new(),
            edit_mask: String::new(),
            get_text_hook: false,
            set_text_hook: false,
            validate_hook: false,
            lookup_key: String::new(),
            lookup_display: Vec::new(),
            button_hook: false,
            drop_down_count: 8,
            field_type: "UNKNOWN".into(),
            draw_cell_hook: false,
            style_rules: Vec::new(),
        },
    ];

    let cols = GridColumn::from_design(&names, &numeric, &specs);
    assert_eq!(cols.len(), 2);

    assert_eq!(cols[0].name, "amount");
    assert_eq!(cols[0].title, "Amount");
    assert_eq!(cols[0].align, ColumnAlign::Right);
    assert_eq!(cols[0].source, Some(2));
    assert_eq!(cols[0].width, Some(150.0));
    assert_eq!(cols[0].display_format, "$ #,##0.00");

    assert_eq!(cols[1].name, "name");
    assert_eq!(cols[1].title, "Customer Name");
    assert_eq!(cols[1].align, ColumnAlign::Center);
    assert_eq!(cols[1].source, Some(1));
    assert_eq!(cols[1].width, None);
}

#[test]
fn row_projection_handles_hidden_reordered_and_formatted_columns() {
    let names = vec!["id".into(), "name".into(), "salary".into()];
    let numeric = vec![true, false, true];
    let specs = vec![
        ColumnSpec {
            field: "salary".into(),
            title: "Salary".into(),
            width: 120,
            align: "taRightJustify".into(),
            display_format: "$ #,##0.00".into(),
            visible: true,
            title_alignment: "taLeftJustify".into(),
            min_width: 0,
            max_width: 0,
            null_text: String::new(),
            boolean_texts: String::new(),
            value_map: String::new(),
            renderer: "crText".into(),
            color: Color::transparent(),
            text_color: Color::transparent(),
            title_color: Color::transparent(),
            vertical_alignment: "tlCenter".into(),
            padding: Edges::uniform(0.0),
            ellipsis: false,
            word_wrap: false,
            title_word_wrap: false,
            title_hint: String::new(),
            cell_hint: false,
            title_font: Font {
                name: String::new(),
                size: 0.0,
                ..Default::default()
            },
            font: Font {
                name: String::new(),
                size: 0.0,
                ..Default::default()
            },
            auto_size: false,
            resizable: true,
            movable: true,
            sortable: true,
            sort_order: "soNone".into(),
            sort_priority: 0,
            nulls_position: "npLast".into(),
            groupable: true,
            group_index: 0,
            summary: "smNone".into(),
            summary_format: String::new(),
            group_summary: "smNone".into(),
            filterable: false,
            filter_kind: "fkAuto".into(),
            read_only: false,
            required: false,
            default_value: String::new(),
            max_length: 0,
            min_value: 0,
            max_value: 0,
            tab_stop: true,
            editor: "edAuto".into(),
            pick_list: Vec::new(),
            edit_mask: String::new(),
            get_text_hook: false,
            set_text_hook: false,
            validate_hook: false,
            lookup_key: String::new(),
            lookup_display: Vec::new(),
            button_hook: false,
            drop_down_count: 8,
            field_type: "UNKNOWN".into(),
            draw_cell_hook: false,
            style_rules: Vec::new(),
        },
        ColumnSpec {
            field: "name".into(),
            title: "Employee".into(),
            width: 200,
            align: "taLeftJustify".into(),
            display_format: String::new(),
            visible: true,
            title_alignment: "taLeftJustify".into(),
            min_width: 0,
            max_width: 0,
            null_text: String::new(),
            boolean_texts: String::new(),
            value_map: String::new(),
            renderer: "crText".into(),
            color: Color::transparent(),
            text_color: Color::transparent(),
            title_color: Color::transparent(),
            vertical_alignment: "tlCenter".into(),
            padding: Edges::uniform(0.0),
            ellipsis: false,
            word_wrap: false,
            title_word_wrap: false,
            title_hint: String::new(),
            cell_hint: false,
            title_font: Font {
                name: String::new(),
                size: 0.0,
                ..Default::default()
            },
            font: Font {
                name: String::new(),
                size: 0.0,
                ..Default::default()
            },
            auto_size: false,
            resizable: true,
            movable: true,
            sortable: true,
            sort_order: "soNone".into(),
            sort_priority: 0,
            nulls_position: "npLast".into(),
            groupable: true,
            group_index: 0,
            summary: "smNone".into(),
            summary_format: String::new(),
            group_summary: "smNone".into(),
            filterable: false,
            filter_kind: "fkAuto".into(),
            read_only: false,
            required: false,
            default_value: String::new(),
            max_length: 0,
            min_value: 0,
            max_value: 0,
            tab_stop: true,
            editor: "edAuto".into(),
            pick_list: Vec::new(),
            edit_mask: String::new(),
            get_text_hook: false,
            set_text_hook: false,
            validate_hook: false,
            lookup_key: String::new(),
            lookup_display: Vec::new(),
            button_hook: false,
            drop_down_count: 8,
            field_type: "UNKNOWN".into(),
            draw_cell_hook: false,
            style_rules: Vec::new(),
        },
        // "id" is hidden
        ColumnSpec {
            field: "id".into(),
            title: "ID".into(),
            width: 50,
            align: "taRightJustify".into(),
            display_format: String::new(),
            visible: false,
            title_alignment: "taLeftJustify".into(),
            min_width: 0,
            max_width: 0,
            null_text: String::new(),
            boolean_texts: String::new(),
            value_map: String::new(),
            renderer: "crText".into(),
            color: Color::transparent(),
            text_color: Color::transparent(),
            title_color: Color::transparent(),
            vertical_alignment: "tlCenter".into(),
            padding: Edges::uniform(0.0),
            ellipsis: false,
            word_wrap: false,
            title_word_wrap: false,
            title_hint: String::new(),
            cell_hint: false,
            title_font: Font {
                name: String::new(),
                size: 0.0,
                ..Default::default()
            },
            font: Font {
                name: String::new(),
                size: 0.0,
                ..Default::default()
            },
            auto_size: false,
            resizable: true,
            movable: true,
            sortable: true,
            sort_order: "soNone".into(),
            sort_priority: 0,
            nulls_position: "npLast".into(),
            groupable: true,
            group_index: 0,
            summary: "smNone".into(),
            summary_format: String::new(),
            group_summary: "smNone".into(),
            filterable: false,
            filter_kind: "fkAuto".into(),
            read_only: false,
            required: false,
            default_value: String::new(),
            max_length: 0,
            min_value: 0,
            max_value: 0,
            tab_stop: true,
            editor: "edAuto".into(),
            pick_list: Vec::new(),
            edit_mask: String::new(),
            get_text_hook: false,
            set_text_hook: false,
            validate_hook: false,
            lookup_key: String::new(),
            lookup_display: Vec::new(),
            button_hook: false,
            drop_down_count: 8,
            field_type: "UNKNOWN".into(),
            draw_cell_hook: false,
            style_rules: Vec::new(),
        },
    ];

    let cols = GridColumn::from_design(&names, &numeric, &specs);
    let state = GridState::new(cols);
    let screen = datagrid::Screen {
        lines: vec![datagrid::ScreenLine::Row {
            index: 0,
            id: 0,
            cells: vec![
                Some("42".into()),
                Some("Alice".into()),
                Some("125000".into()),
            ],
        }],
        ..Default::default()
    };
    let view = state.view(&screen, Instant::now());
    assert_eq!(view.lines.len(), 1);
    match &view.lines[0] {
        ViewLine::Row { cells, .. } => {
            // Projected order: salary ($ 125,000.00), name ("Alice")
            assert_eq!(cells, &["$ 125,000.00", "Alice"]);
        }
        other => panic!("expected a row line, got {other:?}"),
    }
}

#[test]
fn share_widths_keeps_fixed_widths_and_shares_the_rest() {
    assert_eq!(
        share_widths(600.0, &[Some(100.0), None, None]),
        vec![100.0, 250.0, 250.0]
    );
    // Fixed widths wider than the grid leave nothing, never a negative share.
    assert_eq!(share_widths(100.0, &[Some(150.0), None]), vec![150.0, 0.0]);
    assert!(share_widths(100.0, &[]).is_empty());
}

/// The catalogue stores `alignment` as an enum: the spec must read it.
#[test]
fn column_specs_of_a_grid_read_its_column_children_in_order() {
    use crawrad_ir::{Document, Node, Value};
    let mut doc = Document::new("form1", "Form");
    let mut grid = Node::new("grid1", "Grid");
    grid.children = vec!["c2".into(), "c1".into(), "b1".into()];
    let mut c1 = Node::new("c1", GRID_COLUMN_CLASS);
    c1.props.insert("field".into(), Value::Str("name".into()));
    c1.props.insert("visible".into(), Value::Bool(false));
    let mut c2 = Node::new("c2", GRID_COLUMN_CLASS);
    c2.props
        .insert("field".into(), Value::Str("balance".into()));
    c2.props
        .insert("alignment".into(), Value::Enum("taRightJustify".into()));
    c2.props.insert("width".into(), Value::Int(90));
    c2.props
        .insert("display_format".into(), Value::Str("#,##0.00".into()));
    let other = Node::new("b1", "Button");
    for n in [grid.clone(), c1, c2, other] {
        doc.nodes.insert(n.id.clone(), n);
    }
    let specs = ColumnSpec::of_grid(&doc, &grid);
    assert_eq!(specs.len(), 2);
    assert_eq!(specs[0].field, "balance");
    assert_eq!(specs[0].align, "taRightJustify");
    assert_eq!(specs[0].width, 90);
    assert_eq!(specs[0].display_format, "#,##0.00");
    assert!(specs[0].visible);
    assert_eq!(specs[1].field, "name");
    assert!(!specs[1].visible);
}

/// A null cell displays null_text, while an empty string cell stays empty.
#[test]
fn cell_text_distinguishes_null_from_empty_string() {
    let mut col = GridColumn::new("name", "Name", ColumnAlign::Left);
    col.null_text = "N/A".into();

    assert_eq!(cell_text(&col, None, None), "N/A");
    assert_eq!(cell_text(&col, None, Some("")), "");
    assert_eq!(cell_text(&col, None, Some("John")), "John");

    col.null_text = String::new();
    assert_eq!(cell_text(&col, None, None), "");
}

/// A boolean column with boolean_texts maps truthy and falsy values.
#[test]
fn cell_text_applies_boolean_texts_on_boolean_columns() {
    let mut col = GridColumn::new("active", "Active", ColumnAlign::Center);
    col.boolean_texts = "Yes;No".into();

    // With explicit BOOLEAN kind
    assert_eq!(cell_text(&col, Some("BOOLEAN"), Some("1")), "Yes");
    assert_eq!(cell_text(&col, Some("BOOLEAN"), Some("true")), "Yes");
    assert_eq!(cell_text(&col, Some("BOOLEAN"), Some("TRUE")), "Yes");
    assert_eq!(cell_text(&col, Some("BOOLEAN"), Some("0")), "No");
    assert_eq!(cell_text(&col, Some("BOOLEAN"), Some("false")), "No");
    assert_eq!(cell_text(&col, Some("BOOLEAN"), Some("FALSE")), "No");
    assert_eq!(cell_text(&col, Some("BOOLEAN"), Some("other")), "other");

    // With col.kind set
    col.kind = Some("BOOLEAN".into());
    assert_eq!(cell_text(&col, None, Some("1")), "Yes");
    assert_eq!(cell_text(&col, None, Some("0")), "No");

    // Non-boolean kind leaves raw text unchanged
    assert_eq!(cell_text(&col, Some("INTEGER"), Some("1")), "1");
    assert_eq!(cell_text(&col, Some("INTEGER"), Some("0")), "0");

    // Empty boolean_texts leaves text unchanged
    col.boolean_texts = String::new();
    assert_eq!(cell_text(&col, Some("BOOLEAN"), Some("1")), "1");
}

/// Cell text applies display format after boolean formatting.
#[test]
fn cell_text_applies_display_format() {
    let mut col = GridColumn::new("price", "Price", ColumnAlign::Right);
    col.display_format = "$ #,##0.00".into();

    assert_eq!(cell_text(&col, Some("FLOAT"), Some("1234.5")), "$ 1,234.50");
    assert_eq!(cell_text(&col, Some("FLOAT"), None), "");

    col.null_text = "—".into();
    assert_eq!(cell_text(&col, Some("FLOAT"), None), "—");
}

/// HeaderCell gets title_align independently of cell align.
#[test]
fn header_cells_use_title_alignment() {
    let mut col = GridColumn::new("score", "Score", ColumnAlign::Right);
    col.title_align = ColumnAlign::Center;
    let state = GridState::new(vec![col]);
    let cells = state.header_cells();
    assert_eq!(cells.len(), 1);
    assert_eq!(cells[0].align, ColumnAlign::Center);
}

/// Clamped width sharing enforces min_width and redistributes among free columns.
#[test]
fn share_widths_clamped_min_width() {
    let widths = [None, None, None];
    let bounds = [(120.0, None), (0.0, None), (0.0, None)];
    let res = share_widths_clamped(300.0, &widths, &bounds);
    assert_eq!(res, vec![120.0, 90.0, 90.0]);
}

/// Clamped width sharing enforces max_width and redistributes among free columns.
#[test]
fn share_widths_clamped_max_width() {
    let widths = [None, None, None];
    let bounds = [(0.0, Some(80.0)), (0.0, None), (0.0, None)];
    let res = share_widths_clamped(300.0, &widths, &bounds);
    assert_eq!(res, vec![80.0, 110.0, 110.0]);
}

/// Clamped width sharing cascades across multiple iterations.
#[test]
fn share_widths_clamped_cascading() {
    let widths = [None, None, None];
    let bounds = [(120.0, None), (105.0, None), (0.0, None)];
    let res = share_widths_clamped(300.0, &widths, &bounds);
    assert_eq!(res, vec![120.0, 105.0, 75.0]);
}

/// Clamped width sharing respects fixed columns alongside bounded free columns.
#[test]
fn share_widths_clamped_with_fixed_columns() {
    let widths = [Some(200.0), None, None];
    let bounds = [(0.0, None), (0.0, Some(100.0)), (0.0, None)];
    let res = share_widths_clamped(500.0, &widths, &bounds);
    assert_eq!(res, vec![200.0, 100.0, 200.0]);
}

/// ColumnVAlign::from_catalogue maps tlTop, tlCenter, tlBottom and defaults to Center.
#[test]
fn column_valign_from_catalogue() {
    assert_eq!(ColumnVAlign::from_catalogue("tlTop"), ColumnVAlign::Top);
    assert_eq!(
        ColumnVAlign::from_catalogue("tlCenter"),
        ColumnVAlign::Center
    );
    assert_eq!(
        ColumnVAlign::from_catalogue("tlBottom"),
        ColumnVAlign::Bottom
    );
    assert_eq!(
        ColumnVAlign::from_catalogue("unknown"),
        ColumnVAlign::Center
    );
    assert_eq!(ColumnVAlign::from_catalogue(""), ColumnVAlign::Center);
}

/// from_design maps every field from ColumnSpec, including defaults to None/transparent/Center.
#[test]
fn from_design_maps_every_field_including_defaults() {
    let names = vec!["name".to_string(), "salary".to_string()];
    let numeric = vec![false, true];
    let default_spec = plain_spec("name");
    let custom_spec = ColumnSpec {
        field: "salary".to_string(),
        title: "Base Salary".to_string(),
        width: 120,
        align: "taRightJustify".to_string(),
        display_format: "$ #,##0.00".to_string(),
        visible: true,
        null_text: "N/A".to_string(),
        boolean_texts: "".to_string(),
        value_map: "A=Active;I=Inactive".to_string(),
        renderer: "crProgressBar".to_string(),
        title_alignment: "taCenter".to_string(),
        min_width: 80,
        max_width: 200,
        color: Color::rgb(240, 240, 240),
        text_color: Color::rgb(20, 20, 20),
        title_color: Color::rgb(10, 20, 30),
        vertical_alignment: "tlBottom".to_string(),
        padding: Edges {
            left: 4.0,
            top: 2.0,
            right: 6.0,
            bottom: 8.0,
        },
        ellipsis: true,
        word_wrap: true,
        title_word_wrap: true,
        title_hint: "Salary hint".to_string(),
        cell_hint: true,
        title_font: Font::new("Arial", 16.0).color(Color::rgb(10, 20, 30)),
        font: Font::new("Calibri", 12.0).color(Color::rgb(20, 30, 40)),
        auto_size: true,
        resizable: false,
        movable: false,
        sortable: false,
        sort_order: "soDesc".to_string(),
        sort_priority: 2,
        nulls_position: "npFirst".to_string(),
        groupable: false,
        group_index: 0,
        summary: "smSum".to_string(),
        summary_format: "$ #,##0.00".to_string(),
        group_summary: "smAvg".to_string(),
        filterable: true,
        filter_kind: "fkRange".to_string(),
        field_type: "UNKNOWN".into(),
        read_only: false,
        required: false,
        default_value: String::new(),
        max_length: 0,
        min_value: 0,
        max_value: 0,
        tab_stop: true,
        editor: "edAuto".into(),
        pick_list: Vec::new(),
        edit_mask: String::new(),
        get_text_hook: false,
        set_text_hook: false,
        validate_hook: false,
        lookup_key: String::new(),
        lookup_display: Vec::new(),
        button_hook: false,
        drop_down_count: 8,
        draw_cell_hook: false,
        style_rules: Vec::new(),
    };
    let cols = GridColumn::from_design(&names, &numeric, &[default_spec, custom_spec]);
    assert_eq!(cols.len(), 2);

    // Default column checks
    assert_eq!(cols[0].value_map, "");
    assert_eq!(cols[0].renderer, CellRenderer::Text);
    assert_eq!(cols[0].title_color, Color::transparent());
    assert_eq!(cols[0].valign, ColumnVAlign::Center);
    assert_eq!(cols[0].padding, None);
    assert!(!cols[0].ellipsis);
    assert!(!cols[0].word_wrap);
    assert!(!cols[0].title_word_wrap);
    assert_eq!(cols[0].title_hint, "");
    assert!(!cols[0].cell_hint);
    assert_eq!(
        cols[0].title_font,
        Font {
            name: String::new(),
            size: 0.0,
            ..Default::default()
        }
    );
    assert_eq!(
        cols[0].font,
        Font {
            name: String::new(),
            size: 0.0,
            ..Default::default()
        }
    );
    assert!(!cols[0].auto_size);
    assert!(cols[0].resizable);
    assert!(cols[0].movable);
    assert!(cols[0].sortable);
    assert_eq!(cols[0].sort_order, "soNone");
    assert_eq!(cols[0].sort_priority, 0);
    assert_eq!(cols[0].nulls_position, "npLast");
    assert!(cols[0].groupable);
    assert_eq!(cols[0].group_index, 0);
    assert_eq!(cols[0].summary, "smNone");
    assert_eq!(cols[0].summary_format, "");
    assert_eq!(cols[0].group_summary, "smNone");
    assert!(!cols[0].filterable);
    assert_eq!(cols[0].filter_kind, "fkAuto");

    // Custom column checks
    assert_eq!(cols[1].title, "Base Salary");
    assert_eq!(cols[1].value_map, "A=Active;I=Inactive");
    assert_eq!(cols[1].renderer, CellRenderer::ProgressBar);
    assert_eq!(cols[1].title_color, Color::rgb(10, 20, 30));
    assert_eq!(cols[1].valign, ColumnVAlign::Bottom);
    assert!(cols[1].auto_size);
    assert!(!cols[1].resizable);
    assert!(!cols[1].movable);
    assert!(!cols[1].sortable);
    assert_eq!(cols[1].sort_order, "soDesc");
    assert_eq!(cols[1].sort_priority, 2);
    assert_eq!(cols[1].nulls_position, "npFirst");
    assert!(!cols[1].groupable);
    assert_eq!(cols[1].group_index, 0);
    assert_eq!(cols[1].summary, "smSum");
    assert_eq!(cols[1].summary_format, "$ #,##0.00");
    assert_eq!(cols[1].group_summary, "smAvg");
    assert!(cols[1].filterable);
    assert_eq!(cols[1].filter_kind, "fkRange");
    assert!(cols[1].ellipsis);
    assert!(cols[1].word_wrap);
    assert!(cols[1].title_word_wrap);
    assert_eq!(cols[1].title_hint, "Salary hint");
    assert!(cols[1].cell_hint);
    assert_eq!(
        cols[1].title_font,
        Font::new("Arial", 16.0).color(Color::rgb(10, 20, 30))
    );
    assert_eq!(
        cols[1].font,
        Font::new("Calibri", 12.0).color(Color::rgb(20, 30, 40))
    );
    assert_eq!(
        cols[1].padding,
        Some(Edges {
            left: 4.0,
            top: 2.0,
            right: 6.0,
            bottom: 8.0,
        })
    );
}

/// HeaderCell carries title_font from the column.
#[test]
fn header_cells_carry_title_font() {
    let mut col = GridColumn::new("score", "Score", ColumnAlign::Right);
    col.title_font = Font::new("Arial", 15.0);
    let state = GridState::new(vec![col]);
    let cells = state.header_cells();
    assert_eq!(cells.len(), 1);
    assert_eq!(cells[0].title_font, Font::new("Arial", 15.0));
}

/// HeaderCell carries title_color from the column.
#[test]
fn header_cells_carry_title_color() {
    let mut col = GridColumn::new("score", "Score", ColumnAlign::Right);
    col.title_color = Color::rgb(50, 60, 70);
    let state = GridState::new(vec![col]);
    let cells = state.header_cells();
    assert_eq!(cells.len(), 1);
    assert_eq!(cells[0].title_color, Color::rgb(50, 60, 70));
}

/// HeaderCell carries title_hint from the column.
#[test]
fn header_cells_carry_title_hint() {
    let mut col = GridColumn::new("score", "Score", ColumnAlign::Right);
    col.title_hint = "Sort score".to_string();
    let state = GridState::new(vec![col]);
    let cells = state.header_cells();
    assert_eq!(cells.len(), 1);
    assert_eq!(cells[0].title_hint, "Sort score");
}

/// Tests fit_text without ellipsis: longest prefix that fits.
#[test]
fn fit_text_without_ellipsis() {
    let width_fn = |s: &str| s.chars().count() as f64;
    assert_eq!(fit_text("hello", 10.0, false, width_fn), "hello");
    assert_eq!(fit_text("hello", 5.0, false, width_fn), "hello");
    assert_eq!(fit_text("hello", 3.0, false, width_fn), "hel");
    assert_eq!(fit_text("hello", 0.0, false, width_fn), "");
}

/// Tests fit_text with ellipsis: longest prefix + "…" that fits.
#[test]
fn fit_text_with_ellipsis() {
    let width_fn = |s: &str| s.chars().count() as f64;
    assert_eq!(fit_text("hello", 10.0, true, width_fn), "hello");
    assert_eq!(fit_text("hello", 5.0, true, width_fn), "hello");
    assert_eq!(fit_text("hello", 4.0, true, width_fn), "hel…");
    assert_eq!(fit_text("hello", 2.0, true, width_fn), "h…");
    assert_eq!(fit_text("hello", 1.0, true, width_fn), "…");
    assert_eq!(fit_text("hello", 0.5, true, width_fn), "");
}

/// Tests wrap_text: empty text, single line fit, space breaks, overflowing words, and ellipsis.
#[test]
fn wrap_text_cases() {
    let width_fn = |s: &str| s.chars().count() as f64;

    // Empty text
    assert_eq!(
        wrap_text("", 10.0, 3, false, width_fn),
        Vec::<String>::new()
    );

    // Fits in single line
    assert_eq!(
        wrap_text("hello world", 20.0, 3, false, width_fn),
        vec!["hello world".to_string()]
    );

    // Wraps cleanly at space boundary
    assert_eq!(
        wrap_text("hello world foo bar", 11.0, 3, false, width_fn),
        vec!["hello world".to_string(), "foo bar".to_string()]
    );

    // Word wider than max_width cut by fit_text
    assert_eq!(
        wrap_text("supercalifragilistic", 5.0, 2, false, width_fn),
        vec!["super".to_string(), "calif".to_string()]
    );

    // Ellipsis on the last visible line
    let wrapped_ellipsis = wrap_text("one two three four five", 9.0, 2, true, width_fn);
    assert_eq!(wrapped_ellipsis.len(), 2);
    assert_eq!(wrapped_ellipsis[0], "one two");
    assert!(wrapped_ellipsis[1].ends_with('…'));
    assert_eq!(width_fn(&wrapped_ellipsis[1]), 9.0);

    // Max lines drops excess without ellipsis
    assert_eq!(
        wrap_text("one two three four", 4.0, 2, false, width_fn),
        vec!["one".to_string(), "two".to_string()]
    );
}

/// HeaderCell carries title_word_wrap from the column.
#[test]
fn header_cells_carry_title_word_wrap() {
    let mut col = GridColumn::new("score", "Score", ColumnAlign::Right);
    col.title_word_wrap = true;
    let state = GridState::new(vec![col]);
    let cells = state.header_cells();
    assert_eq!(cells.len(), 1);
    assert!(cells[0].title_word_wrap);
}

/// Tests auto_width: widest text (title or cell) + padding clamped to [min, max].
#[test]
fn auto_width_calculates_widest_plus_padding_and_clamps() {
    let len_fn = |s: &str| s.chars().count() as f64 * 10.0;

    // Title is wider: "Title" (50) + 16 = 66
    let w1 = auto_width("Title", &["a", "bb"], 16.0, 0.0, None, len_fn);
    assert_eq!(w1, 66.0);

    // A cell is wider: "SuperLongCell" (130) + 16 = 146
    let w2 = auto_width(
        "Title",
        &["short", "SuperLongCell"],
        16.0,
        0.0,
        None,
        len_fn,
    );
    assert_eq!(w2, 146.0);

    // Clamped by min_width (min = 200.0)
    let w3 = auto_width("Title", &["a"], 16.0, 200.0, None, len_fn);
    assert_eq!(w3, 200.0);

    // Clamped by max_width (max = 100.0)
    let w4 = auto_width(
        "Title",
        &["VeryVeryLongCell"],
        16.0,
        0.0,
        Some(100.0),
        len_fn,
    );
    assert_eq!(w4, 100.0);
}

/// Tests move_column on order vector.
#[test]
fn move_column_reorders_order_vector() {
    let mut order = vec![0, 1, 2, 3];
    move_column(&mut order, 1, 3);
    assert_eq!(order, vec![0, 2, 3, 1]);

    move_column(&mut order, 3, 0);
    assert_eq!(order, vec![1, 0, 2, 3]);

    // Same index does nothing
    move_column(&mut order, 2, 2);
    assert_eq!(order, vec![1, 0, 2, 3]);

    // Out of bounds does nothing
    move_column(&mut order, 10, 0);
    assert_eq!(order, vec![1, 0, 2, 3]);
}

/// Tests clamp_column_width enforces at least 24 px and [min, max] bounds.
#[test]
fn clamp_column_width_enforces_bounds() {
    // Width below 24 clamped to 24
    assert_eq!(clamp_column_width(10.0, 0.0, None), 24.0);

    // Width below min clamped to min
    assert_eq!(clamp_column_width(50.0, 80.0, None), 80.0);

    // Width above max clamped to max
    assert_eq!(clamp_column_width(150.0, 30.0, Some(100.0)), 100.0);

    // Normal width between min and max preserved
    assert_eq!(clamp_column_width(75.0, 30.0, Some(100.0)), 75.0);
}

/// HeaderCell carries auto_size, resizable, and movable from column.
#[test]
fn header_cells_carry_auto_size_resizable_and_movable() {
    let mut col = GridColumn::new("score", "Score", ColumnAlign::Right);
    col.auto_size = true;
    col.resizable = false;
    col.movable = false;
    let state = GridState::new(vec![col]);
    let cells = state.header_cells();
    assert_eq!(cells.len(), 1);
    assert!(cells[0].auto_size);
    assert!(!cells[0].resizable);
    assert!(!cells[0].movable);
}

#[test]
fn test_map_value() {
    let map = "A=Active; I=Inactive ; B = Blocked ";
    assert_eq!(map_value(map, "A"), Some("Active"));
    assert_eq!(map_value(map, "I"), Some("Inactive"));
    assert_eq!(map_value(map, "B"), Some("Blocked"));
    assert_eq!(map_value(map, " B "), Some("Blocked"));
    assert_eq!(map_value(map, "X"), None);
    assert_eq!(map_value("", "A"), None);
    assert_eq!(map_value("   ", "A"), None);
}

#[test]
fn test_cell_renderer_from_key() {
    assert_eq!(CellRenderer::from_key("crText"), CellRenderer::Text);
    assert_eq!(CellRenderer::from_key("crCheckBox"), CellRenderer::CheckBox);
    assert_eq!(
        CellRenderer::from_key("crProgressBar"),
        CellRenderer::ProgressBar
    );
    assert_eq!(CellRenderer::from_key("crLink"), CellRenderer::Link);
    assert_eq!(
        CellRenderer::from_key("crColourSwatch"),
        CellRenderer::ColourSwatch
    );
    assert_eq!(CellRenderer::from_key("unknown"), CellRenderer::Text);
    assert_eq!(CellRenderer::from_key(""), CellRenderer::Text);
}

#[test]
fn test_checkbox_state() {
    assert_eq!(checkbox_state("1"), Some(true));
    assert_eq!(checkbox_state("true"), Some(true));
    assert_eq!(checkbox_state("True"), Some(true));
    assert_eq!(checkbox_state("TRUE"), Some(true));
    assert_eq!(checkbox_state(" 1 "), Some(true));

    assert_eq!(checkbox_state("0"), Some(false));
    assert_eq!(checkbox_state("false"), Some(false));
    assert_eq!(checkbox_state("False"), Some(false));
    assert_eq!(checkbox_state("FALSE"), Some(false));
    assert_eq!(checkbox_state(" 0 "), Some(false));

    assert_eq!(checkbox_state("2"), None);
    assert_eq!(checkbox_state("yes"), None);
    assert_eq!(checkbox_state(""), None);
}

#[test]
fn test_progress_fraction() {
    assert_eq!(progress_fraction("0"), Some(0.0));
    assert_eq!(progress_fraction("50"), Some(0.5));
    assert_eq!(progress_fraction("100"), Some(1.0));
    assert_eq!(progress_fraction("-20"), Some(0.0));
    assert_eq!(progress_fraction("150"), Some(1.0));
    assert_eq!(progress_fraction("25.5"), Some(0.255));
    assert_eq!(progress_fraction("not_a_number"), None);
    assert_eq!(progress_fraction(""), None);
}

#[test]
fn test_parse_hex_color() {
    assert_eq!(parse_hex_color("#ff0000"), Some(Color::rgb(255, 0, 0)));
    assert_eq!(
        parse_hex_color("#00ff0080"),
        Some(Color {
            r: 0,
            g: 255,
            b: 0,
            a: 128
        })
    );
    assert_eq!(
        parse_hex_color(" #112233 "),
        Some(Color::rgb(0x11, 0x22, 0x33))
    );
    assert_eq!(parse_hex_color("112233"), None);
    assert_eq!(parse_hex_color("#123"), None);
    assert_eq!(parse_hex_color("red"), None);
    assert_eq!(parse_hex_color(""), None);
}

#[test]
fn test_cell_text_with_value_map_and_display_format_order() {
    let mut col = GridColumn::new("status", "Status", ColumnAlign::Left);
    col.null_text = "N/A".into();
    col.value_map = "A=Active;I=Inactive;1=1234.5".into();
    col.display_format = "$ #,##0.00".into();

    // 1. NULL shows null_text, even with ValueMap and DisplayFormat
    assert_eq!(cell_text(&col, None, None), "N/A");

    // 2. ValueMap applied, but result is not a number -> display_format leaves it unchanged
    assert_eq!(cell_text(&col, None, Some("A")), "Active");
    assert_eq!(cell_text(&col, None, Some("I")), "Inactive");

    // 3. ValueMap applied before DisplayFormat: "1" maps to "1234.5", then formatted as "$ 1,234.50"
    assert_eq!(cell_text(&col, None, Some("1")), "$ 1,234.50");

    // 4. Unknown value shows raw, passed through display_format
    assert_eq!(cell_text(&col, None, Some("Unknown")), "Unknown");
    assert_eq!(cell_text(&col, None, Some("500")), "$ 500.00");
}

#[test]
fn test_initial_sort_order_priority_ties_nulls_and_sonone_skipped() {
    let mut c1 = GridColumn::new("c1", "C1", ColumnAlign::Left);
    c1.sort_order = "soAsc".to_string();
    c1.sort_priority = 2;
    c1.nulls_position = "npLast".to_string();

    let mut c2 = GridColumn::new("c2", "C2", ColumnAlign::Left);
    c2.sort_order = "soDesc".to_string();
    c2.sort_priority = 0;
    c2.nulls_position = "npFirst".to_string();

    let mut c3 = GridColumn::new("c3", "C3", ColumnAlign::Left);
    c3.sort_order = "soNone".to_string();
    c3.sort_priority = 0;

    let mut c4 = GridColumn::new("c4", "C4", ColumnAlign::Left);
    c4.sort_order = "soAsc".to_string();
    c4.sort_priority = 1;
    c4.nulls_position = "npLast".to_string();

    let mut c5 = GridColumn::new("c5", "C5", ColumnAlign::Left);
    c5.sort_order = "soDesc".to_string();
    c5.sort_priority = 1;
    c5.nulls_position = "npFirst".to_string();

    let sort = initial_sort(&[c1, c2, c3, c4, c5]);
    // c3 is skipped (soNone).
    // Priority order: c2 (0), then ties at priority 1: c4 then c5, then c1 (2).
    assert_eq!(sort.len(), 4);
    assert_eq!(sort[0].column, "c2");
    assert_eq!(sort[0].order, datagrid::Order::Desc);
    assert_eq!(sort[0].nulls, datagrid::Nulls::First);

    assert_eq!(sort[1].column, "c4");
    assert_eq!(sort[1].order, datagrid::Order::Asc);
    assert_eq!(sort[1].nulls, datagrid::Nulls::Last);

    assert_eq!(sort[2].column, "c5");
    assert_eq!(sort[2].order, datagrid::Order::Desc);
    assert_eq!(sort[2].nulls, datagrid::Nulls::First);

    assert_eq!(sort[3].column, "c1");
    assert_eq!(sort[3].order, datagrid::Order::Asc);
    assert_eq!(sort[3].nulls, datagrid::Nulls::Last);
}

#[test]
fn test_initial_groups_order_by_index_and_negative_skipped() {
    let mut c1 = GridColumn::new("c1", "C1", ColumnAlign::Left);
    c1.group_index = 2;

    let mut c2 = GridColumn::new("c2", "C2", ColumnAlign::Left);
    c2.group_index = 0;

    let mut c3 = GridColumn::new("c3", "C3", ColumnAlign::Left);
    c3.group_index = 1;

    let mut c4 = GridColumn::new("c4", "C4", ColumnAlign::Left);
    c4.group_index = 1;

    let groups = initial_groups(&[c1, c2, c3, c4]);
    // c2 is skipped (0 = not grouped).
    // Order: c3 (1), c4 (1, tie keeps column order), c1 (2).
    assert_eq!(groups.len(), 3);
    assert_eq!(groups[0].column, "c3");
    assert_eq!(groups[1].column, "c4");
    assert_eq!(groups[2].column, "c1");
}

#[test]
fn click_header_on_non_sortable_column_leaves_sort_unchanged() {
    let mut c1 = GridColumn::new("id", "Id", ColumnAlign::Right);
    c1.sortable = false;
    c1.nulls_position = "npFirst".to_string();

    let mut c2 = GridColumn::new("name", "Name", ColumnAlign::Left);
    c2.sortable = true;
    c2.nulls_position = "npFirst".to_string();

    let mut state = GridState::new(vec![c1, c2]);
    assert!(state.sort.is_empty());

    // Click non-sortable column (0): sort remains empty
    let q0 = state.click_header(0);
    assert!(q0.sort.is_empty());
    assert!(state.sort.is_empty());

    // Click sortable column (1): sorted by name with npFirst
    let q1 = state.click_header(1);
    assert_eq!(q1.sort.len(), 1);
    assert_eq!(q1.sort[0].column, "name");
    assert_eq!(q1.sort[0].order, datagrid::Order::Asc);
    assert_eq!(q1.sort[0].nulls, datagrid::Nulls::First);

    // Click non-sortable column (0) again: sort remains on name
    let q2 = state.click_header(0);
    assert_eq!(q2.sort.len(), 1);
    assert_eq!(q2.sort[0].column, "name");
}

#[test]
fn group_by_column_on_non_groupable_column_leaves_group_by_unchanged() {
    let mut c1 = GridColumn::new("city", "City", ColumnAlign::Left);
    c1.groupable = false;

    let mut c2 = GridColumn::new("customer", "Customer", ColumnAlign::Left);
    c2.groupable = true;

    let mut state = GridState::new(vec![c1, c2]);
    assert!(state.group_by.is_empty());

    // Attempt to group by non-groupable column 0 (city)
    let q0 = state.group_by_column(0);
    assert!(q0.group_by.is_empty());
    assert!(state.group_by.is_empty());

    // Group by groupable column 1 (customer)
    let q1 = state.group_by_column(1);
    assert_eq!(q1.group_by.len(), 1);
    assert_eq!(q1.group_by[0].column, "customer");

    // Attempt to group by non-groupable column 0 again: stays 1
    let q2 = state.group_by_column(0);
    assert_eq!(q2.group_by.len(), 1);
    assert_eq!(q2.group_by[0].column, "customer");
}

/// A group line reads its key the way the column's cells read it (ValueMap).
#[test]
fn a_group_key_goes_through_the_value_map() {
    let mut status = GridColumn::new("status", "Status", ColumnAlign::Left);
    status.value_map = "A=Active;I=Inactive".into();
    let state = GridState::new(vec![status]);
    let screen = datagrid::Screen {
        first: 0,
        lines: vec![datagrid::ScreenLine::Header {
            level: 0,
            column: "status".into(),
            key: "A".into(),
            row_count: 3,
            totals: vec![],
        }],
        ..Default::default()
    };
    match &state.view(&screen, Instant::now()).lines[0] {
        ViewLine::Header { title, .. } => assert_eq!(title, "▼ Status: Active"),
        other => panic!("expected a group line, got {other:?}"),
    }
}

#[test]
fn test_summary_text_cases() {
    let mut col = GridColumn::new("total", "Total", ColumnAlign::Right);
    col.summary = "smSum".into();
    col.display_format = "$ #,##0.00".into();

    // 1. None / NULL -> ""
    assert_eq!(summary_text(&col, None), "");

    // 2. Sum with display_format
    assert_eq!(summary_text(&col, Some(1234.5)), "Σ $ 1,234.50");

    // 3. Sum with summary_format overriding display_format
    col.summary_format = "EUR #,##0.0".into();
    assert_eq!(summary_text(&col, Some(1234.5)), "Σ EUR 1,234.5");

    // 4. Count -> integer, no mask, no prefix
    col.summary = "smCount".into();
    assert_eq!(summary_text(&col, Some(42.0)), "42");
    assert_eq!(summary_text(&col, Some(42.8)), "43");

    // 5. Avg -> prefix "avg "
    col.summary = "smAvg".into();
    col.summary_format = "".into();
    assert_eq!(summary_text(&col, Some(1234.5)), "avg $ 1,234.50");

    // 6. Min -> prefix "min "
    col.summary = "smMin".into();
    assert_eq!(summary_text(&col, Some(1234.5)), "min $ 1,234.50");

    // 7. Max -> prefix "max "
    col.summary = "smMax".into();
    assert_eq!(summary_text(&col, Some(1234.5)), "max $ 1,234.50");

    // 8. smNone -> ""
    col.summary = "smNone".into();
    assert_eq!(summary_text(&col, Some(1234.5)), "");
}

#[test]
fn test_summarize_cases() {
    let rows = vec![
        Some("10.5"),
        None,
        Some("20.5"),
        Some("not_a_num"),
        Some("5.0"),
    ];

    // Count: row count regardless of nulls
    assert_eq!(summarize(&rows, "smCount"), Some(5.0));

    // Sum: 10.5 + 20.5 + 5.0 = 36.0
    assert_eq!(summarize(&rows, "smSum"), Some(36.0));

    // Avg: 36.0 / 3 = 12.0
    assert_eq!(summarize(&rows, "smAvg"), Some(12.0));

    // Min: 5.0
    assert_eq!(summarize(&rows, "smMin"), Some(5.0));

    // Max: 20.5
    assert_eq!(summarize(&rows, "smMax"), Some(20.5));

    // smNone: None
    assert_eq!(summarize(&rows, "smNone"), None);

    // Empty rows: Count is 0, others None
    let empty: Vec<Option<&str>> = Vec::new();
    assert_eq!(summarize(&empty, "smCount"), Some(0.0));
    assert_eq!(summarize(&empty, "smSum"), None);
    assert_eq!(summarize(&empty, "smAvg"), None);
    assert_eq!(summarize(&empty, "smMin"), None);
    assert_eq!(summarize(&empty, "smMax"), None);
}

#[test]
fn test_query_puts_summary_and_group_summary_in_totals() {
    let mut c1 = GridColumn::new("dept", "Dept", ColumnAlign::Left);
    c1.group_index = 1;
    c1.groupable = true;

    let mut c2 = GridColumn::new("salary", "Salary", ColumnAlign::Right);
    c2.summary = "smSum".into();
    c2.group_summary = "smAvg".into();

    let mut c3 = GridColumn::new("emp_count", "Employees", ColumnAlign::Right);
    c3.summary = "smCount".into();
    c3.group_summary = "smCount".into();

    let state = GridState::new(vec![c1, c2, c3]);
    let query = state.query();

    // query.summary has c2 (Sum) and c3 (Count)
    assert_eq!(query.summary.len(), 2);
    assert_eq!(query.summary[0].column, "salary");
    assert_eq!(query.summary[0].aggregate, datagrid::Aggregate::Sum);
    assert_eq!(query.summary[1].column, "emp_count");
    assert_eq!(query.summary[1].aggregate, datagrid::Aggregate::Count);

    // query.group_by has dept level, whose totals have c2 (Avg) and c3 (Count)
    assert_eq!(query.group_by.len(), 1);
    assert_eq!(query.group_by[0].column, "dept");
    assert_eq!(query.group_by[0].totals.len(), 2);
    assert_eq!(query.group_by[0].totals[0].column, "salary");
    assert_eq!(
        query.group_by[0].totals[0].aggregate,
        datagrid::Aggregate::Avg
    );
    assert_eq!(query.group_by[0].totals[1].column, "emp_count");
    assert_eq!(
        query.group_by[0].totals[1].aggregate,
        datagrid::Aggregate::Count
    );
}

#[test]
fn test_effective_filter_kind() {
    let mut col = GridColumn::new("x", "X", ColumnAlign::Left);

    // Explicit kinds
    col.filter_kind = "fkText".into();
    assert_eq!(effective_filter_kind(&col), "fkText");
    col.filter_kind = "fkList".into();
    assert_eq!(effective_filter_kind(&col), "fkList");
    col.filter_kind = "fkRange".into();
    assert_eq!(effective_filter_kind(&col), "fkRange");
    col.filter_kind = "fkDate".into();
    assert_eq!(effective_filter_kind(&col), "fkDate");

    // fkAuto based on kind
    col.filter_kind = "fkAuto".into();
    col.kind = None;
    assert_eq!(effective_filter_kind(&col), "fkText");
    col.kind = Some("VARCHAR".into());
    assert_eq!(effective_filter_kind(&col), "fkText");
    col.kind = Some("TEXT".into());
    assert_eq!(effective_filter_kind(&col), "fkText");
    col.kind = Some("INTEGER".into());
    assert_eq!(effective_filter_kind(&col), "fkRange");
    col.kind = Some("FLOAT".into());
    assert_eq!(effective_filter_kind(&col), "fkRange");
    col.kind = Some("DECIMAL".into());
    assert_eq!(effective_filter_kind(&col), "fkRange");
    col.kind = Some("DATE".into());
    assert_eq!(effective_filter_kind(&col), "fkDate");
    col.kind = Some("DATETIME".into());
    assert_eq!(effective_filter_kind(&col), "fkDate");
    col.kind = Some("BOOLEAN".into());
    assert_eq!(effective_filter_kind(&col), "fkList");
}

#[test]
fn test_parse_filter_date() {
    assert_eq!(parse_filter_date(""), None);
    assert_eq!(parse_filter_date("invalid"), None);
    assert_eq!(parse_filter_date("2026-05-15"), Some("2026-05-15".into()));
    assert_eq!(parse_filter_date("15/05/2026"), Some("2026-05-15".into()));
    assert_eq!(parse_filter_date("15/05/26"), Some("2026-05-15".into()));
    // Leap year checks
    assert_eq!(parse_filter_date("29/02/2024"), Some("2024-02-29".into()));
    assert_eq!(parse_filter_date("29/02/24"), Some("2024-02-29".into()));
    assert_eq!(parse_filter_date("29/02/2025"), None);
    assert_eq!(parse_filter_date("29/02/25"), None);
    // Bounds checks
    assert_eq!(parse_filter_date("32/01/2026"), None);
    assert_eq!(parse_filter_date("15/13/2026"), None);
}

#[test]
fn test_filter_of_each_kind() {
    let mut col = GridColumn::new("name", "Name", ColumnAlign::Left);

    // Text
    assert_eq!(filter_of(&col, &FilterInput::Text("".into())), None);
    assert_eq!(filter_of(&col, &FilterInput::Text("   ".into())), None);
    assert_eq!(
        filter_of(&col, &FilterInput::Text("alice".into())),
        Some(datagrid::Filter::Contains {
            column: "name".into(),
            text: "alice".into(),
            ignore_case: true,
        })
    );

    // List
    assert_eq!(filter_of(&col, &FilterInput::List(vec![])), None);
    assert_eq!(
        filter_of(
            &col,
            &FilterInput::List(vec![Some("Alice".into()), Some("Bob".into())])
        ),
        Some(datagrid::Filter::In(
            "name".into(),
            vec![
                datagrid::filter::Owned::Text("Alice".into()),
                datagrid::filter::Owned::Text("Bob".into())
            ]
        ))
    );
    // (empty) list entry
    let empty_res = filter_of(&col, &FilterInput::List(vec![None]));
    assert_eq!(
        empty_res,
        Some(datagrid::Filter::Or(vec![
            datagrid::Filter::IsNull("name".into()),
            datagrid::Filter::Eq("name".into(), datagrid::filter::Owned::Text("".into())),
        ]))
    );

    // Range
    col.name = "age".into();
    assert_eq!(
        filter_of(
            &col,
            &FilterInput::Range {
                min: "".into(),
                max: "".into()
            }
        ),
        None
    );
    assert_eq!(
        filter_of(
            &col,
            &FilterInput::Range {
                min: "18".into(),
                max: "".into()
            }
        ),
        Some(datagrid::Filter::Ge(
            "age".into(),
            datagrid::filter::Owned::Float(18.0)
        ))
    );
    assert_eq!(
        filter_of(
            &col,
            &FilterInput::Range {
                min: "".into(),
                max: "65".into()
            }
        ),
        Some(datagrid::Filter::Le(
            "age".into(),
            datagrid::filter::Owned::Float(65.0)
        ))
    );
    assert_eq!(
        filter_of(
            &col,
            &FilterInput::Range {
                min: "18".into(),
                max: "65".into()
            }
        ),
        Some(datagrid::Filter::Between(
            "age".into(),
            datagrid::filter::Owned::Float(18.0),
            datagrid::filter::Owned::Float(65.0)
        ))
    );

    // Date
    col.name = "created".into();
    assert_eq!(
        filter_of(
            &col,
            &FilterInput::Date {
                from: "".into(),
                to: "".into()
            }
        ),
        None
    );
    assert_eq!(
        filter_of(
            &col,
            &FilterInput::Date {
                from: "01/01/2026".into(),
                to: "".into()
            }
        ),
        Some(datagrid::Filter::Ge(
            "created".into(),
            datagrid::filter::Owned::Text("2026-01-01".into())
        ))
    );
    assert_eq!(
        filter_of(
            &col,
            &FilterInput::Date {
                from: "".into(),
                to: "01/01/2026".into()
            }
        ),
        Some(datagrid::Filter::Lt(
            "created".into(),
            datagrid::filter::Owned::Text("2026-01-02".into())
        ))
    );
    assert_eq!(
        filter_of(
            &col,
            &FilterInput::Date {
                from: "01/01/2026".into(),
                to: "05/01/2026".into()
            }
        ),
        Some(datagrid::Filter::And(vec![
            datagrid::Filter::Ge(
                "created".into(),
                datagrid::filter::Owned::Text("2026-01-01".into())
            ),
            datagrid::Filter::Lt(
                "created".into(),
                datagrid::filter::Owned::Text("2026-01-06".into())
            ),
        ]))
    );
}

#[test]
fn test_query_combines_filters_and_query_without() {
    let mut c1 = GridColumn::new("name", "Name", ColumnAlign::Left);
    c1.filterable = true;
    c1.filter_kind = "fkText".into();

    let mut c2 = GridColumn::new("age", "Age", ColumnAlign::Right);
    c2.filterable = true;
    c2.filter_kind = "fkRange".into();

    let mut state = GridState::new(vec![c1, c2]);
    assert!(!state.is_filtered());

    state.filters.insert(0, FilterInput::Text("Alice".into()));
    state.filters.insert(
        1,
        FilterInput::Range {
            min: "20".into(),
            max: "40".into(),
        },
    );

    assert!(state.is_filtered());

    let q = state.query();
    assert_eq!(
        q.filter,
        Some(datagrid::Filter::And(vec![
            datagrid::Filter::Contains {
                column: "name".into(),
                text: "Alice".into(),
                ignore_case: true
            },
            datagrid::Filter::Between(
                "age".into(),
                datagrid::filter::Owned::Float(20.0),
                datagrid::filter::Owned::Float(40.0)
            ),
        ]))
    );

    // query_without(0) leaves only c2
    let qw0 = state.query_without(0);
    assert_eq!(
        qw0.filter,
        Some(datagrid::Filter::Between(
            "age".into(),
            datagrid::filter::Owned::Float(20.0),
            datagrid::filter::Owned::Float(40.0)
        ))
    );

    // query_without(1) leaves only c1
    let qw1 = state.query_without(1);
    assert_eq!(
        qw1.filter,
        Some(datagrid::Filter::Contains {
            column: "name".into(),
            text: "Alice".into(),
            ignore_case: true
        })
    );
}

#[test]
fn test_is_filtered_and_footer() {
    let mut c = GridColumn::new("dept", "Dept", ColumnAlign::Left);
    c.filterable = true;
    let mut state = GridState::new(vec![c]);
    let screen = datagrid::Screen {
        total: Some(100),
        ..Default::default()
    };

    let f0 = state.footer(&screen);
    assert_eq!(f0.position, "Row 1 of 100");

    state.filters.insert(0, FilterInput::Text("Sales".into()));
    let f1 = state.footer(&screen);
    assert_eq!(f1.position, "Row 1 of 100 · filtered");
}

#[test]
fn test_view_filter_row() {
    let mut c1 = GridColumn::new("dept", "Dept", ColumnAlign::Left);
    c1.filterable = true;
    c1.filter_kind = "fkList".into();

    let c2 = GridColumn::new("salary", "Salary", ColumnAlign::Right); // not filterable

    let mut state = GridState::new(vec![c1, c2]);
    let screen = datagrid::Screen::default();
    let view0 = state.view(&screen, Instant::now());

    assert_eq!(view0.filter_row.len(), 2);
    assert_eq!(view0.filter_row[0].column, 0);
    assert_eq!(view0.filter_row[0].kind, "fkList");
    assert!(view0.filter_row[0].enabled);
    assert_eq!(view0.filter_row[0].caption, "All ▼");

    assert_eq!(view0.filter_row[1].column, 1);
    assert!(!view0.filter_row[1].enabled);

    // When list filter is set
    state
        .filters
        .insert(0, FilterInput::List(vec![Some("HR".into()), None]));
    let view1 = state.view(&screen, Instant::now());
    assert_eq!(view1.filter_row[0].caption, "HR, (empty) ▼");
}

#[test]
fn a_checked_list_value_is_typed_like_its_column() {
    let mut col = GridColumn::new("qty", "Qty", ColumnAlign::Right);
    col.kind = Some("INTEGER".into());
    let input = FilterInput::List(vec![Some("3".into())]);
    assert_eq!(
        filter_of(&col, &input),
        Some(datagrid::Filter::In("qty".into(), vec![Owned::Int(3)]))
    );
    col.kind = Some("BOOLEAN".into());
    let input = FilterInput::List(vec![Some("true".into())]);
    assert_eq!(
        filter_of(&col, &input),
        Some(datagrid::Filter::In("qty".into(), vec![Owned::Bool(true)]))
    );
}

/// SQLite reports a DATE column as text; the design's FieldType must win, or
/// the filter row would offer a text box where the owner approved from–to.
#[test]
fn the_design_field_type_beats_the_schema_for_the_filter_kind() {
    let mut doc = crawrad_ir::Document::new("form_main", "Form");
    doc.add_child("form_main", crawrad_ir::Node::new("grid1", "Grid"))
        .unwrap();
    let mut col = crawrad_ir::Node::new("c1", "GridColumn");
    col.props
        .insert("field".into(), crawrad_ir::Value::Str("since".into()));
    col.props
        .insert("field_type".into(), crawrad_ir::Value::Str("DATE".into()));
    col.props
        .insert("filterable".into(), crawrad_ir::Value::Bool(true));
    doc.add_child("grid1", col).unwrap();
    let specs = ColumnSpec::of_grid(&doc, doc.node("grid1").unwrap());
    let schema = [datagrid::ColumnInfo {
        name: "since".into(),
        kind: datagrid::ColumnKind::Text,
        nullable: true,
        hint: datagrid::DisplayHint::Plain,
    }];
    let cols = GridColumn::from_schema_and_design(&schema, &specs);
    assert_eq!(cols[0].kind.as_deref(), Some("DATE"));
    assert_eq!(effective_filter_kind(&cols[0]), "fkDate");
}

#[test]
fn from_design_maps_every_new_field_and_defaults() {
    let spec_default = plain_spec("name");
    let cols_default = GridColumn::from_design(&["name".into()], &[false], &[spec_default]);
    let col0 = &cols_default[0];
    assert!(!col0.read_only);
    assert!(!col0.required);
    assert_eq!(col0.default_value, "");
    assert_eq!(col0.max_length, 0);
    assert_eq!(col0.min_value, 0);
    assert_eq!(col0.max_value, 0);
    assert!(col0.tab_stop);
    assert_eq!(col0.editor, "edAuto");
    assert!(col0.pick_list.is_empty());

    let spec_custom = ColumnSpec {
        read_only: true,
        required: true,
        default_value: "admin".into(),
        max_length: 32,
        min_value: 1,
        max_value: 10,
        tab_stop: false,
        editor: "edList".into(),
        pick_list: vec!["admin".into(), "guest".into()],
        ..plain_spec("role")
    };
    let cols_custom = GridColumn::from_design(&["role".into()], &[false], &[spec_custom]);
    let col1 = &cols_custom[0];
    assert!(col1.read_only);
    assert!(col1.required);
    assert_eq!(col1.default_value, "admin");
    assert_eq!(col1.max_length, 32);
    assert_eq!(col1.min_value, 1);
    assert_eq!(col1.max_value, 10);
    assert!(!col1.tab_stop);
    assert_eq!(col1.editor, "edList");
    assert_eq!(col1.pick_list, &["admin", "guest"]);
}

#[test]
fn effective_editor_derives_correct_kind() {
    let mut col = GridColumn::new("f", "Field", ColumnAlign::Left);
    col.editor = "edDate".into();
    assert_eq!(effective_editor(&col), "edDate");

    col.editor = "edAuto".into();
    col.pick_list = vec!["Option1".into()];
    assert_eq!(effective_editor(&col), "edList");

    col.pick_list.clear();
    col.kind = Some("BOOLEAN".into());
    assert_eq!(effective_editor(&col), "edCheck");

    col.kind = Some("INTEGER".into());
    assert_eq!(effective_editor(&col), "edNumber");

    col.kind = Some("FLOAT".into());
    assert_eq!(effective_editor(&col), "edNumber");

    col.kind = Some("DATE".into());
    assert_eq!(effective_editor(&col), "edDate");

    col.kind = Some("TEXT".into());
    assert_eq!(effective_editor(&col), "edText");
}

#[test]
fn parse_cell_and_validate_cell_rules() {
    let mut col_num = GridColumn::new("price", "Price", ColumnAlign::Right);
    col_num.kind = Some("FLOAT".into());
    col_num.min_value = 10;
    col_num.max_value = 100;
    col_num.required = true;

    assert_eq!(
        validate_cell(&col_num, "   "),
        Err("Price is required".into())
    );

    assert_eq!(
        validate_cell(&col_num, "abc"),
        Err("Price is not a number".into())
    );
    assert_eq!(
        validate_cell(&col_num, "1.234,5"),
        Err("Price is not a number".into())
    );

    assert_eq!(
        validate_cell(&col_num, "5"),
        Err("Price must be at least 10".into())
    );
    assert_eq!(
        validate_cell(&col_num, "150"),
        Err("Price must be at most 100".into())
    );

    assert_eq!(
        validate_cell(&col_num, "50.5"),
        Ok(Some(Owned::Float(50.5)))
    );

    let mut col_int = GridColumn::new("qty", "Quantity", ColumnAlign::Right);
    col_int.kind = Some("INTEGER".into());
    assert_eq!(
        parse_cell(&col_int, "1234.5"),
        Err("Quantity is not a whole number".into())
    );
    assert_eq!(parse_cell(&col_int, "1,234"), Ok(Some(Owned::Int(1234))));

    let mut col_str = GridColumn::new("code", "Code", ColumnAlign::Left);
    col_str.max_length = 3;
    assert_eq!(
        validate_cell(&col_str, "ABCD"),
        Err("Code is longer than 3 characters".into())
    );
    assert_eq!(
        validate_cell(&col_str, "ABC"),
        Ok(Some(Owned::Text("ABC".into())))
    );

    let mut col_date = GridColumn::new("dt", "Date", ColumnAlign::Left);
    col_date.kind = Some("DATE".into());
    assert_eq!(
        parse_cell(&col_date, "invalid"),
        Err("Date is not a date".into())
    );
    assert_eq!(
        parse_cell(&col_date, "31/02/2026"),
        Err("Date is not a date".into())
    );
    assert_eq!(
        parse_cell(&col_date, "15/02/2026"),
        Ok(Some(Owned::Text("2026-02-15".into())))
    );

    let mut col_bool = GridColumn::new("active", "Active", ColumnAlign::Center);
    col_bool.kind = Some("BOOLEAN".into());
    assert_eq!(parse_cell(&col_bool, "true"), Ok(Some(Owned::Bool(true))));
    assert_eq!(parse_cell(&col_bool, "0"), Ok(Some(Owned::Bool(false))));
}

/// A writable grid of text columns `names`.
fn editable_grid(names: &[&str]) -> GridState {
    let columns = names
        .iter()
        .map(|n| GridColumn::new(*n, n.to_uppercase(), ColumnAlign::Left))
        .collect();
    let mut state = GridState::new(columns);
    state.can_write = true;
    state
}

/// Row `index` (source id `id`) holding `raw`, as the app hands it to the grid.
fn row<'a>(index: usize, id: u64, raw: &'a [Option<String>]) -> CurrentRow<'a> {
    CurrentRow {
        index,
        id,
        raw,
        new: false,
    }
}

/// Types `text` into the open editor, the way the app's text box does.
fn type_in(state: &mut GridState, text: &str) {
    state.edit.as_mut().expect("an open editor").editor_text = text.into();
}

#[test]
fn a_read_only_column_or_a_source_that_cannot_write_opens_no_editor() {
    let raw = [Some("1".to_string())];
    let mut state = editable_grid(&["id"]);
    state.columns[0].read_only = true;
    assert_eq!(
        state.edit_key(EditKey::F2, Some(&row(0, 7, &raw)), 1),
        EditOutcome::Ignored
    );
    assert!(state.edit.is_none());

    state.columns[0].read_only = false;
    state.can_write = false;
    assert_eq!(
        state.edit_key(EditKey::F2, Some(&row(0, 7, &raw)), 1),
        EditOutcome::Ignored
    );

    state.can_write = true;
    assert_eq!(
        state.edit_key(EditKey::F2, Some(&row(0, 7, &raw)), 1),
        EditOutcome::Handled
    );
    let edit = state.edit.as_ref().unwrap();
    assert_eq!(
        (edit.open, edit.editor_text.as_str()),
        (Some(0), "1"),
        "F2 starts from the value"
    );
}

#[test]
fn tab_skips_read_only_and_non_tab_stop_columns() {
    let mut state = editable_grid(&["c0", "c1", "c2", "c3"]);
    state.columns[1].read_only = true;
    state.columns[2].tab_stop = false;
    assert_eq!(state.next_editable(0, false), Some(3));
    assert_eq!(state.next_editable(3, true), Some(0));
    assert_eq!(state.next_editable(3, false), None);

    // Tab commits the open editor and opens the next stop on the same row.
    let raw = [
        Some("a".into()),
        Some("b".into()),
        Some("c".into()),
        Some("d".into()),
    ];
    state.edit_key(EditKey::Char("x".into()), Some(&row(0, 1, &raw)), 1);
    state.edit_key(EditKey::Tab { back: false }, Some(&row(0, 1, &raw)), 1);
    assert_eq!(state.focus_col, 3);
    let edit = state.edit.as_ref().unwrap();
    assert_eq!(edit.texts.get(&0).map(String::as_str), Some("x"));
    assert_eq!((edit.open, edit.editor_text.as_str()), (Some(3), "d"));
}

#[test]
fn esc_first_drops_the_typing_then_the_whole_row() {
    let raw = [Some("Ana".to_string()), Some("Natal".to_string())];
    let mut state = editable_grid(&["name", "city"]);
    let r = row(0, 5, &raw);
    state.edit_key(EditKey::F2, Some(&r), 1);
    type_in(&mut state, "Bia");
    state.edit_key(EditKey::Tab { back: false }, Some(&r), 1); // "Bia" kept
    type_in(&mut state, "Recife");
    state.edit_key(EditKey::Escape, Some(&r), 1);
    let edit = state.edit.as_ref().unwrap();
    assert_eq!(edit.open, None);
    assert_eq!(
        edit.texts.len(),
        1,
        "the city typing is gone, the name stays"
    );

    state.edit_key(EditKey::Escape, Some(&r), 1);
    assert!(state.edit.is_none());
    assert_eq!(
        state.edit_key(EditKey::Escape, Some(&r), 1),
        EditOutcome::Ignored
    );
}

#[test]
fn enter_posts_only_the_edited_columns_and_the_write_waits_for_its_answer() {
    let raw = [Some("Ana".to_string()), Some("Natal".to_string())];
    let mut state = editable_grid(&["name", "city"]);
    let r = row(0, 100, &raw);
    state.edit_key(EditKey::F2, Some(&r), 1);
    assert_eq!(
        state.edit_key(EditKey::Enter, Some(&r), 1),
        EditOutcome::Handled,
        "unchanged"
    );
    assert!(state.edit.is_none());

    state.edit_key(EditKey::Char("B".into()), Some(&r), 1);
    let EditOutcome::Write(write) = state.edit_key(EditKey::Enter, Some(&r), 1) else {
        panic!("a write");
    };
    assert_eq!(
        write,
        datagrid::Write::Update {
            id: 100,
            changes: vec![("name".into(), Some(Owned::Text("B".into())))],
        }
    );
    assert!(state.edit.is_none() && state.in_flight.is_some());

    state.sent(9);
    state.on_written(&datagrid::Written {
        ticket: 9,
        result: Ok(100),
    });
    assert!(state.in_flight.is_none() && state.edit.is_none());
}

#[test]
fn a_refused_write_brings_the_row_back_with_the_message() {
    let raw = [Some("Ana".to_string())];
    let mut state = editable_grid(&["name"]);
    state.rec_no = 3;
    state.edit_key(EditKey::Char("X".into()), Some(&row(3, 30, &raw)), 10);
    let EditOutcome::Write(_) = state.edit_key(EditKey::Down, Some(&row(3, 30, &raw)), 10) else {
        panic!("leaving the row posts it");
    };
    assert_eq!(state.rec_no, 4, "the user moved on");
    state.sent(1);
    state.on_written(&datagrid::Written {
        ticket: 2,
        result: Err("other".into()),
    });
    assert!(state.edit.is_none(), "another ticket is not ours");
    state.on_written(&datagrid::Written {
        ticket: 1,
        result: Err("locked".into()),
    });
    let edit = state.edit.as_ref().unwrap();
    assert_eq!(edit.error, Some((0, "locked".into())));
    assert_eq!(edit.texts.get(&0).map(String::as_str), Some("X"));
    assert_eq!(state.rec_no, 3, "back on the row in error");
}

#[test]
fn an_invalid_row_keeps_the_user_on_it() {
    let raw = [Some("5".to_string())];
    let mut state = editable_grid(&["qty"]);
    state.columns[0].kind = Some("INTEGER".into());
    state.columns[0].max_value = 10;
    state.edit_key(EditKey::Char("99".into()), Some(&row(0, 1, &raw)), 3);
    assert_eq!(
        state.edit_key(EditKey::Down, Some(&row(0, 1, &raw)), 3),
        EditOutcome::Handled
    );
    assert_eq!(state.rec_no, 0);
    let edit = state.edit.as_ref().unwrap();
    assert_eq!(edit.error, Some((0, "QTY must be at most 10".into())));
    assert_eq!(
        (edit.open, edit.editor_text.as_str()),
        (Some(0), "99"),
        "the editor reopens on it"
    );
}

#[test]
fn a_new_row_starts_with_the_defaults_and_inserts_only_what_it_has() {
    let mut state = editable_grid(&["name", "city", "note"]);
    state.columns[0].required = true;
    state.columns[1].default_value = "Natal".into();
    assert_eq!(
        state.edit_key(EditKey::Insert, None, 4),
        EditOutcome::Handled
    );
    assert_eq!(state.rec_no, 4);
    assert_eq!(state.edit.as_ref().unwrap().open, Some(0));

    assert!(matches!(
        state.edit_key(EditKey::Enter, None, 4),
        EditOutcome::Handled
    ));
    assert_eq!(
        state.edit.as_ref().unwrap().error,
        Some((0, "NAME is required".into())),
        "a Required column is checked though never edited"
    );

    type_in(&mut state, "Bia");
    let EditOutcome::Write(write) = state.edit_key(EditKey::Enter, None, 4) else {
        panic!("an insert");
    };
    assert_eq!(
        write,
        datagrid::Write::Insert(vec![
            ("name".into(), Some(Owned::Text("Bia".into()))),
            ("city".into(), Some(Owned::Text("Natal".into()))),
        ]),
        "the empty note is left to the database default"
    );
}

#[test]
fn down_on_the_last_row_adds_a_row_and_esc_takes_it_away() {
    let raw = [Some("Ana".to_string())];
    let mut state = editable_grid(&["name"]);
    state.rec_no = 1;
    assert_eq!(
        state.edit_key(EditKey::Down, Some(&row(1, 2, &raw)), 2),
        EditOutcome::Handled
    );
    assert!(state.edit.as_ref().is_some_and(|e| e.index.is_none()));
    assert_eq!(state.rec_no, 2);
    state.edit_key(EditKey::Escape, None, 2);
    state.edit_key(EditKey::Escape, None, 2);
    assert!(state.edit.is_none());
    assert_eq!(state.rec_no, 1);
}

#[test]
fn a_check_column_flips_on_space_or_a_click_and_has_no_text_box() {
    let raw = [Some("true".to_string())];
    let mut state = editable_grid(&["active"]);
    state.columns[0].kind = Some("BOOLEAN".into());
    state.edit_key(EditKey::Char(" ".into()), Some(&row(0, 1, &raw)), 1);
    let edit = state.edit.as_ref().unwrap();
    assert_eq!(
        (edit.open, edit.texts.get(&0).map(String::as_str)),
        (None, Some("false"))
    );
    assert_eq!(
        state.edit_key(EditKey::Char("x".into()), Some(&row(0, 1, &raw)), 1),
        EditOutcome::Ignored
    );

    // A click on the row the user is on flips it back.
    state.click_cell(&row(0, 1, &raw), 0);
    assert_eq!(state.edited_text(0), Some("true"));
}

#[test]
fn a_click_on_another_row_posts_the_edited_one_and_moves() {
    let raw0 = [Some("Ana".to_string())];
    let raw1 = [Some("Bia".to_string())];
    let mut state = editable_grid(&["name"]);
    state.rec_no = 9;
    assert_eq!(state.click_cell(&row(0, 1, &raw0), 0), EditOutcome::Handled);
    assert!(state.edit.is_none(), "the first click only selects");
    state.click_cell(&row(0, 1, &raw0), 0);
    type_in(&mut state, "Ada");
    let EditOutcome::Write(_) = state.click_cell(&row(1, 2, &raw1), 0) else {
        panic!("leaving posts");
    };
    assert_eq!(state.rec_no, 1);
    assert!(state.edit.is_none());
}

#[test]
fn view_shows_gutter_markers_and_new_row_line() {
    let mut state = editable_grid(&["name"]);
    let screen = datagrid::Screen {
        lines: vec![datagrid::ScreenLine::Row {
            index: 0,
            id: 1,
            cells: vec![Some("Alice".into())],
        }],
        total: Some(1),
        ..Default::default()
    };
    let view0 = state.view(&screen, Instant::now());
    assert!(view0.gutter);
    assert!(matches!(&view0.lines[0], ViewLine::Row { marker: "", .. }));
    assert_eq!(view0.status_text, "");

    let raw = [Some("Alice".to_string())];
    state.edit_key(EditKey::Char("B".into()), Some(&row(0, 1, &raw)), 1);
    state.edit_key(EditKey::Tab { back: true }, Some(&row(0, 1, &raw)), 1);
    let view1 = state.view(&screen, Instant::now());
    assert!(matches!(&view1.lines[0], ViewLine::Row { marker: "✎", .. }));
    assert_eq!(view1.open_editor, Some(0));
    assert_eq!(
        view1.status_text,
        "editing — Enter or row change saves, Esc undoes the row"
    );

    state.edit_key(EditKey::Escape, None, 1);
    state.edit_key(EditKey::Escape, None, 1);
    state.edit_key(EditKey::Insert, None, 1);
    let view2 = state.view(&screen, Instant::now());
    assert_eq!(view2.lines.len(), 2);
    assert!(matches!(
        &view2.lines[1],
        ViewLine::Row {
            marker: "*",
            selected: true,
            ..
        }
    ));

    state.can_write = false;
    assert!(!state.view(&screen, Instant::now()).gutter);
}

/// A column spec with every catalogue default, for `field`.
fn plain_spec(field: &str) -> ColumnSpec {
    ColumnSpec {
        field: field.to_string(),
        title: "".to_string(),
        width: 0,
        align: "taLeftJustify".to_string(),
        display_format: "".to_string(),
        visible: true,
        null_text: "".to_string(),
        boolean_texts: "".to_string(),
        value_map: "".to_string(),
        renderer: "crText".to_string(),
        title_alignment: "taLeftJustify".to_string(),
        min_width: 0,
        max_width: 0,
        color: Color::transparent(),
        text_color: Color::transparent(),
        title_color: Color::transparent(),
        vertical_alignment: "tlCenter".to_string(),
        padding: Edges::uniform(0.0),
        ellipsis: false,
        word_wrap: false,
        title_word_wrap: false,
        title_hint: String::new(),
        cell_hint: false,
        title_font: Font {
            name: String::new(),
            size: 0.0,
            ..Default::default()
        },
        font: Font {
            name: String::new(),
            size: 0.0,
            ..Default::default()
        },
        auto_size: false,
        resizable: true,
        movable: true,
        sortable: true,
        sort_order: "soNone".to_string(),
        sort_priority: 0,
        nulls_position: "npLast".to_string(),
        groupable: true,
        group_index: 0,
        summary: "".to_string(),
        summary_format: "".to_string(),
        group_summary: "".to_string(),
        filterable: false,
        filter_kind: "fkAuto".to_string(),
        field_type: "UNKNOWN".into(),
        read_only: false,
        required: false,
        default_value: String::new(),
        max_length: 0,
        min_value: 0,
        max_value: 0,
        tab_stop: true,
        editor: "edAuto".into(),
        pick_list: Vec::new(),
        edit_mask: String::new(),
        get_text_hook: false,
        set_text_hook: false,
        validate_hook: false,
        lookup_key: String::new(),
        lookup_display: Vec::new(),
        button_hook: false,
        drop_down_count: 8,
        draw_cell_hook: false,
        style_rules: Vec::new(),
    }
}

/// A writable source keyed by `id`.
struct KeyedSource;

impl datagrid::RowSource for KeyedSource {
    fn schema(&self) -> &[datagrid::ColumnInfo] {
        &[]
    }
    fn capabilities(&self) -> datagrid::Capabilities {
        datagrid::Capabilities::default()
    }
    fn fetch(
        &self,
        _: &datagrid::Query,
        _: std::ops::Range<usize>,
    ) -> Result<datagrid::Rows, datagrid::SourceError> {
        Err(datagrid::SourceError::Unsupported("fetch"))
    }
    fn count(&self, _: &datagrid::Query) -> Result<usize, datagrid::SourceError> {
        Ok(0)
    }
    fn can_write(&self) -> bool {
        true
    }
    fn key_column(&self) -> Option<&str> {
        Some("id")
    }
}

#[test]
fn the_source_key_column_is_never_edited() {
    let mut state = editable_grid(&["ID", "name"]);
    state.can_write = false;
    state.writes_through(&KeyedSource);
    assert!(state.can_write);
    assert!(state.columns[0].read_only && !state.columns[1].read_only);
    assert_eq!(
        state.edit_key(EditKey::Insert, None, 3),
        EditOutcome::Handled
    );
    assert_eq!(
        state.edit.as_ref().unwrap().open,
        Some(1),
        "the new row starts past the key"
    );
}

#[test]
fn row_values_case_insensitive_get_and_fields_iteration() {
    let mut state = editable_grid(&["id", "Name", "City"]);
    let raw = [
        Some("10".to_string()),
        Some("Alice".to_string()),
        Some("Recife".to_string()),
    ];
    let cr = row(0, 10, &raw);
    let rv = state.row_values(0, 10, Some(&raw));
    assert_eq!(rv.get("id"), Some("10"));
    assert_eq!(rv.get("ID"), Some("10"));
    assert_eq!(rv.get("name"), Some("Alice"));
    assert_eq!(rv.get("NAME"), Some("Alice"));
    assert_eq!(rv.get("City"), Some("Recife"));
    assert_eq!(rv.get("city"), Some("Recife"));
    assert_eq!(rv.get("unknown"), None);
    let fields: Vec<_> = rv.fields().collect();
    assert_eq!(fields, vec!["id", "Name", "City"]);

    // When row is being edited, edited buffer values apply to RowValues.
    state.edit_key(EditKey::F2, Some(&cr), 1);
    type_in(&mut state, "11");
    state.edit_key(EditKey::Tab { back: false }, Some(&cr), 1);
    type_in(&mut state, "Alicia");
    // the open editor reaches the buffer when it is committed (post commits first)
    state.commit_editor();
    let rv_edited = state.row_values(0, 10, Some(&raw));
    assert_eq!(rv_edited.get("id"), Some("11"));
    assert_eq!(rv_edited.get("name"), Some("Alicia"));
    assert_eq!(rv_edited.get("city"), Some("Recife"));
}

#[test]
fn text_requests_and_set_texts_reach_the_view() {
    let mut state = editable_grid(&["id", "code", "desc"]);
    state.columns[1].get_text_hook = true; // only code has hook
    let screen = datagrid::Screen {
        lines: vec![
            datagrid::ScreenLine::Row {
                index: 0,
                id: 1,
                cells: vec![Some("1".into()), Some("A100".into()), Some("Item A".into())],
            },
            datagrid::ScreenLine::Row {
                index: 1,
                id: 2,
                cells: vec![Some("2".into()), Some("B200".into()), Some("Item B".into())],
            },
        ],
        total: Some(2),
        ..Default::default()
    };

    // First call returns requests for column 1 only for each visible row.
    let mut reqs = state.text_requests(&screen);
    assert_eq!(reqs.len(), 2);
    assert_eq!(reqs[0].id, 1);
    assert_eq!(reqs[0].column, 1);
    assert_eq!(reqs[0].field, "code");
    assert_eq!(reqs[0].text, "A100");
    assert_eq!(reqs[1].id, 2);
    assert_eq!(reqs[1].column, 1);
    assert_eq!(reqs[1].field, "code");
    assert_eq!(reqs[1].text, "B200");

    // Second call for the same screen returns empty (generation counter).
    assert!(state.text_requests(&screen).is_empty());

    // Answer requests and set them.
    reqs[0].text = "FORMATTED-A100".to_string();
    reqs[1].text = "FORMATTED-B200".to_string();
    state.set_texts(reqs);

    // View uses the answered texts for the cell.
    let view = state.view(&screen, Instant::now());
    let ViewLine::Row { cells: c0, .. } = &view.lines[0] else {
        panic!("expected row 0");
    };
    assert_eq!(c0[1], "FORMATTED-A100");
    assert_eq!(c0[0], "1");
    let ViewLine::Row { cells: c1, .. } = &view.lines[1] else {
        panic!("expected row 1");
    };
    assert_eq!(c1[1], "FORMATTED-B200");
    assert_eq!(c1[0], "2");
}

#[test]
fn post_returns_check_only_with_hooks_and_checked_handles_errors_and_rewrites() {
    let raw = [Some("10".to_string()), Some("Alice".to_string())];
    let mut state = editable_grid(&["id", "name"]);
    state.columns[1].set_text_hook = true;
    state.columns[1].validate_hook = true;

    let cr = row(0, 10, &raw);
    state.edit_key(EditKey::F2, Some(&cr), 1);
    // Tab to name (column 1)
    state.edit_key(EditKey::Tab { back: false }, Some(&cr), 1);
    type_in(&mut state, "bob");

    // Enter triggers post -> EditOutcome::Check
    let outcome = state.edit_key(EditKey::Enter, Some(&cr), 1);
    let EditOutcome::Check(mut check) = outcome else {
        panic!("expected Check outcome, got {:?}", outcome);
    };
    assert_eq!(check.cells.len(), 1);
    assert_eq!(check.cells[0].field, "name");
    assert_eq!(check.cells[0].text, "bob");
    assert!(check.cells[0].set_text);
    assert!(check.cells[0].validate);

    // If check returns with an error, checked() sets error and returns Handled.
    let mut bad_check = check.clone();
    bad_check.cells[0].error = "Name cannot be bob".to_string();
    let handled = state.checked(bad_check);
    assert_eq!(handled, EditOutcome::Handled);
    assert_eq!(
        state.edit.as_ref().unwrap().error,
        Some((1, "Name cannot be bob".to_string()))
    );

    // If check rewrites text and has no error, checked() writes rewritten value.
    check.cells[0].text = "Bob".to_string();
    let clean = state.checked(check);
    let EditOutcome::Write(datagrid::Write::Update { changes, .. }) = clean else {
        panic!("expected clean Write outcome, got {:?}", clean);
    };
    assert_eq!(
        changes,
        vec![("name".to_string(), Some(Owned::Text("Bob".into())))]
    );

    // When no columns have hooks, post returns Write directly without Check.
    let mut unhooked_state = editable_grid(&["id", "name"]);
    let cr2 = row(0, 20, &raw);
    unhooked_state.edit_key(EditKey::F2, Some(&cr2), 1);
    type_in(&mut unhooked_state, "21");
    let unhooked_outcome = unhooked_state.edit_key(EditKey::Enter, Some(&cr2), 1);
    assert!(matches!(unhooked_outcome, EditOutcome::Write(_)));
}

#[test]
fn edit_mask_display_validation_and_written_value() {
    let mut col = GridColumn::new("phone", "Phone", ColumnAlign::Left);
    col.edit_mask = "(00) 0000-0000".to_string();

    // Display formatted cell text
    assert_eq!(cell_text(&col, None, Some("1199998888")), "(11) 9999-8888");
    assert_eq!(cell_text(&col, None, None), "");

    // Incomplete mask returns validation error right after Required
    assert_eq!(
        validate_cell(&col, "11999"),
        Err("Phone is incomplete".to_string())
    );

    // Complete mask parses and writes EditMask::text(shown)
    // the mask decides what is written: without `;0` it keeps its literals ...
    assert_eq!(
        validate_cell(&col, "(11) 9999-8888"),
        Ok(Some(Owned::Text("(11) 9999-8888".into())))
    );
    // ... with `;0` only what was typed
    col.edit_mask = "(00) 0000-0000;0;_".to_string();
    assert_eq!(
        validate_cell(&col, "(11) 9999-8888"),
        Ok(Some(Owned::Text("1199998888".into())))
    );
}

struct MockLookup {
    calls: std::sync::atomic::AtomicUsize,
    data: Vec<(datagrid::filter::Owned, Vec<String>)>,
}

impl MockLookup {
    fn new(data: Vec<(datagrid::filter::Owned, Vec<String>)>) -> Self {
        Self {
            calls: std::sync::atomic::AtomicUsize::new(0),
            data,
        }
    }
}

impl datagrid::LookupSource for MockLookup {
    fn labels(
        &self,
        keys: &[datagrid::filter::Owned],
    ) -> Result<Vec<(datagrid::filter::Owned, Vec<String>)>, datagrid::SourceError> {
        self.calls.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        let mut res = Vec::new();
        for k in keys {
            if let Some((_, disp)) = self.data.iter().find(|(dk, _)| dk == k) {
                res.push((k.clone(), disp.clone()));
            }
        }
        Ok(res)
    }

    fn search(
        &self,
        prefix: &str,
        limit: usize,
    ) -> Result<Vec<(datagrid::filter::Owned, Vec<String>)>, datagrid::SourceError> {
        let mut res = Vec::new();
        for (k, disp) in &self.data {
            if disp
                .first()
                .is_some_and(|d| d.to_lowercase().starts_with(&prefix.to_lowercase()))
            {
                res.push((k.clone(), disp.clone()));
                if res.len() >= limit {
                    break;
                }
            }
        }
        Ok(res)
    }
}

#[test]
fn effective_editor_fallbacks_for_lookup_and_button() {
    let mut col_lookup = GridColumn::new("country_id", "Country", ColumnAlign::Left);
    col_lookup.editor = "edLookup".into();
    // edLookup without lookup -> edText
    assert_eq!(effective_editor(&col_lookup), "edText");

    // edLookup with lookup but no key or display -> edText
    let mock = std::sync::Arc::new(MockLookup::new(Vec::new()));
    col_lookup.lookup = LookupHandle(Some(mock.clone()));
    assert_eq!(effective_editor(&col_lookup), "edText");

    col_lookup.lookup_key = "id".into();
    assert_eq!(effective_editor(&col_lookup), "edText");

    col_lookup.lookup_display = vec!["name".into()];
    assert_eq!(effective_editor(&col_lookup), "edLookup");

    let mut col_button = GridColumn::new("calc", "Calculate", ColumnAlign::Left);
    col_button.editor = "edButton".into();
    // edButton without button_hook -> edText ("no dead button")
    assert_eq!(effective_editor(&col_button), "edText");

    col_button.button_hook = true;
    assert_eq!(effective_editor(&col_button), "edButton");
}

#[test]
fn lookup_label_cache_one_call_per_screen_and_raw_key_on_miss() {
    let mock = std::sync::Arc::new(MockLookup::new(vec![
        (datagrid::filter::Owned::Int(1), vec!["Brazil".into()]),
        (datagrid::filter::Owned::Int(2), vec!["Canada".into()]),
    ]));

    let mut col = GridColumn::new("country_id", "Country", ColumnAlign::Left);
    col.editor = "edLookup".into();
    col.kind = Some("INTEGER".into());
    col.lookup_key = "id".into();
    col.lookup_display = vec!["name".into()];
    col.lookup = LookupHandle(Some(mock.clone()));

    let mut state = GridState::new(vec![col]);

    let screen = datagrid::Screen {
        first: 0,
        total: Some(3),
        lines: vec![
            datagrid::ScreenLine::Row {
                index: 0,
                id: 10,
                cells: vec![Some("1".into())],
            },
            datagrid::ScreenLine::Row {
                index: 1,
                id: 20,
                cells: vec![Some("2".into())],
            },
            datagrid::ScreenLine::Row {
                index: 2,
                id: 30,
                cells: vec![Some("99".into())], // missing key
            },
        ],
        summary: Vec::new(),
        distinct: None,
        busy: None,
        error: None,
        pending: None,
        ..Default::default()
    };

    // The first drawing fills the cache; drawing the same screen again asks nothing
    // — and no OnGetText is needed for it.
    state.update_lookups(&screen);
    state.update_lookups(&screen);
    assert_eq!(mock.calls.load(std::sync::atomic::Ordering::SeqCst), 1);

    // Check rendered lines in view
    let view = state.view(&screen, std::time::Instant::now());
    let ViewLine::Row { cells: r0, .. } = &view.lines[0] else {
        panic!()
    };
    assert_eq!(r0[0], "Brazil");
    let ViewLine::Row { cells: r1, .. } = &view.lines[1] else {
        panic!()
    };
    assert_eq!(r1[0], "Canada");
    // Missing key shows raw key
    let ViewLine::Row { cells: r2, .. } = &view.lines[2] else {
        panic!()
    };
    assert_eq!(r2[0], "99");

    // Second call on same screen makes no extra labels calls
    state.text_requests(&screen);
    assert_eq!(mock.calls.load(std::sync::atomic::Ordering::SeqCst), 1);
}

#[test]
fn lookup_search_highlight_and_pick_storing_key() {
    let mock = std::sync::Arc::new(MockLookup::new(vec![
        (datagrid::filter::Owned::Int(10), vec!["Alpha".into()]),
        (datagrid::filter::Owned::Int(20), vec!["Beta".into()]),
    ]));

    let mut col = GridColumn::new("item_id", "Item", ColumnAlign::Left);
    col.editor = "edLookup".into();
    col.kind = Some("INTEGER".into());
    col.lookup_key = "id".into();
    col.lookup_display = vec!["name".into()];
    col.lookup = LookupHandle(Some(mock));

    let mut state = GridState::new(vec![col]);
    state.can_write = true;

    let raw = [Some("10".to_string())];
    let cr = CurrentRow {
        index: 0,
        id: 1,
        raw: &raw,
        new: false,
    };

    // Open cell with F2
    state.edit_key(EditKey::F2, Some(&cr), 1);
    let edit = state.edit.as_ref().unwrap();
    assert!(edit.lookup.is_some());
    let lookup_list = edit.lookup.as_ref().unwrap();
    assert_eq!(lookup_list.rows.len(), 2);
    assert_eq!(lookup_list.highlighted, 0);

    // Down moves highlighted to 1 (Beta)
    state.edit_key(EditKey::Down, Some(&cr), 1);
    assert_eq!(
        state
            .edit
            .as_ref()
            .unwrap()
            .lookup
            .as_ref()
            .unwrap()
            .highlighted,
        1
    );

    // Enter picks item 20 ("Beta")
    state.edit_key(EditKey::Enter, Some(&cr), 1);
    let edit_after = state.edit.as_ref().unwrap();
    assert!(edit_after.lookup.is_none());
    assert_eq!(edit_after.texts.get(&0), Some(&"20".to_string()));
    assert_eq!(edit_after.editor_text, "Beta");

    // Buffer row shows the label of the key chosen
    let line = state.row_line(0, 1, Some(&raw), "✎");
    let ViewLine::Row { cells, .. } = line else {
        panic!()
    };
    assert_eq!(cells[0], "Beta");
}

#[test]
fn a_picked_lookup_writes_its_key_not_the_label_it_shows() {
    let mock = std::sync::Arc::new(MockLookup::new(vec![
        (datagrid::filter::Owned::Int(10), vec!["Alpha".into()]),
        (datagrid::filter::Owned::Int(20), vec!["Beta".into()]),
    ]));
    let mut col = GridColumn::new("item_id", "Item", ColumnAlign::Left);
    col.editor = "edLookup".into();
    col.kind = Some("INTEGER".into());
    col.lookup_key = "id".into();
    col.lookup_display = vec!["name".into()];
    col.lookup = LookupHandle(Some(mock));
    let mut state = GridState::new(vec![col]);
    state.can_write = true;
    let raw = [Some("10".to_string())];
    let cr = row(0, 1, &raw);

    state.edit_key(EditKey::F2, Some(&cr), 1);
    state.edit_key(EditKey::Down, Some(&cr), 1);
    state.edit_key(EditKey::Enter, Some(&cr), 1); // picks Beta
    let outcome = state.edit_key(EditKey::Enter, Some(&cr), 1); // posts the row
    let EditOutcome::Write(datagrid::Write::Update { changes, .. }) = outcome else {
        panic!("expected a write, got {outcome:?}");
    };
    assert_eq!(changes, vec![("item_id".to_string(), Some(Owned::Int(20)))]);
}

#[test]
fn lookup_esc_closes_list_first_then_undoes_row() {
    let mock = std::sync::Arc::new(MockLookup::new(vec![(
        datagrid::filter::Owned::Int(1),
        vec!["One".into()],
    )]));

    let mut col = GridColumn::new("num", "Num", ColumnAlign::Left);
    col.editor = "edLookup".into();
    col.kind = Some("INTEGER".into());
    col.lookup_key = "id".into();
    col.lookup_display = vec!["name".into()];
    col.lookup = LookupHandle(Some(mock));

    let mut state = GridState::new(vec![col]);
    state.can_write = true;

    let raw = [Some("1".to_string())];
    let cr = CurrentRow {
        index: 0,
        id: 1,
        raw: &raw,
        new: false,
    };

    state.edit_key(EditKey::F2, Some(&cr), 1);
    assert!(state.edit.as_ref().unwrap().lookup.is_some());
    assert_eq!(state.edit.as_ref().unwrap().open, Some(0));

    // First Esc closes the lookup list only
    state.edit_key(EditKey::Escape, Some(&cr), 1);
    assert!(state.edit.as_ref().unwrap().lookup.is_none());
    assert_eq!(state.edit.as_ref().unwrap().open, Some(0));

    // Second Esc closes editor
    state.edit_key(EditKey::Escape, Some(&cr), 1);
    assert!(state.edit.as_ref().unwrap().open.is_none());

    // Third Esc drops the row edit buffer
    state.edit_key(EditKey::Escape, Some(&cr), 1);
    assert!(state.edit.is_none());
}

#[test]
fn button_request_round_trip() {
    let mut col = GridColumn::new("action", "Action", ColumnAlign::Left);
    col.editor = "edButton".into();
    col.button_hook = true;

    let mut state = GridState::new(vec![col]);
    state.can_write = true;

    let raw = [Some("initial".to_string())];
    let cr = CurrentRow {
        index: 0,
        id: 100,
        raw: &raw,
        new: false,
    };

    state.edit_key(EditKey::F2, Some(&cr), 1);
    assert_eq!(state.edit.as_ref().unwrap().open, Some(0));

    // CtrlEnter on edButton returns Button outcome
    let outcome = state.edit_key(EditKey::CtrlEnter, Some(&cr), 1);
    let EditOutcome::Button(mut req) = outcome else {
        panic!("expected Button outcome, got {:?}", outcome);
    };
    assert_eq!(req.column, 0);
    assert_eq!(req.field, "action");
    assert_eq!(req.text, "initial");

    // Simulating user handler modifying text and calling button_done
    req.text = "computed_value".to_string();
    state.button_done(req);

    let edit = state.edit.as_ref().unwrap();
    assert_eq!(edit.open, Some(0));
    assert_eq!(edit.editor_text, "computed_value");
    assert_eq!(edit.texts.get(&0), Some(&"computed_value".to_string()));
}

#[test]
fn style_rule_first_match_order_and_transparent_colors_none() {
    let mut col = GridColumn::new("balance", "Balance", ColumnAlign::Right);
    let rule1 = StyleRule {
        when: crawrad_ir::style_rule::parse("balance < 0").expect("valid rule"),
        color: Some(Color::rgb(255, 0, 0)),
        text_color: None, // transparent -> None
        bold: true,
        italic: false,
    };
    let rule2 = StyleRule {
        when: crawrad_ir::style_rule::parse("balance < 100").expect("valid rule"),
        color: Some(Color::rgb(255, 255, 0)),
        text_color: Some(Color::rgb(0, 0, 0)),
        bold: false,
        italic: true,
    };
    col.style_rules = vec![rule1, rule2];

    let row_negative = RowValues::new(vec![("balance".to_string(), Some("-50".to_string()))]);
    let style_neg = rule_style(&col, &row_negative, "-50.00").expect("rule matches");
    assert_eq!(style_neg.color, Some(Color::rgb(255, 0, 0)));
    assert_eq!(style_neg.text_color, None); // transparent color was None
    assert!(style_neg.bold);
    assert!(!style_neg.italic);
    assert_eq!(style_neg.text, "-50.00");

    let row_positive = RowValues::new(vec![("balance".to_string(), Some("50".to_string()))]);
    let style_pos = rule_style(&col, &row_positive, "50.00").expect("second rule matches");
    assert_eq!(style_pos.color, Some(Color::rgb(255, 255, 0)));
    assert_eq!(style_pos.text_color, Some(Color::rgb(0, 0, 0)));
    assert!(!style_pos.bold);
    assert!(style_pos.italic);

    let row_large = RowValues::new(vec![("balance".to_string(), Some("500".to_string()))]);
    assert_eq!(rule_style(&col, &row_large, "500.00"), None);
}

#[test]
fn on_draw_cell_answer_overrides_rule_style() {
    let mut col = GridColumn::new("score", "Score", ColumnAlign::Right);
    col.draw_cell_hook = true;
    col.style_rules = vec![StyleRule {
        when: crawrad_ir::style_rule::parse("score > 50").expect("valid rule"),
        color: Some(Color::rgb(0, 255, 0)),
        text_color: None,
        bold: true,
        italic: false,
    }];

    let mut state = GridState::new(vec![col]);
    let screen = datagrid::Screen {
        lines: vec![datagrid::ScreenLine::Row {
            index: 0,
            id: 10,
            cells: vec![Some("85".to_string())],
        }],
        total: Some(1),
        ..Default::default()
    };

    let mut reqs = state.style_requests(&screen);
    assert_eq!(reqs.len(), 1);
    assert_eq!(reqs[0].style.color, Some(Color::rgb(0, 255, 0)));
    assert!(reqs[0].style.bold);

    // Prior to answering, cell_style returns the rule style
    let initial_style = state.cell_style(10, 0).expect("rule style present");
    assert_eq!(initial_style.color, Some(Color::rgb(0, 255, 0)));

    // User handler overrides the style
    reqs[0].style.color = Some(Color::rgb(0, 0, 255));
    reqs[0].style.text = "85%".to_string();
    reqs[0].style.italic = true;
    state.set_styles(reqs);

    let overridden = state.cell_style(10, 0).expect("custom style present");
    assert_eq!(overridden.color, Some(Color::rgb(0, 0, 255)));
    assert_eq!(overridden.text, "85%");
    assert!(overridden.bold);
    assert!(overridden.italic);
}

#[test]
fn no_rule_and_no_hook_returns_none() {
    let col = GridColumn::new("notes", "Notes", ColumnAlign::Left);
    let mut state = GridState::new(vec![col]);
    let screen = datagrid::Screen {
        lines: vec![datagrid::ScreenLine::Row {
            index: 0,
            id: 1,
            cells: vec![Some("plain text".to_string())],
        }],
        total: Some(1),
        ..Default::default()
    };
    let reqs = state.style_requests(&screen);
    assert!(reqs.is_empty());
    assert_eq!(state.cell_style(1, 0), None);
}

#[test]
fn an_edited_value_is_styled_again_without_the_row_moving() {
    let mut col = GridColumn::new("balance", "Balance", ColumnAlign::Right);
    col.style_rules = vec![StyleRule {
        when: crawrad_ir::style_rule::parse("balance < 0").expect("valid rule"),
        color: None,
        text_color: Some(Color::rgb(224, 108, 117)),
        bold: true,
        italic: false,
    }];
    let mut state = GridState::new(vec![col]);
    let screen_with = |value: &str| datagrid::Screen {
        lines: vec![datagrid::ScreenLine::Row {
            index: 0,
            id: 7,
            cells: vec![Some(value.to_string())],
        }],
        total: Some(1),
        ..Default::default()
    };

    // Rules alone (no OnDrawCell): the widget's refresh is all it takes.
    let positive = screen_with("5");
    assert!(!state.has_stale_screen(&positive));
    state.refresh_rule_styles(&positive);
    assert_eq!(state.cell_style(7, 0), None);

    // Same row, same place — only the value changed after an edit.
    let negative = screen_with("-3");
    state.refresh_rule_styles(&negative);
    assert!(
        state
            .cell_style(7, 0)
            .is_some_and(|s| s.bold && s.text == "-3")
    );

    // OnGetText's answer is the text the styled cell shows.
    state.set_texts(vec![TextRequest {
        id: 7,
        column: 0,
        field: "balance".to_string(),
        row: RowValues::new(Vec::new()),
        text: "(3)".to_string(),
    }]);
    assert_eq!(state.cell_style(7, 0).map(|s| s.text.as_str()), Some("(3)"));
}

#[test]
fn on_draw_cell_is_asked_once_per_change_of_the_visible_rows() {
    let mut col = GridColumn::new("balance", "Balance", ColumnAlign::Right);
    col.draw_cell_hook = true;
    let mut state = GridState::new(vec![col]);
    let screen = datagrid::Screen {
        lines: vec![datagrid::ScreenLine::Row {
            index: 0,
            id: 7,
            cells: vec![Some("5".to_string())],
        }],
        total: Some(1),
        ..Default::default()
    };
    // The widget painted first: the rules are fresh but OnDrawCell was not asked yet.
    state.refresh_rule_styles(&screen);
    assert!(state.has_stale_screen(&screen));
    assert_eq!(state.style_requests(&screen).len(), 1);
    assert!(!state.has_stale_screen(&screen));
    assert!(state.style_requests(&screen).is_empty());
}

#[test]
fn design_cell_style_evaluates_rules_on_sample_rows() {
    let mut col = GridColumn::new("balance", "Balance", ColumnAlign::Right);
    let when = crawrad_ir::style_rule::parse("balance < 0").expect("valid rule");
    col.style_rules.push(StyleRule {
        when,
        color: None,
        text_color: Some(Color::rgb(255, 0, 0)),
        bold: true,
        italic: false,
    });
    let header = vec!["balance".to_string()];
    let row_negative = vec![Some("-120.50".to_string())];
    let row_positive = vec![Some("3.70".to_string())];

    let styled = design_cell_style(&col, &header, &row_negative, "-120.50");
    assert!(styled.is_some_and(|s| s.bold
        && s.text == "-120.50"
        && s.text_color == Some(Color::rgb(255, 0, 0))));

    let unstyled = design_cell_style(&col, &header, &row_positive, "3.70");
    assert!(unstyled.is_none());
}

/// The grid says "backend error:" once, followed by the error's own words.
#[test]
fn an_unavailable_source_says_backend_error_once() {
    use datagrid::RowSource;
    let e = datagrid::SourceError::Backend("database file not found: C:/p/a.db".into());
    let source = UnavailableSource::from_error(&e);
    let shown = source
        .count(&datagrid::Query::default())
        .unwrap_err()
        .to_string();
    assert_eq!(shown, "backend error: database file not found: C:/p/a.db");
}
