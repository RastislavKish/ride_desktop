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
use std::sync::{Arc, mpsc, mpsc::{Receiver, Sender}};
use std::thread;
use std::time::Duration;

use gtk::prelude::*;

use gtk::{Application, ApplicationWindow};

use gio::ApplicationFlags;

mod ride_screen;

use ride_screen::{RideScreen, GtkThreadMessage, RideThreadMessage};
use ride_screen::screen::KeyboardShortcut;

fn main() {
    bass::Sound::init();

    let (gtk_tx, ride_rx) = mpsc::channel::<GtkThreadMessage>();
    let (ride_tx, gtk_rx) = mpsc::channel::<RideThreadMessage>();
    let gtk_rx=Rc::new(gtk_rx);

    let handle=thread::spawn(move || {
        let file_path=match env::args().skip(1).next() {
            Some(path) => path,
            None => "".to_string(),
            };

        let mut ride_screen=RideScreen::new(&file_path, ride_tx);

        for received in ride_rx {
            match received {
                GtkThreadMessage::KeyPress(key) => ride_screen.on_key_pressed(&key),
                GtkThreadMessage::ApplicationExit => break,
                };
            }

        ride_screen.on_exit();
        });

    let application=Application::new(None, ApplicationFlags::HANDLES_OPEN);

    let activate_gtk_tx=gtk_tx.clone();
    let activate_gtk_rx=gtk_rx.clone();
    application.connect_activate(move |app| {
        activate_window(app, activate_gtk_tx.clone(), activate_gtk_rx.clone());
        });

    let open_gtk_tx=gtk_tx.clone();
    let open_gtk_rx=gtk_rx.clone();
    application.connect_open(move |app, _, _| {
        activate_window(app, open_gtk_tx.clone(), open_gtk_rx.clone());
        });

    application.run();

    gtk_tx.send(GtkThreadMessage::ApplicationExit).unwrap();
    handle.join().unwrap();
    }

fn activate_window(app: &Application, gtk_tx: Sender<GtkThreadMessage>, gtk_rx: Rc<Receiver<RideThreadMessage>>) {
    let tx_key_press_event=gtk_tx.clone();

    let window=Arc::new(ApplicationWindow::new(app));
    window.set_title("Ride");
    window.set_default_size(350, 70);

    window.connect_key_press_event(move |_, key| {

        let keyboard_shortcut=KeyboardShortcut::from_eventkey(&key);
        tx_key_press_event.send(GtkThreadMessage::KeyPress(keyboard_shortcut)).unwrap();

        Inhibit(false)
        });

    let window_timer=window.clone();
    let test_rx=gtk_rx.clone();
    glib::source::timeout_add_local(Duration::from_millis(100), move || {

        if let Ok(message) = test_rx.try_recv() {
            match message {
                RideThreadMessage::SetWindowTitle(title) => window_timer.set_title(&title),
                RideThreadMessage::HideWindow => window_timer.hide(),
                RideThreadMessage::ShowWindow => window_timer.show(),
                };
            }

        Continue(true)
        });

    //ride_screen.set_window(window.clone());

    window.show_all();
    }

