use std::{ffi::CStr, fs::File, mem::MaybeUninit, os::fd::AsRawFd as _};

pub mod mixer;
pub mod dsp;
pub mod sys;

pub use mixer::Mixer;
pub use dsp::Dsp;

fn basic_ioctl_inout<T>(f: &File, cmd: i32, mut buf: T) -> std::io::Result<T> {
    let fd = f.as_raw_fd();
    let r = unsafe { libc::ioctl(fd, cmd, &mut buf) };
    if r != 0 {
        return Err(std::io::Error::last_os_error());
    }

    Ok(buf)
}

fn basic_ioctl<T>(f: &File, cmd: i32) -> std::io::Result<T> {
    let mut buf: MaybeUninit<T> = MaybeUninit::uninit();

    let fd = f.as_raw_fd();
    let r = unsafe { libc::ioctl(fd, cmd, buf.as_mut_ptr()) };
    if r != 0 {
        return Err(std::io::Error::last_os_error());
    }

    Ok(unsafe { buf.assume_init() })
}

fn basic_ioctl_noarg(f: &File, cmd: i32) -> std::io::Result<()> {
    let fd = f.as_raw_fd();
    let r = unsafe { libc::ioctl(fd, cmd) };
    if r != 0 {
        return Err(std::io::Error::last_os_error());
    }

    Ok(())
}

fn c_chars_to_string(input: &[libc::c_char]) -> Option<String> {
    let input = unsafe { std::mem::transmute(input) };
    let cs = CStr::from_bytes_until_nul(input).ok()?;
    let s = cs.to_str().ok()?;
    Some(s.to_string())
}
