use crawrad_note::note;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

#[note("memory-aefosmemory")]
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct AefosMemory {
    /// The raw text of global instructions shared with AI assistants.
    pub instructions: String,
    /// Last modified ISO timestamp.
    #[serde(default)]
    pub updated_at: Option<String>,
}

impl AefosMemory {
    pub fn new(instructions: impl Into<String>) -> Self {
        Self {
            instructions: instructions.into(),
            updated_at: None,
        }
    }

    #[note("memory-aefosmemory-character-count")]
    pub fn character_count(&self) -> usize {
        self.instructions.chars().count()
    }

    #[note("memory-aefosmemory-estimated-tokens")]
    pub fn estimated_tokens(&self) -> usize {
        self.character_count().div_ceil(4)
    }

    #[note("memory-aefosmemory-default-path")]
    pub fn default_path() -> PathBuf {
        if let Some(app_data) = std::env::var_os("APPDATA") {
            let base = PathBuf::from(&app_data);
            let crawrad = base.join("CrawRAD").join("memory.json");
            if !crawrad.exists() {
                let legacy = base.join("Aefos").join("memory.json");
                if legacy.exists() {
                    return legacy;
                }
            }
            crawrad
        } else if let Some(home) =
            std::env::var_os("HOME").or_else(|| std::env::var_os("USERPROFILE"))
        {
            let base = PathBuf::from(&home);
            let crawrad = base.join(".crawrad").join("memory.json");
            if !crawrad.exists() {
                let legacy = base.join(".aefos").join("memory.json");
                if legacy.exists() {
                    return legacy;
                }
            }
            crawrad
        } else {
            PathBuf::from("memory.json")
        }
    }

    #[note("memory-aefosmemory-load-from-file")]
    pub fn load_from_file(path: &Path) -> Result<Self, std::io::Error> {
        if !path.exists() {
            return Ok(Self::default());
        }
        let content = std::fs::read_to_string(path)?;
        let memory: Self = serde_json::from_str(&content)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
        Ok(memory)
    }

    #[note("memory-aefosmemory-save")]
    pub fn save(&self) -> Result<(), std::io::Error> {
        let dest = if let Some(app_data) = std::env::var_os("APPDATA") {
            PathBuf::from(app_data).join("CrawRAD").join("memory.json")
        } else if let Some(home) =
            std::env::var_os("HOME").or_else(|| std::env::var_os("USERPROFILE"))
        {
            PathBuf::from(home).join(".crawrad").join("memory.json")
        } else {
            PathBuf::from("memory.json")
        };
        self.save_to_file(&dest)
    }

    #[note("memory-aefosmemory-save-to-file")]
    pub fn save_to_file(&self, path: &Path) -> Result<(), std::io::Error> {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let json = serde_json::to_string_pretty(self)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
        let tmp = path.with_extension("tmp");
        std::fs::write(&tmp, json)?;
        let _ = std::fs::remove_file(path);
        std::fs::rename(&tmp, path)?;
        Ok(())
    }
}
