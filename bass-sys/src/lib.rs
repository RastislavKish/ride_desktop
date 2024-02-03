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

use std::ffi::c_void;
use std::os::raw::c_char;

extern {
    pub fn BASS_Init(device: i32, freq: u32, flags: u32, p1: *mut c_void, p2: *mut c_void) -> bool;
    pub fn BASS_ErrorGetCode() -> i32;
    pub fn BASS_SampleLoad(mem: bool, file: *const c_char, offset: u64, length: u32, max: u32, flags: u32) -> u32;
    pub fn BASS_SampleGetChannel(handle: u32, onlynew: bool) -> u32;
    pub fn BASS_ChannelPlay(handle: u32, restart: bool) -> bool;

    }
