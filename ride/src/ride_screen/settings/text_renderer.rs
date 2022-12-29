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

use serde::{Serialize, Deserialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct TextRenderer {
    characters_definitions: HashMap<char, String>,
    strings_definitions: HashMap<String, String>,
    }

impl TextRenderer {

    pub fn new() -> TextRenderer {
        TextRenderer {characters_definitions: HashMap::new(), strings_definitions: HashMap::new()}
        }

    pub fn render_text(&self, text: &str) -> String {
        if text=="" || text=="\n" {
            return "blank".to_string();
            }

        let mut local_text=text.to_string();

        for k in self.strings_definitions.keys() {
            local_text=local_text.replace(k, &self.strings_definitions[k]);
            }

        local_text
        }

    pub fn render_character(&self, character: char) -> Option<String> {
        if character=='\n' {
            return Some("blank".to_string());
            }

        if self.characters_definitions.contains_key(&character) {
            return Some((&self.characters_definitions[&character]).to_string());
            }

        None
        }

    pub fn add_character_definition(&mut self, character: char, definition: &str) {
        self.characters_definitions.insert(character, definition.to_string());
        }

    pub fn add_string_definition(&mut self, string: &str, definition: &str) {
        self.strings_definitions.insert(string.to_string(), definition.to_string());
        }

    }

impl Default for TextRenderer {

    fn default() -> Self {
        TextRenderer::new()
        }

    }

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
        }
    }
