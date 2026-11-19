//! Scalar property binding connecting single source and target values (ADR 0041).

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};

use crate::converter::{IdentityConverter, ValueConverter};
use crate::mode::BindingMode;
use crate::observable::{Observable, SubscriptionId};
use crawrad_note::note;

#[note("scalar-scalarbinding")]
pub struct ScalarBinding<S, T, C = IdentityConverter<S>> {
    source: Observable<S>,
    target: Observable<T>,
    mode: BindingMode,
    converter: Arc<C>,
    source_sub: Arc<Mutex<Option<SubscriptionId>>>,
    target_sub: Arc<Mutex<Option<SubscriptionId>>>,
    is_bound: Arc<AtomicBool>,
}

impl<S, T, C> Clone for ScalarBinding<S, T, C> {
    fn clone(&self) -> Self {
        Self {
            source: self.source.clone(),
            target: self.target.clone(),
            mode: self.mode,
            converter: Arc::clone(&self.converter),
            source_sub: Arc::clone(&self.source_sub),
            target_sub: Arc::clone(&self.target_sub),
            is_bound: Arc::clone(&self.is_bound),
        }
    }
}

impl<S, T, C> ScalarBinding<S, T, C> {
    #[note("scalar-scalarbinding-mode")]
    pub fn mode(&self) -> BindingMode {
        self.mode
    }

    #[note("scalar-scalarbinding-is-bound")]
    pub fn is_bound(&self) -> bool {
        self.is_bound.load(Ordering::SeqCst)
    }

    #[note("scalar-scalarbinding-source")]
    pub fn source(&self) -> &Observable<S> {
        &self.source
    }

    #[note("scalar-scalarbinding-target")]
    pub fn target(&self) -> &Observable<T> {
        &self.target
    }

    #[note("scalar-scalarbinding-unbind")]
    pub fn unbind(&self) {
        if !self.is_bound.swap(false, Ordering::SeqCst) {
            return;
        }

        if let Some(sub_id) = self.source_sub.lock().unwrap().take() {
            self.source.unsubscribe(sub_id);
        }

        if let Some(sub_id) = self.target_sub.lock().unwrap().take() {
            self.target.unsubscribe(sub_id);
        }
    }
}

impl<S, T, C> ScalarBinding<S, T, C>
where
    S: Clone + PartialEq + Send + Sync + 'static,
    T: Clone + PartialEq + Send + Sync + 'static,
    C: ValueConverter<S, T>,
{
    #[note("scalar-scalarbinding-new")]
    pub fn new(
        source: Observable<S>,
        target: Observable<T>,
        mode: BindingMode,
        converter: C,
    ) -> Self {
        Self {
            source,
            target,
            mode,
            converter: Arc::new(converter),
            source_sub: Arc::new(Mutex::new(None)),
            target_sub: Arc::new(Mutex::new(None)),
            is_bound: Arc::new(AtomicBool::new(false)),
        }
    }

    #[note("scalar-scalarbinding-sync-to-target")]
    pub fn sync_to_target(&self) -> Result<(), String> {
        let src_val = self.source.get();
        let converted = self.converter.convert(&src_val)?;
        self.target.set(converted);
        Ok(())
    }

    #[note("scalar-scalarbinding-sync-to-source")]
    pub fn sync_to_source(&self) -> Result<(), String> {
        let tgt_val = self.target.get();
        let converted_back = self.converter.convert_back(&tgt_val)?;
        self.source.set(converted_back);
        Ok(())
    }

    #[note("scalar-scalarbinding-bind")]
    pub fn bind(&self) -> Result<(), String> {
        if self.is_bound.swap(true, Ordering::SeqCst) {
            return Ok(()); // already bound
        }

        // 1. Initial synchronization
        match self.mode {
            BindingMode::OneWay | BindingMode::TwoWay | BindingMode::OneTime => {
                self.sync_to_target()?;
            }
            BindingMode::OneWayToSource => {
                self.sync_to_source()?;
            }
        }

        if !self.mode.is_continuous() {
            return Ok(());
        }

        // 2. Setup Source -> Target subscription if applicable
        if self.mode.propagates_source_to_target() {
            let target = self.target.clone();
            let converter = Arc::clone(&self.converter);
            let sub_id = self.source.subscribe(move |src_val| {
                if let Ok(converted) = converter.convert(src_val)
                    && target.get() != converted
                {
                    target.set(converted);
                }
            });
            *self.source_sub.lock().unwrap() = Some(sub_id);
        }

        // 3. Setup Target -> Source subscription if applicable
        if self.mode.propagates_target_to_source() {
            let source = self.source.clone();
            let converter = Arc::clone(&self.converter);

            let sub_id = self.target.subscribe(move |tgt_val| {
                if let Ok(converted_back) = converter.convert_back(tgt_val)
                    && source.get() != converted_back
                {
                    source.set(converted_back);
                }
            });
            *self.target_sub.lock().unwrap() = Some(sub_id);
        }

        Ok(())
    }
}

impl<T> ScalarBinding<T, T, IdentityConverter<T>>
where
    T: Clone + PartialEq + Send + Sync + 'static,
{
    #[note("scalar-scalarbinding-new-identity")]
    pub fn new_identity(source: Observable<T>, target: Observable<T>, mode: BindingMode) -> Self {
        Self::new(source, target, mode, IdentityConverter::new())
    }
}

impl<S, T, C> Drop for ScalarBinding<S, T, C> {
    fn drop(&mut self) {
        if Arc::strong_count(&self.is_bound) == 1 {
            self.unbind();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::converter::DisplayFromStrConverter;

    #[test]
    fn one_way_binding() {
        let source = Observable::new(10);
        let target = Observable::new(0);

        let binding =
            ScalarBinding::new_identity(source.clone(), target.clone(), BindingMode::OneWay);
        binding.bind().unwrap();

        // Initial sync
        assert_eq!(target.get(), 10);

        // Source change propagates to target
        source.set(20);
        assert_eq!(target.get(), 20);

        // Target change does NOT propagate to source
        target.set(99);
        assert_eq!(source.get(), 20);
    }

    #[test]
    fn two_way_binding_with_converter() {
        let source = Observable::new(42);
        let target = Observable::new("0".to_string());

        let binding = ScalarBinding::new(
            source.clone(),
            target.clone(),
            BindingMode::TwoWay,
            DisplayFromStrConverter::<i32>::new(),
        );
        binding.bind().unwrap();

        // Initial sync: source -> target
        assert_eq!(target.get(), "42");

        // UI change -> propagates to source
        target.set("100".to_string());
        assert_eq!(source.get(), 100);

        // Source change -> propagates to UI
        source.set(250);
        assert_eq!(target.get(), "250");
    }

    #[test]
    fn one_time_binding() {
        let source = Observable::new(55);
        let target = Observable::new(0);

        let binding =
            ScalarBinding::new_identity(source.clone(), target.clone(), BindingMode::OneTime);
        binding.bind().unwrap();

        // Initial sync happened
        assert_eq!(target.get(), 55);

        // Later changes do not propagate
        source.set(999);
        assert_eq!(target.get(), 55);
    }

    #[test]
    fn one_way_to_source_binding() {
        let source = Observable::new(0);
        let target = Observable::new(77);

        let binding = ScalarBinding::new_identity(
            source.clone(),
            target.clone(),
            BindingMode::OneWayToSource,
        );
        binding.bind().unwrap();

        // Initial sync: target -> source
        assert_eq!(source.get(), 77);

        // Target change propagates to source
        target.set(88);
        assert_eq!(source.get(), 88);

        // Source change does NOT propagate to target
        source.set(100);
        assert_eq!(target.get(), 88);
    }
}
