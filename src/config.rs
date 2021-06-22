use std::collections::{HashMap};
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

use serde::{Deserialize};

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum Ipv4Value {
    Ip(Ipv4Addr),
    Str(String)
}

#[derive(Deserialize, Debug)]
pub struct Ipv4Detail {
    pub ip: Ipv4Value,
    pub reverse: Option<bool>
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum Ipv4Type {
    Ip(Ipv4Addr),
    Str(String),
    Detail(Ipv4Detail)
}

// ----------------------------------------------------------------------------

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum Ipv6Value {
    Ip(Ipv6Addr),
    Str(String)
}

#[derive(Deserialize, Debug)]
pub struct Ipv6Detail {
    pub ip: Ipv6Value,
    pub reverse: Option<bool>
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum Ipv6Type {
    Ip(Ipv6Addr),
    Str(String),
    Detail(Ipv6Detail)
}

// ----------------------------------------------------------------------------

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum Ipv4Address {
    Single(Ipv4Type),
    Multiple(Vec<Ipv4Type>)
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum Ipv6Address {
    Single(Ipv6Type),
    Multiple(Vec<Ipv6Type>)
}

// ----------------------------------------------------------------------------

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum PtrValue {
    Ip(IpAddr),
    Str(String)
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum PtrAddress {
    Single(PtrValue),
    Multiple(Vec<PtrValue>)
}

// ----------------------------------------------------------------------------

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum ReverseValue {
    Bool(bool),
    Str(String)
}

type DomainName = Option<String>;

pub type KeysMap = HashMap<String, String>;

#[derive(Deserialize, Debug)]
#[serde(tag = "type")]
pub enum Record {
    Soa {
        domain: String,
        email: String,

        serial: i32,
        refresh: i32,
        retry: i32,
        expire: i32,
        nct: i32
    },

    Ns {
        domain: String
    },

    A {
        address: Ipv4Address,
        reverse: Option<ReverseValue>
    },

    Aaaa {
        address: Ipv6Address,
        reverse: Option<ReverseValue>
    },

    Mx {
        priority: usize,
        domain: String
    },

    Cname {
        alias: String
    },

    Txt {
        value: String
    },

    Ptr {
        address: PtrAddress
    }
}

#[derive(Deserialize, Debug)]
pub struct SubDomain {
    pub name: String,
    pub domain: DomainName,

    pub reverse: Option<ReverseValue>,

    pub records: Vec<Record>,
}

#[derive(Deserialize, Debug)]
pub enum ReverseType {
    V4, V6
}

#[derive(Deserialize, Debug)]
pub struct Zone {
    pub name: String,
    pub domain: DomainName,

    pub reverse_type: Option<ReverseType>,

    pub ttl: Option<usize>,
    pub reverse: Option<ReverseValue>,

    pub keys: Option<KeysMap>,

    pub subdomains: Vec<SubDomain>
}

#[derive(Deserialize, Debug)]
pub struct Config {
    pub name: String,
    pub directory: Option<String>,

    pub keys: Option<KeysMap>,

    pub zones: Vec<Zone>
}

