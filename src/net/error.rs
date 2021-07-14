use std::{fmt};

#[derive(Debug)]
pub enum Error {
    InvalidV4Cidr(u8),
    InvalidV6Cidr(u8)
}

impl Error {

    pub fn get_msg(&self) -> String {
        match self {
            Error::InvalidV4Cidr(cidr) =>
                format!("given cidr is invalid for Ipv4 address. must be between 1 and 32 given: {}", cidr),
            Error::InvalidV6Cidr(cidr) =>
                format!("given cidr is invalid for Ipv6 address. must be between 1 and 128 given: {}", cidr)
        }
    }

}

impl fmt::Display for Error {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let msg = self.get_msg();
        f.write_str(msg.as_str())
    }

}

pub type Result<T> = std::result::Result<T, Error>;