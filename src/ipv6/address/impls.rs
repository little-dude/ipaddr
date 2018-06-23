use std::num::ParseIntError;
use std::ops::*;

use core::fmt::{self, Binary, Formatter, LowerHex, UpperHex};

use {Ipv6Address, Ipv6Mask};

impl Ipv6Address {
    pub fn min_value() -> Ipv6Address {
        u128::min_value().into()
    }

    pub fn max_value() -> Ipv6Address {
        u128::max_value().into()
    }

    pub fn from_str_radix(src: &str, radix: u32) -> Result<Ipv6Address, ParseIntError> {
        u128::from_str_radix(src, radix).map(Ipv6Address)
    }

    pub fn count_ones(self) -> u32 {
        self.0.count_ones()
    }

    pub fn count_zeros(self) -> u32 {
        self.0.count_zeros()
    }

    pub fn leading_zeros(self) -> u32 {
        self.0.leading_zeros()
    }

    pub fn trailing_zeros(self) -> u32 {
        self.0.trailing_zeros()
    }

    pub fn rotate_left(self, n: u32) -> Ipv6Address {
        self.0.rotate_left(n).into()
    }

    pub fn rotate_right(self, n: u32) -> Ipv6Address {
        self.0.rotate_right(n).into()
    }

    pub fn swap_bytes(self) -> Ipv6Address {
        self.0.swap_bytes().into()
    }

    // pub fn reverse_bits(self) -> Ipv6Address {
    //     self.0.reverse_bits().into()
    // }

    pub fn from_be(x: u128) -> Ipv6Address {
        u128::from_be(x).into()
    }

    pub fn from_le(x: u128) -> Ipv6Address {
        u128::from_le(x).into()
    }

    pub fn to_be(self) -> u128 {
        self.0.to_be()
    }

    pub fn to_le(self) -> u128 {
        self.0.to_le()
    }

    pub fn checked_add(self, rhs: u128) -> Option<Ipv6Address> {
        self.0.checked_add(rhs).map(Ipv6Address)
    }

    pub fn checked_sub(self, rhs: u128) -> Option<Ipv6Address> {
        self.0.checked_sub(rhs).map(Ipv6Address)
    }

    pub fn checked_mul(self, rhs: u128) -> Option<Ipv6Address> {
        self.0.checked_mul(rhs).map(Ipv6Address)
    }

    pub fn checked_div(self, rhs: u128) -> Option<Ipv6Address> {
        self.0.checked_div(rhs).map(Ipv6Address)
    }

    // pub fn checked_div_euc(self, rhs: u128) -> Option<Ipv6Address> {
    //     self.0.checked_div_euc(rhs).map(Ipv6Address)
    // }

    pub fn checked_rem(self, rhs: u128) -> Option<Ipv6Address> {
        self.0.checked_rem(rhs).map(Ipv6Address)
    }

    // pub fn checked_mod_euc(self, rhs: u128) -> Option<Ipv6Address> {
    //     self.0.checked_mod_euc(rhs).map(Ipv6Address)
    // }

    pub fn checked_neg(self) -> Option<Ipv6Address> {
        self.0.checked_neg().map(Ipv6Address)
    }

    pub fn checked_shl(self, rhs: u32) -> Option<Ipv6Address> {
        self.0.checked_shl(rhs).map(Ipv6Address)
    }

    pub fn checked_shr(self, rhs: u32) -> Option<Ipv6Address> {
        self.0.checked_shr(rhs).map(Ipv6Address)
    }

    // pub fn checked_pow(self, rhs: u32) -> Option<Ipv6Address> {
    //     self.0.checked_pow(rhs).map(Ipv6Address)
    // }

    pub fn saturating_add(self, rhs: u128) -> Ipv6Address {
        self.0.saturating_add(rhs).into()
    }

    pub fn saturating_sub(self, rhs: u128) -> Ipv6Address {
        self.0.saturating_sub(rhs).into()
    }

    pub fn saturating_mul(self, rhs: u128) -> Ipv6Address {
        self.0.saturating_mul(rhs).into()
    }

    pub fn wrapping_add(self, rhs: u128) -> Ipv6Address {
        self.0.wrapping_add(rhs).into()
    }

    pub fn wrapping_sub(self, rhs: u128) -> Ipv6Address {
        self.0.wrapping_sub(rhs).into()
    }

    pub fn wrapping_mul(self, rhs: u128) -> Ipv6Address {
        self.0.wrapping_mul(rhs).into()
    }

    pub fn wrapping_div(self, rhs: u128) -> Ipv6Address {
        self.0.wrapping_div(rhs).into()
    }

    // pub fn wrapping_div_euc(self, rhs: u128) -> Ipv6Address {
    //     self.0.wrapping_div_euc(rhs).into()
    // }

    pub fn wrapping_rem(self, rhs: u128) -> Ipv6Address {
        self.0.wrapping_rem(rhs).into()
    }

    // pub fn wrapping_mod_euc(self, rhs: u128) -> Ipv6Address {
    //     self.0.wrapping_mod_euc(rhs).into()
    // }

    pub fn wrapping_neg(self) -> Ipv6Address {
        self.0.wrapping_neg().into()
    }

    pub fn wrapping_shl(self, rhs: u32) -> Ipv6Address {
        self.0.wrapping_shl(rhs).into()
    }

    pub fn wrapping_shr(self, rhs: u32) -> Ipv6Address {
        self.0.wrapping_shr(rhs).into()
    }

    // pub fn wrapping_pow(self, rhs: u32) -> Ipv6Address {
    //     self.0.wrapping_pow(rhs).into()
    // }

    pub fn overflowing_add(self, rhs: u128) -> (Ipv6Address, bool) {
        let (i, b) = self.0.overflowing_add(rhs);
        (i.into(), b)
    }

    pub fn overflowing_sub(self, rhs: u128) -> (Ipv6Address, bool) {
        let (i, b) = self.0.overflowing_sub(rhs);
        (i.into(), b)
    }

    pub fn overflowing_mul(self, rhs: u128) -> (Ipv6Address, bool) {
        let (i, b) = self.0.overflowing_mul(rhs);
        (i.into(), b)
    }

    pub fn overflowing_div(self, rhs: u128) -> (Ipv6Address, bool) {
        let (i, b) = self.0.overflowing_div(rhs);
        (i.into(), b)
    }

    // pub fn overflowing_div_euc(self, rhs: u128) -> (Ipv6Address, bool) {
    //     self.0.overflowing_div_euc(rhs).map(|res| (res.0.into(), res.1))
    //     (i.into(), b)
    // }

    pub fn overflowing_rem(self, rhs: u128) -> (Ipv6Address, bool) {
        let (i, b) = self.0.overflowing_rem(rhs);
        (i.into(), b)
    }

    // pub fn overflowing_mod_euc(self, rhs: u128) -> (Ipv6Address, bool) {
    //     self.0.overflowing_mod_euc(rhs).map(|res| (res.0.into(), res.1))
    //    (i.into(), b)
    // }

    pub fn overflowing_neg(self) -> (Ipv6Address, bool) {
        let (i, b) = self.0.overflowing_neg();
        (i.into(), b)
    }

    pub fn overflowing_shl(self, rhs: u32) -> (Ipv6Address, bool) {
        let (i, b) = self.0.overflowing_shl(rhs);
        (i.into(), b)
    }

    pub fn overflowing_shr(self, rhs: u32) -> (Ipv6Address, bool) {
        let (i, b) = self.0.overflowing_shr(rhs);
        (i.into(), b)
    }

    // pub fn overflowing_pow(self, rhs: u32) -> (Ipv6Address, bool) {
    //     self.0.overflowing_pow(rhs).map(|res| (res.0.into(), res.1))
    //    (i.into(), b)
    // }
}

impl AsRef<u128> for Ipv6Address {
    fn as_ref(&self) -> &u128 {
        &self.0
    }
}

impl AsMut<u128> for Ipv6Address {
    fn as_mut(&mut self) -> &mut u128 {
        &mut self.0
    }
}

// FIXME does it make send to impl Deref and/or Borrow traits for u128 ?

// impl Deref for Ipv6Address {
//     type Target = u128;
//     fn deref(&self) -> &u128 {
//         &self.0
//     }
// }
//
// impl DerefMut for Ipv6Address {
//     fn deref_mut(&mut self) -> &mut u128 {
//         &mut self.0
//     }
// }

impl Add for Ipv6Address {
    type Output = Ipv6Address;
    fn add(self, rhs: Ipv6Address) -> Self {
        Ipv6Address(self.0.add(rhs.0))
    }
}

impl<'a> Add<Ipv6Address> for &'a Ipv6Address {
    type Output = Ipv6Address;
    fn add(self, rhs: Ipv6Address) -> Self::Output {
        Ipv6Address(self.0.add(rhs.0))
    }
}

impl<'a> Add<&'a Ipv6Address> for Ipv6Address {
    type Output = Ipv6Address;
    fn add(self, rhs: &'a Ipv6Address) -> Self {
        Ipv6Address(self.0.add(rhs.0))
    }
}

impl<'a, 'b> Add<&'a Ipv6Address> for &'b Ipv6Address {
    type Output = Ipv6Address;
    fn add(self, rhs: &'a Ipv6Address) -> Self::Output {
        Ipv6Address(self.0.add(rhs.0))
    }
}

impl Add<u128> for Ipv6Address {
    type Output = Ipv6Address;
    fn add(self, rhs: u128) -> Self::Output {
        Ipv6Address(self.0.add(rhs))
    }
}

impl<'a> Add<u128> for &'a Ipv6Address {
    type Output = Ipv6Address;
    fn add(self, rhs: u128) -> Self::Output {
        Ipv6Address(self.0.add(rhs))
    }
}

impl<'a> Add<&'a u128> for Ipv6Address {
    type Output = Ipv6Address;
    fn add(self, rhs: &'a u128) -> Self::Output {
        Ipv6Address(self.0.add(rhs))
    }
}

impl<'a, 'b> Add<&'a u128> for &'b Ipv6Address {
    type Output = Ipv6Address;
    fn add(self, rhs: &'a u128) -> Self::Output {
        Ipv6Address(self.0.add(rhs))
    }
}

impl AddAssign for Ipv6Address {
    fn add_assign(&mut self, rhs: Ipv6Address) {
        self.0.add_assign(rhs.0)
    }
}

impl<'a> AddAssign<&'a Ipv6Address> for Ipv6Address {
    fn add_assign(&mut self, rhs: &'a Ipv6Address) {
        self.0.add_assign(rhs.0)
    }
}

impl AddAssign<u128> for Ipv6Address {
    fn add_assign(&mut self, rhs: u128) {
        self.0.add_assign(rhs)
    }
}

impl<'a> AddAssign<&'a u128> for Ipv6Address {
    fn add_assign(&mut self, rhs: &'a u128) {
        self.0.add_assign(rhs)
    }
}

impl Sub for Ipv6Address {
    type Output = Ipv6Address;
    fn sub(self, rhs: Ipv6Address) -> Self {
        Ipv6Address(self.0.sub(rhs.0))
    }
}

impl<'a> Sub<Ipv6Address> for &'a Ipv6Address {
    type Output = Ipv6Address;
    fn sub(self, rhs: Ipv6Address) -> Self::Output {
        Ipv6Address(self.0.sub(rhs.0))
    }
}

impl<'a> Sub<&'a Ipv6Address> for Ipv6Address {
    type Output = Ipv6Address;
    fn sub(self, rhs: &'a Ipv6Address) -> Self {
        Ipv6Address(self.0.sub(rhs.0))
    }
}

impl<'a, 'b> Sub<&'a Ipv6Address> for &'b Ipv6Address {
    type Output = Ipv6Address;
    fn sub(self, rhs: &'a Ipv6Address) -> Self::Output {
        Ipv6Address(self.0.sub(rhs.0))
    }
}

impl Sub<u128> for Ipv6Address {
    type Output = Ipv6Address;
    fn sub(self, rhs: u128) -> Self::Output {
        Ipv6Address(self.0.sub(rhs))
    }
}

impl<'a> Sub<u128> for &'a Ipv6Address {
    type Output = Ipv6Address;
    fn sub(self, rhs: u128) -> Self::Output {
        Ipv6Address(self.0.sub(rhs))
    }
}

impl<'a> Sub<&'a u128> for Ipv6Address {
    type Output = Ipv6Address;
    fn sub(self, rhs: &'a u128) -> Self::Output {
        Ipv6Address(self.0.sub(rhs))
    }
}

impl<'a, 'b> Sub<&'a u128> for &'b Ipv6Address {
    type Output = Ipv6Address;
    fn sub(self, rhs: &'a u128) -> Self::Output {
        Ipv6Address(self.0.sub(rhs))
    }
}

impl SubAssign for Ipv6Address {
    fn sub_assign(&mut self, rhs: Ipv6Address) {
        self.0.sub_assign(rhs.0)
    }
}

impl<'a> SubAssign<&'a Ipv6Address> for Ipv6Address {
    fn sub_assign(&mut self, rhs: &'a Ipv6Address) {
        self.0.sub_assign(rhs.0)
    }
}

impl SubAssign<u128> for Ipv6Address {
    fn sub_assign(&mut self, rhs: u128) {
        self.0.sub_assign(rhs)
    }
}

impl<'a> SubAssign<&'a u128> for Ipv6Address {
    fn sub_assign(&mut self, rhs: &'a u128) {
        self.0.sub_assign(rhs)
    }
}

impl Div for Ipv6Address {
    type Output = Ipv6Address;
    fn div(self, rhs: Ipv6Address) -> Self {
        Ipv6Address(self.0.div(rhs.0))
    }
}

impl<'a> Div<Ipv6Address> for &'a Ipv6Address {
    type Output = Ipv6Address;
    fn div(self, rhs: Ipv6Address) -> Self::Output {
        Ipv6Address(self.0.div(rhs.0))
    }
}

impl<'a> Div<&'a Ipv6Address> for Ipv6Address {
    type Output = Ipv6Address;
    fn div(self, rhs: &'a Ipv6Address) -> Self {
        Ipv6Address(self.0.div(rhs.0))
    }
}

impl<'a, 'b> Div<&'a Ipv6Address> for &'b Ipv6Address {
    type Output = Ipv6Address;
    fn div(self, rhs: &'a Ipv6Address) -> Self::Output {
        Ipv6Address(self.0.div(rhs.0))
    }
}

impl Div<u128> for Ipv6Address {
    type Output = Ipv6Address;
    fn div(self, rhs: u128) -> Self::Output {
        Ipv6Address(self.0.div(rhs))
    }
}

impl<'a> Div<u128> for &'a Ipv6Address {
    type Output = Ipv6Address;
    fn div(self, rhs: u128) -> Self::Output {
        Ipv6Address(self.0.div(rhs))
    }
}

impl<'a> Div<&'a u128> for Ipv6Address {
    type Output = Ipv6Address;
    fn div(self, rhs: &'a u128) -> Self::Output {
        Ipv6Address(self.0.div(rhs))
    }
}

impl<'a, 'b> Div<&'a u128> for &'b Ipv6Address {
    type Output = Ipv6Address;
    fn div(self, rhs: &'a u128) -> Self::Output {
        Ipv6Address(self.0.div(rhs))
    }
}

impl DivAssign for Ipv6Address {
    fn div_assign(&mut self, rhs: Ipv6Address) {
        self.0.div_assign(rhs.0)
    }
}

impl<'a> DivAssign<&'a Ipv6Address> for Ipv6Address {
    fn div_assign(&mut self, rhs: &'a Ipv6Address) {
        self.0.div_assign(rhs.0)
    }
}

impl DivAssign<u128> for Ipv6Address {
    fn div_assign(&mut self, rhs: u128) {
        self.0.div_assign(rhs)
    }
}

impl<'a> DivAssign<&'a u128> for Ipv6Address {
    fn div_assign(&mut self, rhs: &'a u128) {
        self.0.div_assign(rhs)
    }
}

impl Rem for Ipv6Address {
    type Output = Ipv6Address;
    fn rem(self, rhs: Ipv6Address) -> Self {
        Ipv6Address(self.0.rem(rhs.0))
    }
}

impl<'a> Rem<Ipv6Address> for &'a Ipv6Address {
    type Output = Ipv6Address;
    fn rem(self, rhs: Ipv6Address) -> Self::Output {
        Ipv6Address(self.0.rem(rhs.0))
    }
}

impl<'a> Rem<&'a Ipv6Address> for Ipv6Address {
    type Output = Ipv6Address;
    fn rem(self, rhs: &'a Ipv6Address) -> Self {
        Ipv6Address(self.0.rem(rhs.0))
    }
}

impl<'a, 'b> Rem<&'a Ipv6Address> for &'b Ipv6Address {
    type Output = Ipv6Address;
    fn rem(self, rhs: &'a Ipv6Address) -> Self::Output {
        Ipv6Address(self.0.rem(rhs.0))
    }
}

impl Rem<u128> for Ipv6Address {
    type Output = Ipv6Address;
    fn rem(self, rhs: u128) -> Self::Output {
        Ipv6Address(self.0.rem(rhs))
    }
}

impl<'a> Rem<u128> for &'a Ipv6Address {
    type Output = Ipv6Address;
    fn rem(self, rhs: u128) -> Self::Output {
        Ipv6Address(self.0.rem(rhs))
    }
}

impl<'a> Rem<&'a u128> for Ipv6Address {
    type Output = Ipv6Address;
    fn rem(self, rhs: &'a u128) -> Self::Output {
        Ipv6Address(self.0.rem(rhs))
    }
}

impl<'a, 'b> Rem<&'a u128> for &'b Ipv6Address {
    type Output = Ipv6Address;
    fn rem(self, rhs: &'a u128) -> Self::Output {
        Ipv6Address(self.0.rem(rhs))
    }
}

impl RemAssign for Ipv6Address {
    fn rem_assign(&mut self, rhs: Ipv6Address) {
        self.0.rem_assign(rhs.0)
    }
}

impl<'a> RemAssign<&'a Ipv6Address> for Ipv6Address {
    fn rem_assign(&mut self, rhs: &'a Ipv6Address) {
        self.0.rem_assign(rhs.0)
    }
}

impl RemAssign<u128> for Ipv6Address {
    fn rem_assign(&mut self, rhs: u128) {
        self.0.rem_assign(rhs)
    }
}

impl<'a> RemAssign<&'a u128> for Ipv6Address {
    fn rem_assign(&mut self, rhs: &'a u128) {
        self.0.rem_assign(rhs)
    }
}

impl Mul for Ipv6Address {
    type Output = Ipv6Address;
    fn mul(self, rhs: Ipv6Address) -> Self {
        Ipv6Address(self.0.mul(rhs.0))
    }
}

impl<'a> Mul<Ipv6Address> for &'a Ipv6Address {
    type Output = Ipv6Address;
    fn mul(self, rhs: Ipv6Address) -> Self::Output {
        Ipv6Address(self.0.mul(rhs.0))
    }
}

impl<'a> Mul<&'a Ipv6Address> for Ipv6Address {
    type Output = Ipv6Address;
    fn mul(self, rhs: &'a Ipv6Address) -> Self {
        Ipv6Address(self.0.mul(rhs.0))
    }
}

impl<'a, 'b> Mul<&'a Ipv6Address> for &'b Ipv6Address {
    type Output = Ipv6Address;
    fn mul(self, rhs: &'a Ipv6Address) -> Self::Output {
        Ipv6Address(self.0.mul(rhs.0))
    }
}

impl Mul<u128> for Ipv6Address {
    type Output = Ipv6Address;
    fn mul(self, rhs: u128) -> Self::Output {
        Ipv6Address(self.0.mul(rhs))
    }
}

impl<'a> Mul<u128> for &'a Ipv6Address {
    type Output = Ipv6Address;
    fn mul(self, rhs: u128) -> Self::Output {
        Ipv6Address(self.0.mul(rhs))
    }
}

impl<'a> Mul<&'a u128> for Ipv6Address {
    type Output = Ipv6Address;
    fn mul(self, rhs: &'a u128) -> Self::Output {
        Ipv6Address(self.0.mul(rhs))
    }
}

impl<'a, 'b> Mul<&'a u128> for &'b Ipv6Address {
    type Output = Ipv6Address;
    fn mul(self, rhs: &'a u128) -> Self::Output {
        Ipv6Address(self.0.mul(rhs))
    }
}

impl MulAssign for Ipv6Address {
    fn mul_assign(&mut self, rhs: Ipv6Address) {
        self.0.mul_assign(rhs.0)
    }
}

impl<'a> MulAssign<&'a Ipv6Address> for Ipv6Address {
    fn mul_assign(&mut self, rhs: &'a Ipv6Address) {
        self.0.mul_assign(rhs.0)
    }
}

impl MulAssign<u128> for Ipv6Address {
    fn mul_assign(&mut self, rhs: u128) {
        self.0.mul_assign(rhs)
    }
}

impl<'a> MulAssign<&'a u128> for Ipv6Address {
    fn mul_assign(&mut self, rhs: &'a u128) {
        self.0.mul_assign(rhs)
    }
}

impl BitAnd for Ipv6Address {
    type Output = Ipv6Address;
    fn bitand(self, rhs: Ipv6Address) -> Self {
        Ipv6Address(self.0.bitand(rhs.0))
    }
}

impl<'a> BitAnd<Ipv6Address> for &'a Ipv6Address {
    type Output = Ipv6Address;
    fn bitand(self, rhs: Ipv6Address) -> Self::Output {
        Ipv6Address(self.0.bitand(rhs.0))
    }
}

impl<'a> BitAnd<&'a Ipv6Address> for Ipv6Address {
    type Output = Ipv6Address;
    fn bitand(self, rhs: &'a Ipv6Address) -> Self {
        Ipv6Address(self.0.bitand(rhs.0))
    }
}

impl<'a, 'b> BitAnd<&'a Ipv6Address> for &'b Ipv6Address {
    type Output = Ipv6Address;
    fn bitand(self, rhs: &'a Ipv6Address) -> Self::Output {
        Ipv6Address(self.0.bitand(rhs.0))
    }
}

impl BitAnd<u128> for Ipv6Address {
    type Output = Ipv6Address;
    fn bitand(self, rhs: u128) -> Self::Output {
        Ipv6Address(self.0.bitand(rhs))
    }
}

impl<'a> BitAnd<u128> for &'a Ipv6Address {
    type Output = Ipv6Address;
    fn bitand(self, rhs: u128) -> Self::Output {
        Ipv6Address(self.0.bitand(rhs))
    }
}

impl<'a> BitAnd<&'a u128> for Ipv6Address {
    type Output = Ipv6Address;
    fn bitand(self, rhs: &'a u128) -> Self::Output {
        Ipv6Address(self.0.bitand(rhs))
    }
}

impl<'a, 'b> BitAnd<&'a u128> for &'b Ipv6Address {
    type Output = Ipv6Address;
    fn bitand(self, rhs: &'a u128) -> Self::Output {
        Ipv6Address(self.0.bitand(rhs))
    }
}

impl BitAndAssign for Ipv6Address {
    fn bitand_assign(&mut self, rhs: Ipv6Address) {
        self.0.bitand_assign(rhs.0)
    }
}

impl<'a> BitAndAssign<&'a Ipv6Address> for Ipv6Address {
    fn bitand_assign(&mut self, rhs: &'a Ipv6Address) {
        self.0.bitand_assign(rhs.0)
    }
}

impl BitAndAssign<u128> for Ipv6Address {
    fn bitand_assign(&mut self, rhs: u128) {
        self.0.bitand_assign(rhs)
    }
}

impl<'a> BitAndAssign<&'a u128> for Ipv6Address {
    fn bitand_assign(&mut self, rhs: &'a u128) {
        self.0.bitand_assign(rhs)
    }
}

impl BitOr for Ipv6Address {
    type Output = Ipv6Address;
    fn bitor(self, rhs: Ipv6Address) -> Self {
        Ipv6Address(self.0.bitor(rhs.0))
    }
}

impl<'a> BitOr<Ipv6Address> for &'a Ipv6Address {
    type Output = Ipv6Address;
    fn bitor(self, rhs: Ipv6Address) -> Self::Output {
        Ipv6Address(self.0.bitor(rhs.0))
    }
}

impl<'a> BitOr<&'a Ipv6Address> for Ipv6Address {
    type Output = Ipv6Address;
    fn bitor(self, rhs: &'a Ipv6Address) -> Self {
        Ipv6Address(self.0.bitor(rhs.0))
    }
}

impl<'a, 'b> BitOr<&'a Ipv6Address> for &'b Ipv6Address {
    type Output = Ipv6Address;
    fn bitor(self, rhs: &'a Ipv6Address) -> Self::Output {
        Ipv6Address(self.0.bitor(rhs.0))
    }
}

impl BitOr<u128> for Ipv6Address {
    type Output = Ipv6Address;
    fn bitor(self, rhs: u128) -> Self::Output {
        Ipv6Address(self.0.bitor(rhs))
    }
}

impl<'a> BitOr<u128> for &'a Ipv6Address {
    type Output = Ipv6Address;
    fn bitor(self, rhs: u128) -> Self::Output {
        Ipv6Address(self.0.bitor(rhs))
    }
}

impl<'a> BitOr<&'a u128> for Ipv6Address {
    type Output = Ipv6Address;
    fn bitor(self, rhs: &'a u128) -> Self::Output {
        Ipv6Address(self.0.bitor(rhs))
    }
}

impl<'a, 'b> BitOr<&'a u128> for &'b Ipv6Address {
    type Output = Ipv6Address;
    fn bitor(self, rhs: &'a u128) -> Self::Output {
        Ipv6Address(self.0.bitor(rhs))
    }
}

impl BitOrAssign for Ipv6Address {
    fn bitor_assign(&mut self, rhs: Ipv6Address) {
        self.0.bitor_assign(rhs.0)
    }
}

impl<'a> BitOrAssign<&'a Ipv6Address> for Ipv6Address {
    fn bitor_assign(&mut self, rhs: &'a Ipv6Address) {
        self.0.bitor_assign(rhs.0)
    }
}

impl BitOrAssign<u128> for Ipv6Address {
    fn bitor_assign(&mut self, rhs: u128) {
        self.0.bitor_assign(rhs)
    }
}

impl<'a> BitOrAssign<&'a u128> for Ipv6Address {
    fn bitor_assign(&mut self, rhs: &'a u128) {
        self.0.bitor_assign(rhs)
    }
}

impl BitXor for Ipv6Address {
    type Output = Ipv6Address;
    fn bitxor(self, rhs: Ipv6Address) -> Self {
        Ipv6Address(self.0.bitxor(rhs.0))
    }
}

impl<'a> BitXor<Ipv6Address> for &'a Ipv6Address {
    type Output = Ipv6Address;
    fn bitxor(self, rhs: Ipv6Address) -> Self::Output {
        Ipv6Address(self.0.bitxor(rhs.0))
    }
}

impl<'a> BitXor<&'a Ipv6Address> for Ipv6Address {
    type Output = Ipv6Address;
    fn bitxor(self, rhs: &'a Ipv6Address) -> Self {
        Ipv6Address(self.0.bitxor(rhs.0))
    }
}

impl<'a, 'b> BitXor<&'a Ipv6Address> for &'b Ipv6Address {
    type Output = Ipv6Address;
    fn bitxor(self, rhs: &'a Ipv6Address) -> Self::Output {
        Ipv6Address(self.0.bitxor(rhs.0))
    }
}

impl BitXor<u128> for Ipv6Address {
    type Output = Ipv6Address;
    fn bitxor(self, rhs: u128) -> Self::Output {
        Ipv6Address(self.0.bitxor(rhs))
    }
}

impl<'a> BitXor<u128> for &'a Ipv6Address {
    type Output = Ipv6Address;
    fn bitxor(self, rhs: u128) -> Self::Output {
        Ipv6Address(self.0.bitxor(rhs))
    }
}

impl<'a> BitXor<&'a u128> for Ipv6Address {
    type Output = Ipv6Address;
    fn bitxor(self, rhs: &'a u128) -> Self::Output {
        Ipv6Address(self.0.bitxor(rhs))
    }
}

impl<'a, 'b> BitXor<&'a u128> for &'b Ipv6Address {
    type Output = Ipv6Address;
    fn bitxor(self, rhs: &'a u128) -> Self::Output {
        Ipv6Address(self.0.bitxor(rhs))
    }
}

impl BitXorAssign for Ipv6Address {
    fn bitxor_assign(&mut self, rhs: Ipv6Address) {
        self.0.bitxor_assign(rhs.0)
    }
}

impl<'a> BitXorAssign<&'a Ipv6Address> for Ipv6Address {
    fn bitxor_assign(&mut self, rhs: &'a Ipv6Address) {
        self.0.bitxor_assign(rhs.0)
    }
}

impl BitXorAssign<u128> for Ipv6Address {
    fn bitxor_assign(&mut self, rhs: u128) {
        self.0.bitxor_assign(rhs)
    }
}

impl<'a> BitXorAssign<&'a u128> for Ipv6Address {
    fn bitxor_assign(&mut self, rhs: &'a u128) {
        self.0.bitxor_assign(rhs)
    }
}

impl Shl for Ipv6Address {
    type Output = Ipv6Address;
    fn shl(self, rhs: Ipv6Address) -> Self {
        Ipv6Address(self.0.shl(rhs.0))
    }
}

impl<'a> Shl<Ipv6Address> for &'a Ipv6Address {
    type Output = Ipv6Address;
    fn shl(self, rhs: Ipv6Address) -> Self::Output {
        Ipv6Address(self.0.shl(rhs.0))
    }
}

impl<'a> Shl<&'a Ipv6Address> for Ipv6Address {
    type Output = Ipv6Address;
    fn shl(self, rhs: &'a Ipv6Address) -> Self::Output {
        Ipv6Address(self.0.shl(rhs.0))
    }
}

impl<'a, 'b> Shl<&'a Ipv6Address> for &'b Ipv6Address {
    type Output = Ipv6Address;
    fn shl(self, rhs: &'a Ipv6Address) -> Self::Output {
        Ipv6Address(self.0.shl(rhs.0))
    }
}

impl Shl<u128> for Ipv6Address {
    type Output = Ipv6Address;
    fn shl(self, rhs: u128) -> Self::Output {
        Ipv6Address(self.0.shl(rhs))
    }
}

impl<'a> Shl<u128> for &'a Ipv6Address {
    type Output = Ipv6Address;
    fn shl(self, rhs: u128) -> Self::Output {
        Ipv6Address(self.0.shl(rhs))
    }
}

impl<'a> Shl<&'a u128> for Ipv6Address {
    type Output = Ipv6Address;
    fn shl(self, rhs: &'a u128) -> Self::Output {
        Ipv6Address(self.0.shl(rhs))
    }
}

impl<'a, 'b> Shl<&'a u128> for &'b Ipv6Address {
    type Output = Ipv6Address;
    fn shl(self, rhs: &'a u128) -> Self::Output {
        Ipv6Address(self.0.shl(rhs))
    }
}

impl Shl<i128> for Ipv6Address {
    type Output = Ipv6Address;
    fn shl(self, rhs: i128) -> Self::Output {
        Ipv6Address(self.0.shl(rhs))
    }
}

impl<'a> Shl<i128> for &'a Ipv6Address {
    type Output = Ipv6Address;
    fn shl(self, rhs: i128) -> Self::Output {
        Ipv6Address(self.0.shl(rhs))
    }
}

impl<'a> Shl<&'a i128> for Ipv6Address {
    type Output = Ipv6Address;
    fn shl(self, rhs: &'a i128) -> Self::Output {
        Ipv6Address(self.0.shl(rhs))
    }
}

impl<'a, 'b> Shl<&'a i128> for &'b Ipv6Address {
    type Output = Ipv6Address;
    fn shl(self, rhs: &'a i128) -> Self::Output {
        Ipv6Address(self.0.shl(rhs))
    }
}

impl Shl<u64> for Ipv6Address {
    type Output = Ipv6Address;
    fn shl(self, rhs: u64) -> Self::Output {
        Ipv6Address(self.0.shl(rhs))
    }
}

impl<'a> Shl<u64> for &'a Ipv6Address {
    type Output = Ipv6Address;
    fn shl(self, rhs: u64) -> Self::Output {
        Ipv6Address(self.0.shl(rhs))
    }
}

impl<'a> Shl<&'a u64> for Ipv6Address {
    type Output = Ipv6Address;
    fn shl(self, rhs: &'a u64) -> Self::Output {
        Ipv6Address(self.0.shl(rhs))
    }
}

impl<'a, 'b> Shl<&'a u64> for &'b Ipv6Address {
    type Output = Ipv6Address;
    fn shl(self, rhs: &'a u64) -> Self::Output {
        Ipv6Address(self.0.shl(rhs))
    }
}

impl Shl<i64> for Ipv6Address {
    type Output = Ipv6Address;
    fn shl(self, rhs: i64) -> Self::Output {
        Ipv6Address(self.0.shl(rhs))
    }
}

impl<'a> Shl<i64> for &'a Ipv6Address {
    type Output = Ipv6Address;
    fn shl(self, rhs: i64) -> Self::Output {
        Ipv6Address(self.0.shl(rhs))
    }
}

impl<'a> Shl<&'a i64> for Ipv6Address {
    type Output = Ipv6Address;
    fn shl(self, rhs: &'a i64) -> Self::Output {
        Ipv6Address(self.0.shl(rhs))
    }
}

impl<'a, 'b> Shl<&'a i64> for &'b Ipv6Address {
    type Output = Ipv6Address;
    fn shl(self, rhs: &'a i64) -> Self::Output {
        Ipv6Address(self.0.shl(rhs))
    }
}

impl Shl<u32> for Ipv6Address {
    type Output = Ipv6Address;
    fn shl(self, rhs: u32) -> Self::Output {
        Ipv6Address(self.0.shl(rhs))
    }
}

impl<'a> Shl<u32> for &'a Ipv6Address {
    type Output = Ipv6Address;
    fn shl(self, rhs: u32) -> Self::Output {
        Ipv6Address(self.0.shl(rhs))
    }
}

impl<'a> Shl<&'a u32> for Ipv6Address {
    type Output = Ipv6Address;
    fn shl(self, rhs: &'a u32) -> Self::Output {
        Ipv6Address(self.0.shl(rhs))
    }
}

impl<'a, 'b> Shl<&'a u32> for &'b Ipv6Address {
    type Output = Ipv6Address;
    fn shl(self, rhs: &'a u32) -> Self::Output {
        Ipv6Address(self.0.shl(rhs))
    }
}

impl Shl<i32> for Ipv6Address {
    type Output = Ipv6Address;
    fn shl(self, rhs: i32) -> Self::Output {
        Ipv6Address(self.0.shl(rhs))
    }
}

impl<'a> Shl<i32> for &'a Ipv6Address {
    type Output = Ipv6Address;
    fn shl(self, rhs: i32) -> Self::Output {
        Ipv6Address(self.0.shl(rhs))
    }
}

impl<'a> Shl<&'a i32> for Ipv6Address {
    type Output = Ipv6Address;
    fn shl(self, rhs: &'a i32) -> Self::Output {
        Ipv6Address(self.0.shl(rhs))
    }
}

impl<'a, 'b> Shl<&'a i32> for &'b Ipv6Address {
    type Output = Ipv6Address;
    fn shl(self, rhs: &'a i32) -> Self::Output {
        Ipv6Address(self.0.shl(rhs))
    }
}

impl Shl<u16> for Ipv6Address {
    type Output = Ipv6Address;
    fn shl(self, rhs: u16) -> Self::Output {
        Ipv6Address(self.0.shl(rhs))
    }
}

impl<'a> Shl<u16> for &'a Ipv6Address {
    type Output = Ipv6Address;
    fn shl(self, rhs: u16) -> Self::Output {
        Ipv6Address(self.0.shl(rhs))
    }
}

impl<'a> Shl<&'a u16> for Ipv6Address {
    type Output = Ipv6Address;
    fn shl(self, rhs: &'a u16) -> Self::Output {
        Ipv6Address(self.0.shl(rhs))
    }
}

impl<'a, 'b> Shl<&'a u16> for &'b Ipv6Address {
    type Output = Ipv6Address;
    fn shl(self, rhs: &'a u16) -> Self::Output {
        Ipv6Address(self.0.shl(rhs))
    }
}

impl Shl<i16> for Ipv6Address {
    type Output = Ipv6Address;
    fn shl(self, rhs: i16) -> Self::Output {
        Ipv6Address(self.0.shl(rhs))
    }
}

impl<'a> Shl<i16> for &'a Ipv6Address {
    type Output = Ipv6Address;
    fn shl(self, rhs: i16) -> Self::Output {
        Ipv6Address(self.0.shl(rhs))
    }
}

impl<'a> Shl<&'a i16> for Ipv6Address {
    type Output = Ipv6Address;
    fn shl(self, rhs: &'a i16) -> Self::Output {
        Ipv6Address(self.0.shl(rhs))
    }
}

impl<'a, 'b> Shl<&'a i16> for &'b Ipv6Address {
    type Output = Ipv6Address;
    fn shl(self, rhs: &'a i16) -> Self::Output {
        Ipv6Address(self.0.shl(rhs))
    }
}

impl Shl<u8> for Ipv6Address {
    type Output = Ipv6Address;
    fn shl(self, rhs: u8) -> Self::Output {
        Ipv6Address(self.0.shl(rhs))
    }
}

impl<'a> Shl<u8> for &'a Ipv6Address {
    type Output = Ipv6Address;
    fn shl(self, rhs: u8) -> Self::Output {
        Ipv6Address(self.0.shl(rhs))
    }
}

impl<'a> Shl<&'a u8> for Ipv6Address {
    type Output = Ipv6Address;
    fn shl(self, rhs: &'a u8) -> Self::Output {
        Ipv6Address(self.0.shl(rhs))
    }
}

impl<'a, 'b> Shl<&'a u8> for &'b Ipv6Address {
    type Output = Ipv6Address;
    fn shl(self, rhs: &'a u8) -> Self::Output {
        Ipv6Address(self.0.shl(rhs))
    }
}

impl Shl<i8> for Ipv6Address {
    type Output = Ipv6Address;
    fn shl(self, rhs: i8) -> Self::Output {
        Ipv6Address(self.0.shl(rhs))
    }
}

impl<'a> Shl<i8> for &'a Ipv6Address {
    type Output = Ipv6Address;
    fn shl(self, rhs: i8) -> Self::Output {
        Ipv6Address(self.0.shl(rhs))
    }
}

impl<'a> Shl<&'a i8> for Ipv6Address {
    type Output = Ipv6Address;
    fn shl(self, rhs: &'a i8) -> Self::Output {
        Ipv6Address(self.0.shl(rhs))
    }
}

impl<'a, 'b> Shl<&'a i8> for &'b Ipv6Address {
    type Output = Ipv6Address;
    fn shl(self, rhs: &'a i8) -> Self::Output {
        Ipv6Address(self.0.shl(rhs))
    }
}

impl Shl<usize> for Ipv6Address {
    type Output = Ipv6Address;
    fn shl(self, rhs: usize) -> Self::Output {
        Ipv6Address(self.0.shl(rhs))
    }
}

impl<'a> Shl<usize> for &'a Ipv6Address {
    type Output = Ipv6Address;
    fn shl(self, rhs: usize) -> Self::Output {
        Ipv6Address(self.0.shl(rhs))
    }
}

impl<'a> Shl<&'a usize> for Ipv6Address {
    type Output = Ipv6Address;
    fn shl(self, rhs: &'a usize) -> Self::Output {
        Ipv6Address(self.0.shl(rhs))
    }
}

impl<'a, 'b> Shl<&'a usize> for &'b Ipv6Address {
    type Output = Ipv6Address;
    fn shl(self, rhs: &'a usize) -> Self::Output {
        Ipv6Address(self.0.shl(rhs))
    }
}
impl Shl<isize> for Ipv6Address {
    type Output = Ipv6Address;
    fn shl(self, rhs: isize) -> Self::Output {
        Ipv6Address(self.0.shl(rhs))
    }
}

impl<'a> Shl<isize> for &'a Ipv6Address {
    type Output = Ipv6Address;
    fn shl(self, rhs: isize) -> Self::Output {
        Ipv6Address(self.0.shl(rhs))
    }
}

impl<'a> Shl<&'a isize> for Ipv6Address {
    type Output = Ipv6Address;
    fn shl(self, rhs: &'a isize) -> Self::Output {
        Ipv6Address(self.0.shl(rhs))
    }
}

impl<'a, 'b> Shl<&'a isize> for &'b Ipv6Address {
    type Output = Ipv6Address;
    fn shl(self, rhs: &'a isize) -> Self::Output {
        Ipv6Address(self.0.shl(rhs))
    }
}

impl Shr for Ipv6Address {
    type Output = Ipv6Address;
    fn shr(self, rhs: Ipv6Address) -> Self {
        Ipv6Address(self.0.shr(rhs.0))
    }
}

impl<'a> Shr<Ipv6Address> for &'a Ipv6Address {
    type Output = Ipv6Address;
    fn shr(self, rhs: Ipv6Address) -> Self::Output {
        Ipv6Address(self.0.shr(rhs.0))
    }
}

impl<'a> Shr<&'a Ipv6Address> for Ipv6Address {
    type Output = Ipv6Address;
    fn shr(self, rhs: &'a Ipv6Address) -> Self::Output {
        Ipv6Address(self.0.shr(rhs.0))
    }
}

impl<'a, 'b> Shr<&'a Ipv6Address> for &'b Ipv6Address {
    type Output = Ipv6Address;
    fn shr(self, rhs: &'a Ipv6Address) -> Self::Output {
        Ipv6Address(self.0.shr(rhs.0))
    }
}

impl Shr<u128> for Ipv6Address {
    type Output = Ipv6Address;
    fn shr(self, rhs: u128) -> Self::Output {
        Ipv6Address(self.0.shr(rhs))
    }
}

impl<'a> Shr<u128> for &'a Ipv6Address {
    type Output = Ipv6Address;
    fn shr(self, rhs: u128) -> Self::Output {
        Ipv6Address(self.0.shr(rhs))
    }
}

impl<'a> Shr<&'a u128> for Ipv6Address {
    type Output = Ipv6Address;
    fn shr(self, rhs: &'a u128) -> Self::Output {
        Ipv6Address(self.0.shr(rhs))
    }
}

impl<'a, 'b> Shr<&'a u128> for &'b Ipv6Address {
    type Output = Ipv6Address;
    fn shr(self, rhs: &'a u128) -> Self::Output {
        Ipv6Address(self.0.shr(rhs))
    }
}

impl Shr<i128> for Ipv6Address {
    type Output = Ipv6Address;
    fn shr(self, rhs: i128) -> Self::Output {
        Ipv6Address(self.0.shr(rhs))
    }
}

impl<'a> Shr<i128> for &'a Ipv6Address {
    type Output = Ipv6Address;
    fn shr(self, rhs: i128) -> Self::Output {
        Ipv6Address(self.0.shr(rhs))
    }
}

impl<'a> Shr<&'a i128> for Ipv6Address {
    type Output = Ipv6Address;
    fn shr(self, rhs: &'a i128) -> Self::Output {
        Ipv6Address(self.0.shr(rhs))
    }
}

impl<'a, 'b> Shr<&'a i128> for &'b Ipv6Address {
    type Output = Ipv6Address;
    fn shr(self, rhs: &'a i128) -> Self::Output {
        Ipv6Address(self.0.shr(rhs))
    }
}

impl Shr<u64> for Ipv6Address {
    type Output = Ipv6Address;
    fn shr(self, rhs: u64) -> Self::Output {
        Ipv6Address(self.0.shr(rhs))
    }
}

impl<'a> Shr<u64> for &'a Ipv6Address {
    type Output = Ipv6Address;
    fn shr(self, rhs: u64) -> Self::Output {
        Ipv6Address(self.0.shr(rhs))
    }
}

impl<'a> Shr<&'a u64> for Ipv6Address {
    type Output = Ipv6Address;
    fn shr(self, rhs: &'a u64) -> Self::Output {
        Ipv6Address(self.0.shr(rhs))
    }
}

impl<'a, 'b> Shr<&'a u64> for &'b Ipv6Address {
    type Output = Ipv6Address;
    fn shr(self, rhs: &'a u64) -> Self::Output {
        Ipv6Address(self.0.shr(rhs))
    }
}

impl Shr<i64> for Ipv6Address {
    type Output = Ipv6Address;
    fn shr(self, rhs: i64) -> Self::Output {
        Ipv6Address(self.0.shr(rhs))
    }
}

impl<'a> Shr<i64> for &'a Ipv6Address {
    type Output = Ipv6Address;
    fn shr(self, rhs: i64) -> Self::Output {
        Ipv6Address(self.0.shr(rhs))
    }
}

impl<'a> Shr<&'a i64> for Ipv6Address {
    type Output = Ipv6Address;
    fn shr(self, rhs: &'a i64) -> Self::Output {
        Ipv6Address(self.0.shr(rhs))
    }
}

impl<'a, 'b> Shr<&'a i64> for &'b Ipv6Address {
    type Output = Ipv6Address;
    fn shr(self, rhs: &'a i64) -> Self::Output {
        Ipv6Address(self.0.shr(rhs))
    }
}

impl Shr<u32> for Ipv6Address {
    type Output = Ipv6Address;
    fn shr(self, rhs: u32) -> Self::Output {
        Ipv6Address(self.0.shr(rhs))
    }
}

impl<'a> Shr<u32> for &'a Ipv6Address {
    type Output = Ipv6Address;
    fn shr(self, rhs: u32) -> Self::Output {
        Ipv6Address(self.0.shr(rhs))
    }
}

impl<'a> Shr<&'a u32> for Ipv6Address {
    type Output = Ipv6Address;
    fn shr(self, rhs: &'a u32) -> Self::Output {
        Ipv6Address(self.0.shr(rhs))
    }
}

impl<'a, 'b> Shr<&'a u32> for &'b Ipv6Address {
    type Output = Ipv6Address;
    fn shr(self, rhs: &'a u32) -> Self::Output {
        Ipv6Address(self.0.shr(rhs))
    }
}

impl Shr<i32> for Ipv6Address {
    type Output = Ipv6Address;
    fn shr(self, rhs: i32) -> Self::Output {
        Ipv6Address(self.0.shr(rhs))
    }
}

impl<'a> Shr<i32> for &'a Ipv6Address {
    type Output = Ipv6Address;
    fn shr(self, rhs: i32) -> Self::Output {
        Ipv6Address(self.0.shr(rhs))
    }
}

impl<'a> Shr<&'a i32> for Ipv6Address {
    type Output = Ipv6Address;
    fn shr(self, rhs: &'a i32) -> Self::Output {
        Ipv6Address(self.0.shr(rhs))
    }
}

impl<'a, 'b> Shr<&'a i32> for &'b Ipv6Address {
    type Output = Ipv6Address;
    fn shr(self, rhs: &'a i32) -> Self::Output {
        Ipv6Address(self.0.shr(rhs))
    }
}

impl Shr<u16> for Ipv6Address {
    type Output = Ipv6Address;
    fn shr(self, rhs: u16) -> Self::Output {
        Ipv6Address(self.0.shr(rhs))
    }
}

impl<'a> Shr<u16> for &'a Ipv6Address {
    type Output = Ipv6Address;
    fn shr(self, rhs: u16) -> Self::Output {
        Ipv6Address(self.0.shr(rhs))
    }
}

impl<'a> Shr<&'a u16> for Ipv6Address {
    type Output = Ipv6Address;
    fn shr(self, rhs: &'a u16) -> Self::Output {
        Ipv6Address(self.0.shr(rhs))
    }
}

impl<'a, 'b> Shr<&'a u16> for &'b Ipv6Address {
    type Output = Ipv6Address;
    fn shr(self, rhs: &'a u16) -> Self::Output {
        Ipv6Address(self.0.shr(rhs))
    }
}

impl Shr<i16> for Ipv6Address {
    type Output = Ipv6Address;
    fn shr(self, rhs: i16) -> Self::Output {
        Ipv6Address(self.0.shr(rhs))
    }
}

impl<'a> Shr<i16> for &'a Ipv6Address {
    type Output = Ipv6Address;
    fn shr(self, rhs: i16) -> Self::Output {
        Ipv6Address(self.0.shr(rhs))
    }
}

impl<'a> Shr<&'a i16> for Ipv6Address {
    type Output = Ipv6Address;
    fn shr(self, rhs: &'a i16) -> Self::Output {
        Ipv6Address(self.0.shr(rhs))
    }
}

impl<'a, 'b> Shr<&'a i16> for &'b Ipv6Address {
    type Output = Ipv6Address;
    fn shr(self, rhs: &'a i16) -> Self::Output {
        Ipv6Address(self.0.shr(rhs))
    }
}

impl Shr<u8> for Ipv6Address {
    type Output = Ipv6Address;
    fn shr(self, rhs: u8) -> Self::Output {
        Ipv6Address(self.0.shr(rhs))
    }
}

impl<'a> Shr<u8> for &'a Ipv6Address {
    type Output = Ipv6Address;
    fn shr(self, rhs: u8) -> Self::Output {
        Ipv6Address(self.0.shr(rhs))
    }
}

impl<'a> Shr<&'a u8> for Ipv6Address {
    type Output = Ipv6Address;
    fn shr(self, rhs: &'a u8) -> Self::Output {
        Ipv6Address(self.0.shr(rhs))
    }
}

impl<'a, 'b> Shr<&'a u8> for &'b Ipv6Address {
    type Output = Ipv6Address;
    fn shr(self, rhs: &'a u8) -> Self::Output {
        Ipv6Address(self.0.shr(rhs))
    }
}

impl Shr<i8> for Ipv6Address {
    type Output = Ipv6Address;
    fn shr(self, rhs: i8) -> Self::Output {
        Ipv6Address(self.0.shr(rhs))
    }
}

impl<'a> Shr<i8> for &'a Ipv6Address {
    type Output = Ipv6Address;
    fn shr(self, rhs: i8) -> Self::Output {
        Ipv6Address(self.0.shr(rhs))
    }
}

impl<'a> Shr<&'a i8> for Ipv6Address {
    type Output = Ipv6Address;
    fn shr(self, rhs: &'a i8) -> Self::Output {
        Ipv6Address(self.0.shr(rhs))
    }
}

impl<'a, 'b> Shr<&'a i8> for &'b Ipv6Address {
    type Output = Ipv6Address;
    fn shr(self, rhs: &'a i8) -> Self::Output {
        Ipv6Address(self.0.shr(rhs))
    }
}

impl Shr<usize> for Ipv6Address {
    type Output = Ipv6Address;
    fn shr(self, rhs: usize) -> Self::Output {
        Ipv6Address(self.0.shr(rhs))
    }
}

impl<'a> Shr<usize> for &'a Ipv6Address {
    type Output = Ipv6Address;
    fn shr(self, rhs: usize) -> Self::Output {
        Ipv6Address(self.0.shr(rhs))
    }
}

impl<'a> Shr<&'a usize> for Ipv6Address {
    type Output = Ipv6Address;
    fn shr(self, rhs: &'a usize) -> Self::Output {
        Ipv6Address(self.0.shr(rhs))
    }
}

impl<'a, 'b> Shr<&'a usize> for &'b Ipv6Address {
    type Output = Ipv6Address;
    fn shr(self, rhs: &'a usize) -> Self::Output {
        Ipv6Address(self.0.shr(rhs))
    }
}

impl Shr<isize> for Ipv6Address {
    type Output = Ipv6Address;
    fn shr(self, rhs: isize) -> Self::Output {
        Ipv6Address(self.0.shr(rhs))
    }
}

impl<'a> Shr<isize> for &'a Ipv6Address {
    type Output = Ipv6Address;
    fn shr(self, rhs: isize) -> Self::Output {
        Ipv6Address(self.0.shr(rhs))
    }
}

impl<'a> Shr<&'a isize> for Ipv6Address {
    type Output = Ipv6Address;
    fn shr(self, rhs: &'a isize) -> Self::Output {
        Ipv6Address(self.0.shr(rhs))
    }
}

impl<'a, 'b> Shr<&'a isize> for &'b Ipv6Address {
    type Output = Ipv6Address;
    fn shr(self, rhs: &'a isize) -> Self::Output {
        Ipv6Address(self.0.shr(rhs))
    }
}

impl Not for Ipv6Address {
    type Output = Ipv6Address;

    fn not(self) -> Self::Output {
        Ipv6Address(self.0.not())
    }
}

impl<'a> Not for &'a Ipv6Address {
    type Output = Ipv6Address;

    fn not(self) -> Self::Output {
        Ipv6Address(self.0.not())
    }
}

impl LowerHex for Ipv6Address {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        <u128 as LowerHex>::fmt(&self.0, f)
    }
}

impl UpperHex for Ipv6Address {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        <u128 as UpperHex>::fmt(&self.0, f)
    }
}

impl Binary for Ipv6Address {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        <u128 as Binary>::fmt(&self.0, f)
    }
}

impl From<u128> for Ipv6Address {
    fn from(value: u128) -> Self {
        Ipv6Address(value)
    }
}

impl<'a> From<&'a u128> for Ipv6Address {
    fn from(value: &'a u128) -> Self {
        Ipv6Address(*value)
    }
}

impl From<Ipv6Mask> for Ipv6Address {
    fn from(value: Ipv6Mask) -> Self {
        Ipv6Address(value.into())
    }
}

impl<'a> From<&'a Ipv6Mask> for Ipv6Address {
    fn from(value: &'a Ipv6Mask) -> Self {
        Ipv6Address(value.into())
    }
}

impl From<Ipv6Address> for u128 {
    fn from(ip: Ipv6Address) -> Self {
        ip.0
    }
}

impl<'a> From<&'a Ipv6Address> for u128 {
    fn from(ip: &'a Ipv6Address) -> Self {
        ip.0
    }
}
