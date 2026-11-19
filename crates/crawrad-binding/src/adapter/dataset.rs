//! Relational dataset adapter implementing the classic RAD lifecycle states (ADR 0041).
//!
//! Provides the canonical `Inactive`, `Browse`, `Edit`, and `Insert` state machine,
//! transactional `post()`, `cancel()`, and `delete()` operations, and automatic
//! transition to `Edit` when a bound visual control is typed into.

use std::collections::{BTreeMap, HashMap};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex, Weak};

use crate::adapter::store::{MemoryStore, RecordStore};
use crate::collection::CollectionBinding;
use crate::observable::Observable;
use crawrad_note::note;

#[note("adapter-dataset-datasetstate")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum DatasetState {
    /// Dataset is closed or not populated.
    #[default]
    Inactive,
    /// Dataset is open and positioned on a record in read mode.
    Browse,
    /// An existing record is currently being edited.
    Edit,
    /// A new record is being inserted.
    Insert,
}

impl DatasetState {
    #[note("adapter-dataset-datasetstate-is-open")]
    #[inline]
    pub const fn is_open(&self) -> bool {
        !matches!(self, Self::Inactive)
    }

    #[note("adapter-dataset-datasetstate-can-modify")]
    #[inline]
    pub const fn can_modify(&self) -> bool {
        matches!(self, Self::Edit | Self::Insert)
    }

    #[note("adapter-dataset-datasetstate-as-str")]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Inactive => "Inactive",
            Self::Browse => "Browse",
            Self::Edit => "Edit",
            Self::Insert => "Insert",
        }
    }
}

impl std::fmt::Display for DatasetState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[note("adapter-dataset-datarecord")]
pub type DataRecord = BTreeMap<String, String>;

#[note("adapter-dataset-datasetadapter")]
#[derive(Clone)]
pub struct DatasetAdapter {
    store: Arc<dyn RecordStore>,
    mirror: Mirror,
}

#[note("adapter-dataset-mirror")]
#[derive(Clone)]
struct Mirror {
    store: Weak<dyn RecordStore>,
    state: Observable<DatasetState>,
    collection: CollectionBinding<DataRecord>,
    fields: Arc<Mutex<HashMap<String, Observable<String>>>>,
    is_syncing: Arc<AtomicBool>,
    last_error: Observable<Option<String>>,
    editing_cell: Arc<Mutex<Option<String>>>,
    /// Cells whose text the store refused, and why. While any is here, `post` refuses:
    /// the cell shows a value the record does not hold.
    refused: Arc<Mutex<BTreeMap<String, String>>>,
}

impl DatasetAdapter {
    #[note("adapter-dataset-datasetadapter-new")]
    pub fn new(schema_fields: &[&str], records: Vec<DataRecord>) -> Self {
        let store = Arc::new(MemoryStore::new(schema_fields, records));
        Self::with_store(store)
    }

    #[note("adapter-dataset-datasetadapter-with-store")]
    pub fn with_store(store: Arc<dyn RecordStore>) -> Self {
        let initial_state = store.state();
        let state = Observable::new(initial_state);
        let collection = CollectionBinding::new();
        let is_syncing = Arc::new(AtomicBool::new(false));
        let last_error = Observable::new(None);
        let editing_cell = Arc::new(Mutex::new(None));
        let refused = Arc::new(Mutex::new(BTreeMap::new()));

        let mut fields_map = HashMap::new();
        for col in store.columns() {
            fields_map.insert(col, Observable::new(String::new()));
        }
        let fields = Arc::new(Mutex::new(fields_map));

        let mirror = Mirror {
            store: Arc::downgrade(&store),
            state,
            collection,
            fields,
            is_syncing,
            last_error,
            editing_cell,
            refused,
        };

        // Wire store notifications -> adapter mirror
        let on_change = mirror.clone();
        store.on_change(Box::new(move || on_change.refresh()));

        // Wire cursor navigation on collection -> store.move_to
        mirror.wire_cursor_sync();

        // Wire auto-edit for all initially known fields
        mirror.wire_all_fields();

        // Initial populate from store
        mirror.refresh();

        Self { store, mirror }
    }

    #[note("adapter-dataset-datasetadapter-state")]
    pub fn state(&self) -> &Observable<DatasetState> {
        &self.mirror.state
    }

    #[note("adapter-dataset-datasetadapter-collection")]
    pub fn collection(&self) -> &CollectionBinding<DataRecord> {
        &self.mirror.collection
    }

    #[note("adapter-dataset-datasetadapter-last-error")]
    pub fn last_error(&self) -> Observable<Option<String>> {
        self.mirror.last_error.clone()
    }

    #[note("adapter-dataset-datasetadapter-field")]
    pub fn field(&self, name: &str) -> Option<Observable<String>> {
        let guard = self.mirror.fields.lock().unwrap();
        guard.get(name).cloned()
    }

    #[note("adapter-dataset-datasetadapter-field-or-create")]
    pub fn field_or_create(&self, name: &str) -> Observable<String> {
        let mut guard = self.mirror.fields.lock().unwrap();
        if let Some(obs) = guard.get(name) {
            obs.clone()
        } else {
            let initial_text = self.store.field_text(name).unwrap_or_default();
            let obs = Observable::new(initial_text);
            self.mirror.wire_single_field(name.to_string(), &obs);
            guard.insert(name.to_string(), obs.clone());
            obs
        }
    }

    #[note("adapter-dataset-datasetadapter-edit")]
    pub fn edit(&self) -> Result<(), String> {
        self.mirror.report(self.store.edit())
    }

    #[note("adapter-dataset-datasetadapter-insert")]
    pub fn insert(&self) -> Result<(), String> {
        self.mirror.report(self.store.insert())
    }

    #[note("adapter-dataset-datasetadapter-post")]
    pub fn post(&self) -> Result<(), String> {
        let first_refused = self
            .mirror
            .refused
            .lock()
            .unwrap()
            .iter()
            .next()
            .map(|(f, why)| (f.clone(), why.clone()));
        if let Some((field, why)) = first_refused {
            return self.mirror.report(Err(format!(
                "cannot post: field '{field}' holds a value the dataset refused ({why})"
            )));
        }
        self.mirror.report(self.store.post())
    }

    #[note("adapter-dataset-datasetadapter-cancel")]
    pub fn cancel(&self) -> Result<(), String> {
        self.mirror.report(self.store.cancel())
    }

    #[note("adapter-dataset-datasetadapter-delete")]
    pub fn delete(&self) -> Result<(), String> {
        self.mirror.report(self.store.delete())
    }

    #[note("adapter-dataset-datasetadapter-refresh")]
    pub fn refresh(&self) -> Result<(), String> {
        self.mirror.report(self.store.refresh())
    }
}

impl Mirror {
    #[note("adapter-dataset-mirror-report")]
    fn report(&self, result: Result<(), String>) -> Result<(), String> {
        match &result {
            Ok(()) => {
                self.last_error.set(None);
            }
            Err(err) => {
                self.last_error.set(Some(err.clone()));
            }
        }
        result
    }

    fn refresh(&self) {
        let Some(store) = self.store.upgrade() else {
            return;
        };
        self.is_syncing.store(true, Ordering::SeqCst);

        // 1. Mirror state
        let current_state = store.state();
        if self.state.get() != current_state {
            self.state.set(current_state);
        }

        // 2. Mirror collection records and cursor
        let count = store.len();
        let records: Vec<DataRecord> = (0..count).filter_map(|i| store.record(i)).collect();
        self.collection.reset(records);
        self.collection.set_cursor(store.position());

        // 3. Mirror field cells
        let active_edit = self.editing_cell.lock().unwrap().clone();
        {
            let mut refused = self.refused.lock().unwrap();
            let guard = self.fields.lock().unwrap();
            for (name, obs) in guard.iter() {
                if Some(name) == active_edit.as_ref() {
                    continue;
                }
                refused.remove(name);
                let text = store.field_text(name).unwrap_or_default();
                if obs.get() != text {
                    obs.set(text);
                }
            }
        }

        self.is_syncing.store(false, Ordering::SeqCst);
    }

    fn wire_cursor_sync(&self) {
        let mirror = self.clone();
        self.collection.on_cursor_changed(move |new_cursor| {
            if mirror.is_syncing.load(Ordering::SeqCst) {
                return;
            }
            let Some(store) = mirror.store.upgrade() else {
                return;
            };
            let Some(target_idx) = new_cursor.filter(|i| store.position() != Some(*i)) else {
                return;
            };
            if mirror.report(store.move_to(target_idx)).is_err() {
                // Refused (editing): the grid snaps back to where the store still is.
                mirror.is_syncing.store(true, Ordering::SeqCst);
                mirror.collection.set_cursor(store.position());
                mirror.is_syncing.store(false, Ordering::SeqCst);
            }
        });
    }

    fn wire_all_fields(&self) {
        let guard = self.fields.lock().unwrap();
        for (name, obs) in guard.iter() {
            self.wire_single_field(name.clone(), obs);
        }
    }

    fn wire_single_field(&self, name: String, obs: &Observable<String>) {
        let mirror = self.clone();
        obs.subscribe(move |new_val| {
            if mirror.is_syncing.load(Ordering::SeqCst) {
                return;
            }
            let Some(store) = mirror.store.upgrade() else {
                return;
            };

            *mirror.editing_cell.lock().unwrap() = Some(name.clone());

            let typed = (|| {
                if store.state() == DatasetState::Browse {
                    store.edit()?;
                }
                store.set_field_text(&name, new_val)
            })();
            match &typed {
                Ok(()) => {
                    mirror.refused.lock().unwrap().remove(&name);
                }
                Err(reason) => {
                    mirror
                        .refused
                        .lock()
                        .unwrap()
                        .insert(name.clone(), reason.clone());
                }
            }
            // A refused text stays in the cell; `last_error` says why.
            let _ = mirror.report(typed);

            *mirror.editing_cell.lock().unwrap() = None;
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dataset_lifecycle_and_auto_edit() {
        let mut row1 = DataRecord::new();
        row1.insert("id".to_string(), "1".to_string());
        row1.insert("name".to_string(), "Alpha Corp".to_string());

        let mut row2 = DataRecord::new();
        row2.insert("id".to_string(), "2".to_string());
        row2.insert("name".to_string(), "Beta Ltd".to_string());

        let dataset = DatasetAdapter::new(&["id", "name"], vec![row1, row2]);

        assert_eq!(dataset.state().get(), DatasetState::Browse);
        let name_field = dataset.field("name").unwrap();
        assert_eq!(name_field.get(), "Alpha Corp");

        // User edits name field in UI -> Triggers auto-transition to Edit!
        name_field.set("Alpha Global Corporation".to_string());
        assert_eq!(dataset.state().get(), DatasetState::Edit);

        // Cancel reverts to original value
        dataset.cancel().unwrap();
        assert_eq!(dataset.state().get(), DatasetState::Browse);
        assert_eq!(name_field.get(), "Alpha Corp");

        // Edit and Post commits changes
        name_field.set("Alpha International".to_string());
        assert_eq!(dataset.state().get(), DatasetState::Edit);
        dataset.post().unwrap();
        assert_eq!(dataset.state().get(), DatasetState::Browse);
        assert_eq!(
            dataset
                .collection()
                .current_item()
                .unwrap()
                .get("name")
                .unwrap(),
            "Alpha International"
        );

        // Cursor navigation syncs fields
        dataset.collection().next();
        assert_eq!(name_field.get(), "Beta Ltd");
    }

    /// Dropping the adapter releases the store.
    ///
    /// Dropping the adapter releases the store: the callbacks it left there hold
    /// the store only weakly, so no circle keeps it alive.
    #[test]
    fn dropping_the_adapter_releases_the_store() {
        let store: Arc<dyn RecordStore> = Arc::new(MemoryStore::new(&["id"], Vec::new()));
        let adapter = DatasetAdapter::with_store(Arc::clone(&store));
        adapter.field_or_create("id");
        drop(adapter);
        assert_eq!(Arc::strong_count(&store), 1);
    }

    #[test]
    fn the_memory_store_refuses_navigation_while_editing() {
        let rows = ["Alpha", "Beta"]
            .iter()
            .map(|n| DataRecord::from([("name".to_string(), (*n).to_string())]))
            .collect();
        let dataset = DatasetAdapter::new(&["name"], rows);
        dataset.field("name").unwrap().set("Alpha 2".to_string());

        dataset.collection().next();
        assert_eq!(dataset.collection().cursor(), Some(0));
        assert!(dataset.last_error().get().is_some());
        assert_eq!(dataset.field("name").unwrap().get(), "Alpha 2");
    }

    #[test]
    fn dataset_insert_and_delete() {
        let dataset = DatasetAdapter::new(&["id", "name"], Vec::new());
        assert_eq!(dataset.state().get(), DatasetState::Inactive);

        dataset.insert().unwrap();
        assert_eq!(dataset.state().get(), DatasetState::Insert);

        let id_field = dataset.field("id").unwrap();
        let name_field = dataset.field("name").unwrap();

        id_field.set("100".to_string());
        name_field.set("New Customer".to_string());
        dataset.post().unwrap();

        assert_eq!(dataset.state().get(), DatasetState::Browse);
        assert_eq!(dataset.collection().len(), 1);
        assert_eq!(name_field.get(), "New Customer");

        dataset.delete().unwrap();
        assert_eq!(dataset.collection().len(), 0);
        assert_eq!(dataset.state().get(), DatasetState::Inactive);
    }
}
