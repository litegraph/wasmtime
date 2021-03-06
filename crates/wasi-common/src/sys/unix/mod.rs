pub(crate) mod fdentry_impl;
pub(crate) mod host_impl;
mod host_string;
pub(crate) mod hostcalls_impl;

mod filetime;

cfg_if::cfg_if! {
    if #[cfg(target_os = "linux")] {
        mod linux;
        use self::linux as sys_impl;
    } else if #[cfg(target_os = "emscripten")] {
        mod emscripten;
        use self::emscripten as sys_impl;
    } else if #[cfg(any(target_os = "macos",
                        target_os = "netbsd",
                        target_os = "freebsd",
                        target_os = "openbsd",
                        target_os = "ios",
                        target_os = "dragonfly"))] {
        mod bsd;
        use self::bsd as sys_impl;
    }
}

use crate::Result;
use std::fs::{File, OpenOptions};
use std::path::Path;

pub(crate) use host_string::{
    hoststring_ends_with_slash, hoststring_from_osstring, osstr_ends_with_slash, HostString,
};

pub(crate) fn dev_null() -> Result<File> {
    OpenOptions::new()
        .read(true)
        .write(true)
        .open("/dev/null")
        .map_err(Into::into)
}

pub fn preopen_dir<P: AsRef<Path>>(path: P) -> Result<File> {
    File::open(path).map_err(Into::into)
}
