//! Adapter for binding pure Rust business models and structs to visual UI controls.

use std::sync::Arc;

use crate::observable::Observable;
use crawrad_note::note;

#[note("adapter-model-modeladapter")]
#[derive(Clone)]
pub struct ModelAdapter<M> {
    model: Observable<M>,
}

impl<M> ModelAdapter<M> {
    #[note("adapter-model-modeladapter-new")]
    pub fn new(initial: M) -> Self {
        Self {
            model: Observable::new(initial),
        }
    }

    #[note("adapter-model-modeladapter-model")]
    pub fn model(&self) -> &Observable<M> {
        &self.model
    }
}

impl<M: Clone> ModelAdapter<M> {
    #[note("adapter-model-modeladapter-get")]
    pub fn get(&self) -> M {
        self.model.get()
    }
}

impl<M: Clone + PartialEq + Send + Sync + 'static> ModelAdapter<M> {
    #[note("adapter-model-modeladapter-set")]
    pub fn set(&self, new_model: M) -> bool {
        self.model.set(new_model)
    }

    #[note("adapter-model-modeladapter-bind-field")]
    pub fn bind_field<T, G, S>(&self, getter: G, setter: S) -> Observable<T>
    where
        T: Clone + PartialEq + Send + Sync + 'static,
        G: Fn(&M) -> T + Send + Sync + 'static,
        S: Fn(&mut M, T) + Send + Sync + 'static,
    {
        let initial_value = self.model.with(|m| getter(m));
        let field_obs = Observable::new(initial_value);

        // 1. When model changes, update field_obs (silent to prevent feedback ping-pong)
        let field_clone = field_obs.clone();
        let getter_arc = Arc::new(getter);
        let g_clone = Arc::clone(&getter_arc);
        self.model.subscribe(move |new_model| {
            let val = g_clone(new_model);
            if field_clone.get() != val {
                field_clone.set_silent(val);
            }
        });

        // 2. When field_obs changes, update the model field
        let model_clone = self.model.clone();
        let setter_arc = Arc::new(setter);
        field_obs.subscribe(move |new_val| {
            let mut current = model_clone.get();
            if getter_arc(&current) != *new_val {
                setter_arc(&mut current, new_val.clone());
                model_clone.set(current);
            }
        });

        field_obs
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone, PartialEq)]
    struct Employee {
        name: String,
        salary: f64,
        active: bool,
    }

    #[test]
    fn model_adapter_field_sync() {
        let adapter = ModelAdapter::new(Employee {
            name: "Alice".to_string(),
            salary: 75_000.0,
            active: true,
        });

        let name_field = adapter.bind_field(|e| e.name.clone(), |e, val| e.name = val);
        let salary_field = adapter.bind_field(|e| e.salary, |e, val| e.salary = val);
        let active_field = adapter.bind_field(|e| e.active, |e, val| e.active = val);

        // Initial values read correctly
        assert_eq!(name_field.get(), "Alice");
        assert_eq!(salary_field.get(), 75_000.0);
        assert!(active_field.get());

        // UI changes field
        name_field.set("Alice Smith".to_string());
        assert_eq!(adapter.get().name, "Alice Smith");

        salary_field.set(80_000.0);
        assert_eq!(adapter.get().salary, 80_000.0);

        active_field.set(false);
        assert!(!adapter.get().active);

        // Model updated as a whole (e.g. from backend response)
        adapter.set(Employee {
            name: "Bob".to_string(),
            salary: 92_000.0,
            active: true,
        });

        assert_eq!(name_field.get(), "Bob");
        assert_eq!(salary_field.get(), 92_000.0);
        assert!(active_field.get());
    }
}
