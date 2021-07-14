use std::io::prelude::*;

use lazy_static::lazy_static;
use regex::{Regex};

mod error;
mod net;
mod dns;
mod config;
mod context;

lazy_static! {
    static ref KEY_REGEX: Regex = Regex::new(r"\{([_0-9a-zA-Z]+)}").unwrap();
}

type ZonePreBuilt = (
    String, // zone name
    context::ZoneContext,
    Vec<config::SubDomain>
);

fn main() {
    std::process::exit(match app_runner() {
        Ok(code) => code,
        Err(err) => {
            println!("{}", err);

            err.get_code()
        }
    });
}

fn v4_test() {
    let test = net::Ipv4AddrCidr::new(154, 40, 43, 205, 32).unwrap();
    let avail = test.available_addresses();
    let addr = test.as_u32();
    let start = test.start_u32();
    let finish = test.finish_u32();
    println!("{}", test);
    println!("  prefix: {}", test.prefix());
    println!("  addr  : {}", test.addr_ref());
    println!("  cidr  : {}", test.cidr_ref());
    println!("  avail : {}", avail);
    println!("  start : {}", test.start());
    println!("  finish: {}", test.finish());
    println!("  cidr_mask : {:0>32b}", avail - 1);
    println!("  addr_bin  : {:0>32b}", addr);
    println!("  start_bin : {:0>32b}", start);
    println!("  finish_bin: {:0>32b}", finish);
    println!("  addr_hex  : {:0>8x}", addr);
    println!("  start_hex : {:0>8x}", start);
    println!("  finish_hex: {:0>8x}", finish);
}

fn v6_test() {
    let test = net::Ipv6AddrCidr::new(0x28e4, 0xd3e8, 0x6ca1, 0x6c21, 0x14f6, 0xc4a8,0x20a0, 0xc409, 48).unwrap();
    let avail = test.available_addresses();
    let addr = test.as_u128();
    let start = test.start_u128();
    let finish = test.finish_u128();
    println!("{}", test);
    println!("  prefix: {}", test.prefix());
    println!("  addr  : {}", test.addr_ref());
    println!("  cidr  : {} {}", test.cidr_ref(), *test.cidr_ref() / 4);
    println!("  avail : {}", avail);
    println!("  start : {}", test.start());
    println!("  finish: {}", test.finish());
    println!("  cidr_mask : {:0>128b}", avail - 1);
    println!("  addr_bin  : {:0>128b}", addr);
    println!("  start_bin : {:0>128b}", start);
    println!("  finish_bin: {:0>128b}", finish);
    println!("  addr_hex  : {:0>32x}", addr);
    println!("  start_hex : {:0>32x}", start);
    println!("  finish_hex: {:0>32x}", finish);
    println!("  prefix_dns: {}", dns::ipv6_reverse_prefix(&test, true).unwrap());
}

fn app_runner() -> error::Result<i32> {
    v4_test();
    v6_test();

    let mut files: Vec<std::path::PathBuf> = vec!();
    let mut args = std::env::args();
    args.next();

    while let Some(arg) = args.next() {
        if let Ok(canonical_path) = std::fs::canonicalize(arg.clone()) {
            if !canonical_path.is_file() {
                return Err(error::RuntimeError::InvalidFile(canonical_path.into_os_string()));
            }

            files.push(canonical_path);
        } else {
            return Err(error::RuntimeError::FileNotFound(arg));
        }
    }

    for file in files {
        let mut new_zones = dns::ZoneStorage::new();
        let mut conf = load_file(file.clone())?;
        let mut pre_builts: Vec<ZonePreBuilt> = Vec::with_capacity(conf.zones.len());
        let zones = std::mem::take(&mut conf.zones);
        let conf_context = context::ConfigContext::new(file, conf);

        new_zones.reserve(zones.len());

        // first pass
        for mut zone in zones {
            let reverse_zone_info = std::mem::take(&mut zone.reverse_zone);
            let subdomains = std::mem::take(&mut zone.subdomains);
            let mut zone_context = context::ZoneContext::new(zone);

            if new_zones.has_zone(zone_context.get_name_ref()) {
                println!("duplicate zone name encountered. name: \"{}\"", zone_context.get_name_ref());
                continue;
            }

            if let Some(info) = reverse_zone_info {
                match info.r#type {
                    config::ReverseType::V4 => {
                        let addr_cidr = net::Ipv4AddrCidr::from_addr(
                            info.addr.parse::<std::net::Ipv4Addr>().map_err(|_| error::RuntimeError::Error(
                                format!("given ipv4 is invalid for reverse zone. given: {}", info.addr)
                            ))?,
                            info.cidr
                        ).map_err(|e| error::RuntimeError::Error(
                            e.get_msg()
                        ))?;

                        zone_context.set_domain(dns::ipv4_reverse_prefix(&addr_cidr, true)?);

                        new_zones.add_v4_rev_zone(
                            dns::zone::Zone::new(
                                zone_context.get_name(),
                                zone_context.get_domain()
                            ),
                            addr_cidr
                        );
                    },
                    config::ReverseType::V6 => {
                        let addr_cidr = net::Ipv6AddrCidr::from_addr(
                            info.addr.parse::<std::net::Ipv6Addr>().map_err(|_| error::RuntimeError::Error(
                                format!("given ipv6 is invalid for reverse zone. given: {}", info.addr)
                            ))?,
                            info.cidr
                        ).map_err(|e| error::RuntimeError::Error(
                            e.get_msg()
                        ))?;

                        zone_context.set_domain(dns::ipv6_reverse_prefix(&addr_cidr, true)?);

                        new_zones.add_v6_rev_zone(
                            dns::zone::Zone::new(
                                zone_context.get_name(),
                                zone_context.get_domain()
                            ),
                            addr_cidr
                        );
                    }
                }
            } else {
                new_zones.add_zone(dns::zone::Zone::new(
                    zone_context.get_name(),
                    zone_context.get_domain()
                ));
            }

            pre_builts.push((zone_context.get_name(), zone_context, subdomains));
        }

        for (name, zone_context, subdomains) in pre_builts {
            let mut first = true;
            new_zones.set_current(name);

            for mut subdomain in subdomains {
                if !first {
                    new_zones.add_record(dns::record::Record::Blank);
                } else {
                    first = false;
                }

                let records = std::mem::take(&mut subdomain.records);
                let subdomain_contex = context::SubDomainContext::new(&zone_context, subdomain);

                for record in records {
                    parse_record(&mut new_zones, &conf_context, &zone_context, &subdomain_contex, record)?;
                }
            }
        }

        for (_, zone) in new_zones.into_inner() {
            println!("handling zone: {}", zone.get_name());
            let mut path = std::path::PathBuf::new();
            path.push(conf_context.get_directory_ref().as_str());
            path.push(zone.get_name_ref().as_str());

            let mut tmp_path = std::path::PathBuf::new();
            tmp_path.push("/tmp");
            tmp_path.push(zone.get_name_ref().as_str());
            let mut tmp_file = std::fs::File::create(tmp_path.as_path())?;

            write!(tmp_file, "; ------------------------------------------------------------------------------\n")?;
            write!(tmp_file, "; zone file generated from dns-zones-builder\n")?;
            write!(tmp_file, "; {}\n", path.display())?;
            write!(tmp_file, "{}\n", zone)?;

            let cmd = std::process::Command::new("named-checkzone")
                .arg(zone.get_origin_ref().as_str())
                .arg(tmp_path.as_path())
                .output()?;

            if cmd.status.code().unwrap() != 0 {
                std::io::stderr().write_all(&cmd.stderr).unwrap();
                std::fs::remove_file(tmp_path.as_path())?;
            } else {
                std::io::stdout().write_all(&cmd.stdout).unwrap();
                std::fs::rename(tmp_path.as_path(), path.as_path())?;
            }
        }
    }

    Ok(0)
}

fn load_file(file: std::path::PathBuf) -> error::Result<config::Config> {
    if let Some(ext) = file.extension() {
        if ext.eq("yaml") || ext.eq("yml") {
            Ok(serde_yaml::from_reader::<
                std::io::BufReader<std::fs::File>,
                config::Config
            >(std::io::BufReader::new(
                std::fs::File::open(&file)?
            ))?)
        } else if ext.eq("json") {
            Ok(serde_json::from_reader::<
                std::io::BufReader<std::fs::File>,
                config::Config
            >(std::io::BufReader::new(
                std::fs::File::open(&file)?
            ))?)
        } else {
            Err(error::RuntimeError::InvalidFileExtension(ext.to_os_string()))
        }
    } else {
        Err(error::RuntimeError::UnknownFileExtension)
    }
}

fn get_reverse(reverse: config::ReverseValue) -> bool {
    match reverse {
        config::ReverseValue::Bool(rtn) => rtn,
        config::ReverseValue::Str(_) => true
    }
}

fn parse_record(
    dns_zone: &mut dns::ZoneStorage,
    config: &context::ConfigContext,
    zone: &context::ZoneContext,
    subdomain: &context::SubDomainContext,
    record: config::Record
) -> error::Result<()> {
    match record {
        config::Record::Soa {
            domain, email, 
            serial, refresh, retry, expire, nct
        } => {
            dns_zone.add_record(dns::record::Record::Soa {
                name: subdomain.get_domain(),
                ttl: zone.get_ttl(),
                domain: zone.with_domain(domain), 
                email: zone.with_domain(email),
                serial, refresh, 
                retry, expire, nct
            });
        },
        config::Record::Ns {domain} => {
            dns_zone.add_record(dns::record::Record::Ns {
                name: subdomain.get_domain(),
                ttl: zone.get_ttl(),
                domain: zone.with_domain(domain)
            });
        },
        config::Record::A {address, reverse} => {
            let reverse = if let Some(v) = reverse {
                get_reverse(v)
            } else {
                subdomain.get_reverse()
            };

            match address {
                config::Ipv4Address::Single(single) => {
                    parse_ipv4_type(dns_zone, config, zone, subdomain, single, reverse)?;
                },
                config::Ipv4Address::Multiple(multiple) => {
                    for addr in multiple {
                        parse_ipv4_type(dns_zone, config, zone, subdomain, addr, reverse)?;
                    }
                }
            };
        },
        config::Record::Aaaa {address, reverse} => {
            let reverse = if let Some(v) = reverse {
                get_reverse(v)
            } else {
                subdomain.get_reverse()
            };

            match address {
                config::Ipv6Address::Single(single) => {
                    parse_ipv6_type(dns_zone, config, zone, subdomain, single, reverse)?;
                },
                config::Ipv6Address::Multiple(multiple) => {
                    for addr in multiple {
                        parse_ipv6_type(dns_zone, config, zone, subdomain, addr, reverse)?;
                    }
                }
            };
        },
        config::Record::Mx {priority, domain} => {
            dns_zone.add_record(dns::record::Record::Mx {
                name: subdomain.get_domain(),
                ttl: zone.get_ttl(),
                priority,
                domain: zone.with_domain(domain)
            });
        },
        config::Record::Cname {alias} => {
            dns_zone.add_record(dns::record::Record::Cname {
                name: subdomain.get_domain(),
                ttl: zone.get_ttl(),
                alias: zone.with_domain(alias)
            });
        },
        config::Record::Txt {value} => {
            dns_zone.add_record(dns::record::Record::Txt {
                name: subdomain.get_domain(),
                ttl: zone.get_ttl(),
                value
            });
        },
        config::Record::Ptr {address} => {
            match address {
                config::PtrAddress::Single(single) => {
                    parse_ptr_value(dns_zone, config, zone, subdomain, single)?;
                },
                config::PtrAddress::Multiple(multiple) => {
                    for addr in multiple {
                        parse_ptr_value(dns_zone, config, zone, subdomain, addr)?;
                    }
                }
            };
        }
    }

    Ok(())
}

fn parse_ipv4_type(
    dns_zone: &mut dns::ZoneStorage,
    config: &context::ConfigContext,
    zone: &context::ZoneContext,
    subdomain: &context::SubDomainContext,
    ipv4_type: config::Ipv4Type,
    mut reverse: bool,
) -> error::Result<()> {
    let ip = match ipv4_type {
        config::Ipv4Type::Ip(ip) => ip,
        config::Ipv4Type::Str(string) => get_ipv4_from_string(config, zone, string)?,
        config::Ipv4Type::Detail(detail) => {
            reverse = detail.reverse.unwrap_or(reverse);

            match detail.ip {
                config::Ipv4Value::Ip(i) => i,
                config::Ipv4Value::Str(string) => get_ipv4_from_string(config, zone, string)?
            }
        }
    };

    if reverse {
        if !dns_zone.add_v4_reverse_record(&ip, dns::record::Record::Ptr {
            name: dns::ipv4_reverse_string(&ip, true)?,
            ttl: zone.get_ttl(),
            domain: subdomain.get_domain()
        }) {
            println!("failed to find reverse zone for ip address: {}", ip);
        }
    }

    dns_zone.add_record(dns::record::Record::A {
        name: subdomain.get_domain(),
        ttl: zone.get_ttl(),
        address: ip
    });

    Ok(())
}

fn parse_keyed_string(
    config: &context::ConfigContext,
    zone: &context::ZoneContext,
    string: String
) -> error::Result<String> {
    let mut working = string.clone();

    for capture in KEY_REGEX.captures_iter(string.as_str()) {
        let key = &capture[1].to_string();

        if let Some(value) = zone.find_key(&key) {
            working = working.replace(&capture[0], value.as_str());
        } else if let Some(value) = config.find_key(&key) {
            working = working.replace(&capture[0], value.as_str());
        } else {
            return Err(error::RuntimeError::Error(
                format!("failed to find requested key: {}", key)
            ));
        }
    }

    Ok(working)
}

fn get_ipv4_from_string(
    config: &context::ConfigContext,
    zone: &context::ZoneContext,
    string: String
) -> error::Result<std::net::Ipv4Addr> {
    let working = parse_keyed_string(config, zone, string)?;

    if let Ok(ip) = working.parse::<std::net::Ipv4Addr>() {
        Ok(ip)
    } else {
        Err(error::RuntimeError::Error(
            format!("invalid ipv4 string given: {}", working)
        ))
    }
}

fn parse_ipv6_type(
    dns_zone: &mut dns::ZoneStorage,
    config: &context::ConfigContext,
    zone: &context::ZoneContext,
    subdomain: &context::SubDomainContext,
    ipv6_type: config::Ipv6Type,
    mut reverse: bool,
) -> error::Result<()> {
    let ip = match ipv6_type {
        config::Ipv6Type::Ip(ip) => ip,
        config::Ipv6Type::Str(string) => get_ipv6_from_string(config, zone, string)?,
        config::Ipv6Type::Detail(detail) => {
            reverse = detail.reverse.unwrap_or(reverse);

            match detail.ip {
                config::Ipv6Value::Ip(ip) => ip,
                config::Ipv6Value::Str(string) => get_ipv6_from_string(config, zone, string)?
            }
        }
    };

    if reverse {
        if !dns_zone.add_v6_reverse_record(&ip, dns::record::Record::Ptr {
            name: dns::ipv6_reverse_string(&ip, true)?,
            ttl: zone.get_ttl(),
            domain: subdomain.get_domain()
        }) {
            println!("failed to find reverse zone for ip address: {}", ip);
        }
    }

    dns_zone.add_record(dns::record::Record::Aaaa {
        name: subdomain.get_domain(),
        ttl: zone.get_ttl(),
        address: ip
    });

    Ok(())
}

fn get_ipv6_from_string(
    config: &context::ConfigContext,
    zone: &context::ZoneContext,
    string: String
) -> error::Result<std::net::Ipv6Addr> {
    let working = parse_keyed_string(config, zone, string)?;

    if let Ok(ip) = working.parse::<std::net::Ipv6Addr>() {
        Ok(ip)
    } else {
        Err(error::RuntimeError::Error(
            format!("invalid ipv6 string given: {}", working)
        ))
    }
}

fn parse_ptr_value(
    dns_zone: &mut dns::ZoneStorage,
    config: &context::ConfigContext,
    zone: &context::ZoneContext,
    subdomain: &context::SubDomainContext,
    ptr_value: config::PtrValue,
) -> error::Result<()> {
    let ip = match ptr_value {
        config::PtrValue::Ip(ip) => ip,
        config::PtrValue::Str(string) => get_ip_from_string(config, zone, string)?
    };

    dns_zone.add_record(dns::record::Record::Ptr {
        name: dns::ip_reverse_string(&ip, true)?,
        ttl: zone.get_ttl(),
        domain: subdomain.get_domain()
    });

    Ok(())
}

fn get_ip_from_string(
    config: &context::ConfigContext,
    zone: &context::ZoneContext,
    string: String
) -> error::Result<std::net::IpAddr> {
    let working = parse_keyed_string(config, zone, string)?;

    if let Ok(ip) = working.parse::<std::net::Ipv4Addr>() {
        Ok(std::net::IpAddr::V4(ip))
    } else if let Ok(ip) = working.parse::<std::net::Ipv6Addr>() {
        Ok(std::net::IpAddr::V6(ip))
    } else {
        Err(error::RuntimeError::Error(
            format!("invalid ipv4/ipv6 string given: {}", working)
        ))
    }
}