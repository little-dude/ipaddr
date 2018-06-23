use std::fmt;
use std::str::FromStr;

use {Ipv6Address, Ipv6Mask, ParsingFailed};

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct Ipv6Network(Ipv6Address, Ipv6Mask);

impl Ipv6Network {
    /// Create a new network address from the given address and mask.
    pub fn new(ip: Ipv6Address, mask: Ipv6Mask) -> Self {
        Ipv6Network(ip, mask)
    }

    pub fn host(&self) -> Ipv6Address {
        self.0 & self.host_mask()
    }

    pub fn network(&self) -> Ipv6Address {
        self.0 & self.1
    }

    pub fn host_mask(&self) -> Ipv6Address {
        !self.1
    }

    pub fn broadcast(&self) -> Ipv6Address {
        self.0 | self.host_mask()
    }

    pub fn len(&self) -> u128 {
        self.host_mask().into()
    }

    pub fn mask(&self) -> Ipv6Mask {
        self.1
    }

    pub fn ip(&self) -> &Ipv6Address {
        &self.0
    }

    pub fn ip_mut(&mut self) -> &mut Ipv6Address {
        &mut self.0
    }

    pub fn mask_mut(&mut self) -> &mut Ipv6Mask {
        &mut self.1
    }

    pub fn to_string(&self) -> String {
        unimplemented!()
    }
}

impl fmt::Display for Ipv6Network {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        unimplemented!()
    }
}

impl fmt::Debug for Ipv6Network {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        unimplemented!()
    }
}

impl FromStr for Ipv6Network {
    type Err = ParsingFailed;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split('/');
        let ip = Ipv6Address::from_str(split.next().ok_or_else(|| ParsingFailed(s.into()))?)?;

        let mask_str = split.next().ok_or_else(|| ParsingFailed(s.into()))?;

        // The most common is to represent the mask with as a prefix length, so we try to parse the
        // string as an integer first.
        if let Ok(prefix_len) = mask_str.parse::<u8>() {
            let mask = Ipv6Mask::from_prefixlen(prefix_len).map_err(|_| ParsingFailed(s.into()))?;
            return Ok(Ipv6Network(ip, mask));
        }

        // If that didn't work, may it has been specified as an IPv6
        let mask = Ipv6Mask::from_str(mask_str)?;

        Ok(Ipv6Network(ip, mask))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;

    const IP: u128 = 0xfe80_0000_0000_0000_aef4_1242_24e6_0c81;

    #[test]
    fn test_from_str() {
        let s = "fe80::aef4:1242:24e6:c81/64";
        let expected = Ipv6Network::new(IP.into(), Ipv6Mask::from_prefixlen(64).unwrap());
        assert_eq!(Ipv6Network::from_str(s).unwrap(), expected);

        let s = "fe80::aef4:1242:24e6:c81/ffff:ffff:ffff:ffff::";
        assert_eq!(Ipv6Network::from_str(s).unwrap(), expected);

        let s = "fe80::aef4:1242:24e6:c81/0";
        let expected = Ipv6Network::new(IP.into(), Ipv6Mask::try_from(0).unwrap());
        assert_eq!(Ipv6Network::from_str(s).unwrap(), expected);

        let s = "fe80::aef4:1242:24e6:c81/128";
        let expected = Ipv6Network::new(IP.into(), Ipv6Mask::try_from(u128::max_value()).unwrap());
        assert_eq!(Ipv6Network::from_str(s).unwrap(), expected);
    }

    fn get_net(prefix: u8) -> Ipv6Network {
        Ipv6Network::new(IP.into(), Ipv6Mask::from_prefixlen(prefix).unwrap())
    }

    #[test]
    fn test_network() {
        assert_eq!(
            get_net(16).network(),
            Ipv6Address::from_str("fe80::").unwrap()
        );
        assert_eq!(
            get_net(64).network(),
            Ipv6Address::from_str("fe80::").unwrap()
        );
        assert_eq!(
            get_net(72).network(),
            Ipv6Address::from_str("fe80:0:0:0:ae00::").unwrap()
        );
        assert_eq!(
            get_net(73).network(),
            Ipv6Address::from_str("fe80:0:0:0:ae80::").unwrap()
        );
        assert_eq!(get_net(128).network(), IP.into());
    }

    #[test]
    fn test_host() {
        assert_eq!(
            get_net(16).host(),
            Ipv6Address::from_str("::aef4:1242:24e6:c81").unwrap()
        );
        assert_eq!(
            get_net(64).host(),
            Ipv6Address::from_str("::aef4:1242:24e6:c81").unwrap()
        );
        assert_eq!(
            get_net(72).host(),
            Ipv6Address::from_str("::00f4:1242:24e6:c81").unwrap()
        );
        assert_eq!(
            get_net(73).host(),
            Ipv6Address::from_str("::0074:1242:24e6:c81").unwrap()
        );
        assert_eq!(get_net(128).host(), 0.into());
    }

    #[test]
    fn test_host_mask() {
        assert_eq!(
            get_net(16).host_mask(),
            Ipv6Address::from_str("::ffff:ffff:ffff:ffff:ffff:ffff:ffff").unwrap()
        );
        assert_eq!(
            get_net(64).host_mask(),
            Ipv6Address::from_str("::ffff:ffff:ffff:ffff").unwrap()
        );
        assert_eq!(
            get_net(72).host_mask(),
            Ipv6Address::from_str("::ff:ffff:ffff:ffff").unwrap()
        );
        assert_eq!(
            get_net(73).host_mask(),
            Ipv6Address::from_str("::7f:ffff:ffff:ffff").unwrap()
        );
        assert_eq!(
            get_net(128).host_mask(),
            Ipv6Address::from_str("::").unwrap()
        );
    }

    #[test]
    fn test_mask() {
        assert_eq!(get_net(16).mask(), Ipv6Mask::from_str("ffff::").unwrap());
        assert_eq!(
            get_net(64).mask(),
            Ipv6Mask::from_str("ffff:ffff:ffff:ffff::").unwrap()
        );
        assert_eq!(
            get_net(72).mask(),
            Ipv6Mask::from_str("ffff:ffff:ffff:ffff:ff00::").unwrap()
        );
        assert_eq!(
            get_net(73).mask(),
            Ipv6Mask::from_str("ffff:ffff:ffff:ffff:ff80::").unwrap()
        );
        assert_eq!(
            get_net(128).mask(),
            Ipv6Mask::try_from(u128::max_value()).unwrap()
        );
    }
}
