//! Type conversion between data source and visual UI target.

use crawrad_note::note;
use std::fmt::Display;
use std::marker::PhantomData;
use std::str::FromStr;

#[note("converter-valueconverter")]
pub trait ValueConverter<S, T>: Send + Sync + 'static {
    #[note("converter-valueconverter-convert")]
    fn convert(&self, source: &S) -> Result<T, String>;

    #[note("converter-valueconverter-convert-back")]
    fn convert_back(&self, target: &T) -> Result<S, String>;
}

#[note("converter-identityconverter")]
#[derive(Debug, Clone, Copy, Default)]
pub struct IdentityConverter<T> {
    _marker: PhantomData<T>,
}

impl<T> IdentityConverter<T> {
    pub fn new() -> Self {
        Self {
            _marker: PhantomData,
        }
    }
}

impl<T: Clone + Send + Sync + 'static> ValueConverter<T, T> for IdentityConverter<T> {
    fn convert(&self, source: &T) -> Result<T, String> {
        Ok(source.clone())
    }

    fn convert_back(&self, target: &T) -> Result<T, String> {
        Ok(target.clone())
    }
}

#[note("converter-displayfromstrconverter")]
#[derive(Debug, Clone, Copy, Default)]
pub struct DisplayFromStrConverter<S> {
    _marker: PhantomData<S>,
}

impl<S> DisplayFromStrConverter<S> {
    pub fn new() -> Self {
        Self {
            _marker: PhantomData,
        }
    }
}

impl<S> ValueConverter<S, String> for DisplayFromStrConverter<S>
where
    S: Display + FromStr + Clone + Send + Sync + 'static,
    <S as FromStr>::Err: Display,
{
    fn convert(&self, source: &S) -> Result<String, String> {
        Ok(source.to_string())
    }

    fn convert_back(&self, target: &String) -> Result<S, String> {
        target.parse::<S>().map_err(|e| e.to_string())
    }
}

#[note("converter-closureconverter")]
pub struct ClosureConverter<S, T, F, B>
where
    F: Fn(&S) -> Result<T, String> + Send + Sync + 'static,
    B: Fn(&T) -> Result<S, String> + Send + Sync + 'static,
{
    forward: F,
    backward: B,
    _marker: PhantomData<(S, T)>,
}

impl<S, T, F, B> ClosureConverter<S, T, F, B>
where
    F: Fn(&S) -> Result<T, String> + Send + Sync + 'static,
    B: Fn(&T) -> Result<S, String> + Send + Sync + 'static,
{
    pub fn new(forward: F, backward: B) -> Self {
        Self {
            forward,
            backward,
            _marker: PhantomData,
        }
    }
}

impl<S, T, F, B> ValueConverter<S, T> for ClosureConverter<S, T, F, B>
where
    S: Send + Sync + 'static,
    T: Send + Sync + 'static,
    F: Fn(&S) -> Result<T, String> + Send + Sync + 'static,
    B: Fn(&T) -> Result<S, String> + Send + Sync + 'static,
{
    fn convert(&self, source: &S) -> Result<T, String> {
        (self.forward)(source)
    }

    fn convert_back(&self, target: &T) -> Result<S, String> {
        (self.backward)(target)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn identity_converter() {
        let conv = IdentityConverter::<i32>::new();
        assert_eq!(conv.convert(&42), Ok(42));
        assert_eq!(conv.convert_back(&42), Ok(42));
    }

    #[test]
    fn display_from_str_converter() {
        let conv = DisplayFromStrConverter::<f64>::new();
        assert_eq!(conv.convert(&123.45), Ok("123.45".to_string()));
        assert_eq!(conv.convert_back(&"123.45".to_string()), Ok(123.45));
        assert!(conv.convert_back(&"not-a-number".to_string()).is_err());
    }

    #[test]
    fn closure_converter() {
        let conv = ClosureConverter::new(|num: &i32| Ok(num * 2), |num: &i32| Ok(num / 2));
        assert_eq!(conv.convert(&10), Ok(20));
        assert_eq!(conv.convert_back(&20), Ok(10));
    }
}
