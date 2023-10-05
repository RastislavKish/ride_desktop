/*
* Copyright (C) 2023 Rastislav Kish
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

use std::sync::Mutex;

use kira::{
    manager::{
        AudioManager, AudioManagerSettings,
        backend::DefaultBackend,
        },
    sound::static_sound::{
        StaticSoundData, StaticSoundSettings,
        },
    };
use lazy_static::lazy_static;

lazy_static! {
    static ref AUDIO_MANAGER: Mutex<AudioManager>=Mutex::new(AudioManager::<DefaultBackend>::new(AudioManagerSettings::default()).unwrap());
    }

pub struct Sound {
    sound_data: Option<StaticSoundData>,
    }
impl Sound {

    pub fn new() -> Self {

        let sound_data=None;

        Self {sound_data}
        }

    pub fn load(&mut self, path: &str) {
        if let Ok(sound_data)=StaticSoundData::from_file(path, StaticSoundSettings::default()) {
            self.sound_data=Some(sound_data);
            }
        }

    pub fn play(&mut self) {
        if let Some(sound_data)=&self.sound_data {
            let mut manager=AUDIO_MANAGER.lock().unwrap();

            manager.play(sound_data.clone()).unwrap();
            }
        }

    }
