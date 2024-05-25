#![allow(non_camel_case_types)]

use libc::{c_char, c_int};

pub const OSSIOCPARM_MASK: c_int = 0x1fff;

pub const OSSIOC_VOID: c_int = 0x00000000;
pub const OSSIOC_OUT: c_int = 0x20000000;
pub const OSSIOC_IN: c_int = 0x40000000;

pub const OSSIOC_INOUT: c_int = OSSIOC_IN | OSSIOC_OUT;

macro_rules! OSSIOC_SZ {
    ($t:ty) => {
        (((std::mem::size_of::<$t>() & (OSSIOCPARM_MASK as usize)) << 16)
            as c_int)
    };
}

macro_rules! __OSSIO {
    ($x:literal, $y:literal) => {
        (OSSIOC_VOID | (x << 8) | y)
    };
}

macro_rules! __OSSIOR {
    ($x:literal, $y:literal, $t:ty) => {
        (OSSIOC_OUT | OSSIOC_SZ!($t) | (($x as c_int) << 8) | $y)
    };
}

pub const SNDCTL_SYSINFO: c_int = __OSSIOR!('X', 1, oss_sysinfo);
pub const OSS_GETVERSION: c_int = __OSSIOR!('M', 118, c_int);

#[derive(Debug)]
#[repr(C)]
pub struct oss_sysinfo {
    pub product: [c_char; 32],
    pub version: [c_char; 32],
    pub versionnum: c_int,
    pub options: [c_char; 128],

    pub numaudios: c_int,
    pub openedaudio: [c_int; 8],

    pub numsynths: c_int,
    pub nummidis: c_int,
    pub numtimers: c_int,
    pub nummixers: c_int,

    pub openedmidi: [c_int; 8],
    pub numcards: c_int,
    pub numaudioengines: c_int,
    pub license: [c_char; 16],
    pub revision_info: [c_char; 256],
    pub filler: [c_int; 172],
}

/*
 * Make sure struct sizes match the C definitions.
 */
const _: () = assert!(std::mem::size_of::<oss_sysinfo>() == 0x4e0);
