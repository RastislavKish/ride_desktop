/*
* Copyright (C) 2022 Rastislav Kish
*
* This program is free software: you can redistribute it and/or modify
* it under the terms of the GNU General Public License as published by
* the Free Software Foundation, version 3.
*
* This program is distributed in the hope that it will be useful,
* but WITHOUT ANY WARRANTY; without even the implied warranty of
* MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
* GNU General Public License for more details.
*
* You should have received a copy of the GNU General Public License
* along with this program. If not, see <https://www.gnu.org/licenses/>.
*/

use std::collections::HashMap;
use std::rc::Rc;

use derivative::Derivative;

pub mod keymap;
pub use keymap::Key;

type Binding<'a, T>=Rc<&'a dyn Fn(&mut T)>;

pub struct KeyboardShortcutsManager<'a, T> {
    keyboard_shortcuts: HashMap<KeyboardShortcut, Binding<'a, T>>,
    }

impl<'a, T> KeyboardShortcutsManager<'a, T> {

    pub fn new() -> KeyboardShortcutsManager<'a, T> {
        let keyboard_shortcuts: HashMap<KeyboardShortcut, Binding<'a, T>>=HashMap::new();

        KeyboardShortcutsManager {keyboard_shortcuts}
        }

    pub fn add_shortcut(&mut self, control: bool, shift: bool, alt: bool, key: Key, function: &'a dyn Fn(&mut T)) {
        self.keyboard_shortcuts.insert(KeyboardShortcut::new(control, shift, alt, key), Rc::new(function));
        }

    pub fn get_function(&self, key: &KeyboardShortcut) -> Option<Binding<'a, T>> {
        if let Some(rc) = self.keyboard_shortcuts.get(key) {
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
    keyval: gdk::keys::Key,
    }

impl KeyboardShortcut {

    pub fn new(control: bool, shift: bool, alt: bool, key: Key) -> KeyboardShortcut {
        let mut modifiers: gdk::ModifierType=gdk::ModifierType::empty();

        if control {
            modifiers.insert(gdk::ModifierType::CONTROL_MASK);
            }
        if shift {
            modifiers.insert(gdk::ModifierType::SHIFT_MASK);
            }
        if alt {
            modifiers.insert(gdk::ModifierType::MOD1_MASK);
            }

        let key=key as u16;

        KeyboardShortcut {modifiers, key, keyval: gdk::keys::Key::from_unicode(' ')}
        }

    pub fn from_eventkey(eventkey: &gdk::EventKey) -> KeyboardShortcut {
        let modifiers=eventkey.state()
        .intersection(gdk::ModifierType::CONTROL_MASK | gdk::ModifierType::SHIFT_MASK | gdk::ModifierType::MOD1_MASK);

        KeyboardShortcut {modifiers, key: eventkey.hardware_keycode(), keyval: eventkey.keyval()}
        }

    pub fn control(&self) -> bool {
        self.modifiers.contains(gdk::ModifierType::CONTROL_MASK)
        }
    pub fn alt(&self) -> bool {
        self.modifiers.contains(gdk::ModifierType::MOD1_MASK)
        }

    pub fn keyval(&self) -> &gdk::keys::Key {
        &self.keyval
        }

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

