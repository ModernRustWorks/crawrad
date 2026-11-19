//! Storage backend abstraction decoupling dataset state and persistence from UI adapters (ADR 0041).
//!
//! Provides the [`RecordStore`] trait defining cursor navigation, transactional edit buffers,
//! and notification callbacks, alongside the in-memory [`MemoryStore`] implementation.

use std::sync::{Arc, Mutex};

use crate::adapter::dataset::{DataRecord, DatasetState};
use crawrad_note::note;

#[note("adapter-store-recordstore")]
pub trait RecordStore: Send + Sync {
    #[note("adapter-store-recordstore-columns")]
    fn columns(&self) -> Vec<String>;

    #[note("adapter-store-recordstore-state")]
    fn state(&self) -> DatasetState;

    #[note("adapter-store-recordstore-len")]
    fn len(&self) -> usize;

    #[note("adapter-store-recordstore-is-empty")]
    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    #[note("adapter-store-recordstore-position")]
    fn position(&self) -> Option<usize>;

    #[note("adapter-store-recordstore-move-to")]
    fn move_to(&self, index: usize) -> Result<(), String>;

    #[note("adapter-store-recordstore-record")]
    fn record(&self, index: usize) -> Option<DataRecord>;

    #[note("adapter-store-recordstore-field-text")]
    fn field_text(&self, name: &str) -> Option<String>;

    #[note("adapter-store-recordstore-edit")]
    fn edit(&self) -> Result<(), String>;

    #[note("adapter-store-recordstore-insert")]
    fn insert(&self) -> Result<(), String>;

    #[note("adapter-store-recordstore-set-field-text")]
    fn set_field_text(&self, name: &str, text: &str) -> Result<(), String>;

    #[note("adapter-store-recordstore-post")]
    fn post(&self) -> Result<(), String>;

    #[note("adapter-store-recordstore-cancel")]
    fn cancel(&self) -> Result<(), String>;

    #[note("adapter-store-recordstore-delete")]
    fn delete(&self) -> Result<(), String>;

    #[note("adapter-store-recordstore-refresh")]
    fn refresh(&self) -> Result<(), String>;

    #[note("adapter-store-recordstore-on-change")]
    fn on_change(&self, callback: Box<dyn Fn() + Send + Sync>);
}

struct MemoryStoreInner {
    columns: Vec<String>,
    records: Vec<DataRecord>,
    cursor: Option<usize>,
    state: DatasetState,
    edit_snapshot: Option<DataRecord>,
    edit_buffer: Option<DataRecord>,
    listeners: Vec<Arc<dyn Fn() + Send + Sync>>,
}

#[note("adapter-store-memorystore")]
pub struct MemoryStore {
    inner: Mutex<MemoryStoreInner>,
}

impl MemoryStore {
    #[note("adapter-store-memorystore-new")]
    pub fn new(schema_fields: &[&str], records: Vec<DataRecord>) -> Self {
        let has_records = !records.is_empty();
        let initial_state = if has_records {
            DatasetState::Browse
        } else {
            DatasetState::Inactive
        };
        let cursor = if has_records { Some(0) } else { None };
        let columns = schema_fields.iter().map(|s| (*s).to_string()).collect();

        Self {
            inner: Mutex::new(MemoryStoreInner {
                columns,
                records,
                cursor,
                state: initial_state,
                edit_snapshot: None,
                edit_buffer: None,
                listeners: Vec::new(),
            }),
        }
    }

    fn notify(&self) {
        let listeners = {
            let guard = self.inner.lock().expect("lock memory store");
            guard.listeners.clone()
        };
        for listener in listeners {
            listener();
        }
    }
}

impl RecordStore for MemoryStore {
    fn columns(&self) -> Vec<String> {
        let guard = self.inner.lock().expect("lock memory store");
        guard.columns.clone()
    }

    fn state(&self) -> DatasetState {
        let guard = self.inner.lock().expect("lock memory store");
        guard.state
    }

    fn len(&self) -> usize {
        let guard = self.inner.lock().expect("lock memory store");
        guard.records.len()
    }

    fn position(&self) -> Option<usize> {
        let guard = self.inner.lock().expect("lock memory store");
        guard.cursor
    }

    fn move_to(&self, index: usize) -> Result<(), String> {
        {
            let mut guard = self.inner.lock().expect("lock memory store");
            if guard.state != DatasetState::Browse {
                return Err(format!(
                    "cannot navigate while dataset is in {} state; post or cancel first",
                    guard.state
                ));
            }
            if index >= guard.records.len() {
                return Err(format!("Index {index} out of bounds"));
            }
            if guard.cursor == Some(index) {
                return Ok(());
            }
            guard.cursor = Some(index);
        }
        self.notify();
        Ok(())
    }

    fn record(&self, index: usize) -> Option<DataRecord> {
        let guard = self.inner.lock().expect("lock memory store");
        guard.records.get(index).cloned()
    }

    fn field_text(&self, name: &str) -> Option<String> {
        let guard = self.inner.lock().expect("lock memory store");
        match guard.state {
            DatasetState::Edit | DatasetState::Insert => {
                if let Some(ref buf) = guard.edit_buffer {
                    buf.get(name).cloned().or_else(|| {
                        if guard.columns.contains(&name.to_string()) {
                            Some(String::new())
                        } else {
                            None
                        }
                    })
                } else if guard.columns.contains(&name.to_string()) {
                    Some(String::new())
                } else {
                    None
                }
            }
            DatasetState::Browse => {
                let idx = guard.cursor?;
                let rec = guard.records.get(idx)?;
                rec.get(name).cloned().or_else(|| {
                    if guard.columns.contains(&name.to_string()) {
                        Some(String::new())
                    } else {
                        None
                    }
                })
            }
            DatasetState::Inactive => {
                if guard.columns.contains(&name.to_string()) {
                    Some(String::new())
                } else {
                    None
                }
            }
        }
    }

    fn edit(&self) -> Result<(), String> {
        {
            let mut guard = self.inner.lock().expect("lock memory store");
            if guard.state == DatasetState::Edit {
                return Ok(());
            }
            if guard.state != DatasetState::Browse {
                return Err(format!("Cannot edit dataset in state {}", guard.state));
            }
            let idx = guard.cursor.ok_or("No active record to edit")?;
            let current_row = guard
                .records
                .get(idx)
                .cloned()
                .ok_or("No active record to edit")?;
            guard.edit_snapshot = Some(current_row.clone());
            guard.edit_buffer = Some(current_row);
            guard.state = DatasetState::Edit;
        }
        self.notify();
        Ok(())
    }

    fn insert(&self) -> Result<(), String> {
        {
            let mut guard = self.inner.lock().expect("lock memory store");
            let mut empty_record = DataRecord::new();
            for col in &guard.columns {
                empty_record.insert(col.clone(), String::new());
            }
            guard.edit_snapshot = None;
            guard.edit_buffer = Some(empty_record);
            guard.state = DatasetState::Insert;
        }
        self.notify();
        Ok(())
    }

    fn set_field_text(&self, name: &str, text: &str) -> Result<(), String> {
        {
            let mut guard = self.inner.lock().expect("lock memory store");
            if !guard.state.can_modify() {
                return Err(format!(
                    "cannot set field when dataset is in {} state",
                    guard.state
                ));
            }
            let buf = guard.edit_buffer.get_or_insert_with(DataRecord::new);
            buf.insert(name.to_string(), text.to_string());
        }
        self.notify();
        Ok(())
    }

    fn post(&self) -> Result<(), String> {
        {
            let mut guard = self.inner.lock().expect("lock memory store");
            if !guard.state.can_modify() {
                return Ok(());
            }
            let mut record = guard.edit_buffer.take().unwrap_or_default();
            for col in &guard.columns {
                record.entry(col.clone()).or_default();
            }

            match guard.state {
                DatasetState::Edit => {
                    let len = guard.records.len();
                    if let Some(idx) = guard.cursor.filter(|i| *i < len) {
                        guard.records[idx] = record;
                    }
                }
                DatasetState::Insert => {
                    guard.records.push(record);
                    guard.cursor = Some(guard.records.len() - 1);
                }
                _ => {}
            }

            guard.edit_snapshot = None;
            guard.state = DatasetState::Browse;
        }
        self.notify();
        Ok(())
    }

    fn cancel(&self) -> Result<(), String> {
        {
            let mut guard = self.inner.lock().expect("lock memory store");
            if !guard.state.can_modify() {
                return Ok(());
            }
            guard.edit_buffer = None;
            guard.edit_snapshot = None;
            guard.state = if guard.records.is_empty() {
                DatasetState::Inactive
            } else {
                DatasetState::Browse
            };
        }
        self.notify();
        Ok(())
    }

    fn delete(&self) -> Result<(), String> {
        {
            let mut guard = self.inner.lock().expect("lock memory store");
            if let Some(idx) = guard.cursor {
                if idx >= guard.records.len() {
                    return Err("No active record to delete".to_string());
                }
                guard.records.remove(idx);
                if guard.records.is_empty() {
                    guard.cursor = None;
                    guard.state = DatasetState::Inactive;
                } else {
                    if idx >= guard.records.len() {
                        guard.cursor = Some(guard.records.len() - 1);
                    } else {
                        guard.cursor = Some(idx);
                    }
                    guard.state = DatasetState::Browse;
                }
            } else {
                return Err("No active record to delete".to_string());
            }
        }
        self.notify();
        Ok(())
    }

    fn refresh(&self) -> Result<(), String> {
        Ok(())
    }

    fn on_change(&self, callback: Box<dyn Fn() + Send + Sync>) {
        let mut guard = self.inner.lock().expect("lock memory store");
        guard.listeners.push(Arc::from(callback));
    }
}
