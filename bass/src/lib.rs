/*
* Copyright (C) 2022 Rastislav Kish
*
* This program is free software: you can redistribute it and/or modify
* it under the terms of the GNU Lesser General Public License as published by
* the Free Software Foundation, version 2.1.
*
* This program is distributed in the hope that it will be useful,
* but WITHOUT ANY WARRANTY; without even the implied warranty of
* MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
* GNU Lesser General Public License for more details.
*
* You should have received a copy of the GNU Lesser General Public License
* along with this program. If not, see <https://www.gnu.org/licenses/>.
*/

use std::ffi::CString;
use std::ptr;

use bass_sys::*;

pub struct Sound {
    sample: u32,
    channel: u32,
    }

impl Sound {

    pub fn init() -> bool {
        unsafe {
            BASS_Init(-1, 44100, 0, ptr::null_mut(), ptr::null_mut())
            }
        }

    pub fn get_error_code() -> i32 {
        unsafe {
            BASS_ErrorGetCode()
            }
        }

    pub fn new() -> Sound {
        Sound {sample: 0, channel: 0}
        }

    pub fn load(&mut self, file_path: &str) -> u32 {
        let file_path=CString::new(file_path).unwrap();
        unsafe {
            self.sample=BASS_SampleLoad(false, file_path.as_ptr(), 0, 0, 1, 0);

            if self.sample!=0 {
                self.channel=BASS_SampleGetChannel(self.sample, false);
                }
            }

        self.channel
        }

    pub fn play(&self) {
        unsafe {
            BASS_ChannelPlay(self.channel, true);
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
