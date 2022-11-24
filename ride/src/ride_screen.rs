use std::sync::mpsc::Sender;

use clipboard::{ClipboardProvider, x11_clipboard::X11ClipboardContext};
use dialog::DialogBox;

//use gtk::prelude::*;
//use gtk::{ButtonsType, DialogFlags, MessageType, MessageDialog, Window};

pub mod screen;

//use std::rc::Rc;

mod ride_text;
mod resources;
mod settings;
mod speech;

use ride_text::{RideText, SearchDirection};
use resources::Resources;
use screen::{KeyboardShortcutsManager, KeyboardShortcut, Key};
use settings::Settings;
use speech::Speech;

pub struct RideScreen<'a> {
    content: RideText,
    lastly_searched_phrase: String,
    keyboard_shortcuts_manager: KeyboardShortcutsManager<'a>,
    resources: Resources,
    settings: Settings,
    speech: Speech,
    ride_tx: Sender<RideThreadMessage>,
    }

impl<'a> RideScreen<'a> {

    pub fn new(file_path: &str, ride_tx: Sender<RideThreadMessage>) -> Self {
        let resources=Resources::new();
        let speech=Speech::new("ride");
        let content=RideText::new();
        let lastly_searched_phrase="".to_string();
        let mut settings=Settings::new();
        settings.load(&(std::env::var("HOME").unwrap()+"/.config/ride/settings.yaml"));

        let mut keyboard_shortcuts_manager=KeyboardShortcutsManager::new();

        //Loading shortcuts

        keyboard_shortcuts_manager.add_shortcut(true, false, false, Key::S, &Self::save);

        //Navigation shortcuts

        keyboard_shortcuts_manager.add_shortcut(false, false, false, Key::Up, &Self::navigate_to_previous_line);
        keyboard_shortcuts_manager.add_shortcut(false, false, false, Key::Down, &Self::navigate_to_next_line);
        keyboard_shortcuts_manager.add_shortcut(false, false, false, Key::Left, &Self::navigate_to_previous_character);
        keyboard_shortcuts_manager.add_shortcut(false, false, false, Key::Right, &Self::navigate_to_next_character);

        keyboard_shortcuts_manager.add_shortcut(true, false, false, Key::Home, &Self::navigate_to_area_beginning);
        keyboard_shortcuts_manager.add_shortcut(true, false, false, Key::End, &Self::navigate_to_area_ending);
        keyboard_shortcuts_manager.add_shortcut(false, false, false, Key::Home, &Self::navigate_to_line_beginning);
        keyboard_shortcuts_manager.add_shortcut(false, false, false, Key::End, &Self::navigate_to_line_ending);

        keyboard_shortcuts_manager.add_shortcut(false, false, true, Key::Right, &Self::increase_indentation_level);
        keyboard_shortcuts_manager.add_shortcut(false, false, true, Key::Left, &Self::decrease_indentation_level);
        keyboard_shortcuts_manager.add_shortcut(true, false, false, Key::J, &Self::jump_to_line);
        keyboard_shortcuts_manager.add_shortcut(true, false, false, Key::F, &Self::find);
        keyboard_shortcuts_manager.add_shortcut(false, false, false, Key::F3, &Self::refind);
        keyboard_shortcuts_manager.add_shortcut(false, true, false, Key::F3, &Self::backward_refind);

        //Editing functions

        keyboard_shortcuts_manager.add_shortcut(false, true, false, Key::Up, &Self::select_previous_line);
        keyboard_shortcuts_manager.add_shortcut(false, true, false, Key::Down, &Self::select_next_line);
        keyboard_shortcuts_manager.add_shortcut(false, false, false, Key::Return, &Self::create_new_line);
        keyboard_shortcuts_manager.add_shortcut(false, true, false, Key::Return, &Self::create_new_block);
        keyboard_shortcuts_manager.add_shortcut(false, false, false, Key::Backspace, &Self::delete_character);
        keyboard_shortcuts_manager.add_shortcut(false, false, false, Key::Delete, &Self::delete);
        keyboard_shortcuts_manager.add_shortcut(true, false, false, Key::C, &Self::copy);
        keyboard_shortcuts_manager.add_shortcut(true, false, false, Key::X, &Self::cut);
        keyboard_shortcuts_manager.add_shortcut(true, false, false, Key::V, &Self::paste);
        keyboard_shortcuts_manager.add_shortcut(true, false, false, Key::I, &Self::reformat);

        //Settings shortcuts

        keyboard_shortcuts_manager.add_shortcut(true, false, false, Key::R, &Self::add_character_definition);
        keyboard_shortcuts_manager.add_shortcut(true, true, false, Key::R, &Self::add_string_definition);

        let mut result=Self {content, lastly_searched_phrase, keyboard_shortcuts_manager, resources, settings, speech, ride_tx};

        result.load_from_file(file_path);

        result
        }

    fn load_from_file(&mut self, file_path: &str) {
        if file_path=="" {
            self.ride_tx.send(RideThreadMessage::SetWindowTitle("Untitled - Ride".to_string())).unwrap();
            return;
            }
        if let Err(message) = self.content.load_from_file(file_path) {
            self.message_box("Error", &message);
            }

        let file_path=self.content.file_path();
        if let Some(file_path)=file_path {
            let file_name=file_path.split("/").last().unwrap();
            self.ride_tx.send(RideThreadMessage::SetWindowTitle(format!("{} - Ride", file_name))).unwrap();
            } else {
            self.ride_tx.send(RideThreadMessage::SetWindowTitle("Untitled - Ride".to_string())).unwrap();
            }
        }

    fn save(&mut self) {
        self.content.save().unwrap();
        }

    //Navigation functions

    fn navigate_to_previous_line(&mut self) {
        self.content.cancel_selection();
        if !self.content.navigate_to_previous_line() {
            self.resources.bump.play();
            }
        self.speak_text(&self.content.get_current_line());
        }

    fn navigate_to_next_line(&mut self) {
        self.content.cancel_selection();
        if !self.content.navigate_to_next_line() {
            self.resources.bump.play();
            }
        self.speak_text(&self.content.get_current_line());
        }

    fn navigate_to_previous_character(&mut self) {
        self.content.cancel_selection();
        if !self.content.navigate_to_previous_character() {
            self.resources.bump.play();
            }
        self.speak_character(self.content.get_current_character());
        }

    fn navigate_to_next_character(&mut self) {
        self.content.cancel_selection();
        if !self.content.navigate_to_next_character() {
            self.resources.bump.play();
            }
        self.speak_character(self.content.get_current_character());
        }

    fn navigate_to_area_beginning(&mut self) {
        self.content.cancel_selection();
        self.content.navigate_to_area_beginning();
        self.resources.bump.play();
        self.speak_text(&self.content.get_current_line());
        }

    fn navigate_to_area_ending(&mut self) {
        self.content.cancel_selection();
        self.content.navigate_to_area_ending();
        self.resources.bump.play();
        self.speak_text(&self.content.get_current_line());
        }

    fn navigate_to_line_beginning(&mut self) {
        self.content.navigate_to_line_beginning();
        }

    fn navigate_to_line_ending(&mut self) {
        self.content.navigate_to_line_ending();
        }

    fn increase_indentation_level(&mut self) {
        self.content.cancel_selection();
        if self.content.increase_indentation_level() {
            self.resources.chil.play();
            }
        self.speak_text(&self.content.get_current_line());
        }

    fn decrease_indentation_level(&mut self) {
        self.content.cancel_selection();
        if self.content.decrease_indentation_level() {
            self.resources.chil.play();
            }
        self.speak_text(&self.content.get_current_line());
        }

    fn jump_to_line(&mut self) {
        self.ride_tx.send(RideThreadMessage::HideWindow).unwrap();
        let input=dialog::Input::new("Enter the number of the line to jump to.").title("Jump to line").show().unwrap();
        if let Some(text)=input {
            if let Ok(n)=text.trim().parse::<usize>() {
                match self.content.jump_to_line(n) {
                    Ok(chil) => {
                        if chil {
                            self.resources.chil.play();
                            }
                        },
                    Err(message) => {
                        dialog::Message::new(&message).title("Error").show().unwrap();
                        },
                    };
                } else {
                dialog::Message::new("Invalid input.").title("Error").show().unwrap();
                }
            }
        self.ride_tx.send(RideThreadMessage::ShowWindow).unwrap();
        }

    fn find(&mut self) {
        self.ride_tx.send(RideThreadMessage::HideWindow).unwrap();

        let input=dialog::Input::new("Enter the phrase to search for.").title("Find").show().unwrap();
        if let Some(text)=input {
            if text=="" {
                return;
                }

            self.lastly_searched_phrase=text.to_string();
            };

        self.ride_tx.send(RideThreadMessage::ShowWindow).unwrap();

        self.refind();
        }

    fn refind(&mut self) {
        if self.lastly_searched_phrase=="" {
            self.find();
            }

        let original_indentation_level=self.content.current_indentation_level();

        if self.content.find(&self.lastly_searched_phrase, SearchDirection::Forward) {
            if self.content.current_indentation_level()!=original_indentation_level {
                self.resources.chil.play();
                }

            self.speak_text(&self.content.get_current_line());
            }
        else {
            self.speak_text("Not found");
            }
        }

    fn backward_refind(&mut self) {
        if self.lastly_searched_phrase=="" {
            self.find();
            }

        let original_indentation_level=self.content.current_indentation_level();

        if self.content.find(&self.lastly_searched_phrase, SearchDirection::Backward) {
            if self.content.current_indentation_level()!=original_indentation_level {
                self.resources.chil.play();
                }

            self.speak_text(&self.content.get_current_line());
            }
        else {
            self.speak_text("Not found");
            }
        }

    //Editing functions

    fn select_previous_line(&mut self) {
        self.content.start_selection();
        if !self.content.navigate_to_previous_line() {
            self.resources.bump.play();
            }
        self.speak_text(&self.content.get_current_line());
        }

    fn select_next_line(&mut self) {
        self.content.start_selection();
        if !self.content.navigate_to_next_line() {
            self.resources.bump.play();
            }
        self.speak_text(&self.content.get_current_line());
        }

    fn create_new_line(&mut self) {
        self.content.cancel_selection();
        self.content.create_new_line();
        }

    fn create_new_block(&mut self) {
        self.content.cancel_selection();
        self.content.create_new_block();
        self.resources.chil.play();
        }

    fn delete_character(&mut self) {
        self.content.cancel_selection();
        if let Some(character) = self.content.delete_character() {
            self.speak_character(character);
            }
        else
            {
            self.resources.bump.play();
            }
        }

    fn delete(&mut self) {
        self.content.delete();
        }

    //Settings shortcuts

    fn copy(&mut self) {
        match self.content.get_selected_text(false) {
            Ok(text) => {
                let mut ctx: X11ClipboardContext= X11ClipboardContext::new().unwrap();
                ctx.set_contents(text).unwrap();
                self.speech.speak("Copied");
                },
            Err(message) => {
                self.speech.speak(&message);
                },
            };
        }

    fn cut(&mut self) {
        match self.content.get_selected_text(true) {
            Ok(text) => {
                let mut ctx: X11ClipboardContext= X11ClipboardContext::new().unwrap();
                ctx.set_contents(text).unwrap();
                self.speech.speak("Cutted");
                },
            Err(message) => {
                self.speech.speak(&message);
                },
            };
        }

    fn paste(&mut self) {
        let mut ctx: X11ClipboardContext= X11ClipboardContext::new().unwrap();
        let text=ctx.get_contents().unwrap();
        match self.content.paste(&text) {
            Ok(()) => {
                self.speech.speak("Pasted");
                },
            Err(message) => {
                self.speech.speak(&message);
                },
            }
        }

    fn reformat(&mut self) {
        self.ride_tx.send(RideThreadMessage::HideWindow).unwrap();
        let beginning_mark=dialog::Input::new("Enter the mark beginning a block").title("Reformat").show().unwrap();

        if let Some(beginning_mark)=beginning_mark {
            let beginning_mark=beginning_mark.trim();
            if beginning_mark!="" {
                let ending_mark=dialog::Input::new("Enter the mark ending a block").title("Reformat").show().unwrap();

                if let Some(ending_mark)=ending_mark {
                    let ending_mark=ending_mark.trim();
                    if ending_mark!="" {

                        self.content.reformat(&beginning_mark, &ending_mark).unwrap();

                        }
                    }
                }
            }

        self.ride_tx.send(RideThreadMessage::ShowWindow).unwrap();
        }

    //Configuration functions

    fn add_character_definition(&mut self) {
        self.ride_tx.send(RideThreadMessage::HideWindow).unwrap();
        //std::thread::sleep(std::time::Duration::from_secs(1));

        let character=dialog::Input::new("Enter the desired character to be defined for character rendering.").title("Add character definition").show().unwrap();
        let definition=dialog::Input::new("Enter the definition for the entered character.").title("Add character definition").show().unwrap();

        if let Some(character) = character {
            if let Some(definition) = definition {
                if character=="" || definition == "" {
                    return;
                    }

                let chars: Vec<char>=character.chars().collect();
                if chars.len()!=1 {
                    dialog::Message::new("You have entered invalid character to be defined.").title("Error").show().unwrap();
                    }

                self.settings.text_renderer.add_character_definition(chars[0], &definition);
                }
            }

        self.ride_tx.send(RideThreadMessage::ShowWindow).unwrap();
        }

    fn add_string_definition(&mut self) {
        self.ride_tx.send(RideThreadMessage::HideWindow).unwrap();
        //std::thread::sleep(std::time::Duration::from_secs(1));

        let string=dialog::Input::new("Enter the desired character to be defined for string rendering.").title("Add character definition").show().unwrap();
        let definition=dialog::Input::new("Enter the definition for the entered character.").title("Add character definition").show().unwrap();

        if let Some(string) = string {
            if let Some(definition) = definition {
                if string=="" || definition == "" {
                    return;
                    }

                self.settings.text_renderer.add_string_definition(&string, &definition);
                }
            }

        self.ride_tx.send(RideThreadMessage::ShowWindow).unwrap();
        }

    }

impl<'a> RideScreen<'a> {

    pub fn on_key_pressed(&mut self, key: &KeyboardShortcut) {
        if let Some(func) = self.keyboard_shortcuts_manager.get_function(key) {
            func(self);
            }
        else if !key.control() {
            if let Some(character) = key.keyval().to_unicode() {
                if !character.is_control() {
                    self.on_text_entered(character);
                    }
                }
            }
        }

    fn on_text_entered(&mut self, character: char) {
        self.content.insert(character);
        }

    pub fn on_exit(&self) {
        self.settings.save(&(std::env::var("HOME").unwrap()+"/.config/ride/settings.yaml"));
        }

    fn speak_text(&self, text: &str) {
        self.speech.speak(&self.settings.text_renderer.render_text(text));
        }

    fn speak_character(&mut self, character: char) {
        if let Some(rendered_character)=self.settings.text_renderer.render_character(character) {
            self.speech.speak(&rendered_character);
            }
        else {
            self.speech.speak_char(&character.to_string());
            }

        if character.is_uppercase() && self.settings.beep_on_capital_characters {
            self.resources.capital.play();
            }
        }

    pub fn message_box(&self, title: &str, message: &str) {
        self.ride_tx.send(RideThreadMessage::HideWindow).unwrap();
        dialog::Message::new(message).title(title).show().unwrap();
        self.ride_tx.send(RideThreadMessage::ShowWindow).unwrap();

        /*
        MessageDialog::new(None::<&Window>,
        DialogFlags::empty(),
        MessageType::Info,
        ButtonsType::Ok, message).run();
        */
        }

    }

pub enum GtkThreadMessage {
    KeyPress(KeyboardShortcut),
    //WindowCreation(Arc<ApplicationWindow>),
    ApplicationExit,
    }

pub enum RideThreadMessage {
    SetWindowTitle(String),
    HideWindow,
    ShowWindow,
    }

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
        }
    }
