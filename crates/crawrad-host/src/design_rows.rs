//! The first rows of a bound `Grid`'s table, read at design time.
//!
//! A `Grid` bound to a `Connection` (`data_source` + `table_name`) shows, on
//! the canvas, the real columns and the first rows of its table; bound to a
//! `Dataset`, the first rows of the dataset's `sql` over its connection. They are read
//! by `crawrad-dbprobe` on a background task — the form never waits for the
//! database — and kept here, keyed by the database file's modification time so
//! a changed file is read again. When the read fails, the grid keeps the
//! columns of the last good read and says why.

use std::collections::{BTreeMap, HashMap};
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant, SystemTime};

use crawrad_ir::design_rows::{DESIGN_ROWS_KEY, GridRows, Sample, grid_binding, limit_of};
use crawrad_ir::{Document, Value};
use crawrad_note::note;
use crawrad_os::tasks::{TaskEvent, TaskId, TaskManager};

use crate::dataset_columns::{
    PROBE_NOT_FOUND, make_url_absolute, probe_next_to_the_ide, run_probe,
};

#[note("design-rows-stamp-interval")]
pub const STAMP_INTERVAL: Duration = Duration::from_secs(1);

/// What the probe reads: a whole table, or a dataset's query.
#[note("design-rows-rowquery")]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum RowQuery {
    /// The `Grid`'s `table_name` (bound to a `Connection`).
    Table(String),
    /// The `Dataset`'s `sql` (bound to a `Dataset`).
    Sql(String),
}

#[note("design-rows-gridsource")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GridSource {
    /// The `Connection`'s url, made absolute against the project.
    pub url: String,
    /// The table or the query the rows come from.
    pub query: RowQuery,
    /// What the footer names: the table, or the dataset.
    pub table: String,
    /// How many rows the Designer shows.
    pub limit: usize,
}

#[note("design-rows-grid-source")]
pub fn grid_source(
    doc: &Document,
    project_dir: Option<&Path>,
    grid_id: &str,
) -> Option<GridSource> {
    let text = |v: Option<&Value>| match v {
        Some(Value::Str(s)) | Some(Value::Enum(s)) => Some(s.trim().to_string()),
        _ => None,
    };
    let grid = doc.node(grid_id).filter(|n| n.class == "Grid")?;
    let limit = limit_of(text(grid.props.get(DESIGN_ROWS_KEY)).as_deref())?;
    let binding = grid_binding(doc, grid)?;
    let url = text(binding.connection.props.get("url")).filter(|u| !u.is_empty())?;
    // Bound to a Dataset, the Grid's own table_name is ignored: the dataset's
    // query is what the app will read at run time.
    let (query, table) = match binding.dataset {
        Some(dataset) => {
            let sql = text(dataset.props.get("sql")).filter(|q| !q.is_empty())?;
            (RowQuery::Sql(sql), dataset.name().to_string())
        }
        None => {
            let table = text(grid.props.get("table_name")).filter(|t| !t.is_empty())?;
            (RowQuery::Table(table.clone()), table)
        }
    };
    Some(GridSource {
        url: make_url_absolute(&url, project_dir),
        query,
        table,
        limit,
    })
}

#[note("design-rows-sqlite-path")]
pub fn sqlite_path(url: &str) -> Option<PathBuf> {
    let rest = url
        .strip_prefix("sqlite://")
        .or_else(|| url.strip_prefix("sqlite:"))?;
    (!rest.is_empty() && rest != ":memory:" && !rest.starts_with("file:"))
        .then(|| PathBuf::from(rest))
}

type Key = (String, RowQuery, usize, Option<SystemTime>);
type Answer = Result<Sample, String>;

enum Entry {
    Loading,
    Ready(Sample),
    Failed(String),
}

#[note("design-rows-designrows")]
pub struct DesignRows {
    cache: BTreeMap<Key, Entry>,
    last_columns: BTreeMap<(String, RowQuery), Vec<String>>,
    stamps: HashMap<String, Option<SystemTime>>,
    stamped_at: Option<Instant>,
    tasks: TaskManager<Answer>,
    keys: HashMap<TaskId, Key>,
    revision: Option<u64>,
    probe: fn() -> Option<PathBuf>,
    run: fn(&Path, &str, &RowQuery, usize) -> Answer,
}

impl Default for DesignRows {
    fn default() -> Self {
        Self::with_probe(probe_next_to_the_ide, probe_rows)
    }
}

impl DesignRows {
    #[note("design-rows-designrows-with-probe")]
    pub fn with_probe(
        probe: fn() -> Option<PathBuf>,
        run: fn(&Path, &str, &RowQuery, usize) -> Answer,
    ) -> Self {
        Self {
            cache: BTreeMap::new(),
            last_columns: BTreeMap::new(),
            stamps: HashMap::new(),
            stamped_at: None,
            tasks: TaskManager::new(),
            keys: HashMap::new(),
            revision: None,
            probe,
            run,
        }
    }

    #[note("design-rows-designrows-ensure")]
    pub fn ensure(&mut self, doc: &Document, revision: u64, project_dir: Option<&Path>) {
        let edited = self.revision != Some(revision);
        if edited {
            self.revision = Some(revision);
            self.cache.retain(|_, e| !matches!(e, Entry::Failed(_)));
        }
        let sources: Vec<GridSource> = grids(doc)
            .filter_map(|id| grid_source(doc, project_dir, id))
            .collect();
        if edited
            || self
                .stamped_at
                .is_none_or(|t| t.elapsed() >= STAMP_INTERVAL)
        {
            self.stamped_at = Some(Instant::now());
            self.stamps = sources
                .iter()
                .map(|s| (s.url.clone(), stamp(&s.url)))
                .collect();
        }
        let wanted: Vec<Key> = sources.into_iter().map(|s| self.key(s)).collect();
        self.cache
            .retain(|k, e| matches!(e, Entry::Loading) || wanted.contains(k));
        for key in wanted {
            if self.cache.contains_key(&key) {
                continue;
            }
            let Some(exe) = (self.probe)() else {
                self.cache
                    .insert(key, Entry::Failed(PROBE_NOT_FOUND.to_string()));
                continue;
            };
            self.cache.insert(key.clone(), Entry::Loading);
            let ((url, table, limit, _), run) = (key.clone(), self.run);
            let id = self.tasks.spawn("dbprobe-rows", move |r| {
                r.done(run(&exe, &url, &table, limit))
            });
            self.keys.insert(id, key);
        }
    }

    #[note("design-rows-designrows-pump")]
    pub fn pump(&mut self) -> bool {
        let mut changed = false;
        for (id, event) in self.tasks.drain() {
            let entry = match event {
                TaskEvent::Done(Ok(sample)) => Entry::Ready(sample),
                TaskEvent::Done(Err(e)) | TaskEvent::Failed(e) => Entry::Failed(e),
                TaskEvent::Progress(_) => continue,
            };
            if let Some(key) = self.keys.remove(&id) {
                if let Entry::Ready(s) = &entry {
                    self.last_columns
                        .insert((key.0.clone(), key.1.clone()), s.columns.clone());
                }
                self.cache.insert(key, entry);
                changed = true;
            }
        }
        changed
    }

    #[note("design-rows-designrows-rows")]
    pub fn rows(
        &self,
        doc: &Document,
        project_dir: Option<&Path>,
        grid_id: &str,
    ) -> Option<GridRows> {
        let source = grid_source(doc, project_dir, grid_id)?;
        let columns = self
            .last_columns
            .get(&(source.url.clone(), source.query.clone()))
            .cloned()
            .unwrap_or_default();
        let (table, limit) = (source.table.clone(), source.limit);
        let file = sqlite_path(&source.url)
            .and_then(|p| p.file_name().map(|f| f.to_string_lossy().into_owned()));
        Some(match self.cache.get(&self.key(source)) {
            Some(Entry::Ready(sample)) => GridRows::Ready {
                sample: sample.clone(),
                table,
                file,
            },
            Some(Entry::Failed(reason)) => GridRows::Offline {
                columns,
                reason: reason.clone(),
                table,
            },
            Some(Entry::Loading) | None => GridRows::Reading {
                columns,
                limit,
                table,
            },
        })
    }

    fn key(&self, s: GridSource) -> Key {
        let stamp = self.stamps.get(&s.url).copied().flatten();
        (s.url, s.query, s.limit, stamp)
    }
}

fn grids(doc: &Document) -> impl Iterator<Item = &str> {
    doc.nodes
        .values()
        .filter(|n| n.class == "Grid")
        .map(|n| n.id.as_str())
}

fn stamp(url: &str) -> Option<SystemTime> {
    sqlite_path(url).and_then(|p| std::fs::metadata(p).ok()?.modified().ok())
}

#[note("design-rows-probe-rows")]
pub fn probe_rows(exe: &Path, url: &str, query: &RowQuery, limit: usize) -> Answer {
    let limit = limit.to_string();
    let (flag, target) = match query {
        RowQuery::Table(table) => ("--table", table),
        RowQuery::Sql(sql) => ("--sql", sql),
    };
    parse_sample(&run_probe(
        exe,
        &["--url", url, flag, target, "--rows", &limit],
    )?)
}

#[note("design-rows-parse-sample")]
pub fn parse_sample(text: &str) -> Answer {
    let json: serde_json::Value = serde_json::from_str(text.trim())
        .map_err(|e| format!("crawrad-dbprobe answered something that is not JSON: {e}"))?;
    let strings = |v: &serde_json::Value| -> Vec<String> {
        v.as_array()
            .map(|a| {
                a.iter()
                    .map(|c| c.as_str().unwrap_or_default().to_string())
                    .collect()
            })
            .unwrap_or_default()
    };
    let opt_strings = |v: &serde_json::Value| -> Vec<Option<String>> {
        v.as_array()
            .map(|a| {
                a.iter()
                    .map(|c| c.as_str().map(|s| s.to_string()))
                    .collect()
            })
            .unwrap_or_default()
    };
    match (json.get("columns"), json.get("rows")) {
        (Some(cols), Some(rows)) => Ok(Sample {
            columns: strings(cols),
            kinds: json.get("kinds").map(strings).unwrap_or_default(),
            rows: rows
                .as_array()
                .map(|a| a.iter().map(opt_strings).collect())
                .unwrap_or_default(),
            total: json.get("total").and_then(|t| t.as_u64()).unwrap_or(0),
        }),
        _ => Err(json
            .get("error")
            .and_then(|e| e.as_str())
            .unwrap_or("crawrad-dbprobe gave no rows")
            .to_string()),
    }
}

#[cfg(test)]
#[path = "test/design_rows/test.rs"]
mod tests;
