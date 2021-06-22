use std::{fmt};

use crate::dns::record;

pub struct Zone {
    name: String,
    directory: String,

    origin: String,

    ttl: usize,

    records: Vec<record::Record>
}

impl Zone {

    pub fn new(
        name: String, directory: String,
        origin: String,
        ttl: usize
    ) -> Zone {
        Zone {
            name, directory,
            origin,
            ttl, 
            records: vec!()
        }
    }

    pub fn get_name_ref(&self) -> &String {
        &self.name
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_directory(&self) -> String {
        self.directory.clone()
    }

    pub fn get_origin(&self) -> String {
        self.origin.clone()
    }

    pub fn add_record(&mut self, record: record::Record) {
        self.records.push(record.into());
    }
}

impl fmt::Display for Zone {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut first: bool = true;
        write!(f, "$TTL\t{}\n", self.ttl)?;

        for item in &self.records {
            if first {
                write!(f, "{}", item)?;
                first = false;
            } else {
                write!(f, "\n{}", item)?;
            }
        }

        Ok(())
    }
}