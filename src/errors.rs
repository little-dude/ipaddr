use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct MalformedAddress;
impl fmt::Display for MalformedAddress {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "malformed IP address")
    }
}

impl Error for MalformedAddress {
    fn description(&self) -> &str {
        "malformed IP address"
    }

    fn cause(&self) -> Option<&Error> {
        None
    }
}

#[derive(Debug)]
/// Error returned when a string cannot be parsed into a valid IPv4 or IPv6 address. It contains
/// the string that triggered the error.
pub struct ParsingFailed(pub String);

impl fmt::Display for ParsingFailed {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "malformed address: \"{}\"", self.0)
    }
}

impl Error for ParsingFailed {
    fn description(&self) -> &str {
        "the string cannot be parsed as an IP address"
    }

    fn cause(&self) -> Option<&Error> {
        None
    }
}

#[derive(Debug)]
/// An integer was used as a CIDR mask, but it is not a valid CIDR.
pub struct InvalidMask;

impl fmt::Display for InvalidMask {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid mask")
    }
}

impl Error for InvalidMask {
    fn description(&self) -> &str {
        "not a valid mask"
    }

    fn cause(&self) -> Option<&Error> {
        None
    }
}
