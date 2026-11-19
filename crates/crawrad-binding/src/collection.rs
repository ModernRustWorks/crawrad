//! Collection binding providing cursor navigation and virtual windowing (ADR 0041).
//!
//! Ideal for tabular controls such as `Grid`, `ListBox`, and `TreeView`,
//! enabling 60 FPS GPU rendering across large data sets through virtualization.

use std::sync::{Arc, Mutex};

use crate::observable::SubscriptionId;
use crawrad_note::note;

#[note("collection-collectionchange")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CollectionChange {
    /// The entire collection was reset or reloaded.
    Reset,
    /// An item was inserted at the specified index.
    ItemAdded { index: usize },
    /// An item was removed from the specified index.
    ItemRemoved { index: usize },
    /// An item at the specified index was updated.
    ItemUpdated { index: usize },
}

type CursorListener = Arc<dyn Fn(Option<usize>) + Send + Sync + 'static>;
type CollectionListener = Arc<dyn Fn(&CollectionChange) + Send + Sync + 'static>;

struct CollectionInner<T> {
    items: Vec<T>,
    cursor: Option<usize>,
    cursor_listeners: Vec<(SubscriptionId, CursorListener)>,
    change_listeners: Vec<(SubscriptionId, CollectionListener)>,
}

#[note("collection-collectionbinding")]
pub struct CollectionBinding<T> {
    inner: Arc<Mutex<CollectionInner<T>>>,
}

impl<T> Clone for CollectionBinding<T> {
    fn clone(&self) -> Self {
        Self {
            inner: Arc::clone(&self.inner),
        }
    }
}

impl<T: std::fmt::Debug> std::fmt::Debug for CollectionBinding<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Ok(guard) = self.inner.lock() {
            f.debug_struct("CollectionBinding")
                .field("len", &guard.items.len())
                .field("cursor", &guard.cursor)
                .finish()
        } else {
            f.debug_struct("CollectionBinding")
                .field("state", &"<poisoned>")
                .finish()
        }
    }
}

impl<T: Clone + Send + Sync + 'static> CollectionBinding<T> {
    #[note("collection-collectionbinding-new")]
    pub fn new() -> Self {
        Self {
            inner: Arc::new(Mutex::new(CollectionInner {
                items: Vec::new(),
                cursor: None,
                cursor_listeners: Vec::new(),
                change_listeners: Vec::new(),
            })),
        }
    }

    #[note("collection-collectionbinding-from-vec")]
    pub fn from_vec(items: Vec<T>) -> Self {
        let cursor = if items.is_empty() { None } else { Some(0) };
        Self {
            inner: Arc::new(Mutex::new(CollectionInner {
                items,
                cursor,
                cursor_listeners: Vec::new(),
                change_listeners: Vec::new(),
            })),
        }
    }

    #[note("collection-collectionbinding-len")]
    pub fn len(&self) -> usize {
        let guard = self.inner.lock().expect("collection mutex poisoned");
        guard.items.len()
    }

    #[note("collection-collectionbinding-is-empty")]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    #[note("collection-collectionbinding-cursor")]
    pub fn cursor(&self) -> Option<usize> {
        let guard = self.inner.lock().expect("collection mutex poisoned");
        guard.cursor
    }

    #[note("collection-collectionbinding-current-item")]
    pub fn current_item(&self) -> Option<T> {
        let guard = self.inner.lock().expect("collection mutex poisoned");
        guard.cursor.and_then(|idx| guard.items.get(idx).cloned())
    }

    #[note("collection-collectionbinding-get")]
    pub fn get(&self, index: usize) -> Option<T> {
        let guard = self.inner.lock().expect("collection mutex poisoned");
        guard.items.get(index).cloned()
    }

    #[note("collection-collectionbinding-window")]
    pub fn window(&self, offset: usize, limit: usize) -> Vec<T> {
        let guard = self.inner.lock().expect("collection mutex poisoned");
        let total = guard.items.len();
        if offset >= total || limit == 0 {
            return Vec::new();
        }
        let end = (offset + limit).min(total);
        guard.items[offset..end].to_vec()
    }

    #[note("collection-collectionbinding-set-cursor")]
    pub fn set_cursor(&self, new_cursor: Option<usize>) -> bool {
        let (changed, listeners) = {
            let mut guard = self.inner.lock().expect("collection mutex poisoned");
            let valid_cursor = match new_cursor {
                Some(idx) if idx < guard.items.len() => Some(idx),
                _ => None,
            };

            if guard.cursor == valid_cursor {
                (false, Vec::new())
            } else {
                guard.cursor = valid_cursor;
                let listeners: Vec<_> = guard
                    .cursor_listeners
                    .iter()
                    .map(|(_, l)| Arc::clone(l))
                    .collect();
                (true, listeners)
            }
        };

        if changed {
            for listener in listeners {
                listener(new_cursor);
            }
        }

        changed
    }

    #[note("collection-collectionbinding-first")]
    pub fn first(&self) -> bool {
        let len = self.len();
        if len > 0 {
            self.set_cursor(Some(0))
        } else {
            self.set_cursor(None)
        }
    }

    #[note("collection-collectionbinding-last")]
    pub fn last(&self) -> bool {
        let len = self.len();
        if len > 0 {
            self.set_cursor(Some(len - 1))
        } else {
            self.set_cursor(None)
        }
    }

    #[note("collection-collectionbinding-next")]
    pub fn next(&self) -> bool {
        let current = self.cursor();
        let len = self.len();
        match current {
            Some(idx) if idx + 1 < len => self.set_cursor(Some(idx + 1)),
            None if len > 0 => self.set_cursor(Some(0)),
            _ => false,
        }
    }

    #[note("collection-collectionbinding-previous")]
    pub fn previous(&self) -> bool {
        let current = self.cursor();
        match current {
            Some(idx) if idx > 0 => self.set_cursor(Some(idx - 1)),
            _ => false,
        }
    }

    #[note("collection-collectionbinding-push")]
    pub fn push(&self, item: T) {
        let (change_listeners, cursor_listeners, new_cursor) = {
            let mut guard = self.inner.lock().expect("collection mutex poisoned");
            guard.items.push(item);
            let mut cursor_changed = false;
            if guard.cursor.is_none() {
                guard.cursor = Some(0);
                cursor_changed = true;
            }

            let c_listeners: Vec<_> = guard
                .change_listeners
                .iter()
                .map(|(_, l)| Arc::clone(l))
                .collect();

            let cur_listeners = if cursor_changed {
                guard
                    .cursor_listeners
                    .iter()
                    .map(|(_, l)| Arc::clone(l))
                    .collect()
            } else {
                Vec::new()
            };

            (c_listeners, cur_listeners, guard.cursor)
        };

        let change = CollectionChange::ItemAdded {
            index: self.len() - 1,
        };
        for listener in change_listeners {
            listener(&change);
        }
        for listener in cursor_listeners {
            listener(new_cursor);
        }
    }

    #[note("collection-collectionbinding-remove")]
    pub fn remove(&self, index: usize) -> Option<T> {
        let (removed, change_listeners, cursor_listeners, new_cursor) = {
            let mut guard = self.inner.lock().expect("collection mutex poisoned");
            if index >= guard.items.len() {
                return None;
            }

            let item = guard.items.remove(index);
            let mut cursor_changed = false;

            if guard.items.is_empty() {
                if guard.cursor.is_some() {
                    guard.cursor = None;
                    cursor_changed = true;
                }
            } else if let Some(cur) = guard.cursor
                && cur >= guard.items.len()
            {
                guard.cursor = Some(guard.items.len() - 1);
                cursor_changed = true;
            }

            let c_listeners: Vec<_> = guard
                .change_listeners
                .iter()
                .map(|(_, l)| Arc::clone(l))
                .collect();

            let cur_listeners = if cursor_changed {
                guard
                    .cursor_listeners
                    .iter()
                    .map(|(_, l)| Arc::clone(l))
                    .collect()
            } else {
                Vec::new()
            };

            (Some(item), c_listeners, cur_listeners, guard.cursor)
        };

        let change = CollectionChange::ItemRemoved { index };
        for listener in change_listeners {
            listener(&change);
        }
        for listener in cursor_listeners {
            listener(new_cursor);
        }

        removed
    }

    #[note("collection-collectionbinding-reset")]
    pub fn reset(&self, new_items: Vec<T>) {
        let (change_listeners, cursor_listeners, new_cursor) = {
            let mut guard = self.inner.lock().expect("collection mutex poisoned");
            guard.items = new_items;
            guard.cursor = if guard.items.is_empty() {
                None
            } else {
                Some(0)
            };

            let c_listeners: Vec<_> = guard
                .change_listeners
                .iter()
                .map(|(_, l)| Arc::clone(l))
                .collect();

            let cur_listeners: Vec<_> = guard
                .cursor_listeners
                .iter()
                .map(|(_, l)| Arc::clone(l))
                .collect();

            (c_listeners, cur_listeners, guard.cursor)
        };

        for listener in change_listeners {
            listener(&CollectionChange::Reset);
        }
        for listener in cursor_listeners {
            listener(new_cursor);
        }
    }

    #[note("collection-collectionbinding-on-cursor-changed")]
    pub fn on_cursor_changed(
        &self,
        callback: impl Fn(Option<usize>) + Send + Sync + 'static,
    ) -> SubscriptionId {
        let id = SubscriptionId::next();
        let mut guard = self.inner.lock().expect("collection mutex poisoned");
        guard.cursor_listeners.push((id, Arc::new(callback)));
        id
    }

    #[note("collection-collectionbinding-on-collection-changed")]
    pub fn on_collection_changed(
        &self,
        callback: impl Fn(&CollectionChange) + Send + Sync + 'static,
    ) -> SubscriptionId {
        let id = SubscriptionId::next();
        let mut guard = self.inner.lock().expect("collection mutex poisoned");
        guard.change_listeners.push((id, Arc::new(callback)));
        id
    }
}

impl<T: Clone + Send + Sync + 'static> Default for CollectionBinding<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};

    #[test]
    fn cursor_navigation() {
        let col = CollectionBinding::from_vec(vec!["Item A", "Item B", "Item C"]);
        assert_eq!(col.len(), 3);
        assert_eq!(col.cursor(), Some(0));
        assert_eq!(col.current_item(), Some("Item A"));

        assert!(col.next());
        assert_eq!(col.cursor(), Some(1));
        assert_eq!(col.current_item(), Some("Item B"));

        assert!(col.next());
        assert_eq!(col.cursor(), Some(2));
        assert_eq!(col.current_item(), Some("Item C"));

        // At end, next returns false
        assert!(!col.next());
        assert_eq!(col.cursor(), Some(2));

        assert!(col.previous());
        assert_eq!(col.cursor(), Some(1));

        assert!(col.first());
        assert_eq!(col.cursor(), Some(0));

        assert!(col.last());
        assert_eq!(col.cursor(), Some(2));
    }

    #[test]
    fn virtual_windowing() {
        let data: Vec<i32> = (0..1000).collect();
        let col = CollectionBinding::from_vec(data);

        let window = col.window(10, 5);
        assert_eq!(window, vec![10, 11, 12, 13, 14]);

        let end_window = col.window(998, 10);
        assert_eq!(end_window, vec![998, 999]);

        let out_of_bounds = col.window(1500, 10);
        assert!(out_of_bounds.is_empty());
    }

    #[test]
    fn mutation_and_events() {
        let col = CollectionBinding::<String>::new();
        let added_count = Arc::new(AtomicUsize::new(0));

        let added_clone = Arc::clone(&added_count);
        col.on_collection_changed(move |change| {
            if let CollectionChange::ItemAdded { .. } = change {
                added_clone.fetch_add(1, Ordering::SeqCst);
            }
        });

        col.push("First".to_string());
        col.push("Second".to_string());

        assert_eq!(col.len(), 2);
        assert_eq!(added_count.load(Ordering::SeqCst), 2);
        assert_eq!(col.cursor(), Some(0));

        let removed = col.remove(0);
        assert_eq!(removed, Some("First".to_string()));
        assert_eq!(col.len(), 1);
        assert_eq!(col.current_item(), Some("Second".to_string()));
    }
}
