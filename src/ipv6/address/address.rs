use std::convert::TryFrom;
use std::fmt;

use {IPV6_LOOPBACK, IPV6_UNSPECIFIED, Ipv6Formatter, MalformedAddress};

#[derive(Copy, Eq, PartialEq, Hash, Clone)]
pub enum Ipv6AddressScope {
    InterfaceLocal,
    LinkLocal,
    AdminLocal,
    SiteLocal,
    OrganizationLocal,
    Global,
    Reserved(u8),
    Unassigned(u8),
}

impl From<Ipv6AddressScope> for u8 {
    fn from(scope: Ipv6AddressScope) -> u8 {
        use self::Ipv6AddressScope::*;
        match scope {
            Reserved(val) => val,
            InterfaceLocal => 0x01,
            LinkLocal => 0x02,
            AdminLocal => 0x04,
            SiteLocal => 0x05,
            Unassigned(val) => val,
            OrganizationLocal => 0x08,
            Global => 0x0e,
        }
    }
}

// An Ipv6 address
#[derive(Copy, Eq, PartialEq, Hash, Clone)]
pub struct Ipv6Address(pub(crate) u128);

impl Ipv6Address {
    /// Return `true` if this address is `::`
    pub fn is_unspecified(&self) -> bool {
        *self == IPV6_UNSPECIFIED
    }

    /// Return `true` if this address is `::1`
    pub fn is_loopback(&self) -> bool {
        *self == IPV6_LOOPBACK
    }

    /// Return `true` if this address is a multicast address
    pub fn is_multicast(&self) -> bool {
        self.0 & 0xff00_0000_0000_0000_0000_0000_0000_0000
            == 0xff00_0000_0000_0000_0000_0000_0000_0000
    }

    pub fn scope(&self) -> Ipv6AddressScope {
        use self::Ipv6AddressScope::*;
        match (self.0 & 0x000f_0000_0000_0000_0000_0000_0000_0000) >> 120 {
            i if i == 0x00 || i == 0x03 || i == 0x0f => Reserved(i as u8),
            0x01 => InterfaceLocal,
            0x02 => LinkLocal,
            0x04 => AdminLocal,
            0x05 => SiteLocal,
            i if i == 0x06 || i == 0x07 || (0x09..=0x0d).contains(&i) => Unassigned(i as u8),
            0x08 => OrganizationLocal,
            0x0e => Global,
            _ => unreachable!(),
        }
    }

    pub fn set_scope(&mut self, scope: Ipv6AddressScope) {
        self.0 = (self.0 & 0xfff0_ffff_ffff_ffff_ffff_ffff_ffff_ffff)
            | (u128::from(u8::from(scope)) << 120)
    }

    /// Return `true` if this address is a link-local unicast address. See
    /// [RFC4991](https://tools.ietf.org/html/rfc4291#section-2.5.6).
    ///
    /// Link-Local addresses are for use on a single link. Link-Local addresses have the following
    /// format:
    ///
    /// ```no_rust
    /// |   10     |
    /// |  bits    |         54 bits         |          64 bits           |
    /// +----------+-------------------------+----------------------------+
    /// |1111111010|           0             |       interface ID         |
    /// +----------+-------------------------+----------------------------+
    /// ```
    ///
    /// Link-Local addresses are designed to be used for addressing on a single link for purposes
    /// such as automatic address configuration, neighbor discovery, or when no routers are
    /// present.
    ///
    /// Routers must not forward any packets with Link-Local source or destination addresses to
    /// other links.
    ///
    /// ```rust
    /// # use ipaddr::Ipv6Address;
    /// # use std::str::FromStr;
    /// # fn main() {
    /// assert!(Ipv6Address::from_str("fe80::").unwrap().is_link_local_unicast());
    /// assert!(Ipv6Address::from_str("fe80::ffff:ffff:ffff:ffff").unwrap().is_link_local_unicast());
    ///
    /// // Some implementations are incorrect and just check that the 10 leftmost bits are
    /// // 1111110101. This implementation implements RFC4291, and so does not recognize the
    /// // following addresses as link-local addresses:
    /// assert!(!Ipv6Address::from_str("febf:ffff:ffff:ffff:ffff:ffff:ffff:ffff").unwrap().is_link_local_unicast());
    /// assert!(!Ipv6Address::from_str("fe81::").unwrap().is_link_local_unicast());
    /// assert!(!Ipv6Address::from_str("fe80::1:ffff:ffff:ffff:ffff").unwrap().is_link_local_unicast());
    /// # }
    /// ```
    pub fn is_link_local_unicast(&self) -> bool {
        self.0 & 0xffff_ffff_ffff_ffff_0000_0000_0000_0000
            == 0xfe80_0000_0000_0000_0000_0000_0000_0000
    }

    /// Return `true` if this address is a global unicast address. A global unicast address is an
    /// address that is:
    ///
    /// - not link local, i.e. not in the `fe80::/10` range
    /// - not multicast i.e. not in the `ff00::/8` range
    /// - not loopback i.e. not `1`
    /// - not unspecified i.e. is not `0`
    pub fn is_global_unicast(&self) -> bool {
        !self.is_link_local_unicast()
            && !self.is_unspecified()
            && !self.is_loopback()
            && !self.is_multicast()
    }

    /// Return the address as an `u128`
    pub fn value(&self) -> u128 {
        self.0
    }

    /// Return `true` if the address is an IPv4-compatible address. See
    /// [RFC4991](https://tools.ietf.org/html/rfc4291#section-2.5.5.1).
    ///
    /// The "IPv4-Compatible IPv6 address" was defined to assist in the IPv6 transition.  The
    /// format of the "IPv4-Compatible IPv6 address" is as follows:
    ///
    /// ```no_rust
    /// |                80 bits               | 16 |      32 bits        |
    /// +--------------------------------------+--------------------------+
    /// |0000..............................0000|0000|    IPv4 address     |
    /// +--------------------------------------+----+---------------------+
    /// ```
    ///
    ///
    /// Note: The IPv4 address used in the "IPv4-Compatible IPv6 address" must be a globally-unique
    /// IPv4 unicast address.
    ///
    /// The "IPv4-Compatible IPv6 address" is now deprecated because the current IPv6 transition
    /// mechanisms no longer use these addresses.  New or updated implementations are not required
    /// to support this address type.
    ///
    /// ```rust
    /// # use ipaddr::Ipv6Address;
    /// # use std::str::FromStr;
    /// # fn main() {
    /// assert!(Ipv6Address::from_str("::1.2.3.4").unwrap().is_ipv4_compatible());
    /// # }
    /// ```
    pub fn is_ipv4_compatible(&self) -> bool {
        self.0 & 0xffff_ffff_ffff_ffff_ffff_ffff_0000_0000 == 0
    }

    /// Return `true` if the address is an IPv4-mapped address. The format of IPv4-mapped IPv6
    /// addresses is as follows:
    ///
    /// ```no_rust
    /// |                80 bits               | 16 |      32 bits        |
    /// +--------------------------------------+--------------------------+
    /// |0000..............................0000|FFFF|    IPv4 address     |
    /// +--------------------------------------+----+---------------------+
    /// ```
    ///
    /// ```rust
    /// # use ipaddr::Ipv6Address;
    /// # use std::str::FromStr;
    /// # fn main() {
    /// assert!(Ipv6Address::from_str("::ffff:1.2.3.4").unwrap().is_ipv4_mapped());
    /// # }
    /// ```
    pub fn is_ipv4_mapped(&self) -> bool {
        *self & 0xffff_ffff_ffff_ffff_ffff_ffff_0000_0000 == 0xffff_0000_0000.into()
    }

    /// Return the address as an array of bytes
    pub fn octets(&self) -> [u8; 16] {
        let mut bytes: [u8; 16] = [0; 16];
        for (i, b) in bytes.iter_mut().enumerate() {
            *b = self.octet(i);
        }
        bytes
    }

    /// Return the address as an array of bytes
    pub fn hextets(&self) -> [u16; 8] {
        let mut hextets: [u16; 8] = [0; 8];
        for (i, h) in hextets.iter_mut().enumerate() {
            *h = self.hextet(i);
        }
        hextets
    }

    fn octet(&self, i: usize) -> u8 {
        ((self.0 >> ((15 - i) * 8)) & 0xff) as u8
    }

    fn hextet(&self, i: usize) -> u16 {
        ((self.0 >> ((7 - i) * 16)) & 0xffff) as u16
    }

    pub fn from_slice_unchecked(bytes: &[u8]) -> Ipv6Address {
        let mut shift = 120;
        let mut address = Ipv6Address(0);
        for b in bytes {
            address += u128::from(*b) << shift;
            shift -= 8;
        }
        address
    }
    pub fn from_slice(bytes: &[u8]) -> Result<Ipv6Address, MalformedAddress> {
        if bytes.len() != 16 {
            return Err(MalformedAddress);
        }
        Ok(Self::from_slice_unchecked(bytes))
    }

    /// Create a formatter to stringify this IPv6 address.
    pub fn formatter<'a, W: fmt::Write>(&self, writer: &'a mut W) -> Ipv6Formatter<'a, W> {
        Ipv6Formatter::new(writer, self.hextets())
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
    /// # use ipaddr::{Ipv6Address};
    /// # fn main() {
    /// let ip = Ipv6Address::from(0xfe80_0000_0000_0000_8657_e6fe_08d5_5325);
    /// assert_eq!(ip.to_string(), "fe80::8657:e6fe:8d5:5325");
    ///
    /// // the same representation is used in the Display implementation
    /// assert_eq!(format!("{}", ip), "fe80::8657:e6fe:8d5:5325");
    /// # }
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

impl fmt::Display for Ipv6Address {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.formatter(f).write()
    }
}

impl fmt::Debug for Ipv6Address {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Ipv6Address(")?;
        self.formatter(f).write()?;
        f.write_str(")")
    }
}

impl<'a> TryFrom<&'a [u8]> for Ipv6Address {
    type Error = MalformedAddress;
    fn try_from(bytes: &'a [u8]) -> Result<Self, Self::Error> {
        Ipv6Address::from_slice(bytes)
    }
}

impl From<[u8; 16]> for Ipv6Address {
    fn from(bytes: [u8; 16]) -> Self {
        Ipv6Address::from_slice_unchecked(&bytes[..])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_packed() {
        let expected: [u8; 16] = [
            0xfe, 0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x86, 0x57, 0xe6, 0xfe, 0x08, 0xd5,
            0x53, 0x25,
        ];

        let ip = Ipv6Address(0xfe80_0000_0000_0000_8657_e6fe_08d5_5325);
        assert_eq!(ip.octets(), expected);
    }
    #[test]
    fn test_from_bytes() {
        let bytes: [u8; 16] = [
            0xfe, 0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x86, 0x57, 0xe6, 0xfe, 0x08, 0xd5,
            0x53, 0x25,
        ];

        let expected = Ipv6Address(0xfe800000000000008657e6fe08d55325);
        assert_eq!(Ipv6Address::from(bytes), expected);
    }
    #[test]
    fn test_link_local() {
        use std::str::FromStr;
        assert!(
            Ipv6Address::from_str("fe80::")
                .unwrap()
                .is_link_local_unicast()
        );
        assert!(
            Ipv6Address::from_str("fe80::ffff:ffff:ffff:ffff")
                .unwrap()
                .is_link_local_unicast()
        );

        assert!(
            !Ipv6Address::from_str("febf:ffff:ffff:ffff:ffff:ffff:ffff:ffff")
                .unwrap()
                .is_link_local_unicast()
        );
        assert!(
            !Ipv6Address::from_str("fe81::")
                .unwrap()
                .is_link_local_unicast()
        );
        assert!(
            !Ipv6Address::from_str("fe80::1:ffff:ffff:ffff:ffff")
                .unwrap()
                .is_link_local_unicast()
        );
    }
}
