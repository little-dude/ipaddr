use {Ipv4Address, Ipv6Address, ParsingFailed};

use std::str::FromStr;

impl FromStr for Ipv6Address {
    type Err = ParsingFailed;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // We'll manipulate bytes instead of UTF-8 characters, because the characters that
        // represent an IPv6 address are supposed to be ASCII characters.
        let bytes = s.as_bytes();
        Ipv6Address::parse(bytes)
            .map_err(|()| ParsingFailed(s.into()))
            .map(Ipv6Address)
    }
}

impl Ipv6Address {
    fn parse(bytes: &[u8]) -> Result<u128, ()> {
        // The maximimum length of a string representing an IPv6 is the length of:
        //
        //      1111:2222:3333:4444:5555:6666:123.123.123.123
        //
        // The minimum length of a string representing an IPv6 is the length of:
        //
        //      ::
        //
        if bytes.len() > 46 || bytes.len() < 2 {
            return Err(());
        }

        let mut offset = 0;
        let mut ellipsis: Option<usize> = None;

        // Handle the special case where the IP start with "::"
        if bytes[0] == b':' {
            if bytes[1] == b':' {
                if bytes.len() == 2 {
                    return Ok(0);
                }
                ellipsis = Some(0);
                offset += 2;
            } else {
                // An IPv6 cannot start with a single column. It must be a double column.
                // So this is an invalid address
                return Err(());
            }
        }

        // When dealing with IPv6, it's easier to reason in terms of "hextets" instead of octets.
        // An IPv6 is 8 hextets. At the end, we'll convert that array into an u128.
        let mut address: [u16; 8] = [0; 8];

        // Keep track of the number of hextets we process
        let mut hextet_index = 0;

        loop {
            if offset == bytes.len() {
                break;
            }

            // Try to read an hextet
            let (bytes_read, hextet) = read_hextet(&bytes[offset..]);

            // Handle the case where we could not read an hextet
            if bytes_read == 0 {
                match bytes[offset] {
                    // We could not read an hextet because the first character in the slace was ":"
                    // This may be because we have two consecutive columns.
                    b':' => {
                        // Check if already saw an ellipsis. If so, fail parsing, because an IPv6
                        // can only have one ellipsis.
                        if ellipsis.is_some() {
                            return Err(());
                        }
                        // Otherwise, remember the position of the ellipsis. We'll need that later
                        // to count the number of zeros the ellipsis represents.
                        ellipsis = Some(hextet_index);
                        offset += 1;
                        // Continue and try to read the next hextet
                        continue;
                    }
                    // We now the first character does not represent an hexadecimal digit
                    // (otherwise read_hextet() would have read at least one character), and that
                    // it's not ":", so the string does not represent an IPv6 address
                    _ => return Err(()),
                }
            }

            // At this point, we know we read an hextet.

            address[hextet_index] = hextet;
            offset += bytes_read;
            hextet_index += 1;

            // If this was the last hextet of if we reached the end of the buffer, we should be
            // done
            if hextet_index == 8 || offset == bytes.len() {
                break;
            }

            // Read the next charachter. After a hextet, we usually expect a column, but there's a special
            // case for IPv6 that ends with an IPv4.
            match bytes[offset] {
                // We saw the column, we can continue
                b':' => {
                    offset += 1;
                    if offset == bytes.len() {
                        // We cannot terminate with a single column
                        return Err(());
                    }
                }
                // Handle the special IPv4 case, ie address like. Note that the hextet we just read
                // is part of that IPv4 address:
                //
                // aaaa:bbbb:cccc:dddd:eeee:ffff:a.b.c.d.
                //                               ^^
                //                               ||
                // hextet we just read, that  ---+|
                // is actually the first byte of  +--- dot we're handling
                // the ipv4.
                b'.' => {
                    // The hextet was actually part of the IPv4, so not that we start reading the
                    // IPv4 at `offset - bytes_read`.
                    let ipv4: u32 = Ipv4Address::parse(&bytes[offset - bytes_read..])?;
                    // Important: is Ipv4Address succeeds, we know we reached the end of the
                    // buffer! If there were trailing characters, the method would fail! We set
                    // the offset because we have a check later to make sure the offset is equal to
                    // the number of bytes in the buffer.
                    offset = bytes.len();
                    // Replace the hextet we just read by the 16 most significant bits of the
                    // IPv4 address (a.b in the comment above)
                    address[hextet_index - 1] = ((ipv4 & 0xffff_0000) >> 16) as u16;
                    // Set the last hextet to the 16 least significant bits of the IPv4 address
                    // (c.d in the comment above)
                    address[hextet_index] = (ipv4 & 0x0000_ffff) as u16;
                    hextet_index += 1;
                    // After successfully parsing an IPv4, we should be done.
                    // If there are bytes left in the buffer, or if we didn't read enough hextet,
                    // we'll fail later.
                    break;
                }
                _ => return Err(()),
            }
        } // end of loop

        // If we exited the loop, we should have reached the end of the buffer.
        // If there are trailing characters, parsing should fail.
        if offset < bytes.len() {
            return Err(());
        }

        if hextet_index == 8 && ellipsis.is_some() {
            // We parsed an address that looks like 1111:2222::3333:4444:5555:6666:7777,
            // ie with an empty ellipsis.
            return Err(());
        }

        // We didn't parse enough hextets, but this may be due to an ellipsis
        if hextet_index < 8 {
            if let Some(ellipsis_index) = ellipsis {
                // Count how many zeros the ellipsis accounts for
                let nb_zeros = 8 - hextet_index;
                // Shift the hextet that we read after the ellipsis by the number of zeros
                for index in (ellipsis_index..hextet_index).rev() {
                    address[index + nb_zeros] = address[index];
                    address[index] = 0;
                }
            } else {
                return Err(());
            }
        }

        // Build the IPv6 address from the array of hextets
        Ok((u128::from(address[0]) << 112)
            + (u128::from(address[1]) << 96)
            + (u128::from(address[2]) << 80)
            + (u128::from(address[3]) << 64)
            + (u128::from(address[4]) << 48)
            + (u128::from(address[5]) << 32)
            + (u128::from(address[6]) << 16)
            + u128::from(address[7]))
    }
}

/// Check whether an ASCII character represents an hexadecimal digit
fn is_hex_digit(byte: u8) -> bool {
    match byte {
        b'0'...b'9' | b'a'...b'f' | b'A'...b'F' => true,
        _ => false,
    }
}

/// Convert an ASCII character that represents an hexadecimal digit into this digit
fn hex_to_digit(byte: u8) -> u8 {
    match byte {
        b'0'...b'9' => byte - b'0',
        b'a'...b'f' => byte - b'a' + 10,
        b'A'...b'F' => byte - b'A' + 10,
        _ => unreachable!(),
    }
}

/// Read up to four ASCII characters that represent hexadecimal digits, and return their value, as
/// well as the number of characters that were read. If not character is read, `(0, 0)` is
/// returned.
fn read_hextet(bytes: &[u8]) -> (usize, u16) {
    let mut count = 0;
    let mut digits: [u8; 4] = [0; 4];

    for b in bytes {
        if is_hex_digit(*b) {
            digits[count] = hex_to_digit(*b);
            count += 1;
            if count == 4 {
                break;
            }
        } else {
            break;
        }
    }

    if count == 0 {
        return (0, 0);
    }

    let mut shift = (count - 1) * 4;
    let mut res = 0;
    for digit in &digits[0..count] {
        res += u16::from(*digit) << shift;
        if shift >= 4 {
            shift -= 4;
        } else {
            break;
        }
    }

    (count, res)
}

#[cfg(test)]
mod tests_ipv6 {
    use super::*;

    #[test]
    fn test_parse_zero() {
        assert_eq!(Ipv6Address::from_str("::").unwrap(), Ipv6Address(0));
    }

    #[test]
    fn test_parse_one() {
        assert_eq!(Ipv6Address::from_str("::1").unwrap(), Ipv6Address(1));
    }

    #[test]
    fn test_parse_link_local() {
        assert_eq!(
            Ipv6Address::from_str("fe80:0000:0000:0000:8657:e6fe:8d5::").unwrap(),
            Ipv6Address(0xfe800000000000008657e6fe08d50000)
        );
        assert_eq!(
            Ipv6Address::from_str("fe80:0000:0000:0000:8657:e6fe:8d5:5325").unwrap(),
            Ipv6Address(0xfe800000000000008657e6fe08d55325)
        );
        assert_eq!(
            Ipv6Address::from_str("fe80:0:0:0:8657:e6fe:8d5:5325").unwrap(),
            Ipv6Address(0xfe800000000000008657e6fe08d55325)
        );
        assert_eq!(
            Ipv6Address::from_str("fe80::8657:e6fe:8d5:5325").unwrap(),
            Ipv6Address(0xfe800000000000008657e6fe08d55325)
        );
        assert_eq!(
            Ipv6Address::from_str("ffff:ffff:ffff:ffff::").unwrap(),
            Ipv6Address(0xffff_ffff_ffff_ffff_0000_0000_0000_0000)
        );
    }

    #[test]
    fn test_parse_trailing_zeros() {
        assert_eq!(
            Ipv6Address::from_str("ffff::").unwrap(),
            Ipv6Address(0xffff0000000000000000000000000000)
        );
    }

    #[test]
    fn test_parse_with_ipv4() {
        assert_eq!(
            Ipv6Address::from_str("ffff::1.2.3.4").unwrap(),
            Ipv6Address(0xffff_0000_0000_0000_0000_0000_01020304)
        );
        assert_eq!(
            Ipv6Address::from_str("::1.2.3.4").unwrap(),
            Ipv6Address(0x0000_0000_0000_0000_0000_0000_01020304)
        );
        assert_eq!(
            Ipv6Address::from_str("fe80::8657:e6fe:128.128.128.128").unwrap(),
            Ipv6Address(0xfe800000000000008657e6fe_80808080)
        );
        assert_eq!(
            Ipv6Address::from_str("fe80:0:0:0:8657:e6fe:0.0.0.0").unwrap(),
            Ipv6Address(0xfe800000000000008657e6fe_00000000)
        );
        assert_eq!(
            Ipv6Address::from_str("fe80:0000:0000:0000:8657:e6fe:255.255.255.255").unwrap(),
            Ipv6Address(0xfe800000000000008657e6fe_ffffffff)
        );
    }

    #[test]
    fn test_errors() {
        assert!(Ipv6Address::from_str("").is_err());
        assert!(Ipv6Address::from_str("  ").is_err());
        assert!(Ipv6Address::from_str(":").is_err());
        assert!(Ipv6Address::from_str(":: ").is_err());
        assert!(Ipv6Address::from_str("::::").is_err());
        assert!(Ipv6Address::from_str("::1::").is_err());
        assert!(Ipv6Address::from_str("ffff::1::").is_err());
        assert!(Ipv6Address::from_str("ffff::1:").is_err());
        assert!(Ipv6Address::from_str(":ffff::").is_err());
        assert!(Ipv6Address::from_str("::ffff::").is_err());
        assert!(Ipv6Address::from_str("::ffff:").is_err());
        assert!(Ipv6Address::from_str("::ffff ").is_err());
        assert!(Ipv6Address::from_str(" ::ffff").is_err());
        assert!(Ipv6Address::from_str("1fe80:0000:0000:0000:8657:e6fe:255.255.255.255").is_err());
        assert!(Ipv6Address::from_str("fe80:0000:0000:0000:8657:e6fe:1234").is_err());
    }
}
