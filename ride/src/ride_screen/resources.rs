use std::env;
use std::path::Path;

use bass::Sound;

pub struct Resources {
    pub bump: Sound,
    pub chil: Sound,
    pub capital: Sound,
    }

impl Resources {

    pub fn new() -> Resources {

        let mut bump=Sound::new();
        let mut chil=Sound::new();
        let mut capital=Sound::new();

        let mut root=env::current_exe().unwrap();
        root.pop();

        let sounds_root=Path::join(&root, "Sounds");

        bump.load(Path::join(&sounds_root, "Bump.wav").to_str().unwrap());
        chil.load(Path::join(&sounds_root, "Chil.wav").to_str().unwrap());
        capital.load(Path::join(&sounds_root, "Capital.wav").to_str().unwrap());

        Resources {bump, chil, capital}
        }
    }

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
        }
    }
