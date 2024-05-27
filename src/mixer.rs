use std::{fs::File, os::fd::AsRawFd, path::Path};

use crate::{basic_ioctl, basic_ioctl_inout, c_chars_to_string, sys};

/**
 * The "/dev/mixer" pseudo-device allows enumeration of audio devices in the
 * system.  This object requests that information but does not allow mixer
 * control.
 */
pub struct Mixer {
    f: File,
    maj: u32,
    min: u32,
}

impl Mixer {
    pub fn open() -> std::io::Result<Self> {
        Self::open_path("/dev/mixer")
    }

    pub fn open_path<P: AsRef<Path>>(mixer: P) -> std::io::Result<Self> {
        let p = mixer.as_ref();
        let f = std::fs::OpenOptions::new().read(true).write(true).open(p)?;

        /*
         * Perform an initial ioctl to get the OSS API version.
         */
        let mut ver: libc::c_int = 0;
        let fd = f.as_raw_fd();
        let r = unsafe { libc::ioctl(fd, sys::OSS_GETVERSION, &mut ver) };
        if r != 0 {
            return Err(std::io::Error::last_os_error());
        }

        let maj = ((ver as u32) & 0xFFFF0000u32) >> 16;
        let min = (ver as u32) & 0xFFFFu32;

        if maj != 4 {
            /*
             * XXX errors, please
             */
            return Err(std::io::Error::from_raw_os_error(libc::EINVAL));
        }

        Ok(Self { f, maj, min })
    }

    pub fn version(&self) -> (u32, u32) {
        (self.maj, self.min)
    }

    pub fn sysinfo(&self) -> std::io::Result<SysInfo> {
        let buf: sys::oss_sysinfo = basic_ioctl(&self.f, sys::SNDCTL_SYSINFO)?;

        let product = c_chars_to_string(&buf.product).unwrap();
        let version = c_chars_to_string(&buf.version).unwrap();
        let licence = c_chars_to_string(&buf.license).unwrap();

        Ok(SysInfo {
            product,
            version,
            maj: ((buf.versionnum as u32) & 0xFFFF0000u32) >> 16,
            min: (buf.versionnum as u32) & 0xFFFFu32,
            num_audios: buf.numaudios.try_into().unwrap(),
            num_mixers: buf.nummixers.try_into().unwrap(),
            num_cards: buf.numcards.try_into().unwrap(),
            num_audio_engines: buf.numaudioengines.try_into().unwrap(),
            licence,
        })
    }

    pub fn audioinfo(&self, index: u32) -> std::io::Result<AudioInfo> {
        let buf: sys::oss_audioinfo = basic_ioctl_inout(
            &self.f,
            sys::SNDCTL_AUDIOINFO,
            sys::oss_audioinfo {
                dev: index.try_into().unwrap(),
                ..Default::default()
            },
        )?;

        let caps = sys::AudioCaps::from_bits(buf.caps).unwrap();
        let caps_revision = (buf.caps & sys::PCM_CAP_REVISION) as u32;

        Ok(AudioInfo {
            dev: buf.dev.try_into().unwrap(),
            name: c_chars_to_string(&buf.name).unwrap(),
            card_number: buf.card_number.try_into().unwrap(),
            mixer_dev: buf.mixer_dev.try_into().unwrap(),
            caps,
            caps_revision,
            min_rate: buf.min_rate.try_into().unwrap(),
            max_rate: buf.max_rate.try_into().unwrap(),
            min_channels: buf.min_channels.try_into().unwrap(),
            max_channels: buf.max_channels.try_into().unwrap(),
            devnode: c_chars_to_string(&buf.devnode).unwrap(),
        })
    }

    pub fn cardinfo(&self, index: u32) -> std::io::Result<CardInfo> {
        let buf: sys::oss_card_info = basic_ioctl_inout(
            &self.f,
            sys::SNDCTL_CARDINFO,
            sys::oss_card_info {
                card: index.try_into().unwrap(),
                ..Default::default()
            },
        )?;

        Ok(CardInfo {
            shortname: c_chars_to_string(&buf.shortname).unwrap(),
            longname: c_chars_to_string(&buf.longname).unwrap(),
            hw_info: c_chars_to_string(&buf.hw_info).unwrap(),
        })
    }

    pub fn mixerinfo(&self, index: u32) -> std::io::Result<MixerInfo> {
        let buf: sys::oss_mixerinfo = basic_ioctl_inout(
            &self.f,
            sys::SNDCTL_MIXERINFO,
            sys::oss_mixerinfo {
                dev: index.try_into().unwrap(),
                ..Default::default()
            },
        )?;

        Ok(MixerInfo {
            dev: buf.dev.try_into().unwrap(),
            name: c_chars_to_string(&buf.name).unwrap(),
            card_number: buf.card_number.try_into().unwrap(),
            modify_counter: buf.modify_counter.try_into().unwrap(),
            nrext: buf.nrext.try_into().unwrap(),
            priority: buf.priority,
            devnode: c_chars_to_string(&buf.devnode).unwrap(),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SysInfo {
    pub product: String,
    pub version: String,
    pub maj: u32,
    pub min: u32,
    pub num_audios: u32,
    pub num_mixers: u32,
    pub num_cards: u32,
    pub num_audio_engines: u32,
    pub licence: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AudioInfo {
    pub dev: u32,
    pub name: String,
    pub card_number: u32,
    pub mixer_dev: u32,
    pub caps_revision: u32,
    pub caps: sys::AudioCaps,
    pub min_rate: u32,
    pub max_rate: u32,
    pub min_channels: u32,
    pub max_channels: u32,
    pub devnode: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CardInfo {
    pub shortname: String,
    pub longname: String,
    pub hw_info: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MixerInfo {
    pub dev: u32,
    pub name: String,
    pub modify_counter: u32,
    pub card_number: u32,
    pub nrext: u32,
    pub priority: i32,
    pub devnode: String,
}
