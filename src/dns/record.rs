use std::{fmt};
use std::net::{Ipv4Addr, Ipv6Addr};

type RecordName = String;

pub enum Record {
    Soa {
        name: RecordName,

        domain: String,
        email: String,

        serial: i32,
        refresh: i32,
        retry: i32,
        expire: i32,
        nct: i32
    },

    Ns {
        name: RecordName,
        domain: String
    },

    A {
        name: RecordName,
        address: Ipv4Addr
    },

    Aaaa {
        name: RecordName,
        address: Ipv6Addr
    },

    Mx {
        name: RecordName,
        priority: usize,
        domain: String
    },

    Cname {
        name: RecordName,
        alias: String
    },

    Txt {
        name: RecordName,
        value: String
    },

    Ptr {
        name: String,

        domain: String
    },

    Blank
}

impl fmt::Display for Record {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &*self {
            Record::Soa {name, domain, email, serial, refresh, retry, expire, nct} =>
                write!(f, "{name}\tIN\tSOA\t{domain}\t{email} ( \
                         {serial} \
                         {refresh} \
                         {retry} \
                         {expire} \
                         {nct} )",
                    name=name, domain=domain, email=email,
                    serial=serial, refresh=refresh, retry=retry, expire=expire, nct=nct
                ),
            Record::Ns {name, domain} => 
                write!(f, "{name}\tIN\tNS\t{domain}", name=name, domain=domain),
            Record::A {name, address} =>
                write!(f, "{name}\tIN\tA\t{address}", name=name, address=address),
            Record::Aaaa {name, address} =>
                write!(f, "{name}\tIN\tAAAA\t{address}", name=name, address=address),
            Record::Mx {name, priority, domain} =>
                write!(f, "{name}\tIN\tMX\t{priority}\t{domain}", name=name, priority=priority, domain=domain),
            Record::Cname {name, alias} =>
                write!(f, "{name}\tIN\tCNAME\t{alias}", name=name, alias=alias),
            Record::Txt {name, value} =>
                write!(f, "{name}\tIN\tTXT\t{value}", name=name, value=value),
            Record::Ptr {name, domain} =>
                write!(f, "{name}\tIN\tPTR\t{domain}", name=name, domain=domain),
            Record::Blank =>
                write!(f, "")
        }
    }
    
}