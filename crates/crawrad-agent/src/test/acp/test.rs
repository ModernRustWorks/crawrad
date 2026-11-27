use super::*;
use std::path::{Path, PathBuf};
use std::sync::{Mutex, MutexGuard};

static TEST_LOCK: Mutex<()> = Mutex::new(());

/// RAII guard that serializes configuration directory access and injects a temporary directory.
/// Avoids mutating process environment variables (which requires unsafe in Rust 2024 and is
/// forbidden by `#![forbid(unsafe_code)]` per ADR 0009).
struct TempAppdataGuard {
    _lock: MutexGuard<'static, ()>,
    temp_dir: PathBuf,
}

impl TempAppdataGuard {
    fn new(test_name: &str) -> Self {
        let lock = TEST_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        let temp_dir = std::env::temp_dir().join(format!(
            "crawrad_test_acp_{test_name}_{}_{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_nanos())
                .unwrap_or(0)
        ));
        let _ = std::fs::remove_dir_all(&temp_dir);
        std::fs::create_dir_all(&temp_dir).expect("create temp appdata dir");
        AcpConfig::set_config_dir_override(Some(temp_dir.clone()));
        Self {
            _lock: lock,
            temp_dir,
        }
    }

    fn crawrad_config_path(&self) -> PathBuf {
        self.temp_dir.join("CrawRAD").join("acp_config.json")
    }

    fn aefos_config_path(&self) -> PathBuf {
        self.temp_dir.join("Aefos").join("acp_config.json")
    }

    fn write_config(&self, path: &Path, agent_id: &str, provider: &str) {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).expect("create parent directory");
        }
        let json = serde_json::json!({
            "agent_id": agent_id,
            "provider": provider,
            "model": "sonnet",
            "endpoint": "stdio:claude-agent",
            "endpoint_args": [],
            "auto_approve_reads": true,
            "auto_approve_plans": false,
            "enable_thinking": true,
            "allow_native_agent_tools": false
        });
        std::fs::write(path, serde_json::to_string_pretty(&json).unwrap())
            .expect("write test config file");
    }
}

impl Drop for TempAppdataGuard {
    fn drop(&mut self) {
        AcpConfig::set_config_dir_override(None);
        let _ = std::fs::remove_dir_all(&self.temp_dir);
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// TASK 1: Ensure acp_check never writes to the user configuration file
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn test_acp_check_never_modifies_config_file() {
    let guard = TempAppdataGuard::new("acp_check_purity");
    let crawrad_cfg = guard.crawrad_config_path();
    let aefos_cfg = guard.aefos_config_path();

    guard.write_config(&crawrad_cfg, "claude", "Claude Agent");
    guard.write_config(&aefos_cfg, "claude", "Claude Agent");

    let crawrad_bytes_before = std::fs::read(&crawrad_cfg).expect("read crawrad config before");
    let aefos_bytes_before = std::fs::read(&aefos_cfg).expect("read aefos config before");

    // Run the acp_check verification path
    crate::acp_check::run_acp_check();

    let crawrad_bytes_after = std::fs::read(&crawrad_cfg).expect("read crawrad config after");
    let aefos_bytes_after = std::fs::read(&aefos_cfg).expect("read aefos config after");

    assert_eq!(
        crawrad_bytes_before, crawrad_bytes_after,
        "acp_check must not modify CrawRAD's config file (byte-for-byte check failed)"
    );
    assert_eq!(
        aefos_bytes_before, aefos_bytes_after,
        "acp_check must not modify Aefos legacy config file (byte-for-byte check failed)"
    );
}

// ─────────────────────────────────────────────────────────────────────────────
// TASK 2: Separate config path intentions (read vs write)
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn test_load_reads_legacy_aefos_when_crawrad_does_not_exist() {
    let guard = TempAppdataGuard::new("read_legacy");
    let aefos_cfg = guard.aefos_config_path();
    guard.write_config(&aefos_cfg, "claude", "Claude Agent");

    assert!(!guard.crawrad_config_path().exists());
    assert!(aefos_cfg.exists());

    let loaded = AcpConfig::load();
    assert_eq!(loaded.agent_id, "claude");
    assert_eq!(loaded.provider, "Claude Agent");
    assert_eq!(AcpConfig::config_path_for_read(), aefos_cfg);
}

#[test]
fn test_save_writes_crawrad_and_leaves_aefos_untouched() {
    let guard = TempAppdataGuard::new("save_separation");
    let aefos_cfg = guard.aefos_config_path();
    let crawrad_cfg = guard.crawrad_config_path();

    guard.write_config(&aefos_cfg, "claude", "Claude Agent");
    let aefos_bytes_before = std::fs::read(&aefos_cfg).expect("read aefos before save");

    assert!(!crawrad_cfg.exists());

    let mut cfg = AcpConfig::load();
    assert_eq!(cfg.agent_id, "claude");

    // Mutate to an agent different from legacy
    cfg.switch_provider("OpenAI / Codex ACP Bridge");
    assert_eq!(cfg.agent_id, "codex");

    cfg.save().expect("save config");

    // CrawRAD file must now exist with the new configuration
    assert!(
        crawrad_cfg.exists(),
        "CrawRAD config file must be created on save"
    );
    let loaded_crawrad = AcpConfig::load_from_file(&crawrad_cfg).expect("read saved crawrad file");
    assert_eq!(loaded_crawrad.agent_id, "codex");

    // Aefos file must be completely untouched (byte-for-byte identical)
    let aefos_bytes_after = std::fs::read(&aefos_cfg).expect("read aefos after save");
    assert_eq!(
        aefos_bytes_before, aefos_bytes_after,
        "Aefos legacy file must not be modified when saving CrawRAD config"
    );
}

#[test]
fn test_load_prefers_crawrad_when_both_exist() {
    let guard = TempAppdataGuard::new("both_exist");
    let aefos_cfg = guard.aefos_config_path();
    let crawrad_cfg = guard.crawrad_config_path();

    guard.write_config(&aefos_cfg, "claude", "Claude Agent");
    guard.write_config(&crawrad_cfg, "codex", "OpenAI / Codex ACP Bridge");

    assert!(aefos_cfg.exists());
    assert!(crawrad_cfg.exists());

    let loaded = AcpConfig::load();
    assert_eq!(loaded.agent_id, "codex");
    assert_eq!(loaded.provider, "OpenAI / Codex ACP Bridge");
    assert_eq!(AcpConfig::config_path_for_read(), crawrad_cfg);
    assert_eq!(AcpConfig::config_path_for_write(), crawrad_cfg);
}
