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

use std::env;
use std::rc::Rc;
use std::sync::{mpsc, mpsc::{Receiver, Sender}};
use std::thread;
use std::time::Duration;

use gtk::prelude::*;

use gtk::{Application, ApplicationWindow};
use glib::{ControlFlow, Propagation};

use gio::ApplicationFlags;

mod core;
mod screen;
mod speech;
mod interface;

use interface::{RideScreen, GtkThreadMessage, RideThreadMessage};
use screen::KeyboardShortcut;

fn main() {
    bass::Sound::init();

    let (gtk_sender, ride_receiver) = mpsc::channel::<GtkThreadMessage>();
    let (ride_sender, gtk_receiver) = mpsc::channel::<RideThreadMessage>();
    let gtk_receiver=Rc::new(gtk_receiver);

    let handle=launch_ride_thread(ride_sender, ride_receiver);

    let application=Application::new(None, ApplicationFlags::HANDLES_OPEN);

    connect_application_activate_handler(&application, gtk_sender.clone(), gtk_receiver.clone());
    connect_application_open_handler(&application, gtk_sender.clone(), gtk_receiver.clone());

    application.run();

    gtk_sender.send(GtkThreadMessage::ApplicationExit).unwrap();
    handle.join().unwrap();
    }

fn activate_window(app: &Application, gtk_sender: Sender<GtkThreadMessage>, gtk_receiver: Rc<Receiver<RideThreadMessage>>) {

    let window=Rc::new(ApplicationWindow::new(app));
    window.set_title("Ride");
    window.set_default_size(350, 70);

    connect_key_press_handler(window.clone(), gtk_sender);
    setup_timer(window.clone(), gtk_receiver);

    window.show_all();
    }

fn launch_ride_thread(ride_sender: Sender<RideThreadMessage>, ride_receiver: Receiver<GtkThreadMessage>) -> thread::JoinHandle<()> {
    thread::spawn(move || {
        let file_path=match env::args().nth(1) {
            Some(path) => path,
            None => "".to_string(),
            };

        let mut ride_screen=RideScreen::new(&file_path, ride_sender);

        for received in ride_receiver {
            match received {
                GtkThreadMessage::KeyPress(key) => ride_screen.on_key_pressed(&key),
                GtkThreadMessage::ApplicationExit => break,
                };
            }

        ride_screen.on_exit();
        })
    }
fn connect_application_activate_handler(application: &Application, gtk_sender: Sender<GtkThreadMessage>, gtk_receiver: Rc<Receiver<RideThreadMessage>>) {
    application.connect_activate(move |app| {
        activate_window(app, gtk_sender.clone(), gtk_receiver.clone());
        });
    }
fn connect_application_open_handler(application: &Application, gtk_sender: Sender<GtkThreadMessage>, gtk_receiver: Rc<Receiver<RideThreadMessage>>) {
    application.connect_open(move |app, _, _| {
        activate_window(app, gtk_sender.clone(), gtk_receiver.clone());
        });
    }

fn connect_key_press_handler(window: Rc<ApplicationWindow>, gtk_sender: Sender<GtkThreadMessage>) {
    window.connect_key_press_event(move |_, key| {
        let keyboard_shortcut=KeyboardShortcut::from_eventkey(key);
        gtk_sender.send(GtkThreadMessage::KeyPress(keyboard_shortcut)).unwrap();

        Propagation::Proceed
        });
    }
fn setup_timer(window: Rc<ApplicationWindow>, gtk_receiver: Rc<Receiver<RideThreadMessage>>) {
    glib::source::timeout_add_local(Duration::from_millis(100), move || {
        if let Ok(message) = gtk_receiver.try_recv() {
            match message {
                RideThreadMessage::SetWindowTitle(title) => window.set_title(&title),
                };
            }

        ControlFlow::Continue
        });
    }
