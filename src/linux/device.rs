use std::{
    fs::{File, OpenOptions},
    ops::{Deref, DerefMut},
    os::fd::AsRawFd,
};

use libc::{c_int, c_ulong, IFF_NO_PI, IFF_TAP, IFF_TUN};

use crate::{ifreq, ifreq_new, Result};

const TUNSETIFF: libc::c_ulong = 0x400454CA;

#[derive(Debug, Default)]
pub struct Config {
    pub packet_information: bool,
}

impl Config {
    pub fn enable_packet_information(&mut self) -> &mut Self {
        self.packet_information = true;
        self
    }

    pub fn disable_packet_information(&mut self) -> &mut Self {
        self.packet_information = false;
        self
    }
}

pub struct Tun {
    file: File,
    pub ifname: String,
}

impl Deref for Tun {
    type Target = File;

    fn deref(&self) -> &Self::Target {
        &self.file
    }
}

impl DerefMut for Tun {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.file
    }
}

impl Tun {
    pub fn from_std(file: File, ifname: &str) -> Self {
        Self {
            file,
            ifname: String::from(ifname),
        }
    }

    pub fn into_std(self) -> File {
        self.file
    }

    fn new(ifname: &str, flags: i32) -> Result<Self> {
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .open("/dev/net/tun")?;

        let mut req = ifreq_new(&ifname);
        req.ifru.flags = flags;

        Ok(Self {
            file,
            ifname: String::from(ifname),
        })
    }

    pub fn new_tun(ifname: &str, config: &Config) -> Result<Self> {
        let mut flags = IFF_TUN;

        if !config.packet_information {
            flags |= IFF_NO_PI;
        }

        Self::new(ifname, flags)
    }

    pub fn new_tap(ifname: &str, config: &Config) -> Result<Self> {
        let mut flags = IFF_TAP;

        if !config.packet_information {
            flags |= IFF_NO_PI;
        }

        Self::new(ifname, flags)
    }
}

fn ioctl(fd: c_int, cmd: c_ulong, req: &mut ifreq) -> c_int {
    unsafe { libc::ioctl(fd, cmd as _, req as *mut ifreq) }
}
