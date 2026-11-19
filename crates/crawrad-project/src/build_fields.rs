//! The Build dialog's plain controls — options, presets, environment, the
//! command page's lines and tabs — for the shell's one field binder.

use crate::build_settings::{
    BuildMode, BuildPage, BuildState, BuildTab, TargetPlatform, page_title,
};
use crawrad_actions::dialog_fields::{DialogFields, FieldKind, FieldValue};

/// The output tabs, left to right.
const TABS: [(&str, BuildTab); 3] = [
    ("bld_tab_output", BuildTab::Output),
    ("bld_tab_warnings", BuildTab::Warnings),
    ("bld_tab_errors", BuildTab::Errors),
];

/// The target presets and what each one selects.
const PRESETS: [(&str, &[TargetPlatform]); 5] = [
    ("bld_btn_select_all", TargetPlatform::ALL),
    ("bld_btn_deselect_all", &[]),
    (
        "bld_btn_preset_desktop",
        &[
            TargetPlatform::Win64,
            TargetPlatform::Linux64,
            TargetPlatform::MacSilicon,
        ],
    ),
    (
        "bld_btn_preset_mobile",
        &[TargetPlatform::AndroidArm64, TargetPlatform::IosArm64],
    ),
    ("bld_btn_preset_web", &[TargetPlatform::Wasm32]),
];

const FIELDS: &[(&str, FieldKind)] = &[
    ("bld_profile_btn", FieldKind::Button),
    ("bld_opt_clean", FieldKind::Check),
    ("bld_opt_workspace", FieldKind::Check),
    ("bld_opt_verbose", FieldKind::Check),
    ("bld_opt_messages", FieldKind::Check),
    ("bld_opt_all_features", FieldKind::Check),
    ("bld_opt_no_default", FieldKind::Check),
    ("bld_chip_empty", FieldKind::Label),
    ("bld_features_input", FieldKind::Text),
    ("bld_args_input", FieldKind::Text),
    ("bld_env_k_0", FieldKind::Text),
    ("bld_env_v_0", FieldKind::Text),
    ("bld_env_del_btn_0", FieldKind::Button),
    ("bld_env_add", FieldKind::Button),
    ("bld_btn_select_all", FieldKind::Button),
    ("bld_btn_deselect_all", FieldKind::Button),
    ("bld_btn_preset_desktop", FieldKind::Button),
    ("bld_btn_preset_mobile", FieldKind::Button),
    ("bld_btn_preset_web", FieldKind::Button),
    ("bld_cmd_title", FieldKind::Label),
    ("bld_cmd_meta", FieldKind::Label),
    ("bld_run_headline", FieldKind::Label),
    ("bld_run_counter", FieldKind::Label),
    ("bld_run_clock", FieldKind::Label),
    ("bld_tab_output", FieldKind::Button),
    ("bld_tab_warnings", FieldKind::Button),
    ("bld_tab_errors", FieldKind::Button),
];

impl BuildState {
    /// The option a check box holds.
    fn option(&mut self, field: &str) -> Option<&mut bool> {
        Some(match field {
            "bld_opt_clean" => &mut self.clean_before,
            "bld_opt_workspace" => &mut self.workspace,
            "bld_opt_verbose" => &mut self.verbose,
            "bld_opt_messages" => &mut self.show_messages,
            "bld_opt_all_features" => &mut self.all_features,
            "bld_opt_no_default" => &mut self.no_default_features,
            _ => return None,
        })
    }

    /// What a check box shows.
    fn option_on(&self, field: &str) -> Option<bool> {
        Some(match field {
            "bld_opt_clean" => self.clean_before,
            "bld_opt_workspace" => self.workspace,
            "bld_opt_verbose" => self.verbose,
            "bld_opt_messages" => self.show_messages,
            "bld_opt_all_features" => self.all_features,
            "bld_opt_no_default" => self.no_default_features,
            _ => return None,
        })
    }

    /// The command the page shows, on a command page.
    fn command_page(&self) -> Option<BuildMode> {
        match self.page {
            BuildPage::Command(m) => Some(m),
            BuildPage::Options | BuildPage::Size => None,
        }
    }

    /// A tab's caption: its count, and a dot on the one in front.
    fn tab_caption(&self, tab: BuildTab) -> String {
        let label = match tab {
            BuildTab::Output => ">_ Output".to_string(),
            BuildTab::Warnings => format!("\u{26a0} Warnings ({})", self.warnings_count),
            BuildTab::Errors => format!("\u{1f6ab} Errors ({})", self.errors_count),
        };
        let dot = if self.tab == tab { " \u{25cf}" } else { "" };
        format!("{label}{dot}")
    }
}

impl DialogFields for BuildState {
    fn fields(&self) -> &'static [(&'static str, FieldKind)] {
        FIELDS
    }

    fn value(&self, field: &str) -> Option<FieldValue> {
        let text = |t: String| Some(FieldValue::Text(t));
        let run = |i: usize| {
            let (headline, counter, clock, _) = self.command_view_info(self.command_page()?);
            Some(FieldValue::Text(
                [headline, counter, clock].into_iter().nth(i)?,
            ))
        };
        match field {
            "bld_profile_btn" => text(format!("{} \u{25be}", self.profile.label())),
            "bld_chip_empty" => text(
                if self.selected_targets.is_empty() {
                    "No targets selected"
                } else {
                    ""
                }
                .into(),
            ),
            "bld_features_input" => text(self.features.clone()),
            "bld_args_input" => text(self.extra_args.clone()),
            "bld_env_k_0" => self.env_vars.first().and_then(|(k, _)| text(k.clone())),
            "bld_env_v_0" => self.env_vars.first().and_then(|(_, v)| text(v.clone())),
            "bld_cmd_title" => self.command_page().and_then(|m| text(page_title(m).into())),
            "bld_cmd_meta" => self.command_page().and_then(|_| text(self.meta_line())),
            "bld_run_headline" => run(0),
            "bld_run_counter" => run(1),
            "bld_run_clock" => run(2),
            f => match TABS.iter().find(|(n, _)| *n == f) {
                Some((_, tab)) => text(self.tab_caption(*tab)),
                None => self.option_on(f).map(FieldValue::On),
            },
        }
    }

    fn input(&mut self, field: &str, value: FieldValue) {
        match (field, value) {
            ("bld_features_input", FieldValue::Text(t)) => self.features = t,
            ("bld_args_input", FieldValue::Text(t)) => self.extra_args = t,
            ("bld_env_k_0", FieldValue::Text(t)) => {
                if let Some((k, _)) = self.env_vars.first_mut() {
                    *k = t;
                }
            }
            ("bld_env_v_0", FieldValue::Text(t)) => {
                if let Some((_, v)) = self.env_vars.first_mut() {
                    *v = t;
                }
            }
            ("bld_profile_btn", _) => self.profile = self.profile.toggle(),
            ("bld_env_del_btn_0", _) => {
                if !self.env_vars.is_empty() {
                    self.env_vars.remove(0);
                }
            }
            ("bld_env_add", _) => {
                let (k, v) = if self.env_vars.is_empty() {
                    ("RUST_LOG", "info")
                } else {
                    ("KEY", "val")
                };
                self.env_vars.push((k.into(), v.into()));
            }
            (f, FieldValue::On(on)) if self.option(f).is_some() => {
                if let Some(slot) = self.option(f) {
                    *slot = on;
                }
            }
            (f, _) => {
                if let Some((_, targets)) = PRESETS.iter().find(|(n, _)| *n == f) {
                    self.selected_targets = targets.to_vec();
                    self.sort_targets();
                } else if let Some((_, tab)) = TABS.iter().find(|(n, _)| *n == f) {
                    self.tab = *tab;
                }
            }
        }
    }
}

#[cfg(test)]
#[path = "test/build_fields/test.rs"]
mod build_fields_test;
