use std::env;
use std::rc::Rc;
use std::sync::{Arc, mpsc};
use std::thread;
use std::time::Duration;

use gtk::prelude::*;

use gtk::{Application, ApplicationWindow};

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

let application=Application::new(None, Default::default());

let tx_application=gtk_tx.clone();
application.connect_activate(move |app| {
let tx_key_press_event=tx_application.clone();

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
});

application.run();

gtk_tx.send(GtkThreadMessage::ApplicationExit).unwrap();
handle.join().unwrap();
}

