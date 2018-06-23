use core::fmt::{self, Binary, Formatter, LowerHex, UpperHex};
use std::convert::TryFrom;
use std::ops::{BitAnd, Not};
use std::str::FromStr;

use {IPV6_MAX_PREFIXLEN, InvalidMask, Ipv6Address, Ipv6Formatter, ParsingFailed};

/// Check whether the given integer represents a valid IPv6 mask.
// see https://codereview.stackexchange.com/a/197138/118470
fn is_valid_mask(value: u128) -> bool {
    value.count_zeros() == value.trailing_zeros()
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct Ipv6Mask(u128);

impl Ipv6Mask {
    /// Return the prefix length that correspond to this mask.
    ///
    /// ```rust
    /// # use ipaddr::{Ipv6Mask};
    /// # use std::str::FromStr;
    /// # fn main() {
    /// let mask = Ipv6Mask::from_str("ffff:ffff:ffff:ffff::").unwrap();
    /// assert_eq!(mask.prefixlen(), 64);
    /// # }
    /// ```
    pub fn prefixlen(&self) -> u32 {
        self.0.count_ones()
    }

    /// Create an `Ipv6Mask` corresponding to the given prefix length
    ///
    /// ```rust
    /// # use ipaddr::{Ipv6Mask};
    /// # fn main() {
    /// let mask = Ipv6Mask::from_prefixlen(64).unwrap();
    /// assert_eq!(u128::from(mask), 0xffff_ffff_ffff_ffff_0000_0000_0000_0000);
    /// # }
    /// ```
    pub fn from_prefixlen(prefixlen: u8) -> Result<Self, InvalidMask> {
        match prefixlen {
            IPV6_MAX_PREFIXLEN => Ok(Ipv6Mask(u128::max_value())),
            0 => Ok(Ipv6Mask(0)),
            n if n < IPV6_MAX_PREFIXLEN => {
                let mask: u128 = (1 << n) - 1;
                let mask = mask << (IPV6_MAX_PREFIXLEN - n);
                Ok(Ipv6Mask(mask))
            }
            _ => Err(InvalidMask),
        }
    }

    /// Create a formatter to stringify this IPv6 mask.
    /// See [`Ipv6Formatter<'a, W>`](struct.Ipv6Formatter.html) for more details.
    pub fn formatter<'a, W: fmt::Write>(&self, writer: &'a mut W) -> Ipv6Formatter<'a, W> {
        Ipv6Address::from(self).formatter(writer)
    }

    /// Return a string representation of this IPv6 address. There are several ways to represent an
    /// IPv6 address. This method uses the convention documented in
    /// [RFC5952](https://tools.ietf.org/html/rfc5952):
    ///
    /// - lower case
    /// - the longest sequence of zeros is replaced by `::`
    /// - no leading zero (`fe80::123` instead of `fe80::0123`)
    ///
    /// For other formatting options, use [`formatter()`](#method.formatter`).
    ///
    /// ```rust
    /// # use ipaddr::{Ipv6Mask};
    /// # fn main() {
    /// let mask = Ipv6Mask::from_prefixlen(64).unwrap();
    /// assert_eq!(mask.to_string(), "ffff:ffff:ffff:ffff::");
    ///
    /// // the same representation is used in the Display implementation
    /// assert_eq!(format!("{}", mask), "ffff:ffff:ffff:ffff::");
    /// # }
    /// ```
    pub fn to_string(&self) -> String {
        let mut s = String::with_capacity(40);
        // https://doc.rust-lang.org/std/fmt/index.html#formatting-traits
        // Formatting strings is infaillible. The write! macro we use in our formatter can only
        // fail when writing in the underlying string, but here is a String so it should not fail.
        self.formatter(&mut s)
            .rfc_5952()
            .write()
            .expect("string formatting failed?!");
        s
    }
}

impl fmt::Display for Ipv6Mask {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.formatter(f).write()
    }
}

impl fmt::Debug for Ipv6Mask {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Ipv6Mask(")?;
        self.formatter(f).write()?;
        f.write_str(")")
    }
}

// XXX: do not impl AsMut<u128>, otherwise we might end up with an invalid mask
impl AsRef<u128> for Ipv6Mask {
    fn as_ref(&self) -> &u128 {
        &self.0
    }
}

// FIXME: impl AsRef<Ipv6Address> ?

impl From<Ipv6Mask> for u128 {
    fn from(value: Ipv6Mask) -> u128 {
        value.0
    }
}

impl<'a> From<&'a Ipv6Mask> for u128 {
    fn from(value: &'a Ipv6Mask) -> u128 {
        value.0
    }
}

impl TryFrom<u128> for Ipv6Mask {
    type Error = InvalidMask;

    fn try_from(value: u128) -> Result<Ipv6Mask, Self::Error> {
        if is_valid_mask(value) {
            Ok(Ipv6Mask(value))
        } else {
            Err(InvalidMask)
        }
    }
}

impl<'a> TryFrom<&'a u128> for Ipv6Mask {
    type Error = InvalidMask;
    fn try_from(value: &'a u128) -> Result<Ipv6Mask, Self::Error> {
        Ipv6Mask::try_from(*value)
    }
}

impl TryFrom<Ipv6Address> for Ipv6Mask {
    type Error = InvalidMask;
    fn try_from(value: Ipv6Address) -> Result<Ipv6Mask, Self::Error> {
        Ipv6Mask::try_from(u128::from(value))
    }
}

impl<'a> TryFrom<&'a Ipv6Address> for Ipv6Mask {
    type Error = InvalidMask;
    fn try_from(value: &'a Ipv6Address) -> Result<Ipv6Mask, Self::Error> {
        Ipv6Mask::try_from(u128::from(value))
    }
}

impl Not for Ipv6Mask {
    type Output = Ipv6Address;

    fn not(self) -> Self::Output {
        Ipv6Address(self.0.not())
    }
}

impl<'a> Not for &'a Ipv6Mask {
    type Output = Ipv6Address;

    fn not(self) -> Self::Output {
        Ipv6Address(self.0.not())
    }
}

impl LowerHex for Ipv6Mask {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        <u128 as LowerHex>::fmt(&self.0, f)
    }
}

impl UpperHex for Ipv6Mask {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        <u128 as UpperHex>::fmt(&self.0, f)
    }
}

impl Binary for Ipv6Mask {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        <u128 as Binary>::fmt(&self.0, f)
    }
}

impl BitAnd<Ipv6Mask> for Ipv6Address {
    type Output = Ipv6Address;
    fn bitand(self, rhs: Ipv6Mask) -> Self::Output {
        Ipv6Address(self.0.bitand(rhs.0))
    }
}

impl<'a> BitAnd<Ipv6Mask> for &'a Ipv6Address {
    type Output = Ipv6Address;
    fn bitand(self, rhs: Ipv6Mask) -> Self::Output {
        Ipv6Address(self.0.bitand(rhs.0))
    }
}

impl<'a> BitAnd<&'a Ipv6Mask> for Ipv6Address {
    type Output = Ipv6Address;
    fn bitand(self, rhs: &'a Ipv6Mask) -> Self::Output {
        Ipv6Address(self.0.bitand(rhs.0))
    }
}

impl<'a, 'b> BitAnd<&'a Ipv6Mask> for &'b Ipv6Address {
    type Output = Ipv6Address;
    fn bitand(self, rhs: &'a Ipv6Mask) -> Self::Output {
        Ipv6Address(self.0.bitand(rhs.0))
    }
}

impl FromStr for Ipv6Mask {
    type Err = ParsingFailed;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let ip = Ipv6Address::from_str(s)?;
        Ipv6Mask::try_from(ip).map_err(|_| ParsingFailed(s.into()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cidr_simple() {
        let mask = Ipv6Mask(0xff00_0000_0000_0000_0000_0000_0000_0000);
        assert_eq!(mask.prefixlen(), 8);
        let mask = Ipv6Mask(0xffff_ffff_0000_0000_0000_0000_0000_0000);
        assert_eq!(mask.prefixlen(), 32);
        let mask = Ipv6Mask(0xffff_ffff_ffff_ffff_ffff_ffff_ffff_ffff);
        assert_eq!(mask.prefixlen(), 128);
        let mask = Ipv6Mask(0x0000_0000_0000_0000_0000_0000_0000_0000);
        assert_eq!(mask.prefixlen(), 0);
    }

    #[test]
    fn test_from_int() {
        assert!(Ipv6Mask::try_from(0xff00_0000_0000_0000_0000_0000_0000_0000).is_ok());
        assert!(Ipv6Mask::try_from(0xffff_ffff_0000_0000_0000_0000_0000_0000).is_ok());
        assert!(Ipv6Mask::try_from(0xffff_ffff_ffff_ffff_ffff_ffff_ffff_ffff).is_ok());
        assert!(Ipv6Mask::try_from(0x0000_0000_0000_0000_0000_0000_0000_0000).is_ok());
        assert!(Ipv6Mask::try_from(0xfe00_0000_0000_0000_0000_0000_0000_0000).is_ok());
        assert!(Ipv6Mask::try_from(0xf000_0000_0000_0000_0000_0000_0000_0000).is_ok());
        assert!(Ipv6Mask::try_from(0xffff_fff8_0000_0000_0000_0000_0000_0000).is_ok());

        assert!(Ipv6Mask::try_from(0xff70_0000_0000_0000_0000_0000_0000_0000).is_err());
        assert!(Ipv6Mask::try_from(0xffff_ffff_0000_0000_0000_0000_0000_0001).is_err());
        assert!(Ipv6Mask::try_from(0xffff_ffff_ffff_ffff_ffff_ffff_fff0_ffff).is_err());
        assert!(Ipv6Mask::try_from(0x0000_0000_0000_0000_0010_0000_0000_0000).is_err());
        assert!(Ipv6Mask::try_from(0xfe10_0000_0000_0000_0000_0000_0000_0000).is_err());
    }

    #[test]
    fn test_from_prefixlen_simple() {
        let mask = Ipv6Mask(0xff00_0000_0000_0000_0000_0000_0000_0000);
        assert_eq!(Ipv6Mask::from_prefixlen(8).unwrap(), mask);
        let mask = Ipv6Mask(0xffff_ffff_0000_0000_0000_0000_0000_0000);
        assert_eq!(Ipv6Mask::from_prefixlen(32).unwrap(), mask);
        let mask = Ipv6Mask(0xffff_ffff_ffff_ffff_ffff_ffff_ffff_ffff);
        assert_eq!(Ipv6Mask::from_prefixlen(128).unwrap(), mask);
        let mask = Ipv6Mask(0x0000_0000_0000_0000_0000_0000_0000_0000);
        assert_eq!(Ipv6Mask::from_prefixlen(0).unwrap(), mask);
    }

    #[test]
    fn test_cidr_single_bits() {
        let mask = Ipv6Mask(0xfe00_0000_0000_0000_0000_0000_0000_0000);
        assert_eq!(mask.prefixlen(), 7);
        let mask = Ipv6Mask(0xf000_0000_0000_0000_0000_0000_0000_0000);
        assert_eq!(mask.prefixlen(), 4);
        let mask = Ipv6Mask(0xffff_fff8_0000_0000_0000_0000_0000_0000);
        assert_eq!(mask.prefixlen(), 29);
    }

    #[test]
    fn test_from_prefixlen_single_bits() {
        let mask = Ipv6Mask(0xfe00_0000_0000_0000_0000_0000_0000_0000);
        assert_eq!(Ipv6Mask::from_prefixlen(7).unwrap(), mask);
        let mask = Ipv6Mask(0xf000_0000_0000_0000_0000_0000_0000_0000);
        assert_eq!(Ipv6Mask::from_prefixlen(4).unwrap(), mask);
        let mask = Ipv6Mask(0xffff_fff8_0000_0000_0000_0000_0000_0000);
        assert_eq!(Ipv6Mask::from_prefixlen(29).unwrap(), mask);
    }

    #[test]
    fn from_str() {
        let expected = Ipv6Mask(0xffff_ffff_ffff_ffff_0000_0000_0000_0000);
        assert_eq!(
            Ipv6Mask::from_str("ffff:ffff:ffff:ffff::").unwrap(),
            expected
        );
        assert_eq!(
            Ipv6Mask::from_str("ffff:ffff:ffff:ffff:0:0:0:0").unwrap(),
            expected
        );
        assert_eq!(
            Ipv6Mask::from_str("ffff:ffff:ffff:ffff:0000:0000:0000:0000").unwrap(),
            expected
        );
    }
}
