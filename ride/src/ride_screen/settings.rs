use std::collections::HashMap;
use std::fs;

use serde::{Serialize, Deserialize};

mod text_renderer;

use text_renderer::TextRenderer;

#[derive(Serialize, Deserialize)]
enum Value {
    Bool(bool),
    TextRenderer(TextRenderer),
    }

pub struct Settings {
    pub beep_on_capital_characters: bool,
    pub text_renderer: TextRenderer,
    }

impl Settings {

    pub fn new() -> Settings {
        Settings {beep_on_capital_characters: true, text_renderer: TextRenderer::new()}
        }

    pub fn load(&mut self, file_path: &str) {
        let Ok(data)=fs::read_to_string(file_path) else {return;};
        let Ok(saved_settings)=serde_yaml::from_str::<HashMap<String, Value>>(&data[..]) else {
            return;
            };

        for (key, value) in saved_settings.into_iter() {
            match (&key[..], value) {
                ("BeepOnCapitalCharacters", Value::Bool(val)) => {
                    self.beep_on_capital_characters=val;
                    },
                ("TextRenderer", Value::TextRenderer(val)) => {
                    self.text_renderer=val;
                    },
                _ => (),
                };
            }

        }

    pub fn save(&self, file_path: &str) {
        let path=std::path::Path::new(file_path);
        if !path.exists() {
            let prefix=path.parent().unwrap();
            fs::create_dir_all(prefix).unwrap();
            }
        let mut settings_to_save: HashMap<String, Value>=HashMap::new();

        settings_to_save.insert("BeepOnCapitalCharacters".to_string(), Value::Bool(self.beep_on_capital_characters));
        settings_to_save.insert("TextRenderer".to_string(), Value::TextRenderer(self.text_renderer.clone()));

        fs::write(file_path, serde_yaml::to_string(&settings_to_save).unwrap()).unwrap();
        }

    }

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
        }
    }
