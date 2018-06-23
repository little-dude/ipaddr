#![cfg_attr(nightly, allow(unused_attributes))]
#![cfg_attr(nightly, rustfmt::skip)]

use Ipv6Address;

/// IPv6 unspecified address: `::`
pub const IPV6_UNSPECIFIED: Ipv6Address = Ipv6Address(0);

/// IPv6 loopback address: `::1`
pub const IPV6_LOOPBACK: Ipv6Address = Ipv6Address(1);

/// Multicast address for all the address on the interface: `ff01::1`
pub const IPV6_INTERFACE_LOCAL_ALL_NODES: Ipv6Address = Ipv6Address(0xff01_0000_0000_0000_0000_0000_0000_0001);

/// Multicast address for all the nodes on the network segment: `ff02::1`
pub const IPV6_LINK_LOCAL_ALL_NODES: Ipv6Address = Ipv6Address(0xff02_0000_0000_0000_0000_0000_0000_0001);

/// Multicast address for all the routers on the network segment: `ff02::2`
pub const IPV6_LINK_LOCAL_ALL_ROUTERS: Ipv6Address = Ipv6Address(0xff02_0000_0000_0000_0000_0000_0000_0002);

pub(crate) const IPV6_MAX_PREFIXLEN: u8 = 128;
