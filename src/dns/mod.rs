use std::{fmt};
use std::fmt::{Write};
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
use std::collections::{HashMap};

pub type FmtResult<T> = Result<T, fmt::Error>;

pub mod record;
pub mod zone;

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

pub fn ipv6_reverse_string(ipv6: &Ipv6Addr, include_suffix: bool) -> FmtResult<String> {
    let mut rtn: String = String::with_capacity(if include_suffix { 73 } else { 63 });
    let mut first: bool = true;
    let segments = ipv6.segments();

    for index in (0..8).rev() {
        let mut hex_str: String = format!("{:04x}", segments[index]);
        reverse_string(&mut hex_str);

        for c in hex_str.drain(..) {
            if first {
                write!(&mut rtn, "{}", c)?;
                first = false;
            } else {
                write!(&mut rtn, ".{}", c)?;
            }
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
    current: Option<String>,
    reverse_v4: Option<String>,
    reverse_v6: Option<String>
}

impl ZoneStorage {

    pub fn new() -> ZoneStorage {
        ZoneStorage {
            zones: HashMap::new(),
            current: None,
            reverse_v4: None,
            reverse_v6: None,
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

    pub fn set_rv4(&mut self, name: String) -> bool {
        if self.zones.contains_key(&name) {
            self.reverse_v4 = Some(name);
            true
        } else {
            false
        }
    }

    pub fn set_rv6(&mut self, name: String) -> bool {
        if self.zones.contains_key(&name) {
            self.reverse_v6 = Some(name);
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
    
    pub fn zone_add_record(&mut self, name: &String, record: record::Record) -> bool {
        if let Some(zone) = self.zones.get_mut(name) {
            zone.add_record(record);
            true
        } else {
            false
        }
    }

    pub fn rv4_add_record(&mut self, record: record::Record) -> bool {
        if let Some(reverse_v4) = self.reverse_v4.as_ref() {
            self.zones.get_mut(reverse_v4).unwrap().add_record(record);
            true
        } else {
            false
        }
    }

    pub fn rv6_add_record(&mut self, record: record::Record) -> bool {
        if let Some(reverse_v6) = self.reverse_v6.as_ref() {
            self.zones.get_mut(reverse_v6).unwrap().add_record(record);
            true
        } else {
            false
        }
    }

    pub fn reserve(&mut self, additional: usize) {
        self.zones.reserve(additional);
    }

    pub fn into_inner(self) -> HashMap<String, zone::Zone> {
        self.zones
    }
}