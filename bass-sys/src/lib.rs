use std::ffi::c_void;
use std::os::raw::c_char;

extern {
    pub fn BASS_Init(device: i32, freq: u32, flags: u32, p1: *mut c_void, p2: *mut c_void) -> bool;
    pub fn BASS_ErrorGetCode() -> i32;
    pub fn BASS_SampleLoad(mem: bool, file: *const c_char, offset: u64, length: u32, max: u32, flags: u32) -> u32;
    pub fn BASS_SampleGetChannel(handle: u32, onlynew: bool) -> u32;
    pub fn BASS_ChannelPlay(handle: u32, restart: bool) -> bool;

    }
