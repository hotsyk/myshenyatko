pub mod reader;
pub mod registry;
pub mod writer;

use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", content = "value")]
pub enum SettingValue {
    Float(f64),
    Bool(bool),
    Int(i64),
    Str(String),
}

impl fmt::Display for SettingValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SettingValue::Float(v) => write!(f, "{v:.1}"),
            SettingValue::Bool(v) => write!(f, "{}", if *v { "On" } else { "Off" }),
            SettingValue::Int(v) => write!(f, "{v}"),
            SettingValue::Str(v) => write!(f, "{v}"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValueType {
    Float,
    Bool,
    Int,
    Str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SettingGroup {
    Mouse,
    MouseHardware,
    Trackpad,
    TrackpadHardware,
    ScrollWindow,
    CursorAccessibility,
    Keyboard,
    KeyboardText,
}

impl fmt::Display for SettingGroup {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SettingGroup::Mouse => write!(f, "Mouse"),
            SettingGroup::MouseHardware => write!(f, "Mouse Hardware"),
            SettingGroup::Trackpad => write!(f, "Trackpad"),
            SettingGroup::TrackpadHardware => write!(f, "Trackpad Hardware"),
            SettingGroup::ScrollWindow => write!(f, "Scroll & Windows"),
            SettingGroup::CursorAccessibility => write!(f, "Cursor & Accessibility"),
            SettingGroup::Keyboard => write!(f, "Keyboard"),
            SettingGroup::KeyboardText => write!(f, "Text Input"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tab {
    Mouse,
    Trackpad,
    ScrollWindow,
    Cursor,
    Keyboard,
}

impl Tab {
    pub const ALL: [Tab; 5] = [Tab::Mouse, Tab::Trackpad, Tab::ScrollWindow, Tab::Cursor, Tab::Keyboard];

    pub fn label(self) -> &'static str {
        match self {
            Tab::Mouse => "Mouse",
            Tab::Trackpad => "Trackpad",
            Tab::ScrollWindow => "Scroll & Windows",
            Tab::Cursor => "Cursor",
            Tab::Keyboard => "Keyboard",
        }
    }

    pub fn groups(self) -> &'static [SettingGroup] {
        match self {
            Tab::Mouse => &[SettingGroup::Mouse, SettingGroup::MouseHardware],
            Tab::Trackpad => &[SettingGroup::Trackpad, SettingGroup::TrackpadHardware],
            Tab::ScrollWindow => &[SettingGroup::ScrollWindow],
            Tab::Cursor => &[SettingGroup::CursorAccessibility],
            Tab::Keyboard => &[SettingGroup::Keyboard, SettingGroup::KeyboardText],
        }
    }

    pub fn next(self) -> Tab {
        match self {
            Tab::Mouse => Tab::Trackpad,
            Tab::Trackpad => Tab::ScrollWindow,
            Tab::ScrollWindow => Tab::Cursor,
            Tab::Cursor => Tab::Keyboard,
            Tab::Keyboard => Tab::Mouse,
        }
    }

    pub fn prev(self) -> Tab {
        match self {
            Tab::Mouse => Tab::Keyboard,
            Tab::Trackpad => Tab::Mouse,
            Tab::ScrollWindow => Tab::Trackpad,
            Tab::Cursor => Tab::ScrollWindow,
            Tab::Keyboard => Tab::Cursor,
        }
    }
}

#[derive(Debug, Clone)]
pub struct FloatRange {
    pub min: f64,
    pub max: f64,
    pub step: f64,
}

#[derive(Debug, Clone)]
pub struct IntRange {
    pub min: i64,
    pub max: i64,
}

#[derive(Debug, Clone)]
pub enum Constraint {
    FloatRange(FloatRange),
    IntRange(IntRange),
    StringOptions(Vec<&'static str>),
    None,
}

#[derive(Debug, Clone)]
pub struct SettingDef {
    pub id: &'static str,
    pub domain: &'static str,
    pub key: &'static str,
    pub value_type: ValueType,
    pub constraint: Constraint,
    pub description: &'static str,
    pub group: SettingGroup,
    pub mirror_domains: &'static [&'static str],
    pub requires_logout: bool,
    pub help: &'static str,
}

impl SettingDef {
    pub fn default_value(&self) -> SettingValue {
        match &self.constraint {
            Constraint::FloatRange(FloatRange { min, .. }) => SettingValue::Float(*min),
            Constraint::IntRange(IntRange { min, .. }) => SettingValue::Int(*min),
            Constraint::StringOptions(opts) => SettingValue::Str(opts[0].to_string()),
            Constraint::None => match self.value_type {
                ValueType::Float => SettingValue::Float(0.0),
                ValueType::Bool => SettingValue::Bool(false),
                ValueType::Int => SettingValue::Int(0),
                ValueType::Str => SettingValue::Str(String::new()),
            },
        }
    }
}
