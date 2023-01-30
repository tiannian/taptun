use std::{
    mem,
    net::{SocketAddr, SocketAddrV4, SocketAddrV6},
};

use libc::{
    c_char, c_int, c_short, c_void, in6_addr, in_addr, sockaddr_in, sockaddr_in6, AF_INET,
    AF_INET6, IF_NAMESIZE,
};

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ifreq {
    pub ifrn: [c_char; IF_NAMESIZE],
    pub ifru: ifru,
}

pub fn ifreq_new(name: &str) -> ifreq {
    let mut a = unsafe { mem::zeroed::<ifreq>() };
    // set name.
    for (i, byte) in name.as_bytes().iter().enumerate() {
        a.ifrn[i] = *byte as libc::c_char
    }
    a
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union ifru {
    pub addr: sockaddr_in,
    pub addr6: sockaddr_in6,

    pub flags: i32,
    pub ivalue: c_int,
    pub mtu: c_int,
    pub slave: [c_char; IF_NAMESIZE],
    pub newname: [c_char; IF_NAMESIZE],
    pub data: *mut c_void,
}

impl From<SocketAddr> for ifru {
    fn from(value: SocketAddr) -> Self {
        match value {
            SocketAddr::V4(addr) => Self {
                addr: build_sockaddr_in(addr),
            },
            SocketAddr::V6(addr) => Self {
                addr6: build_sockaddr_in6(addr),
            },
        }
    }
}

fn build_sockaddr_in(addr: SocketAddrV4) -> sockaddr_in {
    let mut a = unsafe { mem::zeroed::<sockaddr_in>() };

    let ip = addr.ip();
    a.sin_family = AF_INET as u16;
    a.sin_port = addr.port();
    a.sin_addr = in_addr {
        s_addr: u32::from_ne_bytes(ip.octets()),
    };

    a
}

fn build_sockaddr_in6(addr: SocketAddrV6) -> sockaddr_in6 {
    let mut a = unsafe { mem::zeroed::<sockaddr_in6>() };

    let ip = addr.ip();
    a.sin6_family = AF_INET6 as u16;
    a.sin6_port = addr.port();
    a.sin6_scope_id = addr.scope_id();
    a.sin6_flowinfo = addr.flowinfo();
    a.sin6_addr = in6_addr {
        s6_addr: ip.octets(),
    };

    a
}
