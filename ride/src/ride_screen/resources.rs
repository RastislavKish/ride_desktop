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
