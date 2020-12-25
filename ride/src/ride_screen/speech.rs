use speech_dispatcher::Connection;

pub struct Speech {
connection: Connection,
}

impl Speech {

pub fn new(app_name: &str) -> Speech {
let app_name=app_name.to_string();
let mut connection_name=String::clone(&app_name);
connection_name+="-connection";

let connection=Connection::open(app_name, connection_name, "".to_string(), speech_dispatcher::Mode::Single);
connection.set_language("sk");
connection.set_voice_pitch(10);
connection.set_voice_rate(10);

Speech {connection}
}

pub fn speak(&self, text: &str) {
self.connection.say(speech_dispatcher::Priority::Text, text);
}

}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
