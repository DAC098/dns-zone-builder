use std::{fmt};
use std::net::{Ipv4Addr, Ipv6Addr};

type RecordName = String;

pub enum Record {
    Soa {
        name: RecordName,

        ttl: usize,

        domain: String,
        email: String,

        serial: u32,
        refresh: u32,
        retry: u32,
        expire: u32,
        nct: u32
    },

    Ns {
        name: RecordName,
        ttl: usize,
        domain: String
    },

    A {
        name: RecordName,
        ttl: usize,
        address: Ipv4Addr
    },

    Aaaa {
        name: RecordName,
        ttl: usize,
        address: Ipv6Addr
    },

    Mx {
        name: RecordName,
        ttl: usize,
        priority: usize,
        domain: String
    },

    Cname {
        name: RecordName,
        ttl: usize,
        alias: String
    },

    Txt {
        name: RecordName,
        ttl: usize,
        value: String
    },

    Ptr {
        name: String,
        ttl: usize,
        domain: String
    },

    Blank
}

impl fmt::Display for Record {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &*self {
            Record::Soa {name, ttl, domain, email, serial, refresh, retry, expire, nct} =>
                write!(f, "{name}\t{ttl}\tIN\tSOA\t{domain}\t{email} ( \
                         {serial} \
                         {refresh} \
                         {retry} \
                         {expire} \
                         {nct} )",
                    name=name, ttl=ttl, domain=domain, email=email,
                    serial=serial, refresh=refresh, retry=retry, expire=expire, nct=nct
                ),
            Record::Ns {name, ttl, domain} => 
                write!(f, "{name}\t{ttl}\tIN\tNS\t{domain}", name=name, ttl=ttl, domain=domain),
            Record::A {name, ttl, address} =>
                write!(f, "{name}\t{ttl}\tIN\tA\t{address}", name=name, ttl=ttl, address=address),
            Record::Aaaa {name, ttl, address} =>
                write!(f, "{name}\t{ttl}\tIN\tAAAA\t{address}", name=name, ttl=ttl, address=address),
            Record::Mx {name, ttl, priority, domain} =>
                write!(f, "{name}\t{ttl}\tIN\tMX\t{priority}\t{domain}", name=name, ttl=ttl, priority=priority, domain=domain),
            Record::Cname {name, ttl, alias} =>
                write!(f, "{name}\t{ttl}\tIN\tCNAME\t{alias}", name=name, ttl=ttl, alias=alias),
            Record::Txt {name, ttl, value} =>
                write!(f, "{name}\t{ttl}\tIN\tTXT\t{value}", name=name, ttl=ttl, value=value),
            Record::Ptr {name, ttl, domain} =>
                write!(f, "{name}\t{ttl}\tIN\tPTR\t{domain}", name=name, ttl=ttl, domain=domain),
            Record::Blank =>
                write!(f, "")
        }
    }
    
}