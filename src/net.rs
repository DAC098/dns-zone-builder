use std::net::{Ipv4Addr, Ipv6Addr};

pub fn v4_cidr_calc(classless: u8) -> Option<u32> {
    if classless > 32 {
        None
    } else {
        Some(2u32.pow(32u32 - (classless as u32)))
    }
}

pub fn v6_cidr_calc(classless: u8) -> Option<u128> {
    if classless > 128 {
        None
    } else {
        Some(2u128.pow(128u32 - (classless as u32)))
    }
}

pub fn v4_in_cidr_range(check: &Ipv4Addr, start: &Ipv4Addr, cidr: u8) -> Option<bool> {
    if let Some(avail) = v4_cidr_calc(cidr) {
        let start_value = u32::from(start.clone());
        let check_value = u32::from(check.clone());

        Some(check_value < (start_value + avail) && check_value >= start_value)
    } else {
        None
    }
}

pub fn v4_in_addr_range(check: &Ipv4Addr, start: &Ipv4Addr, end: &Ipv4Addr) -> {
    let start_value = u32::from(start.clone());
    let end_value = u32::from(end.clone());
    let check_value = u32::from(check.clone());

    check_value < end_value && check_value >= start_value
}

pub fn v6_in_cidr_range(check: &Ipv6Addr, start: &Ipv6Addr, cidr: u8) -> Option<bool> {
    if let Some(avail) = v6_cidr_calc(cidr) {
        let start_value = u128::from(start.clone());
        let check_value = u128::from(check.clone());

        Some(check_value < (start_value + avail) && check_value >= start_value)
    } else {
        None
    }
}

pub fn v6_in_addr_range(check: &Ipv6Addr, start: &Ipv6Addr, end: &Ipv6Addr) -> {
    let start_value = u128::from(start.clone());
    let end_value = u128::from(end.clone());
    let check_value = u128::from(check.clone());

    check_value < end_value && check_value >= start_value
}