use std::collections::{HashMap, HashSet};

use crate::profiles::Profile;
use crate::settings::registry::all_settings;
use crate::settings::writer::write_setting;
use crate::settings::{
    Constraint, FloatRange, IntRange, SettingDef, SettingValue, Tab,
};
use crate::settings::reader::{read_all, available_setting_ids};
use crate::profiles::storage as profile_storage;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum View {
    Settings,
    Review,
    Profiles,
    ProfileNameInput,
}

#[derive(Debug, Clone)]
pub enum Message {
    Quit,
    NextTab,
    PrevTab,
    NavigateUp,
    NavigateDown,
    AdjustLeft,
    AdjustRight,
    Toggle,
    OpenReview,
    ApplyChanges,
    CancelReview,
    SaveProfile,
    CreateProfile,
    OpenProfiles,
    DeleteProfile,
    Back,
    TypeChar(char),
    Backspace,
    ConfirmInput,
}

pub struct App {
    pub running: bool,
    pub view: View,
    pub tab: Tab,
    pub selected_row: usize,
    pub settings_defs: Vec<SettingDef>,
    pub available_ids: HashSet<String>,
    pub live_values: HashMap<String, SettingValue>,
    pub pending_changes: HashMap<String, SettingValue>,
    pub profile_names: Vec<String>,
    pub profile_selected: usize,
    pub status_message: Option<String>,
    pub input_buffer: String,
    pub name_input_return_view: View,
}

impl App {
    pub fn new() -> Self {
        let settings_defs = all_settings();
        let available_ids = available_setting_ids(&settings_defs);
        let live_values = read_all(&settings_defs);
        let profile_names = profile_storage::list().unwrap_or_default();

        Self {
            running: true,
            view: View::Settings,
            tab: Tab::Mouse,
            selected_row: 0,
            settings_defs,
            available_ids,
            live_values,
            pending_changes: HashMap::new(),
            profile_names,
            profile_selected: 0,
            status_message: None,
            input_buffer: String::new(),
            name_input_return_view: View::Settings,
        }
    }

    pub fn visible_settings(&self) -> Vec<&SettingDef> {
        let groups = self.tab.groups();
        self.settings_defs
            .iter()
            .filter(|s| groups.contains(&s.group) && self.available_ids.contains(s.id))
            .collect()
    }

    pub fn pending_change_count(&self) -> usize {
        self.pending_changes.len()
    }

    pub fn pending_diffs(&self) -> Vec<(&SettingDef, Option<&SettingValue>, &SettingValue)> {
        let mut diffs = Vec::new();
        for (id, new_val) in &self.pending_changes {
            if let Some(def) = self.settings_defs.iter().find(|d| d.id == id) {
                let old_val = self.live_values.get(id);
                if old_val.is_some_and(|old| old == new_val) {
                    continue;
                }
                diffs.push((def, old_val, new_val));
            }
        }
        diffs.sort_by_key(|(d, _, _)| d.id);
        diffs
    }

    pub fn any_requires_logout(&self) -> bool {
        self.pending_changes.keys().any(|id| {
            self.settings_defs
                .iter()
                .find(|d| d.id == id)
                .is_some_and(|d| d.requires_logout)
        })
    }

    pub fn effective_value(&self, id: &str) -> Option<&SettingValue> {
        self.pending_changes
            .get(id)
            .or_else(|| self.live_values.get(id))
    }

    pub fn update(&mut self, msg: Message) {
        match msg {
            Message::Quit => {
                if self.view == View::Settings {
                    self.running = false;
                } else {
                    self.view = View::Settings;
                }
            }
            Message::NextTab => {
                if self.view == View::Settings {
                    self.tab = self.tab.next();
                    self.selected_row = 0;
                }
            }
            Message::PrevTab => {
                if self.view == View::Settings {
                    self.tab = self.tab.prev();
                    self.selected_row = 0;
                }
            }
            Message::NavigateUp => match self.view {
                View::Settings => {
                    if self.selected_row > 0 {
                        self.selected_row -= 1;
                    }
                }
                View::Profiles => {
                    if self.profile_selected > 0 {
                        self.profile_selected -= 1;
                    }
                }
                _ => {}
            },
            Message::NavigateDown => match self.view {
                View::Settings => {
                    let max = self.visible_settings().len().saturating_sub(1);
                    if self.selected_row < max {
                        self.selected_row += 1;
                    }
                }
                View::Profiles => {
                    let max = self.profile_names.len().saturating_sub(1);
                    if self.profile_selected < max {
                        self.profile_selected += 1;
                    }
                }
                _ => {}
            },
            Message::AdjustLeft => {
                if self.view == View::Settings {
                    self.adjust_selected(-1);
                }
            }
            Message::AdjustRight => {
                if self.view == View::Settings {
                    self.adjust_selected(1);
                }
            }
            Message::Toggle => {
                match self.view {
                    View::Settings => self.toggle_selected(),
                    View::Profiles => self.apply_selected_profile(),
                    _ => {}
                }
            }
            Message::OpenReview => {
                if !self.pending_changes.is_empty() {
                    self.view = View::Review;
                }
            }
            Message::ApplyChanges => {
                if self.view == View::Review {
                    self.apply_all_changes();
                    self.view = View::Settings;
                }
            }
            Message::CancelReview => {
                if self.view == View::Review {
                    self.pending_changes.clear();
                    self.view = View::Settings;
                    self.status_message = Some("Changes discarded".to_string());
                }
            }
            Message::SaveProfile => {
                if self.view == View::Review && !self.pending_changes.is_empty() {
                    self.input_buffer.clear();
                    self.name_input_return_view = View::Settings;
                    self.view = View::ProfileNameInput;
                }
            }
            Message::CreateProfile => {
                if self.view == View::Profiles {
                    self.input_buffer.clear();
                    self.name_input_return_view = View::Profiles;
                    self.view = View::ProfileNameInput;
                }
            }
            Message::OpenProfiles => {
                self.profile_names = profile_storage::list().unwrap_or_default();
                self.profile_selected = 0;
                self.view = View::Profiles;
            }
            Message::DeleteProfile => {
                if self.view == View::Profiles {
                    self.delete_selected_profile();
                }
            }
            Message::Back => {
                self.view = View::Settings;
            }
            Message::TypeChar(c) => {
                if self.view == View::ProfileNameInput {
                    self.input_buffer.push(c);
                }
            }
            Message::Backspace => {
                if self.view == View::ProfileNameInput {
                    self.input_buffer.pop();
                }
            }
            Message::ConfirmInput => {
                if self.view == View::ProfileNameInput && !self.input_buffer.is_empty() {
                    self.save_current_as_profile();
                    self.view = self.name_input_return_view;
                    if self.view == View::Profiles {
                        self.profile_selected = 0;
                    }
                }
            }
        }
    }

    fn adjust_selected(&mut self, direction: i32) {
        let visible = self.visible_settings();
        let Some(def) = visible.get(self.selected_row) else {
            return;
        };
        let id = def.id.to_string();
        let current = self
            .effective_value(&id)
            .cloned()
            .unwrap_or_else(|| def.default_value());

        let new_value = match (&current, &def.constraint) {
            (SettingValue::Float(v), Constraint::FloatRange(FloatRange { min, max, step })) => {
                let new = (v + direction as f64 * step).clamp(*min, *max);
                Some(SettingValue::Float((new * 100.0).round() / 100.0))
            }
            (SettingValue::Int(v), Constraint::IntRange(IntRange { min, max })) => {
                let new = (*v + direction as i64).clamp(*min, *max);
                Some(SettingValue::Int(new))
            }
            (SettingValue::Str(v), Constraint::StringOptions(opts)) => {
                if let Some(idx) = opts.iter().position(|o| o == v) {
                    let new_idx = (idx as i32 + direction).rem_euclid(opts.len() as i32) as usize;
                    Some(SettingValue::Str(opts[new_idx].to_string()))
                } else {
                    None
                }
            }
            _ => None,
        };

        if let Some(val) = new_value {
            self.pending_changes.insert(id, val);
        }
    }

    fn toggle_selected(&mut self) {
        let visible = self.visible_settings();
        let Some(def) = visible.get(self.selected_row) else {
            return;
        };
        let id = def.id.to_string();
        let current = self
            .effective_value(&id)
            .cloned()
            .unwrap_or_else(|| def.default_value());

        match current {
            SettingValue::Bool(v) => {
                self.pending_changes.insert(id, SettingValue::Bool(!v));
            }
            SettingValue::Str(ref v) => {
                if let Constraint::StringOptions(ref opts) = def.constraint {
                    if let Some(idx) = opts.iter().position(|o| o == v) {
                        let next = (idx + 1) % opts.len();
                        self.pending_changes
                            .insert(id, SettingValue::Str(opts[next].to_string()));
                    }
                }
            }
            _ => {}
        }
    }

    fn apply_all_changes(&mut self) {
        let mut applied = 0;
        let mut errors = Vec::new();
        let defs: Vec<_> = self.settings_defs.clone();
        for (id, value) in &self.pending_changes {
            if let Some(def) = defs.iter().find(|d| d.id == id) {
                match write_setting(def, value) {
                    Ok(()) => {
                        self.live_values.insert(id.clone(), value.clone());
                        applied += 1;
                    }
                    Err(e) => errors.push(format!("{}: {e}", def.description)),
                }
            }
        }
        self.pending_changes.clear();

        self.status_message = if errors.is_empty() {
            Some(format!("{applied} settings applied"))
        } else {
            Some(format!(
                "{applied} applied, {} failed: {}",
                errors.len(),
                errors.join(", ")
            ))
        };
    }

    fn apply_selected_profile(&mut self) {
        if let Some(name) = self.profile_names.get(self.profile_selected) {
            match profile_storage::load(name) {
                Ok(profile) => {
                    for (id, value) in profile.settings {
                        self.pending_changes.insert(id, value);
                    }
                    self.status_message =
                        Some(format!("Loaded profile '{}' as pending changes", name));
                    self.view = View::Review;
                }
                Err(e) => {
                    self.status_message = Some(format!("Error loading profile: {e}"));
                }
            }
        }
    }

    fn delete_selected_profile(&mut self) {
        if let Some(name) = self.profile_names.get(self.profile_selected).cloned() {
            match profile_storage::delete(&name) {
                Ok(()) => {
                    self.status_message = Some(format!("Deleted profile '{name}'"));
                    self.profile_names = profile_storage::list().unwrap_or_default();
                    if self.profile_selected >= self.profile_names.len() {
                        self.profile_selected = self.profile_names.len().saturating_sub(1);
                    }
                }
                Err(e) => {
                    self.status_message = Some(format!("Error deleting profile: {e}"));
                }
            }
        }
    }

    fn save_current_as_profile(&mut self) {
        let mut all_settings = self.live_values.clone();
        for (id, value) in &self.pending_changes {
            all_settings.insert(id.clone(), value.clone());
        }
        let profile = Profile::new(self.input_buffer.clone(), all_settings);
        match profile_storage::save(&profile) {
            Ok(()) => {
                self.status_message =
                    Some(format!("Saved profile '{}'", self.input_buffer));
                self.profile_names = profile_storage::list().unwrap_or_default();
            }
            Err(e) => {
                self.status_message = Some(format!("Error saving profile: {e}"));
            }
        }
    }
}
