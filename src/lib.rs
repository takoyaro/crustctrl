#![deny(clippy::all)]

use enigo::{Direction, Enigo, Key, Keyboard, Settings};
use napi_derive::napi;

#[napi]
pub fn plus_100(input: u32) -> u32 {
  input + 100
}

#[napi(object)]
pub struct Modifiers {
  pub lshift: Option<bool>,
  pub rshift: Option<bool>,
  pub lctrl: Option<bool>,
  pub rctrl: Option<bool>,
  pub lalt: Option<bool>,
  pub meta: Option<bool>,
}

impl Default for Modifiers {
  fn default() -> Self {
    Modifiers {
      lshift: None,
      rshift: None,
      lctrl: None,
      rctrl: None,
      lalt: None,
      meta: None,
    }
  }
}

fn convert_key_to_enigokey(key: String) -> Key {
  match key.to_lowercase().as_str() {
    "f1" => Key::F1,
    "f2" => Key::F2,
    "f3" => Key::F3,
    "f4" => Key::F4,
    "f5" => Key::F5,
    "f6" => Key::F6,
    "f7" => Key::F7,
    "f8" => Key::F8,
    "f9" => Key::F9,
    "f10" => Key::F10,
    "f11" => Key::F11,
    "f12" => Key::F12,
    "f13" => Key::F13,
    "f14" => Key::F14,
    "f15" => Key::F15,
    "f16" => Key::F16,
    "f17" => Key::F17,
    "f18" => Key::F18,
    "f19" => Key::F19,
    "f20" => Key::F20,
    "f21" => Key::F21,
    "f22" => Key::F22,
    "f23" => Key::F23,
    "f24" => Key::F24,
    "f25" => Key::F25,
    "f26" => Key::F26,
    "f27" => Key::F27,
    "f28" => Key::F28,
    "f29" => Key::F29,
    "f30" => Key::F30,
    "arrowup" => Key::UpArrow,
    "arrowdown" => Key::DownArrow,
    "arrowleft" => Key::LeftArrow,
    "arrowright" => Key::RightArrow,
    "escape" => Key::Escape,
    "enter" => Key::Return,
    "tab" => Key::Tab,
    "backspace" => Key::Backspace,
    "delete" => Key::Delete,
    "home" => Key::Home,
    "end" => Key::End,
    "pageup" => Key::PageUp,
    "pagedown" => Key::PageDown,
    "insert" => Key::Insert,
    &_ => Key::Unicode(key.chars().nth(0).expect("Expected a char")),
  }
}

#[napi]
pub fn press_key(key: String, modifiers: Option<Modifiers>) -> String {
  let mut enigo = Enigo::new(&Settings::default()).unwrap();
  enigo.set_delay(10);
  if let Some(mods) = &modifiers {
    // Define all modifier keys to check
    let modifier_keys = [
      (mods.lshift.is_some(), Key::Shift),
      (mods.rshift.is_some(), Key::RShift),
      (mods.lctrl.is_some(), Key::Control),
      (mods.rctrl.is_some(), Key::RControl),
      (mods.lalt.is_some(), Key::Alt),
      (mods.meta.is_some(), Key::Meta),
    ];

    // Press all modifier keys
    for &(should_press, key) in &modifier_keys {
      if should_press {
        enigo.key(key, Direction::Press).ok();
      }
    }

    // Press the main key
    let main_key = if key.len() > 1 {
      convert_key_to_enigokey(key.clone())
    } else {
      Key::Unicode(key.chars().next().expect("Expected a char"))
    };
    enigo.key(main_key, Direction::Press).ok();

    // Release all modifier keys in reverse order
    for &(should_press, key) in modifier_keys.iter().rev() {
      if should_press {
        enigo.key(key, Direction::Release).ok();
      }
    }
    let formatted_key = key;
    return format!("Pressed key {} with modifiers", formatted_key);
  } else {
    // Just press the key without modifiers
    let main_key = if key.len() > 1 {
      convert_key_to_enigokey(key.clone())
    } else {
      Key::Unicode(key.chars().next().expect("Expected a char"))
    };
    enigo.key(main_key, Direction::Press).ok();
    return format!("Pressed key {}", key.as_str());
  }
  
}

#[napi]
pub fn send_text(text:String) {
  let mut enigo = Enigo::new(&Settings::default()).unwrap();
  enigo.set_delay(50);
  for c in text.chars() {
    enigo.key(Key::Unicode(c), Direction::Click).ok();
  }
}