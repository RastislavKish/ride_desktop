use std::collections::HashMap;
use std::rc::Rc;

use derivative::Derivative;

use super::RideScreen;

pub struct KeyboardShortcutsManager<'a> {
keyboard_shortcuts: HashMap<KeyboardShortcut, Rc<&'a dyn Fn(&mut RideScreen<'a>)>>,
}

impl<'a> KeyboardShortcutsManager<'a> {

pub fn new() -> KeyboardShortcutsManager<'a> {
let keyboard_shortcuts: HashMap<KeyboardShortcut, Rc<&'a dyn Fn(&mut RideScreen<'a>)>>=HashMap::new();

KeyboardShortcutsManager {keyboard_shortcuts}
}

pub fn add_shortcut(&mut self, control: bool, shift: bool, alt: bool, key: Key, function: &'a dyn Fn(&mut RideScreen<'a>)) {
self.keyboard_shortcuts.insert(KeyboardShortcut::new(control, shift, alt, key), Rc::new(function));
}

pub fn get_function(&self, key: &KeyboardShortcut) -> Option<Rc<&'a dyn Fn(&mut RideScreen<'a>)>> {
if let Some(rc) = self.keyboard_shortcuts.get(&key) {
return Some(rc.clone());
}

None
}

}

#[derive(Derivative)]
#[derivative(Eq, PartialEq, Hash)]
pub struct KeyboardShortcut {
modifiers: gdk::ModifierType,
key: u16,
#[derivative(PartialEq="ignore")]
#[derivative(Hash="ignore")]
keyval: u32,
}

impl KeyboardShortcut {

pub fn new(control: bool, shift: bool, alt: bool, key: Key) -> KeyboardShortcut {
let mut modifiers: gdk::ModifierType=gdk::ModifierType::empty();

if control {
modifiers=modifiers | gdk::ModifierType::CONTROL_MASK;
}

if shift {
modifiers=modifiers | gdk::ModifierType::SHIFT_MASK;
}

if alt {
modifiers=modifiers | gdk::ModifierType::MOD1_MASK;
}

let key=key as u16;

KeyboardShortcut {modifiers, key, keyval: 0}
}

pub fn from_eventkey(eventkey: &gdk::EventKey) -> KeyboardShortcut {
let modifiers=eventkey.get_state();
let control=modifiers & gdk::ModifierType::CONTROL_MASK == gdk::ModifierType::CONTROL_MASK;
let shift=modifiers & gdk::ModifierType::SHIFT_MASK == gdk::ModifierType::SHIFT_MASK;
let alt=modifiers & gdk::ModifierType::MOD1_MASK == gdk::ModifierType::MOD1_MASK;

let mut modifiers: gdk::ModifierType=gdk::ModifierType::empty();

if control {
modifiers=modifiers | gdk::ModifierType::CONTROL_MASK;
}

if shift {
modifiers=modifiers | gdk::ModifierType::SHIFT_MASK;
}

if alt {
modifiers=modifiers | gdk::ModifierType::MOD1_MASK;
}

KeyboardShortcut {modifiers, key: eventkey.get_hardware_keycode(), keyval: eventkey.get_keyval()}
}

pub fn control(&self) -> bool {
self.modifiers & gdk::ModifierType::CONTROL_MASK==gdk::ModifierType::CONTROL_MASK
}

pub fn keyval(&self) -> u32 {
self.keyval
}

}

#[derive(Eq, PartialEq)]
pub enum Key {
Left=113,
Right=114,
Up=111,
Down=116,
Home=110,
End=115,
Delete=119,
Backspace=22,
Return=36,
X=53,
C=54,
V=55,
S=39,
F=41,
J=44,
R=27,
F3=69,
}

#[cfg(test)]
mod tests {

use super::*;
use std::collections::HashMap;

#[test]
fn keys_hashmap() {
let k1=KeyboardShortcut::new(false, false, false, Key::Up);
let k2=KeyboardShortcut::new(false, false, false, Key::Up);

let mut hm: HashMap<KeyboardShortcut, usize>=HashMap::new();
hm.insert(k1, 2);
assert_eq!(hm.contains_key(&k2), true);
}

#[test]
fn enums_cast_test() {
let k=Key::Up;
assert_eq!(k as u16, 111);
}

}
