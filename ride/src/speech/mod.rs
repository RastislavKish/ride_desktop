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

#[cfg_attr(unix, path="unix.rs")]
#[cfg_attr(windows, path="windows.rs")]
mod imp;

pub struct Speech(imp::Speech);
impl Speech {

    pub fn new(app_name: &str) -> Speech {
        Speech(imp::Speech::new(app_name))
        }

    pub fn speak(&self, text: &str) {
        self.0.speak(text);
        }

    pub fn speak_char(&self, text: &str) {
        self.0.speak_char(text);
        }
    }

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
        }
    }
