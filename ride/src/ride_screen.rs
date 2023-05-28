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

use std::sync::mpsc::Sender;

use gtk::prelude::*;

pub mod screen;

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
        let settings=Settings::from_file(&Settings::get_settings_file_path("ride", "settings.yaml"))
        .unwrap_or(Settings::new());

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
        if let Some(text)=self.input_box("Jump to line", "Enther the number of the line to jump to.") {
            if let Ok(n)=text.trim().parse::<usize>() {
                match self.content.jump_to_line(n) {
                    Ok(chil) => {
                        if chil {
                            self.resources.chil.play();
                            }
                        },
                    Err(message) => {
                        self.message_box("Error", &message);
                        },
                    };
                }
            else {
                self.message_box("Error", "Invalid input.");
                }
            }
        }

    fn find(&mut self) {
        if let Some(text)=self.input_box("Find", "Enter the phrase to search for.") {
            if text.is_empty() {
                return;
                }

            self.lastly_searched_phrase=text.to_string();
            }

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
                self.clipboard_set_text(&text);
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
                self.clipboard_set_text(&text);
                self.speech.speak("Cutted");
                },
            Err(message) => {
                self.speech.speak(&message);
                },
            };
        }

    fn paste(&mut self) {
        let text=self.clipboard_get_text();
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
        if let Some(beginning_mark)=self.input_box("Reformat", "Enter the mark beginning a block") {
            let beginning_mark=beginning_mark.trim();

            if beginning_mark.is_empty() {
                return;
                }

            if let Some(ending_mark)=self.input_box("Reformat", "Enter the mark ending a block") {
                let ending_mark=ending_mark.trim();

                if ending_mark.is_empty() {
                    return;
                    }

                self.content.reformat(beginning_mark, ending_mark).unwrap();
                }
            }
        }

    //Configuration functions

    fn add_character_definition(&mut self) {

        let character=self.content.get_current_character();

        if let Some(definition)=self.input_box("Add character definition", &format!("Enter the definition for the '{character}' character.")) {
            if definition.is_empty() {
                return;
                }

            self.settings.text_renderer.add_character_definition(character, &definition);
            }

        }

    fn add_string_definition(&mut self) {
        if let Some(string)=self.input_box("Add phrase definition", "Enter the desired phrase to be defined for string rendering.") {
            if string.is_empty() {
                return;
                }

            if let Some(definition)=self.input_box("Add phrase definition", "Enter the definition for the entered phrase.") {
                if definition.is_empty() {
                    return;
                    }

                self.settings.text_renderer.add_string_definition(&string, &definition);
                }
            }
        }

    }

impl<'a> RideScreen<'a> {

    pub fn on_key_pressed(&mut self, key: &KeyboardShortcut) {
        if let Some(func) = self.keyboard_shortcuts_manager.get_function(key) {
            func(self);
            }
        else if !(key.control() ^ key.alt()) {
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
        self.settings.save(&Settings::get_settings_file_path("ride", "settings.yaml"));
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
        let (title, message)=(title.to_string(), message.to_string());
        let (message_box_sender, message_box_receiver)=std::sync::mpsc::channel::<()>();

        glib::source::idle_add_once(move || {
            let dialog = gtk::Dialog::new();
            dialog.set_title(&title);

            let label = gtk::Label::new(Some(&message));

            dialog.content_area().add(&label);

            dialog.add_button("Ok", gtk::ResponseType::Ok.into());

            dialog.show_all();

            dialog.run();

            dialog.close();

            message_box_sender.send(()).unwrap();
            });

        message_box_receiver.recv().unwrap();
        }

    pub fn input_box(&self, title: &str, message: &str) -> Option<String> {
        let (title, message)=(title.to_string(), message.to_string());
        let (input_box_sender, input_box_receiver)=std::sync::mpsc::channel::<Option<String>>();

        glib::source::idle_add_once(move || {
            let dialog = std::sync::Arc::new(gtk::Dialog::new());
            dialog.set_title(&title);

            let label = gtk::Label::new(Some(&message));
            let entry = gtk::Entry::new();

            dialog.content_area().add(&label);
            dialog.content_area().add(&entry);

            dialog.add_button("Ok", gtk::ResponseType::Ok.into());
            dialog.add_button("Cancel", gtk::ResponseType::Cancel.into());

            let dialog_clone=dialog.clone();
            entry.connect_key_press_event(move |_, key| {
                if key.keyval()==gdk::keys::constants::Return {
                    dialog_clone.response(gtk::ResponseType::Ok);
                    return Inhibit(true);
                    }

                Inhibit(false)
                });

            dialog.show_all();

            let response=dialog.run();

            let result=if response==gtk::ResponseType::Ok {
                Some(entry.text().to_string())
                }
            else {
                None
                };

            dialog.close();

            input_box_sender.send(result).unwrap();
            });

        input_box_receiver.recv().unwrap()
        }

    pub fn clipboard_get_text(&self) -> String {
        let (clipboard_sender, clipboard_receiver)=std::sync::mpsc::channel::<String>();

        glib::idle_add_once(move || {
            if let Some(display)=gdk::Display::default() {
                if let Some(clipboard)=gtk::Clipboard::default(&display) {
                    clipboard.request_text(move |_, result| {
                        if let Some(text)=result {
                            clipboard_sender.send(text.to_string()).unwrap();
                            }
                        else {
                            clipboard_sender.send("".to_string()).unwrap();
                            }
                        });
                    }
                }
            });

        clipboard_receiver.recv().unwrap()
        }

    pub fn clipboard_set_text(&self, text: &str) {
        let text=text.to_string();

        glib::source::idle_add_once(move || {
            if let Some(display)=gdk::Display::default() {
                if let Some(clipboard)=gtk::Clipboard::default(&display) {
                    clipboard.set_text(&text);
                    }
                }
            });
        }

    }

pub enum GtkThreadMessage {
    KeyPress(KeyboardShortcut),
    //WindowCreation(Arc<ApplicationWindow>),
    ApplicationExit,
    }

pub enum RideThreadMessage {
    SetWindowTitle(String),
    }

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
        }
    }
