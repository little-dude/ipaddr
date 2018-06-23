use std::fmt::{Result, Write};

/// Configurable IPv6 formatter. By default, this formatter will format IPv6 address as described in [RFC5952](https://tools.ietf.org/html/rfc5952):
///
/// - lower case
/// - the longest sequence of zeros is replaced by `::`
/// - no leading zero (`fe80::123` instead of `fe80::0123`)
///
/// `Ipv6Formatter` can be obtained from [`Ipv6Address.formatter()`](struct.Ipv6Address.html#method.formatter).
///
/// Note that the [`write()`](#method.write) method returns a
/// [`fmt::Result`](https://doc.rust-lang.org/std/fmt/type.Result.html), but as explained in the
/// [`std::fmt` documentation](https://doc.rust-lang.org/std/fmt/index.html#formatting-traits),
/// string formatting is infaillible. What may fail is writing in the underlying buffer. That means
/// in your underlying storage is a `String` for example, it is fine to assume the method won't
/// fail.
///
/// # Examples
///
/// ```rust
/// # use ipaddr::Ipv6Address;
/// # fn main() {
/// let ip = Ipv6Address::from(0xfe80_0000_0000_0000_8657_0000_08d5_5325);
/// let mut s = String::with_capacity(40);
///
/// // Default formatting
/// ip.formatter(&mut s).write().unwrap();
/// assert_eq!(s, "fe80::8657:0:8d5:5325");
///
/// // The default formatting is the RFC5952 formatting
/// s.truncate(0);
/// ip.formatter(&mut s).rfc_5952().write().unwrap();
/// assert_eq!(s, "fe80::8657:0:8d5:5325");
///
/// // Use upper case letters
/// s.truncate(0);
/// ip.formatter(&mut s).upper_case(true).write().unwrap();
/// assert_eq!(s, "FE80::8657:0:8D5:5325");
///
/// // Do not reduce the longest 0 sequence to ::
/// s.truncate(0);
/// ip.formatter(&mut s).ellipsis(false).write().unwrap();
/// assert_eq!(s, "fe80:0:0:0:8657:0:8d5:5325");
///
/// // Like the previous one, but with upper case letters
/// s.truncate(0);
/// ip.formatter(&mut s).upper_case(true).ellipsis(false).write().unwrap();
/// assert_eq!(s, "FE80:0:0:0:8657:0:8D5:5325");
///
/// // Print leading zeros: 8d5 is now 08d5, and 0 is 0000
/// // Note that the longest 0 sequence is still shortened to ::
/// s.truncate(0);
/// ip.formatter(&mut s).leading_zeros(true).write().unwrap();
/// assert_eq!(s, "fe80::8657:0000:08d5:5325");
///
/// // Disable ellipsis, and print leading zeros. This is the most expanded form.
/// // With this settings, an IPv6 will always be 40 characters long.
/// s.truncate(0);
/// ip.formatter(&mut s).ellipsis(false).leading_zeros(true).write().unwrap();
/// assert_eq!(s, "fe80:0000:0000:0000:8657:0000:08d5:5325");
///
/// // A shortcut for the previous example:
/// s.truncate(0);
/// ip.formatter(&mut s).expanded().write().unwrap();
/// assert_eq!(s, "fe80:0000:0000:0000:8657:0000:08d5:5325");
///
/// // Expanded and upper case
/// s.truncate(0);
/// ip.formatter(&mut s)
///     .upper_case(true)
///     .ellipsis(false)
///     .leading_zeros(true)
///     .write()
///     .unwrap();
/// assert_eq!(s, "FE80:0000:0000:0000:8657:0000:08D5:5325");
/// # }
/// ```
pub struct Ipv6Formatter<'writer, W: 'writer> {
    ellipsis: bool,
    leading_zeros: bool,
    upper_case: bool,
    writer: &'writer mut W,
    hextets: [u16; 8],
}

impl<'writer, W> Ipv6Formatter<'writer, W>
where
    W: Write,
{
    pub(crate) fn new(writer: &'writer mut W, hextets: [u16; 8]) -> Self {
        Ipv6Formatter {
            ellipsis: true,
            leading_zeros: false,
            upper_case: false,
            writer,
            hextets,
        }
    }

    /// Use the formatting described in
    /// [RFC5952](https://tools.ietf.org/html/rfc5952) (this is the default)
    ///
    /// This is equivalent to:
    ///
    /// ```rust,ignore
    /// formatter.ellipsis(true).leading_zeros(false).upper_case(false)
    /// ```
    ///
    /// For example:
    ///
    /// ```rust
    /// # use ipaddr::{Ipv6Address, Ipv6Formatter};
    /// # fn main() {
    /// let ip = Ipv6Address::from(0xfe80_0000_0000_0000_8657_e6fe_08d5_5325);
    /// let mut s = String::with_capacity(40);
    /// ip.formatter(&mut s).write().unwrap();
    /// assert_eq!(s, "fe80::8657:e6fe:8d5:5325");
    /// # }
    /// ```
    pub fn rfc_5952(self) -> Self {
        self.ellipsis(true).leading_zeros(false).upper_case(false)
    }

    /// Fully expanded representation:
    ///
    /// - the longest sequence of zeros is not elided
    /// - leading zeros are printed, so hextets are always four characters long
    ///
    /// This is equivalent to:
    ///
    /// ```rust,ignore
    /// formatter(&mut writer).ellipsis(false).leading_zeros(true)
    /// ```
    ///
    /// For example:
    ///
    /// ```rust
    /// # use ipaddr::{Ipv6Address, Ipv6Formatter};
    /// # fn main() {
    /// let ip = Ipv6Address::from(0xfe80_0000_0000_0000_8657_e6fe_08d5_5325);
    /// let mut s = String::with_capacity(40);
    /// ip.formatter(&mut s).expanded().write().unwrap();
    /// assert_eq!(s, "fe80:0000:0000:0000:8657:e6fe:08d5:5325");
    /// # }
    /// ```
    pub fn expanded(self) -> Self {
        self.ellipsis(false).leading_zeros(true)
    }

    /// Set whether the longest sequence of zeros should be elided or not. By default, this is
    /// `true`.
    ///
    /// ```rust
    /// # use ipaddr::{Ipv6Address, Ipv6Formatter};
    /// # fn main() {
    /// let ip = Ipv6Address::from(0xfe80_0000_0000_0000_8657_e6fe_08d5_5325);
    /// let mut s = String::with_capacity(40);
    /// ip.formatter(&mut s).write().unwrap();
    /// assert_eq!(s, "fe80::8657:e6fe:8d5:5325");
    ///
    /// s.truncate(0);
    /// ip.formatter(&mut s).ellipsis(false).write().unwrap();
    /// assert_eq!(s, "fe80:0:0:0:8657:e6fe:8d5:5325");
    /// # }
    /// ```
    pub fn ellipsis(mut self, flag: bool) -> Self {
        self.ellipsis = flag;
        self
    }

    /// Set whether leading zeros for hextets smaller than `0x1000` should be printed. By default,
    /// this is `false`:
    ///
    /// ```rust
    /// # use ipaddr::{Ipv6Address, Ipv6Formatter};
    /// # fn main() {
    /// let ip = Ipv6Address::from(0xfe80_0000_0000_0000_8657_e6fe_08d5_5325);
    /// let mut s = String::with_capacity(40);
    /// ip.formatter(&mut s).write().unwrap();
    /// assert_eq!(s, "fe80::8657:e6fe:8d5:5325");
    ///
    /// s.truncate(0);
    /// ip.formatter(&mut s).leading_zeros(true).write().unwrap();
    /// assert_eq!(s, "fe80::8657:e6fe:08d5:5325");
    /// # }
    /// ```
    ///
    /// Note that zeros are also expanded. For instace, the same address with no ellipsis:
    /// ```rust
    /// # use ipaddr::{Ipv6Address, Ipv6Formatter};
    /// # fn main() {
    /// let ip = Ipv6Address::from(0xfe80_0000_0000_0000_8657_e6fe_08d5_5325);
    /// let mut s = String::with_capacity(40);
    /// ip.formatter(&mut s).ellipsis(false).leading_zeros(true).write().unwrap();
    /// assert_eq!(s, "fe80:0000:0000:0000:8657:e6fe:08d5:5325");
    /// # }
    /// ```
    pub fn leading_zeros(mut self, flag: bool) -> Self {
        self.leading_zeros = flag;
        self
    }

    /// Format the address with upper case letters. This is `false` by default.
    ///
    /// ```rust
    /// # use ipaddr::{Ipv6Address, Ipv6Formatter};
    /// # fn main() {
    /// let ip = Ipv6Address::from(0xfe80_0000_0000_0000_8657_e6fe_08d5_5325);
    /// let mut s = String::with_capacity(40);
    ///
    /// ip.formatter(&mut s).write().unwrap();
    /// assert_eq!(s, "fe80::8657:e6fe:8d5:5325");
    ///
    /// s.truncate(0);
    /// ip.formatter(&mut s).upper_case(true).write().unwrap();
    /// assert_eq!(s, "FE80::8657:E6FE:8D5:5325");
    /// # }
    /// ```
    pub fn upper_case(mut self, flag: bool) -> Self {
        self.upper_case = flag;
        self
    }

    fn longest_zero_sequence(&self) -> Option<(usize, usize)> {
        let mut start: Option<usize> = None;
        let mut end: Option<usize> = None;
        let mut longest_seq: Option<(usize, usize)> = None;

        for (i, h) in self.hextets.iter().enumerate() {
            if *h == 0 {
                if start.is_none() {
                    start = Some(i);
                } else {
                    end = Some(i);
                }
            } else if end.is_some() {
                let cur_start = start.unwrap();
                let cur_end = end.unwrap();

                if let Some((prev_start, prev_end)) = longest_seq {
                    if prev_end - prev_start < cur_end - cur_start {
                        longest_seq = Some((cur_start, cur_end));
                        start = None;
                        end = None;
                    }
                } else {
                    longest_seq = Some((cur_start, cur_end));
                    start = None;
                    end = None;
                }
            }
        }
        if end.is_some() && longest_seq.is_none() {
            longest_seq = Some((start.unwrap(), end.unwrap()));
        }
        longest_seq
    }

    /// Write a string with the current configuration.
    ///
    /// The method can only return an error if the writer `W` fails. That means if the
    /// writer cannot fail (for instance if the writer is a `String`), this method won't fail.
    ///
    /// ```rust
    /// # use ipaddr::{Ipv6Address, Ipv6Formatter};
    /// # fn main() {
    /// let ip = Ipv6Address::from(0xfe80_0000_0000_0000_8657_e6fe_08d5_5325);
    /// // our writer will be a string. We reserve just enough space for an IPv6 address.
    /// let mut s = String::with_capacity(40);
    /// // it is ok to unwrap here, writing into a String cannot fail
    /// ip.formatter(&mut s).write().unwrap();
    /// # }
    pub fn write(&mut self) -> Result {
        if !self.ellipsis {
            self.write_hextets(0..=7)
        } else if let Some((start, end)) = self.longest_zero_sequence() {
            self.write_hextets(0..start)?;
            self.writer.write_str("::")?;
            self.write_hextets(end + 1..=7)
        } else {
            self.write_hextets(0..=7)
        }
    }

    // this method is clearer with nested `if`s
    #[cfg_attr(feature = "cargo-clippy", allow(collapsible_if))]
    fn write_hextet(&mut self, hextet: u16) -> Result {
        if self.leading_zeros {
            if self.upper_case {
                write!(self.writer, "{:04X}", hextet)
            } else {
                write!(self.writer, "{:04x}", hextet)
            }
        } else {
            if self.upper_case {
                write!(self.writer, "{:X}", hextet)
            } else {
                write!(self.writer, "{:x}", hextet)
            }
        }
    }

    fn write_hextets<I>(&mut self, mut range: I) -> Result
    where
        I: Iterator<Item = usize>,
    {
        if let Some(i) = range.next() {
            let hextet = self.hextets[i];
            self.write_hextet(hextet)?;
        } else {
            return Ok(());
        }
        for i in range {
            self.writer.write_str(":")?;
            let hextet = self.hextets[i];
            self.write_hextet(hextet)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use ipv6::Ipv6Address;

    #[test]
    fn test_format_link_local() {
        let ip = Ipv6Address(0xfe80_0000_0000_0000_8657_0000_08d5_5325);
        let mut s = String::with_capacity(40);

        // Default formatting
        ip.formatter(&mut s).write().unwrap();
        assert_eq!(s, "fe80::8657:0:8d5:5325");

        // The default formatting is the RFC5952 formatting
        s.truncate(0);
        ip.formatter(&mut s).rfc_5952().write().unwrap();
        assert_eq!(s, "fe80::8657:0:8d5:5325");

        // Use upper case letters
        s.truncate(0);
        ip.formatter(&mut s).upper_case(true).write().unwrap();
        assert_eq!(s, "FE80::8657:0:8D5:5325");

        // Do not reduce the longest 0 sequence to ::
        s.truncate(0);
        ip.formatter(&mut s).ellipsis(false).write().unwrap();
        assert_eq!(s, "fe80:0:0:0:8657:0:8d5:5325");

        // Like the previous one, but with upper case letters
        s.truncate(0);
        ip.formatter(&mut s)
            .upper_case(true)
            .ellipsis(false)
            .write()
            .unwrap();
        assert_eq!(s, "FE80:0:0:0:8657:0:8D5:5325");

        // Print leading zeros: 8d5 is now 08d5, and 0 is 0000
        // Note that the longest 0 sequence is still shortened to ::
        s.truncate(0);
        ip.formatter(&mut s).leading_zeros(true).write().unwrap();
        assert_eq!(s, "fe80::8657:0000:08d5:5325");

        // Disable ellipsis, and print leading zeros. This is the most expanded form.
        // With this settings, an IPv6 will always be 40 characters long.
        s.truncate(0);
        ip.formatter(&mut s)
            .ellipsis(false)
            .leading_zeros(true)
            .write()
            .unwrap();
        assert_eq!(s, "fe80:0000:0000:0000:8657:0000:08d5:5325");

        // A shortcut for the previous example:
        s.truncate(0);
        ip.formatter(&mut s).expanded().write().unwrap();
        assert_eq!(s, "fe80:0000:0000:0000:8657:0000:08d5:5325");

        // Expanded and upper case
        s.truncate(0);
        ip.formatter(&mut s)
            .upper_case(true)
            .ellipsis(false)
            .leading_zeros(true)
            .write()
            .unwrap();
        assert_eq!(s, "FE80:0000:0000:0000:8657:0000:08D5:5325");
    }

    #[test]
    fn test_format_zero() {
        let mut s = String::with_capacity(40);
        let ip = Ipv6Address(0);
        ip.formatter(&mut s).write().unwrap();
        assert_eq!(s, "::");

        s.truncate(0);
        ip.formatter(&mut s).upper_case(true).write().unwrap();
        assert_eq!(s, "::");

        s.truncate(0);
        ip.formatter(&mut s).ellipsis(false).write().unwrap();
        assert_eq!(s, "0:0:0:0:0:0:0:0");

        s.truncate(0);
        ip.formatter(&mut s)
            .upper_case(true)
            .ellipsis(false)
            .write()
            .unwrap();
        assert_eq!(s, "0:0:0:0:0:0:0:0");

        s.truncate(0);
        ip.formatter(&mut s).leading_zeros(true).write().unwrap();
        assert_eq!(s, "::");

        s.truncate(0);
        ip.formatter(&mut s)
            .ellipsis(false)
            .leading_zeros(true)
            .write()
            .unwrap();
        assert_eq!(s, "0000:0000:0000:0000:0000:0000:0000:0000");

        s.truncate(0);
        ip.formatter(&mut s)
            .upper_case(true)
            .ellipsis(false)
            .leading_zeros(true)
            .write()
            .unwrap();
        assert_eq!(s, "0000:0000:0000:0000:0000:0000:0000:0000");
    }

    #[test]
    fn test_format_loopback() {
        let mut s = String::with_capacity(40);
        let ip = Ipv6Address(1);

        ip.formatter(&mut s).write().unwrap();
        assert_eq!(s, "::1");

        s.truncate(0);
        ip.formatter(&mut s).upper_case(true).write().unwrap();
        assert_eq!(s, "::1");

        s.truncate(0);
        ip.formatter(&mut s).ellipsis(false).write().unwrap();
        assert_eq!(s, "0:0:0:0:0:0:0:1");

        s.truncate(0);
        ip.formatter(&mut s)
            .upper_case(true)
            .ellipsis(false)
            .write()
            .unwrap();
        assert_eq!(s, "0:0:0:0:0:0:0:1");

        s.truncate(0);
        ip.formatter(&mut s).leading_zeros(true).write().unwrap();
        assert_eq!(s, "::0001");

        s.truncate(0);
        ip.formatter(&mut s)
            .ellipsis(false)
            .leading_zeros(true)
            .write()
            .unwrap();
        assert_eq!(s, "0000:0000:0000:0000:0000:0000:0000:0001");

        s.truncate(0);
        ip.formatter(&mut s)
            .upper_case(true)
            .ellipsis(false)
            .leading_zeros(true)
            .write()
            .unwrap();
        assert_eq!(s, "0000:0000:0000:0000:0000:0000:0000:0001");
    }
}
