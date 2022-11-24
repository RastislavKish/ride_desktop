use speech_dispatcher::Connection;

pub struct Speech {
    connection: Connection,
    }

impl Speech {

    pub fn new(app_name: &str) -> Speech {
        let app_name=app_name.to_string();
        let mut connection_name=String::clone(&app_name);
        connection_name+="-connection";

        let connection=Connection::open(&app_name[..], &connection_name[..], "", speech_dispatcher::Mode::Threaded).unwrap();

        Speech {connection}
        }

    pub fn speak(&self, text: &str) {
        self.connection.say(speech_dispatcher::Priority::Text, text);
        }

    pub fn speak_char(&self, text: &str) {
        if text!=" " {
            self.connection.char(speech_dispatcher::Priority::Text, text).unwrap();
            }
        else {
            self.connection.char(speech_dispatcher::Priority::Text, "space").unwrap();
            }
        }

    }

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
        }
    }
