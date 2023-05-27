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

use std::sync::Arc;

use tolk::Tolk;

pub struct Speech {
    tolk: Arc<Tolk>,
    }
impl Speech {

    pub fn new(_app_name: &str) -> Speech {
        let tolk=Tolk::new();

        Speech { tolk }
        }

    pub fn speak(&self, text: &str) {
        self.tolk.speak(text, true);
        }

    pub fn speak_char(&self, text: &str) {
        self.tolk.speak(text, true);
        }
    }

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
        }
    }
