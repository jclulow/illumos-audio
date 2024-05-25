use std::{ffi::CStr, fs::File, mem::MaybeUninit, os::fd::AsRawFd, path::Path};

pub mod sys;

/**
 * The "/dev/mixer" pseudo-device allows enumeration of audio devices in the
 * system.  This object requests that information but does not allow mixer
 * control.
 */
pub struct MixerInfo {
    f: File,
    maj: u32,
    min: u32,
}

impl MixerInfo {
    pub fn open() -> std::io::Result<MixerInfo> {
        Self::open_path("/dev/mixer")
    }

    pub fn open_path<P: AsRef<Path>>(mixer: P) -> std::io::Result<MixerInfo> {
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

        Ok(MixerInfo { f, maj, min })
    }

    pub fn version(&self) -> (u32, u32) {
        (self.maj, self.min)
    }

    pub fn sysinfo(&self) -> std::io::Result<SysInfo> {
        //let mut buf: sys::oss_sysinfo = unsafe { std::mem::zeroed() };
        let mut buf: MaybeUninit<sys::oss_sysinfo> = MaybeUninit::uninit();

        let fd = self.f.as_raw_fd();
        let r =
            unsafe { libc::ioctl(fd, sys::SNDCTL_SYSINFO, buf.as_mut_ptr()) };
        if r != 0 {
            return Err(std::io::Error::last_os_error());
        }

        let buf = unsafe { buf.assume_init() };

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
}

fn c_chars_to_string(input: &[libc::c_char]) -> Option<String> {
    let input = unsafe { std::mem::transmute(input) };
    let cs = CStr::from_bytes_until_nul(input).ok()?;
    let s = cs.to_str().ok()?;
    Some(s.to_string())
}

#[derive(Debug)]
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
