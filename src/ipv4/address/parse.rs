use std::str::FromStr;

use {Ipv4Address, ParsingFailed};

impl FromStr for Ipv4Address {
    type Err = ParsingFailed;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::parse(s.as_bytes())
            .map_err(|()| ParsingFailed(s.into()))
            .map(Ipv4Address)
    }
}

impl Ipv4Address {
    pub(crate) fn parse(bytes: &[u8]) -> Result<u32, ()> {
        if bytes.len() > 15 || bytes.len() < 7 {
            return Err(());
        }
        let mut address: u32 = 0;
        let mut offset = 0;
        for i in 0..4 {
            if offset == bytes.len() {
                return Err(());
            }
            let (bytes_read, octet) = read_octet(&bytes[offset..]);
            if bytes_read == 0 {
                // We should have been able to read an octet
                return Err(());
            }
            offset += bytes_read;
            address += u32::from(octet) << ((3 - i) * 8);

            if i == 3 {
                break;
            }

            // read the dot
            if offset >= bytes.len() || bytes[offset] != b'.' {
                return Err(());
            }
            offset += 1;
        }
        if offset < bytes.len() {
            // We finished reading the IP but there are still bytes to read
            return Err(());
        }
        Ok(address)
    }
}

fn read_octet(bytes: &[u8]) -> (usize, u8) {
    let mut count = 0;
    let mut digits: [u8; 3] = [0; 3];

    for b in bytes {
        if is_decimal_digit(*b) {
            digits[count] = decimal_to_digit(*b);
            count += 1;
            if count == 3 {
                break;
            }
        } else {
            break;
        }
    }

    if count == 0 {
        return (0, 0);
    }

    let mut res: u16 = 0;
    for digit in &digits[0..count] {
        res = 10 * res + u16::from(*digit);
    }

    if res > 0xff {
        return (0, 0);
    }

    (count, res as u8)
}

/// Check whether an ASCII character represents a decimal digit
fn is_decimal_digit(byte: u8) -> bool {
    match byte {
        b'0'...b'9' => true,
        _ => false,
    }
}

/// Convert an ASCII character that represents a decimal into this digit
fn decimal_to_digit(byte: u8) -> u8 {
    match byte {
        b'0'...b'9' => byte - b'0',
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_zero() {
        assert_eq!(Ipv4Address::from_str("0.0.0.0").unwrap(), Ipv4Address(0));
    }

    #[test]
    fn test_parse_1234() {
        assert_eq!(
            Ipv4Address::from_str("1.2.3.4").unwrap(),
            Ipv4Address(0x01020304)
        );
    }

    #[test]
    fn test_parse_max() {
        assert_eq!(
            Ipv4Address::from_str("255.255.255.255").unwrap(),
            Ipv4Address(0xffffffff)
        );
    }

    #[test]
    fn test_error() {
        assert!(Ipv4Address::from_str("1.2.3.").is_err());
        assert!(Ipv4Address::from_str("1.2.3.4.").is_err());
        assert!(Ipv4Address::from_str("1.2.3").is_err());
        assert!(Ipv4Address::from_str(".1.2.3.4").is_err());
        assert!(Ipv4Address::from_str("256.0.0.1").is_err());
        assert!(Ipv4Address::from_str("25.0.0.256").is_err());
    }
}
