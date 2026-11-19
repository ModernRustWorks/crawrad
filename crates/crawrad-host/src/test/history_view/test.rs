//! Unit tests for history view model.

use super::*;
use crate::history::{Entry, Origin};
use crawrad_os::clock::days_from_civil;

#[test]
fn identical_texts_have_same_rows_and_no_blocks() {
    let copy = "line 1\nline 2\nline 3\n";
    let buffer = "line 1\nline 2\nline 3\n";
    let view = side_by_side(copy, buffer);

    assert!(view.blocks.is_empty());
    assert_eq!(view.rows.len(), 3);

    for (idx, row) in view.rows.iter().enumerate() {
        let expected_line = idx + 1;
        let expected_text = format!("line {expected_line}");
        assert_eq!(row.kind, RowKind::Same);
        assert_eq!(row.left, Some((expected_line, expected_text.clone())));
        assert_eq!(row.right, Some((expected_line, expected_text)));
    }
}

#[test]
fn one_changed_line_produces_one_changed_row_and_one_block() {
    let copy = "alpha\nbeta\ngamma\n";
    let buffer = "alpha\nbravo\ngamma\n";
    let view = side_by_side(copy, buffer);

    assert_eq!(view.blocks.len(), 1);
    assert_eq!(view.blocks[0], 1..2);
    assert_eq!(view.rows.len(), 3);

    assert_eq!(view.rows[0].kind, RowKind::Same);
    assert_eq!(view.rows[0].left, Some((1, "alpha".to_string())));
    assert_eq!(view.rows[0].right, Some((1, "alpha".to_string())));

    assert_eq!(view.rows[1].kind, RowKind::Changed);
    assert_eq!(view.rows[1].left, Some((2, "beta".to_string())));
    assert_eq!(view.rows[1].right, Some((2, "bravo".to_string())));

    assert_eq!(view.rows[2].kind, RowKind::Same);
    assert_eq!(view.rows[2].left, Some((3, "gamma".to_string())));
    assert_eq!(view.rows[2].right, Some((3, "gamma".to_string())));
}

#[test]
fn lines_removed_and_added_produce_proper_rows_and_line_numbers() {
    let copy = "head\nold_a\nold_b\ntail\n";
    let buffer = "head\nnew_a\ntail\n";
    let view = side_by_side(copy, buffer);

    assert_eq!(view.blocks.len(), 1);
    assert_eq!(view.blocks[0], 1..3);
    assert_eq!(view.rows.len(), 4);

    assert_eq!(view.rows[0].kind, RowKind::Same);
    assert_eq!(view.rows[0].left, Some((1, "head".to_string())));
    assert_eq!(view.rows[0].right, Some((1, "head".to_string())));

    assert_eq!(view.rows[1].kind, RowKind::Changed);
    assert_eq!(view.rows[1].left, Some((2, "old_a".to_string())));
    assert_eq!(view.rows[1].right, Some((2, "new_a".to_string())));

    assert_eq!(view.rows[2].kind, RowKind::Removed);
    assert_eq!(view.rows[2].left, Some((3, "old_b".to_string())));
    assert_eq!(view.rows[2].right, None);

    assert_eq!(view.rows[3].kind, RowKind::Same);
    assert_eq!(view.rows[3].left, Some((4, "tail".to_string())));
    assert_eq!(view.rows[3].right, Some((3, "tail".to_string())));

    let copy_added = "start\nend\n";
    let buffer_added = "start\nins_1\nins_2\nend\n";
    let view_added = side_by_side(copy_added, buffer_added);

    assert_eq!(view_added.blocks.len(), 1);
    assert_eq!(view_added.blocks[0], 1..3);
    assert_eq!(view_added.rows.len(), 4);

    assert_eq!(view_added.rows[0].kind, RowKind::Same);
    assert_eq!(view_added.rows[0].left, Some((1, "start".to_string())));
    assert_eq!(view_added.rows[0].right, Some((1, "start".to_string())));

    assert_eq!(view_added.rows[1].kind, RowKind::Added);
    assert_eq!(view_added.rows[1].left, None);
    assert_eq!(view_added.rows[1].right, Some((2, "ins_1".to_string())));

    assert_eq!(view_added.rows[2].kind, RowKind::Added);
    assert_eq!(view_added.rows[2].left, None);
    assert_eq!(view_added.rows[2].right, Some((3, "ins_2".to_string())));

    assert_eq!(view_added.rows[3].kind, RowKind::Same);
    assert_eq!(view_added.rows[3].left, Some((2, "end".to_string())));
    assert_eq!(view_added.rows[3].right, Some((4, "end".to_string())));
}

#[test]
fn two_separate_change_groups_produce_two_blocks_and_navigation_wraps() {
    let copy = "one\ntwo\nthree\nfour\nfive\n";
    let buffer = "one\nTWO\nthree\nFOUR\nfive\n";
    let view = side_by_side(copy, buffer);

    assert_eq!(view.blocks.len(), 2);
    assert_eq!(view.blocks[0], 1..2);
    assert_eq!(view.blocks[1], 3..4);

    assert_eq!(next_block(&view, None), Some(0));
    assert_eq!(next_block(&view, Some(0)), Some(1));
    assert_eq!(next_block(&view, Some(1)), Some(0));

    assert_eq!(previous_block(&view, None), Some(1));
    assert_eq!(previous_block(&view, Some(1)), Some(0));
    assert_eq!(previous_block(&view, Some(0)), Some(1));

    let empty_view = side_by_side("same\n", "same\n");
    assert_eq!(next_block(&empty_view, None), None);
    assert_eq!(next_block(&empty_view, Some(0)), None);
    assert_eq!(previous_block(&empty_view, None), None);
    assert_eq!(previous_block(&empty_view, Some(0)), None);
}

#[test]
fn crlf_copy_versus_lf_buffer_with_same_lines_has_no_blocks() {
    let copy = "first\r\nsecond\r\nthird\r\n";
    let buffer = "first\nsecond\nthird\n";
    let view = side_by_side(copy, buffer);

    assert!(view.blocks.is_empty());
    assert_eq!(view.rows.len(), 3);
    for row in &view.rows {
        assert_eq!(row.kind, RowKind::Same);
    }
}

#[test]
fn restore_block_first_of_two_changes_only_first_group() {
    let copy = "g1_copy\nkeep\ng2_copy\n";
    let buffer = "g1_buf\nkeep\ng2_buf\n";

    let restored = restore_block(copy, buffer, 0);
    assert_eq!(restored.as_deref(), Some("g1_copy\nkeep\ng2_buf\n"));

    let restored_second = restore_block(copy, buffer, 1);
    assert_eq!(restored_second.as_deref(), Some("g1_buf\nkeep\ng2_copy\n"));
}

#[test]
fn restore_block_added_removes_added_lines() {
    let copy = "top\nbottom\n";
    let buffer = "top\ninserted_1\ninserted_2\nbottom\n";

    let restored = restore_block(copy, buffer, 0);
    assert_eq!(restored.as_deref(), Some("top\nbottom\n"));
}

#[test]
fn restore_block_removed_puts_lines_back() {
    let copy = "top\nremoved_line\nbottom\n";
    let buffer = "top\nbottom\n";

    let restored = restore_block(copy, buffer, 0);
    assert_eq!(restored.as_deref(), Some("top\nremoved_line\nbottom\n"));
}

#[test]
fn restore_block_preserves_buffer_line_ending_and_trailing_newline() {
    let copy = "hello\n";
    let buffer_crlf_trailing = "world\r\n";
    let restored = restore_block(copy, buffer_crlf_trailing, 0);
    assert_eq!(restored.as_deref(), Some("hello\r\n"));

    let buffer_lf_no_trailing = "world";
    let restored_no_trailing = restore_block(copy, buffer_lf_no_trailing, 0);
    assert_eq!(restored_no_trailing.as_deref(), Some("hello"));

    let buffer_crlf_no_trailing = "world\r\nagain";
    let copy_crlf_change = "world\r\nreplaced";
    let restored_crlf = restore_block(copy_crlf_change, buffer_crlf_no_trailing, 0);
    assert_eq!(restored_crlf.as_deref(), Some("world\r\nreplaced"));
}

#[test]
fn restore_block_out_of_range_returns_none() {
    let copy = "a\n";
    let buffer = "b\n";
    assert_eq!(restore_block(copy, buffer, 1), None);
    assert_eq!(restore_block("same\n", "same\n", 0), None);
}

#[test]
fn entry_rows_relative_dates_sizes_and_newest_first() {
    let offset = -10800; // UTC-3
    let now_days = days_from_civil(2026, 10, 20);
    let now_local_secs = now_days * 86400 + 12 * 3600; // 2026-10-20 12:00:00 local
    let now_utc = (now_local_secs - offset) as u64;

    // Today entry: 2026-10-20 14:32:10 local
    let today_local_secs = now_days * 86400 + 14 * 3600 + 32 * 60 + 10;
    let today_utc = (today_local_secs - offset) as u64;

    // Yesterday entry: 2026-10-19 09:15:00 local
    let yest_local_secs = (now_days - 1) * 86400 + 9 * 3600 + 15 * 60;
    let yest_utc = (yest_local_secs - offset) as u64;

    // Same year entry: 2026-10-18 16:45:00 local
    let same_year_days = days_from_civil(2026, 10, 18);
    let same_year_local_secs = same_year_days * 86400 + 16 * 3600 + 45 * 60;
    let same_year_utc = (same_year_local_secs - offset) as u64;

    // Another year entry: 2025-10-18 18:20:30 local
    let diff_year_days = days_from_civil(2025, 10, 18);
    let diff_year_local_secs = diff_year_days * 86400 + 18 * 3600 + 20 * 60 + 30;
    let diff_year_utc = (diff_year_local_secs - offset) as u64;

    let entries = vec![
        (
            Entry {
                time: diff_year_utc,
                hash: "h4".to_string(),
                origin: Origin::External,
            },
            "x".repeat(2_500_000), // ~2.38 MB -> 2.4 MB
        ),
        (
            Entry {
                time: same_year_utc,
                hash: "h3".to_string(),
                origin: Origin::Save,
            },
            "line1\nline2\n".to_string(), // 12 B
        ),
        (
            Entry {
                time: yest_utc,
                hash: "h2".to_string(),
                origin: Origin::External,
            },
            "a".repeat(1843), // ~1.8 KB
        ),
        (
            Entry {
                time: today_utc,
                hash: "h1".to_string(),
                origin: Origin::Save,
            },
            "first\nsecond\nthird\n".to_string(), // 3 lines, 19 B
        ),
    ];

    let rows = entry_rows(&entries, now_utc, offset);

    assert_eq!(rows.len(), 4);

    // Newest first: today
    assert_eq!(rows[0].time, "14:32:10");
    assert_eq!(rows[0].when, "today");
    assert_eq!(rows[0].origin, "Save");
    assert_eq!(rows[0].size, "19 B");
    assert_eq!(rows[0].lines, 3);

    // Yesterday
    assert_eq!(rows[1].time, "09:15:00");
    assert_eq!(rows[1].when, "yesterday");
    assert_eq!(rows[1].origin, "External");
    assert_eq!(rows[1].size, "1.8 KB");
    assert_eq!(rows[1].lines, 1);

    // Same year (18/10)
    assert_eq!(rows[2].time, "16:45:00");
    assert_eq!(rows[2].when, "18/10");
    assert_eq!(rows[2].origin, "Save");
    assert_eq!(rows[2].size, "12 B");
    assert_eq!(rows[2].lines, 2);

    // Another year (18/10/2025)
    assert_eq!(rows[3].time, "18:20:30");
    assert_eq!(rows[3].when, "18/10/2025");
    assert_eq!(rows[3].origin, "External");
    assert_eq!(rows[3].size, "2.4 MB");
    assert_eq!(rows[3].lines, 1);
}

#[test]
fn when_and_time_formats_various_relative_days_and_hours() {
    let offset: i64 = -3 * 3600; // UTC-3
    let now_days = days_from_civil(2026, 10, 20);
    let now_local_secs = now_days * 86400 + 12 * 3600;
    let now_utc = (now_local_secs - offset) as u64;

    // Today at 14:32:10
    let today_local_secs = now_days * 86400 + 14 * 3600 + 32 * 60 + 10;
    let today_utc = (today_local_secs - offset) as u64;
    assert_eq!(
        when_and_time(today_utc, now_utc, offset),
        ("14:32:10".to_string(), "today".to_string())
    );

    // Yesterday at 09:15:30
    let yest_local_secs = (now_days - 1) * 86400 + 9 * 3600 + 15 * 60 + 30;
    let yest_utc = (yest_local_secs - offset) as u64;
    assert_eq!(
        when_and_time(yest_utc, now_utc, offset),
        ("09:15:30".to_string(), "yesterday".to_string())
    );

    // Same year at 18/10 09:15:30
    let same_year_days = days_from_civil(2026, 10, 18);
    let same_year_local_secs = same_year_days * 86400 + 9 * 3600 + 15 * 60 + 30;
    let same_year_utc = (same_year_local_secs - offset) as u64;
    assert_eq!(
        when_and_time(same_year_utc, now_utc, offset),
        ("09:15:30".to_string(), "18/10".to_string())
    );

    // Different year at 18/10/2025 09:15:30
    let diff_year_days = days_from_civil(2025, 10, 18);
    let diff_year_local_secs = diff_year_days * 86400 + 9 * 3600 + 15 * 60 + 30;
    let diff_year_utc = (diff_year_local_secs - offset) as u64;
    assert_eq!(
        when_and_time(diff_year_utc, now_utc, offset),
        ("09:15:30".to_string(), "18/10/2025".to_string())
    );
}

#[test]
fn timeline_lines_sorts_newest_first_and_formats_origin_time_when() {
    let offset: i64 = 0;
    let now_days = days_from_civil(2026, 10, 20);
    let now_utc = (now_days * 86400 + 15 * 3600) as u64;

    let today_utc = (now_days * 86400 + 14 * 3600 + 32 * 60 + 10) as u64;
    let yest_utc = ((now_days - 1) * 86400 + 9 * 3600 + 15 * 60 + 30) as u64;
    let old_utc = (days_from_civil(2026, 10, 18) * 86400 + 9 * 3600 + 15 * 60 + 30) as u64;

    let entries = vec![
        Entry {
            time: yest_utc,
            hash: "h2".to_string(),
            origin: Origin::External,
        },
        Entry {
            time: old_utc,
            hash: "h3".to_string(),
            origin: Origin::Save,
        },
        Entry {
            time: today_utc,
            hash: "h1".to_string(),
            origin: Origin::Save,
        },
    ];

    let lines = timeline_lines(&entries, now_utc, offset);
    assert_eq!(lines.len(), 3);
    assert_eq!(lines[0], "Save · 14:32:10 · today");
    assert_eq!(lines[1], "External · 09:15:30 · yesterday");
    assert_eq!(lines[2], "Save · 09:15:30 · 18/10");

    let empty_lines = timeline_lines(&[], now_utc, offset);
    assert!(empty_lines.is_empty());
}

#[test]
fn session_selects_the_newest_steps_through_blocks_and_restores() {
    let root = std::env::temp_dir().join(format!("crawrad_history_session_{}", std::process::id()));
    std::fs::create_dir_all(&root).unwrap();
    let file = root.join("main.rs");
    let settings = crate::history::LocalHistory::new(
        root.clone(),
        crate::history::HistorySettings {
            merge_seconds: 0,
            ..Default::default()
        },
    );
    settings
        .record(&file, "a\nb\nc\n", crate::history::Origin::Save, 100)
        .unwrap();
    settings
        .record(&file, "a\nB\nc\nd\n", crate::history::Origin::Save, 200)
        .unwrap();

    let mut session = super::HistorySession::open(Some(&settings), file.clone(), 300, 0);
    assert_eq!(session.rows.len(), 2);
    assert_eq!(session.selected, Some(0));
    assert_eq!(session.copy(), "a\nB\nc\nd\n");

    session.select(1);
    assert_eq!(session.copy(), "a\nb\nc\n");
    let buffer = "a\nX\nc\nY\n";
    assert_eq!(session.diff(buffer).blocks.len(), 2);
    assert_eq!(session.step(buffer, true), Some(1));
    assert_eq!(session.step(buffer, true), Some(3));
    assert_eq!(session.restored(buffer).as_deref(), Some("a\nX\nc\n"));
    assert_eq!(session.json(buffer)["blocks"], 2);

    session.select(9);
    assert_eq!(session.selected, Some(1));
    std::fs::remove_dir_all(&root).ok();
}
