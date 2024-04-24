use smithay::input::keyboard::keysyms;
use crate::config::types::KeyModifiersDef;
use indexmap::IndexMap;
mod types;

#[derive(Debug)]
pub struct Config {
    pub workspaces: u8,
    pub keybindings: IndexMap<KeyPattern, Action>,
    pub gaps: (i32, i32),
    pub autostart: Vec<String>,
    pub tile_ratio_update_interval: f32,
}

#[derive(Debug, Clone)]
pub struct OutputConfig((i32, i32), Option<u32>);

pub fn generate_config() -> Config {
    let mut keybinding_map = indexmap::IndexMap::<KeyPattern, Action>::new();
    keybinding_map.insert(
        KeyPattern {
            modifiers: KeyModifiersDef(vec![KeyModifier::Ctrl]).into(),
            key: keysyms::KEY_Return,
        },
        Action::Spawn(String::from("foot")),
    );

    keybinding_map.insert(
        KeyPattern {
            modifiers: KeyModifiersDef(vec![KeyModifier::Ctrl, KeyModifier::Shift]).into(),
            key: keysyms::KEY_q,
        },
        Action::Quit,
    );

    keybinding_map.insert(
        KeyPattern {
            modifiers: KeyModifiersDef(vec![KeyModifier::Ctrl]).into(),
            key: keysyms::KEY_w,
        },
        Action::Close,
    );

    keybinding_map.insert(
        KeyPattern {
            modifiers: KeyModifiersDef(vec![KeyModifier::Ctrl]).into(),
            key: keysyms::KEY_a,
        },
        Action::DecreaseTileRatio,
    );

    keybinding_map.insert(
        KeyPattern {
            modifiers: KeyModifiersDef(vec![KeyModifier::Ctrl]).into(),
            key: keysyms::KEY_d,
        },
        Action::IncreaseTileRatio,
    );

    keybinding_map.insert(
        KeyPattern {
            modifiers: KeyModifiersDef(vec![KeyModifier::Ctrl]).into(),
            key: keysyms::KEY_1,
        },
        Action::Workspace(0),
    );

    keybinding_map.insert(
        KeyPattern {
            modifiers: KeyModifiersDef(vec![KeyModifier::Ctrl]).into(),
            key: keysyms::KEY_2,
        },
        Action::Workspace(1),
    );

    keybinding_map.insert(
        KeyPattern {
            modifiers: KeyModifiersDef(vec![KeyModifier::Ctrl]).into(),
            key: keysyms::KEY_3,
        },
        Action::Workspace(2),
    );

    let gaps = (5,5);
    let ratio_interval = 0.05;
    let cfg = Config {
        workspaces: 3,
        keybindings: keybinding_map,
        gaps,
        autostart: vec![],
        tile_ratio_update_interval: ratio_interval
    };
    cfg
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum KeyModifier {
    Ctrl,
    Alt,
    Shift,
    Super,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct KeyModifiers {
    ctrl: bool,
    alt: bool,
    shift: bool,
    logo: bool,
}

/// Describtion of a key combination that might be
/// handled by the compositor.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]

pub struct KeyPattern {    
    pub modifiers: KeyModifiers,
    pub key: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Action {
    Quit,
    Close,
    Workspace(u8),
    MoveWindow(u8),
    MoveAndSwitch(u8),
    IncreaseTileRatio,
    DecreaseTileRatio,
    Spawn(String),
}
