use std::{fs::File, io::Write as _, os::fd::AsRawFd, path::Path};

use crate::{
    basic_ioctl, basic_ioctl_inout, basic_ioctl_noarg, c_chars_to_string, sys,
};

/**
 * DSP device nodes are how we play or record audio to specific outputs on the
 * system.
 */
pub struct Dsp {
    f: File,
}

impl Dsp {
    pub fn open_path<P: AsRef<Path>>(dsp: P) -> std::io::Result<Self> {
        let p = dsp.as_ref();

        /*
         * XXX the open mode seems like it needs to reflect whether we are
         * expecting to record (read) or play (write) or both...
         */
        let f = std::fs::OpenOptions::new().write(true).open(p)?;

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
        if maj != 4 {
            /*
             * XXX errors, please
             */
            return Err(std::io::Error::from_raw_os_error(libc::EINVAL));
        }

        Ok(Dsp { f })
    }

    pub fn sync(&self) -> std::io::Result<()> {
        basic_ioctl_noarg(&self.f, sys::SNDCTL_DSP_SYNC)
    }

    pub fn halt(&self) -> std::io::Result<()> {
        basic_ioctl_noarg(&self.f, sys::SNDCTL_DSP_HALT)
    }

    pub fn halt_input(&self) -> std::io::Result<()> {
        basic_ioctl_noarg(&self.f, sys::SNDCTL_DSP_HALT_INPUT)
    }

    pub fn halt_output(&self) -> std::io::Result<()> {
        basic_ioctl_noarg(&self.f, sys::SNDCTL_DSP_HALT_OUTPUT)
    }

    pub fn errors(&self) -> std::io::Result<sys::audio_errinfo> {
        basic_ioctl(&self.f, sys::SNDCTL_DSP_GETERROR)
    }

    pub fn volume_play(&self) -> std::io::Result<u8> {
        let v: libc::c_int = basic_ioctl(&self.f, sys::SNDCTL_DSP_GETPLAYVOL)?;
        Ok((v & 0xFF).try_into().unwrap())
    }

    pub fn volume_play_set(&self, percent: u8) -> std::io::Result<()> {
        let v: libc::c_int = percent.into();
        basic_ioctl_inout(&self.f, sys::SNDCTL_DSP_SETPLAYVOL, v)?;
        Ok(())
    }

    pub fn space_output(&self) -> std::io::Result<sys::audio_buf_info> {
        basic_ioctl(&self.f, sys::SNDCTL_DSP_GETOSPACE)
    }

    pub fn space_input(&self) -> std::io::Result<sys::audio_buf_info> {
        basic_ioctl(&self.f, sys::SNDCTL_DSP_GETISPACE)
    }

    pub fn channels(&self) -> std::io::Result<u32> {
        let v = basic_ioctl_inout(&self.f, sys::SNDCTL_DSP_CHANNELS, 0i32)?;
        if v == 0 {
            /*
             * This would be unexpected!
             */
            return Err(std::io::Error::from_raw_os_error(libc::EINVAL));
        }

        Ok(v.try_into().unwrap())
    }

    pub fn channels_set(&self, count: u32) -> std::io::Result<()> {
        if count == 0 {
            /*
             * This value is special; it means query the configured channel
             * count without changing it.  It also wouldn't make any sense as an
             * actual channel count.
             */
            return Err(std::io::Error::from_raw_os_error(libc::EINVAL));
        }

        let v: libc::c_int = basic_ioctl_inout(
            &self.f,
            sys::SNDCTL_DSP_CHANNELS,
            count.try_into().unwrap(),
        )?;

        if v != count.try_into().unwrap() {
            return Err(std::io::Error::from_raw_os_error(libc::EINVAL));
        }

        Ok(())
    }

    pub fn formats(&self) -> std::io::Result<sys::AudioFormats> {
        Ok(sys::AudioFormats::from_bits(basic_ioctl(
            &self.f,
            sys::SNDCTL_DSP_GETFMTS,
        )?)
        .unwrap())
    }

    pub fn format(&self) -> std::io::Result<sys::AudioFormats> {
        let v = basic_ioctl_inout(&self.f, sys::SNDCTL_DSP_SETFMT, 0i32)?;
        Ok(sys::AudioFormats::from_bits(v).unwrap())
    }

    pub fn format_set(&self, format: sys::AudioFormats) -> std::io::Result<()> {
        let bits = format.bits();
        if bits == 0 {
            /*
             * This value is special; it means query the configured format
             * without changing it.
             */
            return Err(std::io::Error::from_raw_os_error(libc::EINVAL));
        }

        let v: libc::c_int = basic_ioctl_inout(
            &self.f,
            sys::SNDCTL_DSP_SETFMT,
            bits,
        )?;

        if v != bits.try_into().unwrap() {
            return Err(std::io::Error::from_raw_os_error(libc::EINVAL));
        }

        Ok(())
    }

    pub fn speed(&self) -> std::io::Result<u32> {
        let v = basic_ioctl_inout(&self.f, sys::SNDCTL_DSP_SPEED, 0i32)?;
        Ok(v.try_into().unwrap())
    }

    pub fn speed_set(&self, speed: u32) -> std::io::Result<()> {
        if speed == 0 {
            /*
             * This value is special; it means query the configured speed
             * without changing it.  It also wouldn't make any sense as an
             * actual speed.
             */
            return Err(std::io::Error::from_raw_os_error(libc::EINVAL));
        }

        let v: libc::c_int = basic_ioctl_inout(
            &self.f,
            sys::SNDCTL_DSP_SPEED,
            speed.try_into().unwrap(),
        )?;

        if v != speed.try_into().unwrap() {
            return Err(std::io::Error::from_raw_os_error(libc::EINVAL));
        }

        Ok(())
    }

    pub fn play(&mut self, buf: &[u8]) -> std::io::Result<()> {
        self.f.write_all(buf)?;
        Ok(())
    }
}
