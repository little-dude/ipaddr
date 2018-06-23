#![cfg_attr(nightly, allow(unused_attributes))]
#![cfg_attr(nightly, rustfmt::skip)]

use Ipv4Address;


/// IPv4 unspecified address: `0.0.0.0`
pub const IPV4_UNSPECIFIED: Ipv4Address = Ipv4Address(0);

/// IPv4 loopback address: `127.0.0.1`
pub const IPV4_LOOPBACK: Ipv4Address = Ipv4Address(0x7f00_0001);
pub(crate) const IPV4_MAX_PREFIXLEN: u8 = 32;
