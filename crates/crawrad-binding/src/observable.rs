//! Observable reactive container for thread-safe value updates and notifications.

use crawrad_note::note;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex};

#[note("observable-subscriptionid")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SubscriptionId(u64);

static NEXT_SUBSCRIPTION_ID: AtomicU64 = AtomicU64::new(1);

impl SubscriptionId {
    #[note("observable-subscriptionid-next")]
    pub fn next() -> Self {
        Self(NEXT_SUBSCRIPTION_ID.fetch_add(1, Ordering::Relaxed))
    }
}

type Listener<T> = Arc<dyn Fn(&T) + Send + Sync + 'static>;

struct Inner<T> {
    value: T,
    listeners: Vec<(SubscriptionId, Listener<T>)>,
    is_notifying: bool,
}

#[note("observable-observable")]
pub struct Observable<T> {
    inner: Arc<Mutex<Inner<T>>>,
}

impl<T> Clone for Observable<T> {
    fn clone(&self) -> Self {
        Self {
            inner: Arc::clone(&self.inner),
        }
    }
}

impl<T: std::fmt::Debug> std::fmt::Debug for Observable<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Ok(guard) = self.inner.lock() {
            f.debug_struct("Observable")
                .field("value", &guard.value)
                .field("listeners_count", &guard.listeners.len())
                .finish()
        } else {
            f.debug_struct("Observable")
                .field("value", &"<poisoned>")
                .finish()
        }
    }
}

impl<T> Observable<T> {
    #[note("observable-observable-new")]
    pub fn new(value: T) -> Self {
        Self {
            inner: Arc::new(Mutex::new(Inner {
                value,
                listeners: Vec::new(),
                is_notifying: false,
            })),
        }
    }

    #[note("observable-observable-with")]
    pub fn with<R>(&self, f: impl FnOnce(&T) -> R) -> R {
        let guard = self.inner.lock().expect("observable mutex poisoned");
        f(&guard.value)
    }

    #[note("observable-observable-set-silent")]
    pub fn set_silent(&self, value: T) {
        let mut guard = self.inner.lock().expect("observable mutex poisoned");
        guard.value = value;
    }

    #[note("observable-observable-subscribe")]
    pub fn subscribe(&self, callback: impl Fn(&T) + Send + Sync + 'static) -> SubscriptionId {
        let id = SubscriptionId::next();
        let mut guard = self.inner.lock().expect("observable mutex poisoned");
        guard.listeners.push((id, Arc::new(callback)));
        id
    }

    #[note("observable-observable-unsubscribe")]
    pub fn unsubscribe(&self, id: SubscriptionId) -> bool {
        let mut guard = self.inner.lock().expect("observable mutex poisoned");
        if let Some(pos) = guard.listeners.iter().position(|(sub_id, _)| *sub_id == id) {
            guard.listeners.swap_remove(pos);
            true
        } else {
            false
        }
    }

    #[note("observable-observable-subscriber-count")]
    pub fn subscriber_count(&self) -> usize {
        let guard = self.inner.lock().expect("observable mutex poisoned");
        guard.listeners.len()
    }
}

impl<T: Clone> Observable<T> {
    #[note("observable-observable-get")]
    pub fn get(&self) -> T {
        self.with(|val| val.clone())
    }
}

impl<T: Clone + PartialEq> Observable<T> {
    #[note("observable-observable-set")]
    pub fn set(&self, new_value: T) -> bool {
        let (changed, listeners_snapshot) = {
            let mut guard = self.inner.lock().expect("observable mutex poisoned");
            if guard.value == new_value {
                (false, Vec::new())
            } else {
                guard.value = new_value.clone();
                if guard.is_notifying {
                    (true, Vec::new())
                } else {
                    guard.is_notifying = true;
                    let listeners: Vec<_> =
                        guard.listeners.iter().map(|(_, l)| Arc::clone(l)).collect();
                    (true, listeners)
                }
            }
        };

        if changed {
            for listener in listeners_snapshot {
                listener(&new_value);
            }
            let mut guard = self.inner.lock().expect("observable mutex poisoned");
            guard.is_notifying = false;
        }

        changed
    }

    #[note("observable-observable-set-force")]
    pub fn set_force(&self, new_value: T) {
        let listeners_snapshot = {
            let mut guard = self.inner.lock().expect("observable mutex poisoned");
            guard.value = new_value.clone();
            guard.is_notifying = true;
            let listeners: Vec<_> = guard.listeners.iter().map(|(_, l)| Arc::clone(l)).collect();
            listeners
        };

        for listener in listeners_snapshot {
            listener(&new_value);
        }

        let mut guard = self.inner.lock().expect("observable mutex poisoned");
        guard.is_notifying = false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::AtomicI32;

    #[test]
    fn observable_get_set() {
        let obs = Observable::new(10);
        assert_eq!(obs.get(), 10);
        assert!(obs.set(20));
        assert_eq!(obs.get(), 20);
        // Setting same value should return false and not notify
        assert!(!obs.set(20));
    }

    #[test]
    fn observable_notifications() {
        let obs = Observable::new("Initial".to_string());
        let count = Arc::new(AtomicI32::new(0));

        let count_clone = Arc::clone(&count);
        let sub_id = obs.subscribe(move |_val| {
            count_clone.fetch_add(1, Ordering::SeqCst);
        });

        obs.set("First".to_string());
        assert_eq!(count.load(Ordering::SeqCst), 1);

        // Same value does not notify
        obs.set("First".to_string());
        assert_eq!(count.load(Ordering::SeqCst), 1);

        obs.set("Second".to_string());
        assert_eq!(count.load(Ordering::SeqCst), 2);

        // Unsubscribe
        assert!(obs.unsubscribe(sub_id));
        obs.set("Third".to_string());
        assert_eq!(count.load(Ordering::SeqCst), 2);
    }

    #[test]
    fn observable_set_silent() {
        let obs = Observable::new(100);
        let notified = Arc::new(AtomicI32::new(0));

        let notified_clone = Arc::clone(&notified);
        obs.subscribe(move |_| {
            notified_clone.fetch_add(1, Ordering::SeqCst);
        });

        obs.set_silent(200);
        assert_eq!(obs.get(), 200);
        assert_eq!(notified.load(Ordering::SeqCst), 0);
    }
}
