use macroquad::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Action {
    ToggleMenu,
    ToggleGrid,
    InvertX,
    InvertY,
    MoveForward,
    MoveBackward,
    MoveLeft,
    MoveRight,
    MoveUp,
    MoveDown,
    Sprint,
    Jump,
    Interact,
}

impl Action {
    pub fn default_key(self) -> KeyCode {
        match self {
            Action::ToggleMenu => KeyCode::Tab,
            Action::ToggleGrid => KeyCode::G,
            Action::InvertX => KeyCode::X,
            Action::InvertY => KeyCode::Y,
            Action::MoveForward => KeyCode::W,
            Action::MoveBackward => KeyCode::S,
            Action::MoveLeft => KeyCode::A,
            Action::MoveRight => KeyCode::D,
            Action::MoveUp => KeyCode::Space,
            Action::MoveDown => KeyCode::LeftControl,
            Action::Sprint => KeyCode::LeftShift,
            Action::Jump => KeyCode::Space,
            Action::Interact => KeyCode::E,
        }
    }

    pub fn display_name(self) -> &'static str {
        match self {
            Action::ToggleMenu => "Toggle Menu",
            Action::ToggleGrid => "Toggle Grid",
            Action::InvertX => "Invert X Axis",
            Action::InvertY => "Invert Y Axis",
            Action::MoveForward => "Move Forward",
            Action::MoveBackward => "Move Backward",
            Action::MoveLeft => "Move Left",
            Action::MoveRight => "Move Right",
            Action::MoveUp => "Move Up",
            Action::MoveDown => "Move Down",
            Action::Sprint => "Sprint",
            Action::Jump => "Jump",
            Action::Interact => "Interact",
        }
    }

    pub fn all() -> &'static [Action] {
        &[
            Action::ToggleMenu,
            Action::ToggleGrid,
            Action::InvertX,
            Action::InvertY,
            Action::MoveForward,
            Action::MoveBackward,
            Action::MoveLeft,
            Action::MoveRight,
            Action::MoveUp,
            Action::MoveDown,
            Action::Sprint,
            Action::Jump,
            Action::Interact,
        ]
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(transparent)]
pub struct SerdeKeyCode {
    code: u16,
}

impl SerdeKeyCode {
    pub fn from_keycode(key: KeyCode) -> Self {
        Self { code: key as u16 }
    }

    pub fn to_keycode(self) -> KeyCode {
        unsafe { std::mem::transmute::<u16, KeyCode>(self.code) }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputConfig {
    bindings: HashMap<Action, SerdeKeyCode>,
}

impl Default for InputConfig {
    fn default() -> Self {
        let mut bindings = HashMap::new();
        for action in Action::all() {
            bindings.insert(*action, SerdeKeyCode::from_keycode(action.default_key()));
        }
        Self { bindings }
    }
}

impl InputConfig {
    pub fn load() -> Self {
        let path = "input_config.json";
        if std::path::Path::new(path).exists() {
            fs::read_to_string(path)
                .ok()
                .and_then(|s| serde_json::from_str(&s).ok())
                .unwrap_or_default()
        } else {
            Self::default()
        }
    }

    pub fn save(&self) {
        let path = "input_config.json";
        if let Ok(json) = serde_json::to_string_pretty(self) {
            let _ = fs::write(path, json);
        }
    }

    pub fn get_key(&self, action: Action) -> KeyCode {
        self.bindings
            .get(&action)
            .map(|k| k.to_keycode())
            .unwrap_or_else(|| action.default_key())
    }

    pub fn set_key(&mut self, action: Action, key: KeyCode) {
        self.bindings.insert(action, SerdeKeyCode::from_keycode(key));
        self.save();
    }

    pub fn reset_to_defaults(&mut self) {
        self.bindings.clear();
        for action in Action::all() {
            self.bindings.insert(*action, SerdeKeyCode::from_keycode(action.default_key()));
        }
        self.save();
    }

    pub fn is_action_pressed(&self, action: Action) -> bool {
        is_key_down(self.get_key(action))
    }

    pub fn is_action_just_pressed(&self, action: Action) -> bool {
        is_key_pressed(self.get_key(action))
    }
}