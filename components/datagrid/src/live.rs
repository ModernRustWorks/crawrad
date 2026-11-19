//! Off-UI-thread row fetching and state snapshotting for datagrid views.
//!
//! # Why `LiveRows` exists
//! In interactive grid applications, queries on large or remote datasets can take
//! hundreds of milliseconds (for instance, a cold far jump on an SQL backend over
//! 1M rows takes ~0.55 s). The visual Grid must never freeze or drop frame rates on
//! the UI thread.
//!
//! [`Cursor`] is synchronous and its underlying [`RowSource`] is `Send + Sync`.
//! `LiveRows` hosts the `Cursor` on a dedicated background worker thread using standard
//! library threads and channels (`std::sync::mpsc`). The UI thread only submits non-blocking
//! requests (`show`, `set_query`) and periodically polls for lightweight state snapshots
//! ([`Screen`]).
//!
//! The approved busy indicator reads telemetry from [`Screen::busy`] and [`Screen::pending`]:
//! - [`Screen::pending`] identifies which row range the worker is currently answering, allowing
//!   the UI to immediately display "Loading rows A – B" while still presenting old rows.
//! - [`Screen::busy`] records exact timestamps from real underlying source calls, avoiding
//!   spurious visual flickers during cache hits.
//!
//! `LiveRows` is completely framework-free: it has no dependencies on UI frameworks or external
//! runtime engines, relying exclusively on `std`.

use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{self, Receiver, Sender};
use std::thread::JoinHandle;
use std::time::Instant;

use crate::cursor::{Cursor, CursorEvent};
use crate::filter::Owned;
use crate::query::Query;
use crate::source::{Change, RowSource};

/// When the current wait for the source began and ended.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Busy {
    pub started: Instant,
    pub ended: Option<Instant>,
}

/// A rendered line within the grid window: either a group header or a data row.
#[derive(Clone, Debug, PartialEq)]
pub enum ScreenLine {
    /// First line of a group, outer levels first.
    Header {
        level: usize,
        column: String,
        key: String,
        row_count: usize,
        totals: Vec<Option<f64>>,
    },
    /// A data row: its index in the query result and its cells as text (None = NULL).
    Row {
        index: usize,
        /// The row's stable id in the source: what a write names (#936 P2).
        id: u64,
        cells: Vec<Option<String>>,
    },
}

/// A write the grid sends to its source (#936 P2): one per posted row.
#[derive(Clone, Debug, PartialEq)]
pub enum Write {
    /// The changed values of the row whose id is `id`.
    Update { id: u64, changes: Vec<Change> },
    /// A new row's values; the source returns its id.
    Insert(Vec<Change>),
}

/// How a write ended: the ticket [`LiveRows::write`] returned and the row's id, or the
/// source's refusal as text.
#[derive(Clone, Debug, PartialEq)]
pub struct Written {
    pub ticket: u64,
    pub result: Result<u64, String>,
}

/// A point-in-time snapshot of the grid viewport state.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Screen {
    /// Row index of the first data row in this window.
    pub first: usize,
    /// Lines in this window: group headers (if grouped) and data rows.
    pub lines: Vec<ScreenLine>,
    /// Total rows of the current query, once known.
    pub total: Option<usize>,
    /// The request the worker is still answering (first, count): the UI shows
    /// "Loading rows A – B" from it while `lines` still holds the OLD lines.
    pub pending: Option<(usize, usize)>,
    /// When the current wait for the source began and ended (see below).
    pub busy: Option<Busy>,
    /// The last source error as text (the worker keeps running).
    pub error: Option<String>,
    /// Grand totals of the columns matching query.summary.
    pub summary: Vec<Option<f64>>,
    /// Distinct values for a column requested via `distinct()`.
    pub distinct: Option<(String, Vec<Option<String>>)>,
    /// The last write's outcome (#936 P2).
    pub written: Option<Written>,
}

impl Screen {
    /// Iterates over the cell slices of all [`ScreenLine::Row`] entries in the screen.
    pub fn rows(&self) -> impl Iterator<Item = &[Option<String>]> {
        self.lines.iter().filter_map(|line| match line {
            ScreenLine::Row { cells, .. } => Some(cells.as_slice()),
            _ => None,
        })
    }
}

enum Request {
    Show { first: usize, count: usize },
    SetQuery(Query),
    Distinct { column: String, query: Query },
    Write { ticket: u64, write: Write },
    Stop,
}

enum Reply {
    Busy(Instant),
    Idle(Instant),
    Rows {
        first: usize,
        lines: Vec<ScreenLine>,
        total: usize,
    },
    Summary(Vec<Option<f64>>),
    Distinct(String, Vec<Option<String>>),
    Written(Written),
    Error(String),
}

/// Manages a background worker thread that owns a `Cursor` and answers row view requests.
pub struct LiveRows {
    tx: Sender<Request>,
    rx: Receiver<Reply>,
    handle: Option<JoinHandle<()>>,
    screen: Screen,
    last_count: usize,
    next_ticket: u64,
}

impl LiveRows {
    /// Starts the worker; it opens `Cursor::with_window(source, query, page, max_pages)`
    /// ON THE WORKER (opening may count rows = a source call).
    pub fn spawn(
        source: Arc<dyn RowSource>,
        query: Query,
        page: usize,
        max_pages: usize,
    ) -> LiveRows {
        let (req_tx, req_rx) = mpsc::channel();
        let (reply_tx, reply_rx) = mpsc::channel();

        let default_page = page;
        let handle = std::thread::spawn(move || {
            worker_loop(source, query, page, max_pages, req_rx, reply_tx);
        });

        LiveRows {
            tx: req_tx,
            rx: reply_rx,
            handle: Some(handle),
            screen: Screen::default(),
            last_count: default_page,
            next_ticket: 0,
        }
    }

    /// Non-blocking. Asks for rows `first .. first + count` (clamped to the total).
    pub fn show(&mut self, first: usize, count: usize) {
        let first = match self.screen.total {
            Some(0) => 0,
            Some(total) => {
                if first >= total {
                    total.saturating_sub(count)
                } else {
                    first
                }
            }
            None => first,
        };

        self.screen.pending = Some((first, count));
        self.last_count = count;
        let _ = self.tx.send(Request::Show { first, count });
    }

    /// Non-blocking. Replaces the query (sort/filter/group); the screen goes back to row 0.
    pub fn set_query(&mut self, query: Query) {
        self.screen.first = 0;
        self.screen.total = None;
        self.screen.summary.clear();
        // `distinct` stays: it was asked for the query WITHOUT that column's own
        // filter, so checking a value in the open list must not empty the list
        // (#936 P3c — it froze on "Loading…"). Opening a list asks again.
        let count = self.last_count;
        self.screen.pending = Some((0, count));
        let _ = self.tx.send(Request::SetQuery(query));
    }

    /// Non-blocking. Requests distinct values for `column` under `query`.
    pub fn distinct(&mut self, column: impl Into<String>, query: Query) {
        let column = column.into();
        let _ = self.tx.send(Request::Distinct { column, query });
    }

    /// Non-blocking. Sends one row's write; the outcome arrives in `screen().written` under
    /// the returned ticket, and the rows, count and totals are read again after it.
    pub fn write(&mut self, write: Write) -> u64 {
        self.next_ticket += 1;
        let ticket = self.next_ticket;
        let _ = self.tx.send(Request::Write { ticket, write });
        ticket
    }

    /// Non-blocking. Drains every reply that arrived; returns true if `screen()` changed.
    pub fn poll(&mut self) -> bool {
        let old = self.screen.clone();
        while let Ok(reply) = self.rx.try_recv() {
            match reply {
                Reply::Busy(started) => {
                    self.screen.busy = Some(Busy {
                        started,
                        ended: None,
                    });
                }
                Reply::Idle(ended) => {
                    if let Some(busy) = &mut self.screen.busy {
                        busy.ended = Some(ended);
                    }
                }
                Reply::Rows {
                    first,
                    lines,
                    total,
                } => {
                    self.screen.first = first;
                    self.screen.lines = lines;
                    self.screen.total = Some(total);
                    self.screen.pending = None;
                    self.screen.error = None;
                }
                Reply::Summary(summary) => {
                    self.screen.summary = summary;
                }
                Reply::Distinct(col, values) => {
                    self.screen.distinct = Some((col, values));
                }
                Reply::Written(written) => {
                    self.screen.written = Some(written);
                }
                Reply::Error(err_msg) => {
                    self.screen.error = Some(err_msg);
                    self.screen.pending = None;
                }
            }
        }
        self.screen != old
    }

    /// Returns the current screen snapshot.
    pub fn screen(&self) -> &Screen {
        &self.screen
    }
}

impl Drop for LiveRows {
    /// Does NOT join: a request may be several source calls (count, then a page),
    /// and closing a form must not wait for all of them. The worker sees `Stop`
    /// (or the closed channel) after the call in flight and ends on its own.
    fn drop(&mut self) {
        let _ = self.tx.send(Request::Stop);
        drop(self.handle.take());
    }
}

fn worker_loop(
    source: Arc<dyn RowSource>,
    query: Query,
    page: usize,
    max_pages: usize,
    rx: Receiver<Request>,
    tx: Sender<Reply>,
) {
    let mut cursor = Cursor::with_window(source, query, page, max_pages);
    // One request can be several source calls (count, then a page), each framed
    // by its own Busy/Idle. The screen wants ONE wait per request, from the first
    // call's start, or the spinner's clock would restart in the middle of it:
    // the first Busy is sent at once, the Idle only when the request is served.
    let waiting = Arc::new(AtomicBool::new(false));
    let event_tx = tx.clone();
    let event_waiting = waiting.clone();
    cursor.on_event(move |event| {
        if *event == CursorEvent::Busy && !event_waiting.swap(true, Ordering::SeqCst) {
            let _ = event_tx.send(Reply::Busy(Instant::now()));
        }
    });

    let mut last_count = page;
    let mut last_first = 0;

    if !cursor.query().summary.is_empty() {
        send_summary(&mut cursor, &tx);
        if waiting.swap(false, Ordering::SeqCst) {
            let _ = tx.send(Reply::Idle(Instant::now()));
        }
    }

    // A closed channel (the `LiveRows` is gone) ends the worker like `Stop`.
    while let Ok(req) = rx.recv() {
        let mut stop = false;
        let mut last_query = None;
        let mut last_show = None;
        let mut distinct_req = None;
        let mut writes = Vec::new();

        let mut process = |r| match r {
            Request::Stop => {
                stop = true;
            }
            Request::SetQuery(q) => {
                last_query = Some(q);
                last_show = None;
            }
            Request::Show { first, count } => {
                last_show = Some((first, count));
            }
            Request::Distinct { column, query } => {
                distinct_req = Some((column, query));
            }
            // Writes are never coalesced: each one is a row the user posted.
            Request::Write { ticket, write } => writes.push((ticket, write)),
        };

        process(req);
        while let Ok(r) = rx.try_recv() {
            process(r);
        }

        if stop {
            break;
        }

        if let Some((column, query)) = distinct_req {
            send_distinct(&mut cursor, &column, &query, &tx);
        }

        // After a write the cached pages, count and totals are stale: read the
        // same query again, at the same place (unless a new query came with it).
        let mut wrote = false;
        for (ticket, write) in writes {
            let result = apply_write(cursor.source().as_ref(), write);
            wrote |= result.is_ok();
            let _ = tx.send(Reply::Written(Written { ticket, result }));
        }
        if wrote && last_query.is_none() {
            last_query = Some(cursor.query().clone());
            last_show = last_show.or(Some((last_first, last_count)));
        }

        if let Some(q) = last_query {
            let has_summary = !q.summary.is_empty();
            cursor.set_query(q);
            if has_summary {
                send_summary(&mut cursor, &tx);
            } else {
                let _ = tx.send(Reply::Summary(Vec::new()));
            }
            if last_show.is_none() {
                last_show = Some((0, last_count));
            }
        }

        if let Some((first, count)) = last_show {
            last_first = first;
            last_count = count;
            serve_show(&mut cursor, first, count, &tx);
        }
        if waiting.swap(false, Ordering::SeqCst) && tx.send(Reply::Idle(Instant::now())).is_err() {
            break;
        }
    }
}

/// Formats an optional owned key operand into its display representation.
fn owned_to_string(val: Option<&Owned>) -> String {
    match val {
        None => "(empty)".to_string(),
        Some(Owned::Int(v)) => v.to_string(),
        Some(Owned::Float(v)) => v.to_string(),
        Some(Owned::Bool(v)) => v.to_string(),
        Some(Owned::Text(s)) => s.clone(),
    }
}

/// Pushes the headers of the current row's groups from `from_level` inward.
fn push_headers(
    cursor: &mut Cursor,
    path: &[Option<Owned>],
    from_level: usize,
    lines: &mut Vec<ScreenLine>,
) -> Result<(), crate::source::SourceError> {
    for (level, key) in path.iter().enumerate().skip(from_level) {
        let column = cursor.query().group_by[level].column.clone();
        let (row_count, totals) = match cursor.group(level)? {
            Some(g) => (g.row_count, g.totals),
            None => (0, Vec::new()),
        };
        lines.push(ScreenLine::Header {
            level,
            column,
            key: owned_to_string(key.as_ref()),
            row_count,
            totals,
        });
    }
    Ok(())
}

/// Applies one write to the source: the row's id, or the refusal as text.
fn apply_write(source: &dyn RowSource, write: Write) -> Result<u64, String> {
    match write {
        Write::Update { id, changes } => source.update(id, &changes).map(|()| id),
        Write::Insert(values) => source.insert(&values),
    }
    .map_err(|e| e.to_string())
}

/// The cursor's current row as a screen line: its index, id and cells as text.
fn current_row(cursor: &Cursor, index: usize) -> ScreenLine {
    ScreenLine::Row {
        index,
        id: cursor.row_id().unwrap_or_default(),
        cells: cursor
            .fields()
            .map(|f| (!f.is_null()).then(|| f.to_string_lossy()))
            .collect(),
    }
}

/// Computes the query's grand totals (#936 Summary) and sends them, or the error.
fn send_summary(cursor: &mut Cursor, tx: &Sender<Reply>) {
    let reply = match cursor.summary() {
        Ok(s) => Reply::Summary(s),
        Err(e) => Reply::Error(e.to_string()),
    };
    let _ = tx.send(reply);
}

/// Computes distinct values for a column (#936 fkList) and sends them, or the error.
fn send_distinct(cursor: &mut Cursor, column: &str, query: &Query, tx: &Sender<Reply>) {
    let reply = match cursor.distinct(query, column, 500) {
        Ok(vals) => Reply::Distinct(column.to_string(), vals),
        Err(e) => Reply::Error(e.to_string()),
    };
    let _ = tx.send(reply);
}

/// Fetches the requested data rows and prepends group header lines.
///
/// # Why headers are emitted
/// When a query has grouping (`group_by`), a visible window typically opens inside
/// one or more enclosing groups rather than at their natural beginning. To give the
/// user immediate visual context without needing to scroll upwards, header lines
/// are emitted for every group level before the first data row of the window.
///
/// For each subsequent data row in the window, its `group_path()` is compared with the
/// previous row. At the first level `L` that differs, headers are emitted for all
/// levels from `L` down to the innermost level.
/// Data row count (`count`) only accounts for rows; headers do not reduce the count.
fn serve_show(cursor: &mut Cursor, first: usize, count: usize, tx: &Sender<Reply>) {
    let total = match cursor.count() {
        Ok(t) => t,
        Err(e) => {
            let _ = tx.send(Reply::Error(e.to_string()));
            return;
        }
    };

    let (first, count) = if total == 0 {
        (0, 0)
    } else {
        let f = if first >= total {
            total.saturating_sub(count)
        } else {
            first
        };
        (f, count)
    };

    let end = (first + count).min(total);
    let mut lines = Vec::with_capacity(end.saturating_sub(first));

    if first < end {
        if let Err(e) = cursor.set_rec_no(first) {
            let _ = tx.send(Reply::Error(e.to_string()));
            return;
        }

        let num_levels = cursor.query().group_by.len();
        let has_group_by = num_levels > 0;

        let mut prev_path = if has_group_by {
            let current_path = match cursor.group_path() {
                Some(p) => p,
                None => {
                    let _ = tx.send(Reply::Error("missing group path".to_string()));
                    return;
                }
            };

            if let Err(e) = push_headers(cursor, &current_path, 0, &mut lines) {
                let _ = tx.send(Reply::Error(e.to_string()));
                return;
            }

            Some(current_path)
        } else {
            None
        };

        lines.push(current_row(cursor, first));

        for i in (first + 1)..end {
            if let Err(e) = cursor.next() {
                let _ = tx.send(Reply::Error(e.to_string()));
                return;
            }

            if has_group_by {
                let current_path = match cursor.group_path() {
                    Some(p) => p,
                    None => {
                        let _ = tx.send(Reply::Error("missing group path".to_string()));
                        return;
                    }
                };

                if let Some(prev) = &prev_path {
                    let mut diff_level = None;
                    for level in 0..num_levels {
                        if prev[level] != current_path[level] {
                            diff_level = Some(level);
                            break;
                        }
                    }

                    if let Some(start_level) = diff_level
                        && let Err(e) = push_headers(cursor, &current_path, start_level, &mut lines)
                    {
                        let _ = tx.send(Reply::Error(e.to_string()));
                        return;
                    }
                }

                prev_path = Some(current_path);
            }

            lines.push(current_row(cursor, i));
        }
    }

    let _ = tx.send(Reply::Rows {
        first,
        lines,
        total,
    });
}

#[cfg(test)]
#[path = "test/live/test.rs"]
mod test;
