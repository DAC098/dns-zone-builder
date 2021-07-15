use std::{fmt};
use std::fmt::{Write};
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
use std::collections::{HashMap};

pub type FmtResult<T> = Result<T, fmt::Error>;

pub mod record;
pub mod zone;

use crate::net::{Ipv4AddrCidr, Ipv6AddrCidr, ipv6_to_u128};

fn reverse_string(string: &mut String) {
    unsafe {
        let vec = string.as_mut_vec();
        vec.reverse();
    }
}

pub fn ipv4_reverse_string(ipv4: &Ipv4Addr, include_suffix: bool) -> FmtResult<String> {
    let mut rtn: String = String::with_capacity(if include_suffix { 29 } else { 15 });
    let mut first: bool = true;

    for octet in ipv4.octets().iter().rev() {
        if first {
            write!(&mut rtn, "{}", octet)?;
            first = false;
        } else {
            write!(&mut rtn, ".{}", octet)?;
        }
    }

    if include_suffix {
        write!(&mut rtn, ".in-addr.arpa.")?;
    }

    rtn.shrink_to_fit();

    Ok(rtn)
}

pub fn ipv4_reverse_prefix(ipv4: &Ipv4AddrCidr, include_suffix: bool) -> FmtResult<String> {
    let mut first = true;
    let prefix_len = (*ipv4.cidr_ref() / 8) as usize;
    let expected_len = prefix_len * 3 + prefix_len - 1;
    let mut rtn: String = String::with_capacity(if include_suffix { expected_len +  14} else { expected_len });

    for octet in ipv4.addr_ref().octets().iter().take(prefix_len).rev() {
        if first {
            write!(&mut rtn, "{}", octet)?;
            first = false;
        } else {
            write!(&mut rtn, ".{}", octet)?;
        }
    }

    if include_suffix {
        write!(&mut rtn, ".in-addr.arpa.")?;
    }

    Ok(rtn)
}

pub fn ipv6_reverse_string(ipv6: &Ipv6Addr, include_suffix: bool) -> FmtResult<String> {
    let mut rtn: String = String::with_capacity(if include_suffix { 73 } else { 63 });
    let mut first: bool = true;
    let mut hex_str: String = format!("{:0>32x}", ipv6_to_u128(ipv6));
    reverse_string(&mut hex_str);

    for c in hex_str.drain(..) {
        if first {
            write!(&mut rtn, "{}", c)?;
            first = false;
        } else {
            write!(&mut rtn, ".{}", c)?;
        }
    }

    if include_suffix {
        write!(&mut rtn, ".ip6.arpa.")?;
    }

    Ok(rtn)
}

pub fn ipv6_reverse_prefix(ipv6: &Ipv6AddrCidr, include_suffix: bool) -> FmtResult<String> {
    let mut first = true;
    let prefix_len = (*ipv6.cidr_ref() / 4) as usize;
    let expected_len = prefix_len * 2 - 1;
    let mut rtn = String::with_capacity(if include_suffix { expected_len + 10 } else { expected_len });
    let mut hex_str: String = format!("{:0>32x}", ipv6.start_u128()).chars().take(prefix_len).collect();
    reverse_string(&mut hex_str);

    for c in hex_str.drain(..) {
        if first {
            write!(&mut rtn, "{}", c)?;
            first = false;
        } else {
            write!(&mut rtn, ".{}", c)?;
        }
    }

    if include_suffix {
        write!(&mut rtn, ".ip6.arpa.")?;
    }

    Ok(rtn)
}

pub fn ip_reverse_string(ip: &IpAddr, include_suffix: bool) -> FmtResult<String> {
    match ip {
        IpAddr::V4(v4) => ipv4_reverse_string(v4, include_suffix),
        IpAddr::V6(v6) => ipv6_reverse_string(v6, include_suffix)
    }
}

pub struct ZoneStorage {
    zones: HashMap<String, zone::Zone>,
    v4_reverse_zones: HashMap<String, Ipv4AddrCidr>,
    v6_reverse_zones: HashMap<String, Ipv6AddrCidr>,
    current: Option<String>,
}

impl ZoneStorage {

    pub fn new() -> ZoneStorage {
        ZoneStorage {
            zones: HashMap::new(),
            v4_reverse_zones: HashMap::new(),
            v6_reverse_zones: HashMap::new(),
            current: None,
        }
    }

    pub fn set_current(&mut self, name: String) -> bool {
        if self.zones.contains_key(&name) {
            self.current = Some(name);
            true
        } else {
            false
        }
    }

    pub fn add_zone(&mut self, zone: zone::Zone) -> bool {
        if self.zones.contains_key(zone.get_name_ref()) {
            false
        } else {
            self.current = Some(zone.get_name());
            self.zones.insert(zone.get_name(), zone);
            true
        }
    }

    pub fn add_v4_rev_zone(&mut self, zone: zone::Zone, addr_cidr: Ipv4AddrCidr) -> bool {
        if self.zones.contains_key(zone.get_name_ref()) {
            false
        } else {
            self.current = Some(zone.get_name());
            self.v4_reverse_zones.insert(zone.get_name(), addr_cidr);
            self.zones.insert(zone.get_name(), zone);
            true
        }
    }

    pub fn add_v6_rev_zone(&mut self, zone: zone::Zone, addr_cidr: Ipv6AddrCidr) -> bool {
        if self.zones.contains_key(zone.get_name_ref()) {
            false
        } else {
            self.current = Some(zone.get_name());
            self.v6_reverse_zones.insert(zone.get_name(), addr_cidr);
            self.zones.insert(zone.get_name(), zone);
            true
        }
    }

    pub fn has_zone(&self, name: &String) -> bool {
        self.zones.contains_key(name)
    }

    pub fn add_record(&mut self, record: record::Record) -> bool {
        if let Some(name) = self.current.as_ref() {
            self.zones.get_mut(name).unwrap().add_record(record);
            true
        } else {
            false
        }
    }

    pub fn add_v4_reverse_record(&mut self, addr: &Ipv4Addr, record: record::Record) -> bool {
        for (name, ip_cidr) in self.v4_reverse_zones.iter() {
            if ip_cidr.in_range(addr) {
                self.zones.get_mut(name).unwrap().add_record(record);
                return true;
            }
        }

        false
    }

    pub fn add_v6_reverse_record(&mut self, addr: &Ipv6Addr, record: record::Record) -> bool {
        for (name, ip_cidr) in self.v6_reverse_zones.iter() {
            if ip_cidr.in_range(addr) {
                self.zones.get_mut(name).unwrap().add_record(record);
                return true;
            }
        }

        false
    }

    pub fn reserve(&mut self, additional: usize) {
        self.zones.reserve(additional);
    }

    pub fn into_inner(self) -> HashMap<String, zone::Zone> {
        self.zones
    }
}