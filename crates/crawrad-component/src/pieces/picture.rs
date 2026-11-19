// This file is EMITTED VERBATIM into the user's project by the code generator.
// It is the source the compiler reads, not a template.
//
// Rules that this imposes (enforced by the emitter gate):
// 1. no //! or #![...] outer attributes -- it is pasted into the middle of a file;
// 2. no dependency outside std -- the user did not ask for our crates;
// 3. no mention of our internal crates -- not even in comments;
// 4. every type must be public and every method must be documented in English;
// 5. no Portuguese anywhere in this file -- the user reads the code we emit.

/// A graphic picture asset holding binary or vector data.
///
/// Corresponds to the classic RAD `TPicture` / graphic holder.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct Picture {
    /// Raw binary data of the graphic asset.
    pub bytes: Vec<u8>,
    /// Optional resource path or file source name.
    pub source: Option<String>,
}

impl Picture {
    /// Creates an empty picture holder.
    pub const fn empty() -> Self {
        Self {
            bytes: Vec::new(),
            source: None,
        }
    }

    /// Whether this picture holds no graphic data.
    pub fn is_empty(&self) -> bool {
        self.bytes.is_empty()
    }

    /// Raw byte slice of the image.
    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }
}

impl std::fmt::Display for Picture {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(src) = &self.source {
            write!(f, "(Picture: {})", src)
        } else if self.is_empty() {
            write!(f, "(None)")
        } else {
            write!(f, "(Picture: {} bytes)", self.bytes.len())
        }
    }
}

impl std::str::FromStr for Picture {
    type Err = core::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let trimmed = s.trim();
        if trimmed.is_empty() || trimmed == "(None)" || trimmed == "(none)" {
            return Ok(Self::empty());
        }
        Ok(Self {
            bytes: Vec::new(),
            source: Some(trimmed.to_string()),
        })
    }
}
