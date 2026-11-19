//! The columns of a bound dataset, read at design time.
//!
//! The Inspector's Data > Field row lists the dataset's columns. The IDE has no
//! database driver (SQLite's bundled C would break the cross-target clippy), so
//! the columns are read by `crawrad-dbprobe`, a separate small program next to the
//! IDE's executable, on a background task; this cache holds its answers.

use std::collections::{BTreeMap, HashMap};
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

use crawrad_ir::{Document, Value};
use crawrad_note::note;
use crawrad_os::tasks::{TaskEvent, TaskId, TaskManager};

#[note("dataset-columns-probe-time-limit")]
pub const PROBE_TIME_LIMIT: Duration = Duration::from_secs(10);

#[note("dataset-columns-probe-not-found")]
pub const PROBE_NOT_FOUND: &str = "crawrad-dbprobe not found next to the IDE";

#[note("dataset-columns-datasetsource")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DatasetSource {
    /// The `Connection`'s url, as written in the form (e.g. `sqlite://shop.db`).
    pub url: String,
    /// The `Dataset`'s table.
    pub table: String,
}

#[note("dataset-columns-dataset-source")]
pub fn dataset_source(doc: &Document, dataset_id: &str) -> Option<DatasetSource> {
    let text = |v: Option<&Value>| match v {
        Some(Value::Str(s)) => Some(s.trim().to_string()).filter(|s| !s.is_empty()),
        _ => None,
    };
    let dataset = doc.node(dataset_id).filter(|n| n.class == "Dataset")?;
    let table = text(dataset.props.get("table"))?;
    let engine_id = match dataset.props.get("connection") {
        Some(Value::Ref(id)) => Some(id.as_str()),
        _ => None,
    }?;
    let engine = doc.node(engine_id).filter(|n| n.class == "Connection")?;
    let url = text(engine.props.get("url"))?;
    Some(DatasetSource { url, table })
}

#[note("dataset-columns-make-url-absolute")]
pub fn make_url_absolute(url: &str, project_dir: Option<&Path>) -> String {
    let relative = url
        .strip_prefix("sqlite://")
        .or_else(|| url.strip_prefix("sqlite:"))
        .filter(|rest| !rest.is_empty() && *rest != ":memory:" && !rest.starts_with("file:"))
        .filter(|rest| Path::new(rest).is_relative());
    match (relative, project_dir) {
        (Some(rest), Some(dir)) => format!(
            "sqlite://{}",
            dir.join(rest).to_string_lossy().replace('\\', "/")
        ),
        _ => url.to_string(),
    }
}

#[note("dataset-columns-columnstate")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ColumnState {
    /// The probe is running.
    Loading,
    /// The columns, in the table's order.
    Ready(Vec<String>),
    /// Why there are none.
    Failed(String),
}

type Key = (String, String);
type Answer = Result<Vec<String>, String>;

#[note("dataset-columns-datasetcolumns")]
pub struct DatasetColumns {
    cache: BTreeMap<Key, ColumnState>,
    tasks: TaskManager<Answer>,
    keys: HashMap<TaskId, Key>,
    revision: Option<u64>,
    probe: fn() -> Option<PathBuf>,
    run: fn(&Path, &str, &str) -> Answer,
}

impl Default for DatasetColumns {
    fn default() -> Self {
        Self::with_probe(probe_next_to_the_ide, probe)
    }
}

impl DatasetColumns {
    #[note("dataset-columns-datasetcolumns-with-probe")]
    pub fn with_probe(
        probe: fn() -> Option<PathBuf>,
        run: fn(&Path, &str, &str) -> Answer,
    ) -> Self {
        Self {
            cache: BTreeMap::new(),
            tasks: TaskManager::new(),
            keys: HashMap::new(),
            revision: None,
            probe,
            run,
        }
    }

    #[note("dataset-columns-datasetcolumns-state")]
    pub fn state(&self, url: &str, table: &str) -> Option<&ColumnState> {
        self.cache.get(&(url.to_string(), table.to_string()))
    }

    #[note("dataset-columns-datasetcolumns-columns")]
    pub fn columns(
        &self,
        doc: &Document,
        project_dir: Option<&Path>,
        dataset_id: &str,
    ) -> Option<Vec<String>> {
        let key = key_of(doc, project_dir, dataset_id)?;
        match self.cache.get(&key) {
            Some(ColumnState::Ready(cols)) => Some(cols.clone()),
            _ => None,
        }
    }

    #[note("dataset-columns-datasetcolumns-answer")]
    pub fn answer(
        &self,
        doc: &Document,
        project_dir: Option<&Path>,
        dataset_id: &str,
    ) -> Option<Result<Vec<String>, String>> {
        let key = key_of(doc, project_dir, dataset_id)?;
        match self.cache.get(&key)? {
            ColumnState::Ready(cols) => Some(Ok(cols.clone())),
            ColumnState::Failed(why) => Some(Err(why.clone())),
            ColumnState::Loading => None,
        }
    }

    #[note("dataset-columns-datasetcolumns-ensure")]
    pub fn ensure(&mut self, doc: &Document, revision: u64, project_dir: Option<&Path>) {
        if self.revision != Some(revision) {
            self.revision = Some(revision);
            self.cache
                .retain(|_, state| !matches!(state, ColumnState::Failed(_)));
        }
        // Every dataset a binding reads, and every dataset a property references
        // (a column's `LookupSource`): the Inspector lists their fields.
        let bound = doc
            .nodes
            .values()
            .flat_map(|n| n.bindings.values())
            .map(|b| b.source.as_str());
        let referenced = doc.nodes.values().flat_map(|n| {
            n.props.values().filter_map(|v| match v {
                crawrad_ir::Value::Ref(id) if !id.is_empty() => Some(id.as_str()),
                _ => None,
            })
        });
        // A key asked twice is skipped by the cache check below.
        let wanted: Vec<Key> = bound
            .chain(referenced)
            .filter_map(|id| key_of(doc, project_dir, id))
            .collect();
        for key in wanted {
            if self.cache.contains_key(&key) {
                continue;
            }
            let Some(exe) = (self.probe)() else {
                self.cache
                    .insert(key, ColumnState::Failed(PROBE_NOT_FOUND.to_string()));
                continue;
            };
            self.cache.insert(key.clone(), ColumnState::Loading);
            let ((url, table), run) = (key.clone(), self.run);
            let id = self
                .tasks
                .spawn("dbprobe", move |r| r.done(run(&exe, &url, &table)));
            self.keys.insert(id, key);
        }
    }

    #[note("dataset-columns-datasetcolumns-pump")]
    pub fn pump(&mut self) -> bool {
        let mut changed = false;
        for (id, event) in self.tasks.drain() {
            let state = match event {
                TaskEvent::Done(Ok(cols)) => ColumnState::Ready(cols),
                TaskEvent::Done(Err(e)) | TaskEvent::Failed(e) => ColumnState::Failed(e),
                TaskEvent::Progress(_) => continue,
            };
            if let Some(key) = self.keys.remove(&id) {
                self.cache.insert(key, state);
                changed = true;
            }
        }
        changed
    }
}

fn key_of(doc: &Document, project_dir: Option<&Path>, dataset_id: &str) -> Option<Key> {
    let src = dataset_source(doc, dataset_id)?;
    Some((make_url_absolute(&src.url, project_dir), src.table))
}

#[note("dataset-columns-probe-next-to-the-ide")]
pub fn probe_next_to_the_ide() -> Option<PathBuf> {
    let exe = std::env::current_exe().ok()?;
    let probe = exe
        .parent()?
        .join(format!("crawrad-dbprobe{}", std::env::consts::EXE_SUFFIX));
    probe.exists().then_some(probe)
}

#[note("dataset-columns-probe")]
pub fn probe(exe: &Path, url: &str, table: &str) -> Answer {
    parse_answer(&run_probe(exe, &["--url", url, "--table", table])?)
}

/// Runs `crawrad-dbprobe` with `args` under [`PROBE_TIME_LIMIT`] and hands back
/// what it printed — the one way both the columns and the design-time rows
/// reach the probe.
pub(crate) fn run_probe(exe: &Path, args: &[&str]) -> Result<String, String> {
    let mut child = crawrad_os::process::no_console(exe)
        .args(args)
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::null())
        .spawn()
        .map_err(|e| format!("could not start crawrad-dbprobe: {e}"))?;
    let start = Instant::now();
    while child.try_wait().map_err(|e| e.to_string())?.is_none() {
        if start.elapsed() > PROBE_TIME_LIMIT {
            let _ = child.kill();
            let _ = child.wait();
            return Err(format!(
                "crawrad-dbprobe took longer than {} s",
                PROBE_TIME_LIMIT.as_secs()
            ));
        }
        std::thread::sleep(Duration::from_millis(50));
    }
    let out = child.wait_with_output().map_err(|e| e.to_string())?;
    Ok(String::from_utf8_lossy(&out.stdout).into_owned())
}

#[note("dataset-columns-parse-answer")]
pub fn parse_answer(text: &str) -> Answer {
    let json: serde_json::Value = serde_json::from_str(text.trim())
        .map_err(|e| format!("crawrad-dbprobe answered something that is not JSON: {e}"))?;
    if let Some(cols) = json.get("columns").and_then(|c| c.as_array()) {
        return Ok(cols
            .iter()
            .filter_map(|c| c.as_str().map(str::to_string))
            .collect());
    }
    Err(json
        .get("error")
        .and_then(|e| e.as_str())
        .unwrap_or("crawrad-dbprobe gave no answer")
        .to_string())
}

#[cfg(test)]
#[path = "test/dataset_columns/test.rs"]
mod tests;
