use std::convert::TryFrom;
use std::ops::{BitAnd, Not};
use std::str::FromStr;

use core::fmt::{self, Binary, Formatter, LowerHex, UpperHex};

use {IPV4_MAX_PREFIXLEN, InvalidMask, Ipv4Address, ParsingFailed};

/// Check whether the given integer represents a valid IPv4 mask.
// see https://codereview.stackexchange.com/a/197138/118470
fn is_valid_mask(value: u32) -> bool {
    value.count_zeros() == value.trailing_zeros()
}

/// A valid IPv4 mask.
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct Ipv4Mask(u32);

impl Ipv4Mask {
    /// Return the mask as a prefix length;
    ///
    /// ```rust
    /// # #![feature(try_from)]
    /// # use ipaddr::Ipv4Mask;
    /// # use std::convert::TryFrom;
    /// # fn main() {
    /// assert_eq!(Ipv4Mask::try_from(0).unwrap().prefixlen(), 0);
    /// assert_eq!(Ipv4Mask::try_from(0xffff_ffff).unwrap().prefixlen(), 32);
    /// assert_eq!(Ipv4Mask::try_from(0xfff8_0000).unwrap().prefixlen(), 13);
    /// # }
    /// ```
    pub fn prefixlen(self) -> u32 {
        self.0.count_ones()
    }

    /// Create an `Ipv4Mask` from a prefix length.
    ///
    /// ```rust
    /// # #![feature(try_from)]
    /// # use ipaddr::Ipv4Mask;
    /// # use std::convert::TryFrom;
    /// # fn main() {
    /// assert_eq!(Ipv4Mask::from_prefixlen(0).unwrap(), Ipv4Mask::try_from(0).unwrap());
    /// assert_eq!(Ipv4Mask::from_prefixlen(32).unwrap(), Ipv4Mask::try_from(0xffff_ffff).unwrap());
    /// assert_eq!(Ipv4Mask::from_prefixlen(13).unwrap(), Ipv4Mask::try_from(0xfff8_0000).unwrap());
    /// # }
    /// ```
    pub fn from_prefixlen(prefixlen: u8) -> Result<Self, InvalidMask> {
        match prefixlen {
            IPV4_MAX_PREFIXLEN => Ok(Ipv4Mask(u32::max_value())),
            0 => Ok(Ipv4Mask(0)),
            n if n < IPV4_MAX_PREFIXLEN => {
                let mask: u32 = (1 << n) - 1;
                let mask = mask << (IPV4_MAX_PREFIXLEN - n);
                Ok(Ipv4Mask(mask))
            }
            _ => Err(InvalidMask),
        }
    }

    /// Return a human readable representation of the IPv4 mask.
    ///
    /// ```rust
    /// # #![feature(try_from)]
    /// # use ipaddr::{Ipv4Mask};
    /// # use std::string::String;
    /// # use std::convert::TryFrom;
    /// # fn main() {
    /// let mask = Ipv4Mask::try_from(0xfff8_0000).unwrap();
    /// assert_eq!(mask.to_string(), "255.248.0.0".to_string());
    /// # }
    /// ```
    pub fn to_string(self) -> String {
        Ipv4Address::from(self).to_string()
    }
}

impl fmt::Display for Ipv4Mask {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&self.to_string())
    }
}

impl fmt::Debug for Ipv4Mask {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Ipv4Mask(")?;
        f.write_str(&self.to_string())?;
        f.write_str(")")
    }
}

// XXX: do not impl AsMut<u32>, otherwise we might end up with an invalid mask
impl AsRef<u32> for Ipv4Mask {
    fn as_ref(&self) -> &u32 {
        &self.0
    }
}

// FIXME: impl AsRef<Ipv4Address> ?

impl From<Ipv4Mask> for u32 {
    fn from(value: Ipv4Mask) -> u32 {
        value.0
    }
}

impl<'a> From<&'a Ipv4Mask> for u32 {
    fn from(value: &'a Ipv4Mask) -> u32 {
        value.0
    }
}

impl TryFrom<u32> for Ipv4Mask {
    type Error = InvalidMask;

    fn try_from(value: u32) -> Result<Ipv4Mask, Self::Error> {
        if is_valid_mask(value) {
            Ok(Ipv4Mask(value))
        } else {
            Err(InvalidMask)
        }
    }
}

impl<'a> TryFrom<&'a u32> for Ipv4Mask {
    type Error = InvalidMask;
    fn try_from(value: &'a u32) -> Result<Ipv4Mask, Self::Error> {
        Ipv4Mask::try_from(*value)
    }
}

impl TryFrom<Ipv4Address> for Ipv4Mask {
    type Error = InvalidMask;
    fn try_from(value: Ipv4Address) -> Result<Ipv4Mask, Self::Error> {
        Ipv4Mask::try_from(u32::from(value))
    }
}

impl<'a> TryFrom<&'a Ipv4Address> for Ipv4Mask {
    type Error = InvalidMask;
    fn try_from(value: &'a Ipv4Address) -> Result<Ipv4Mask, Self::Error> {
        Ipv4Mask::try_from(u32::from(value))
    }
}

impl Not for Ipv4Mask {
    type Output = Ipv4Address;

    fn not(self) -> Self::Output {
        Ipv4Address(self.0.not())
    }
}

impl<'a> Not for &'a Ipv4Mask {
    type Output = Ipv4Address;

    fn not(self) -> Self::Output {
        Ipv4Address(self.0.not())
    }
}

impl LowerHex for Ipv4Mask {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        <u32 as LowerHex>::fmt(&self.0, f)
    }
}

impl UpperHex for Ipv4Mask {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        <u32 as UpperHex>::fmt(&self.0, f)
    }
}

impl Binary for Ipv4Mask {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        <u32 as Binary>::fmt(&self.0, f)
    }
}

impl BitAnd<Ipv4Mask> for Ipv4Address {
    type Output = Ipv4Address;
    fn bitand(self, rhs: Ipv4Mask) -> Self::Output {
        Ipv4Address(self.0.bitand(rhs.0))
    }
}

impl<'a> BitAnd<Ipv4Mask> for &'a Ipv4Address {
    type Output = Ipv4Address;
    fn bitand(self, rhs: Ipv4Mask) -> Self::Output {
        Ipv4Address(self.0.bitand(rhs.0))
    }
}

impl<'a> BitAnd<&'a Ipv4Mask> for Ipv4Address {
    type Output = Ipv4Address;
    fn bitand(self, rhs: &'a Ipv4Mask) -> Self::Output {
        Ipv4Address(self.0.bitand(rhs.0))
    }
}

impl<'a, 'b> BitAnd<&'a Ipv4Mask> for &'b Ipv4Address {
    type Output = Ipv4Address;
    fn bitand(self, rhs: &'a Ipv4Mask) -> Self::Output {
        Ipv4Address(self.0.bitand(rhs.0))
    }
}

impl FromStr for Ipv4Mask {
    type Err = ParsingFailed;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let ip = Ipv4Address::from_str(s)?;
        Ipv4Mask::try_from(ip).map_err(|_| ParsingFailed(s.into()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cidr_simple() {
        let mask = Ipv4Mask(0xff00_0000);
        assert_eq!(mask.prefixlen(), 8);

        let mask = Ipv4Mask(0xffff_ffff);
        assert_eq!(mask.prefixlen(), 32);

        let mask = Ipv4Mask(0x0000_0000);
        assert_eq!(mask.prefixlen(), 0);
    }

    #[test]
    fn test_from_int() {
        assert!(Ipv4Mask::try_from(0xff00_0000).is_ok());
        assert!(Ipv4Mask::try_from(0xffff_ffff).is_ok());
        assert!(Ipv4Mask::try_from(0x0000_0000).is_ok());
        assert!(Ipv4Mask::try_from(0xfe00_0000).is_ok());
        assert!(Ipv4Mask::try_from(0xf000_0000).is_ok());
        assert!(Ipv4Mask::try_from(0xffff_fff8).is_ok());

        assert!(Ipv4Mask::try_from(0xff70_0000).is_err());
        assert!(Ipv4Mask::try_from(0x0000_0001).is_err());
        assert!(Ipv4Mask::try_from(0xfe10_0000).is_err());
    }

    #[test]
    fn test_from_prefixlen_simple() {
        let mask = Ipv4Mask(0xfff8_0000);
        assert_eq!(Ipv4Mask::from_prefixlen(13).unwrap(), mask);

        let mask = Ipv4Mask(0xffff_ffff);
        assert_eq!(Ipv4Mask::from_prefixlen(32).unwrap(), mask);

        let mask = Ipv4Mask(0x0000_0000);
        assert_eq!(Ipv4Mask::from_prefixlen(0).unwrap(), mask);
    }

    #[test]
    fn test_cidr_single_bits() {
        let mask = Ipv4Mask(0xfe00_0000);
        assert_eq!(mask.prefixlen(), 7);

        let mask = Ipv4Mask(0xf000_0000);
        assert_eq!(mask.prefixlen(), 4);

        let mask = Ipv4Mask(0xffff_fff8);
        assert_eq!(mask.prefixlen(), 29);
    }

    #[test]
    fn test_from_prefixlen_single_bits() {
        let mask = Ipv4Mask(0xfe00_0000);
        assert_eq!(Ipv4Mask::from_prefixlen(7).unwrap(), mask);

        let mask = Ipv4Mask(0xf000_0000);
        assert_eq!(Ipv4Mask::from_prefixlen(4).unwrap(), mask);

        let mask = Ipv4Mask(0xffff_fff8);
        assert_eq!(Ipv4Mask::from_prefixlen(29).unwrap(), mask);
    }

    #[test]
    fn from_str() {
        let expected = Ipv4Mask(0xffff_ffff);
        assert_eq!(Ipv4Mask::from_str("255.255.255.255").unwrap(), expected);

        let expected = Ipv4Mask(0xffff_0000);
        assert_eq!(Ipv4Mask::from_str("255.255.0.0").unwrap(), expected);

        let expected = Ipv4Mask(0xffff_f800);
        assert_eq!(Ipv4Mask::from_str("255.255.248.0").unwrap(), expected);

        let expected = Ipv4Mask(0);
        assert_eq!(Ipv4Mask::from_str("0.0.0.0").unwrap(), expected);
    }
}
