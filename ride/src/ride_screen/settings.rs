use std::fs;
use std::error::Error;
use std::path::Path;

use serde::{Serialize, Deserialize};

mod text_renderer;

use text_renderer::TextRenderer;

#[derive(Serialize, Deserialize)]
enum Value {
    Bool(bool),
    TextRenderer(TextRenderer),
    }

#[derive(Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Settings {
    pub beep_on_capital_characters: bool,
    pub text_renderer: TextRenderer,
    }

impl Settings {

    pub fn new() -> Settings {
        Settings {beep_on_capital_characters: true, text_renderer: TextRenderer::new()}
        }

    pub fn from_file(file_path: &str) -> Result<Settings, Box<dyn Error>> {
        let settings: Settings=serde_yaml::from_str(&fs::read_to_string(&file_path)?)?;

        Ok(settings)
        }

    pub fn save(&self, file_path: &str) {
        let path=std::path::Path::new(file_path);
        if !path.exists() {
            let prefix=path.parent().unwrap();
            fs::create_dir_all(prefix).unwrap();
            }

        fs::write(file_path, &serde_yaml::to_string(self).unwrap()).unwrap();
        }

    pub fn get_settings_file_path(project: &str, file_name: &str) -> String {
        let config_dir=dirs::config_dir().unwrap();

        Path::new(&config_dir).join(project).join(file_name).to_str().unwrap().to_string()
        }

    }

impl Default for Settings {

    fn default() -> Self {
        Settings::new()
        }

    }

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
        }
    }
