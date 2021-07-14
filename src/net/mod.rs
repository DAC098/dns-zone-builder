use std::{fmt};
use std::net::{Ipv4Addr, Ipv6Addr};

pub mod error;

#[inline]
pub fn ipv4_to_u32(addr: &Ipv4Addr) -> u32 {
    u32::from_be_bytes(addr.octets())
}

#[inline]
pub fn ipv6_to_u128(addr: &Ipv6Addr) -> u128 {
    u128::from_be_bytes(addr.octets())
}

pub struct Ipv4AddrCidr {
    addr: Ipv4Addr,
    cidr: u8
}

impl Ipv4AddrCidr {

    pub fn check_cidr(cidr: &u8) -> bool {
        *cidr < 1 || *cidr > 32
    }

    pub fn new(a: u8, b: u8, c: u8, d: u8, cidr: u8) -> error::Result<Ipv4AddrCidr> {
        if Ipv4AddrCidr::check_cidr(&cidr) {
            Err(error::Error::InvalidV4Cidr(cidr))
        } else {
            Ok(Ipv4AddrCidr {
                addr: Ipv4Addr::new(a, b, c, d),
                cidr
            })
        }
    }

    pub fn from_addr(addr: Ipv4Addr, cidr: u8) -> error::Result<Ipv4AddrCidr> {
        if Ipv4AddrCidr::check_cidr(&cidr) {
            Err(error::Error::InvalidV4Cidr(cidr))
        } else {
            Ok(Ipv4AddrCidr {addr, cidr})
        }
    }

    pub fn set_addr(&mut self, addr: Ipv4Addr) -> () {
        self.addr = addr;
    }
    
    pub fn addr_ref(&self) -> &Ipv4Addr {
        &self.addr
    }

    pub fn addr_clone(&self) -> Ipv4Addr {
        self.addr.clone()
    }

    pub fn set_cidr(&mut self, cidr: u8) -> bool {
        if Ipv4AddrCidr::check_cidr(&cidr) {
            self.cidr = cidr;
            true
        } else {
            false
        }
    }

    pub fn cidr_ref(&self) -> &u8 {
        &self.cidr
    }

    pub fn cidr_clone(&self) -> u8 {
        self.cidr
    }

    pub fn available_addresses(&self) -> u32 {
        2u32.pow(32 - (self.cidr as u32))
    }

    pub fn as_u32(&self) -> u32 {
        ipv4_to_u32(&self.addr)
    }

    pub fn start_u32(&self) -> u32 {
        self.as_u32() & !(self.available_addresses() - 1)
    }

    pub fn start(&self) -> Ipv4Addr {
        Ipv4Addr::from(self.start_u32())
    }

    pub fn finish_u32(&self) -> u32 {
        self.as_u32() | (self.available_addresses() - 1)
    }

    pub fn finish(&self) -> Ipv4Addr {
        Ipv4Addr::from(self.finish_u32())
    }

    pub fn in_range(&self, check: &Ipv4Addr) -> bool {
        let avail = self.available_addresses();
        let check_value = ipv4_to_u32(check);
        let start = self.as_u32() & !(avail - 1);
        let finish = start | (avail - 1);

        check_value >= start && check_value <= finish
    }

    pub fn prefix(&self) -> String {
        format!("{}/{}", self.start(), self.cidr)
    }
}

impl fmt::Display for Ipv4AddrCidr {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}/{}", self.addr, self.cidr)
    }
    
}

pub struct Ipv6AddrCidr {
    addr: Ipv6Addr,
    cidr: u8
}

impl Ipv6AddrCidr {

    pub fn check_cidr(cidr: &u8) -> bool {
        *cidr < 1 || *cidr > 128
    }
    
    pub fn new(a: u16, b: u16, c: u16, d: u16, e: u16, f: u16, g: u16, h: u16, cidr: u8) -> error::Result<Ipv6AddrCidr> {
        if Ipv6AddrCidr::check_cidr(&cidr) {
            Err(error::Error::InvalidV6Cidr(cidr))
        } else {
            Ok(Ipv6AddrCidr {
                addr: Ipv6Addr::new(a, b, c, d, e, f, g, h),
                cidr
            })
        }
    }

    pub fn from_addr(addr: Ipv6Addr, cidr: u8) -> error::Result<Ipv6AddrCidr> {
        if Ipv6AddrCidr::check_cidr(&cidr) {
            Err(error::Error::InvalidV6Cidr(cidr))
        } else {
            Ok(Ipv6AddrCidr {addr, cidr})
        }
    }

    pub fn addr_ref(&self) -> &Ipv6Addr {
        &self.addr
    }

    pub fn addr_clone(&self) -> Ipv6Addr {
        self.addr.clone()
    }

    pub fn set_cidr(&mut self, cidr: u8) -> bool {
        if Ipv6AddrCidr::check_cidr(&cidr) {
            self.cidr = cidr;
            true
        } else {
            false
        }
    }

    pub fn cidr_ref(&self) -> &u8 {
        &self.cidr
    }

    pub fn cidr_clone(&self) -> u8 {
        self.cidr
    }

    pub fn available_addresses(&self) -> u128 {
        2u128.pow(128 - (self.cidr as u32))
    }

    pub fn as_u128(&self) -> u128 {
        ipv6_to_u128(&self.addr)
    }

    pub fn start_u128(&self) -> u128 {
        self.as_u128() & !(self.available_addresses() - 1)
    }

    pub fn start(&self) -> Ipv6Addr {
        Ipv6Addr::from(self.start_u128())
    }

    pub fn finish_u128(&self) -> u128 {
        self.as_u128() | (self.available_addresses() - 1)
    }

    pub fn finish(&self) -> Ipv6Addr {
        Ipv6Addr::from(self.finish_u128())
    }

    pub fn in_range(&self, check: &Ipv6Addr) -> bool {
        let avail = self.available_addresses();
        let check_value = ipv6_to_u128(check);
        let start = self.as_u128() & !(avail - 1);
        let finish = start | (avail - 1);

        check_value >= start && check_value <= finish
    }

    pub fn prefix(&self) -> String {
        format!("{}/{}", self.start(), self.cidr)
    }
}

impl fmt::Display for Ipv6AddrCidr {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}/{}", self.addr, self.cidr)
    }
    
}

pub enum IpAddrCidr {
    V4(Ipv4AddrCidr),
    V6(Ipv6AddrCidr)
}

impl IpAddrCidr {

    pub fn is_ipv4(&self) -> bool {
        match self {
            IpAddrCidr::V4(_) => true,
            IpAddrCidr::V6(_) => false
        }
    }

    pub fn is_ipv6(&self) -> bool {
        match self {
            IpAddrCidr::V4(_) => false,
            IpAddrCidr::V6(_) => true,
        }
    }

}