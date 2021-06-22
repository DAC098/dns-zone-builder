use crate::config;

pub struct ConfigContext {
    file_path: std::path::PathBuf,
    directory: String,
    keys: config::KeysMap
}

impl ConfigContext {

    pub fn new(file_path: std::path::PathBuf, config: config::Config) -> ConfigContext {
        ConfigContext {
            file_path,
            directory: config.directory.unwrap_or("".to_owned()),
            keys: config.keys.unwrap_or(config::KeysMap::new())
        }
    }

    pub fn get_directory(&self) -> String {
        self.directory.clone()
    }

    pub fn find_key(&self, key: &String) -> Option<&String> {
        self.keys.get(key)
    }
}

pub struct ZoneContext {
    name: String,
    domain: String,
    reverse: bool,

    reverse_type: Option<config::ReverseType>,

    ttl: usize,

    keys: config::KeysMap
}

impl ZoneContext {

    pub fn new(zone: config::Zone) -> ZoneContext {
        ZoneContext {
            name: zone.name.clone(),
            domain: format!("{}.", zone.domain.unwrap_or(zone.name)),
            reverse_type: zone.reverse_type,
            reverse: match zone.reverse {
                Some(which) => match which {
                    config::ReverseValue::Bool(rtn) => rtn,
                    config::ReverseValue::Str(_) => true
                },
                None => false
            },
            ttl: zone.ttl.unwrap_or(604800),
            keys: zone.keys.unwrap_or(config::KeysMap::new())
        }
    }

    pub fn get_domain(&self) -> String {
        self.domain.clone()
    }

    pub fn with_domain(&self, domain: String) -> String {
        if domain.ends_with(".") {
            domain
        } else {
            format!("{}.{}", domain, self.get_domain())
        }
    }

    pub fn get_reverse_type(&self) -> Option<&config::ReverseType> {
        self.reverse_type.as_ref()
    }

    pub fn get_reverse(&self) -> bool {
        self.reverse
    }

    pub fn get_ttl(&self) -> usize {
        self.ttl
    }

    pub fn get_name_ref(&self) -> &String {
        &self.name
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn find_key(&self, key: &String) -> Option<&String> {
        self.keys.get(key)
    }
}

pub struct SubDomainContext {
    name: String,
    domain: String,
    reverse: bool,
}

impl SubDomainContext {

    pub fn new(zone: &ZoneContext, subdomain: config::SubDomain) -> SubDomainContext {
        let mut domain: String = subdomain.domain.unwrap_or(subdomain.name.clone());

        if !domain.ends_with(".") {
            if domain.eq("@") {
                domain = zone.get_domain();
            } else {
                domain = format!("{}.{}", domain, zone.get_domain());
            }
        }
        
        SubDomainContext {
            name: subdomain.name,
            domain,
            reverse: match subdomain.reverse {
                Some(which) => match which {
                    config::ReverseValue::Bool(rtn) => rtn,
                    config::ReverseValue::Str(_) => true
                },
                None => zone.get_reverse()
            }
        }
    }

    pub fn get_domain(&self) -> String {
        self.domain.clone()
    }

    pub fn get_reverse(&self) -> bool {
        self.reverse
    }
}