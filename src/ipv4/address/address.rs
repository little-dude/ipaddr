use std::convert::TryFrom;
use std::fmt;

use {IPV4_LOOPBACK, IPV4_UNSPECIFIED, MalformedAddress};

/// An Ipv4 address
#[derive(Copy, Eq, PartialEq, Hash, Clone)]
pub struct Ipv4Address(pub(crate) u32);

impl Ipv4Address {
    /// Return `true` if this address is `0.0.0.0`
    ///
    /// ```rust
    /// # use ipaddr::Ipv4Address;
    /// # fn main() {
    /// assert!(Ipv4Address::from(0).is_unspecified())
    /// # }
    /// ```
    pub fn is_unspecified(self) -> bool {
        self == IPV4_UNSPECIFIED
    }

    /// Return `true` if this address is `127.0.0.1`
    ///
    /// ```rust
    /// # use ipaddr::Ipv4Address;
    /// # use std::str::FromStr;
    /// # fn main() {
    /// assert!(Ipv4Address::from_str("127.0.0.1").unwrap().is_loopback())
    /// # }
    /// ```
    pub fn is_loopback(self) -> bool {
        self == IPV4_LOOPBACK
    }

    /// Return `true` if this address is a multicast address
    ///
    /// ```rust
    /// # use ipaddr::Ipv4Address;
    /// # use std::str::FromStr;
    /// # fn main() {
    /// let ip = Ipv4Address::from_str("224.0.0.0").unwrap();
    /// // 224.0.0.0 is multicast
    /// assert!(ip.is_multicast());
    /// // 239.255.255.255 is multicast
    /// assert!(! (ip - 1).is_multicast());
    ///
    /// let ip = Ipv4Address::from_str("239.255.255.255").unwrap();
    /// // 239.255.255.255 is multicast
    /// assert!(ip.is_multicast());
    /// // 240.0.0.0 is not multicast
    /// assert!(! (ip + 1).is_multicast());
    /// # }
    /// ```
    pub fn is_multicast(self) -> bool {
        (self.0 & 0xf000_0000) == 0xe000_0000
    }

    /// Return the address as an `u32`
    ///
    ///
    /// ```rust
    /// # use ipaddr::Ipv4Address;
    /// # use std::str::FromStr;
    /// # fn main() {
    /// let ip = Ipv4Address::from_str("255.255.0.0").unwrap();
    /// assert_eq!(ip.value(), 0xffff_0000);
    /// assert_eq!((ip + 1).value(), 0xffff_0001);
    /// # }
    /// ```
    pub fn value(self) -> u32 {
        self.0
    }

    /// Return the address as an array of bytes
    ///
    /// ```rust
    /// # use ipaddr::Ipv4Address;
    /// # use std::str::FromStr;
    /// # fn main() {
    /// let ip = Ipv4Address::from_str("1.2.3.4").unwrap();
    /// assert_eq!(ip.octets(), [1, 2, 3, 4]);
    /// # }
    /// ```
    pub fn octets(self) -> [u8; 4] {
        [
            (self.0 >> 24) as u8,
            (self.0 >> 16) as u8,
            (self.0 >> 8) as u8,
            self.0 as u8,
        ]
    }

    /// Create an `Ipv4Address` from the four first bytes of a slice. This method panics if the
    /// slice is not long enough. If you don't want to validate the slice, use
    /// [`from_slice`](#method.from_slice) instead.
    ///
    /// ```rust
    /// # use ipaddr::Ipv4Address;
    /// # fn main() {
    /// let ip = Ipv4Address::from_slice_unchecked(&[1, 2, 3, 4][..]);
    /// assert_eq!(ip, Ipv4Address::from(0x01020304));
    /// # }
    /// ```
    ///
    /// # Panics
    ///
    /// This method panics if the slice contains more or less than 4 bytes.
    pub fn from_slice_unchecked(bytes: &[u8]) -> Ipv4Address {
        Ipv4Address(
            (u32::from(bytes[0]) << 24)
                + (u32::from(bytes[1]) << 16)
                + (u32::from(bytes[2]) << 8)
                + (u32::from(bytes[3])),
        )
    }

    /// Create an `Ipv4Address` from the four first bytes of a slice. If the slice is not long
    /// enough, this method returns an error.
    ///
    /// ```rust
    /// # use ipaddr::{MalformedAddress, Ipv4Address};
    /// # fn main() {
    /// let ip = Ipv4Address::from_slice(&[1, 2, 3, 4][..]);
    /// assert!(ip.is_ok());
    /// assert_eq!(ip.unwrap(), Ipv4Address::from(0x01020304));
    ///
    /// let ip = Ipv4Address::from_slice(&[1, 2, 3][..]);
    /// assert!(ip.is_err());
    /// # }
    /// ```
    pub fn from_slice(bytes: &[u8]) -> Result<Ipv4Address, MalformedAddress> {
        if bytes.len() != 4 {
            return Err(MalformedAddress);
        }
        Ok(Self::from_slice_unchecked(bytes))
    }

    /// Return a human readable representation of the IPv4 address.
    ///
    /// ```rust
    /// # use ipaddr::{Ipv4Address};
    /// # use std::string::String;
    /// # fn main() {
    /// let ip = Ipv4Address::from(0x01020304);
    /// assert_eq!(ip.to_string(), "1.2.3.4".to_string());
    /// # }
    /// ```
    pub fn to_string(self) -> String {
        let octets = self.octets();
        format!("{}.{}.{}.{}", octets[0], octets[1], octets[2], octets[3])
    }
}

impl fmt::Display for Ipv4Address {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&self.to_string())
    }
}

impl fmt::Debug for Ipv4Address {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Ipv4Address(")?;
        f.write_str(&self.to_string())?;
        f.write_str(")")
    }
}

impl<'a> TryFrom<&'a [u8]> for Ipv4Address {
    type Error = MalformedAddress;
    fn try_from(bytes: &'a [u8]) -> Result<Self, Self::Error> {
        Ipv4Address::from_slice(bytes)
    }
}

impl From<[u8; 16]> for Ipv4Address {
    fn from(bytes: [u8; 16]) -> Self {
        Ipv4Address::from_slice_unchecked(&bytes[..])
    }
}
