#![allow(non_camel_case_types)]

use libc::{c_char, c_int, c_uint};
use bitflags::bitflags;

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

macro_rules! __OSSIOWR {
    ($x:literal, $y:literal, $t:ty) => {
        (OSSIOC_INOUT | OSSIOC_SZ!($t) | (($x as c_int) << 8) | $y)
    };
}

pub const SNDCTL_SYSINFO: c_int = __OSSIOR!('X', 1, oss_sysinfo);
pub const SNDCTL_AUDIOINFO: c_int = __OSSIOWR!('X', 7, oss_audioinfo);
pub const SNDCTL_MIXERINFO: c_int = __OSSIOWR!('X', 10, oss_mixerinfo);
pub const SNDCTL_CARDINFO: c_int = __OSSIOWR!('X', 11, oss_card_info);
pub const OSS_GETVERSION: c_int = __OSSIOR!('M', 118, c_int);

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

pub const OSS_MAX_SAMPLE_RATES: usize = 20;

bitflags! {
    #[repr(transparent)]
    #[derive(Debug)]
    pub struct AudioCaps: libc::c_int {
        const PCM_CAP_DUPLEX = 0x00000100; /* Full duplex rec/play */
        const PCM_CAP_REALTIME = 0x00000200; /* Not supported?! */
        const PCM_CAP_BATCH = 0x00000400; /* Not supported?! */
        const PCM_CAP_COPROC = 0x00000800; /* Not supported?! */
        const PCM_CAP_TRIGGER = 0x00001000; /* Supports SETTRIGGER */
        const PCM_CAP_MMAP = 0x00002000; /* Supports mmap() */
        const PCM_CAP_MULTI = 0x00004000; /* Supports multiple open */
        const PCM_CAP_BIND = 0x00008000; /* Supports channel binding */
        const PCM_CAP_INPUT = 0x00010000; /* Supports recording */
        const PCM_CAP_OUTPUT = 0x00020000; /* Supports playback */
        const PCM_CAP_VIRTUAL = 0x00040000; /* Virtual device */
        const PCM_CAP_SHADOW = 0x01000000; /* "Shadow" device */
        const PCM_CAP_FREERATE = 0x10000000;
        const PCM_CAP_DEFAULT = 0x40000000; /* "Default" device */

        /*
         * Other bits may have been set by the OS.
         */
        const _ = !0;
    }
}

pub const PCM_CAP_REVISION: c_int = 0x000000ff; /* Revision level (0 to 255) */
pub const PCM_CAP_CH_MASK: c_int = 0x06000000; /* See DSP_CH_MASK below */

pub const OSS_LONGNAME_SIZE: usize = 64;
pub const OSS_LABEL_SIZE: usize = 16;
pub const OSS_DEVNODE_SIZE: usize = 32;

#[repr(C)]
pub struct oss_audioinfo {
    pub dev: c_int,
    pub name: [c_char; 64],
    pub busy: c_int,
    pub pid: c_int,
    pub caps: c_int,
    pub iformats: c_int,
    pub oformats: c_int,
    pub magic: c_int,
    pub cmd: [c_char; 64],
    pub card_number: c_int,
    pub port_number: c_int,
    pub mixer_dev: c_int,
    pub legacy_device: c_int,
    pub enabled: c_int,
    pub flags: c_int,
    pub min_rate: c_int,
    pub max_rate: c_int,
    pub min_channels: c_int,
    pub max_channels: c_int,
    pub binding: c_int,
    pub rate_source: c_int,
    pub handle: [c_char; 32],
    pub nrates: c_uint,
    pub rates: [c_uint; OSS_MAX_SAMPLE_RATES],
    pub song_name: [c_char; OSS_LONGNAME_SIZE],
    pub label: [c_char; OSS_LABEL_SIZE],
    pub latency: c_int,
    pub devnode: [c_char; OSS_DEVNODE_SIZE],
    pub next_play_engine: c_int,
    pub next_rec_engine: c_int,
    pub filler: [c_int; 184],
}

impl Default for oss_audioinfo {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

#[repr(C)]
pub struct oss_card_info {
    pub card: c_int,
    pub shortname: [c_char; 16],
    pub longname: [c_char; 128],
    pub flags: c_int,
    pub hw_info: [c_char; 400],
    pub intr_count: c_int,
    pub ack_count: c_int,
    pub filler: [c_int; 154],
}

impl Default for oss_card_info {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

#[repr(C)]
pub struct oss_mixerinfo {
    pub dev: c_int,
    pub id: [c_char; 16],
    pub name: [c_char; 32],
    pub modify_counter: c_int,
    pub card_number: c_int,
    pub port_number: c_int,
    pub handle: [c_char; 32],
    pub magic: c_int,
    pub enabled: c_int,
    pub caps: c_int,
    pub flags: c_int,
    pub nrext: c_int,
    pub priority: c_int,
    pub devnode: [c_char; OSS_DEVNODE_SIZE],
    pub legacy_device: c_int,
    pub filler: [c_int; 245],
}

impl Default for oss_mixerinfo {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/*
 * Make sure struct sizes match the C definitions.
 */
const _: () = assert!(std::mem::size_of::<oss_sysinfo>() == 0x4e0);
const _: () = assert!(std::mem::size_of::<oss_audioinfo>() == 0x49c);
const _: () = assert!(std::mem::size_of::<oss_card_info>() == 0x498);
const _: () = assert!(std::mem::size_of::<oss_mixerinfo>() == 0x470);
