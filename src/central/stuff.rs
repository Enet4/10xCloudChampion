//! Module containing an assortment of base data structures
//! for the game.
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::display::Separating;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Cost {
    pub money: Money,
    /// operations from the base service
    pub base_ops: Ops,
    /// operations from the cool service
    pub super_ops: Ops,
    /// operations from the epic service
    pub epic_ops: Ops,
    /// operations from the awesome service
    pub awesome_ops: Ops,
}

impl Cost {
    /// Equivalent to `default`, but const
    #[inline]
    pub const fn nothing() -> Self {
        Self {
            money: Money(0),
            base_ops: Ops(0),
            super_ops: Ops(0),
            epic_ops: Ops(0),
            awesome_ops: Ops(0),
        }
    }

    #[inline]
    pub const fn money(money: Money) -> Self {
        Self {
            money,
            base_ops: Ops(0),
            super_ops: Ops(0),
            epic_ops: Ops(0),
            awesome_ops: Ops(0),
        }
    }

    #[inline]
    pub const fn cents(money_cents: i64) -> Self {
        Self::money(Money::cents(money_cents))
    }

    #[inline]
    pub const fn dollars(money_dollars: i64) -> Self {
        Self::money(Money::dollars(money_dollars))
    }

    #[inline]
    pub const fn base_ops(base_ops: i32) -> Self {
        Self {
            money: Money(0),
            base_ops: Ops(base_ops as i64),
            super_ops: Ops(0),
            epic_ops: Ops(0),
            awesome_ops: Ops(0),
        }
    }

    #[inline]
    pub const fn super_ops(super_ops: i32) -> Self {
        Self {
            money: Money(0),
            base_ops: Ops(0),
            super_ops: Ops(super_ops as i64),
            epic_ops: Ops(0),
            awesome_ops: Ops(0),
        }
    }

    #[inline]
    pub const fn epic_ops(epic_ops: i32) -> Self {
        Self {
            money: Money(0),
            base_ops: Ops(0),
            super_ops: Ops(0),
            epic_ops: Ops(epic_ops as i64),
            awesome_ops: Ops(0),
        }
    }

    #[inline]
    pub const fn awesome_ops(awesome_ops: i32) -> Self {
        Self {
            money: Money(0),
            base_ops: Ops(0),
            super_ops: Ops(0),
            epic_ops: Ops(0),
            awesome_ops: Ops(awesome_ops as i64),
        }
    }

    /// Const method for adding costs together
    #[inline]
    pub const fn and(self, cost: Cost) -> Self {
        Self {
            money: self.money.plus(cost.money),
            base_ops: Ops(self.base_ops.0 + cost.base_ops.0),
            super_ops: Ops(self.super_ops.0 + cost.super_ops.0),
            epic_ops: Ops(self.epic_ops.0 + cost.epic_ops.0),
            awesome_ops: Ops(self.awesome_ops.0 + cost.awesome_ops.0),
        }
    }

    pub fn is_nothing(&self) -> bool {
        self.money == Money(0)
            && self.base_ops == Ops(0)
            && self.super_ops == Ops(0)
            && self.epic_ops == Ops(0)
            && self.awesome_ops == Ops(0)
    }
}

impl std::ops::Add for Cost {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            money: self.money + rhs.money,
            base_ops: self.base_ops + rhs.base_ops,
            super_ops: self.super_ops + rhs.super_ops,
            epic_ops: self.epic_ops + rhs.epic_ops,
            awesome_ops: self.awesome_ops + rhs.awesome_ops,
        }
    }
}

impl std::iter::Sum for Cost {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::nothing(), |a, b| a + b)
    }
}

impl fmt::Display for Cost {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut some = false;
        if self.awesome_ops != Ops(0) {
            write!(f, "{} awesome ops", Compact(self.awesome_ops))?;
            some = true;
        }
        if self.epic_ops != Ops(0) {
            if some {
                f.write_str(" + ")?;
            }
            write!(f, "{} epic ops", Compact(self.epic_ops))?;
            some = true;
        }
        if self.super_ops != Ops(0) {
            if some {
                f.write_str(" + ")?;
            }
            write!(f, "{} super ops", Compact(self.super_ops))?;
            some = true;
        }
        if self.base_ops != Ops(0) {
            if some {
                f.write_str(" + ")?;
            }
            write!(f, "{} base ops", Compact(self.base_ops))?;
            some = true;
        }
        if self.money.0 != 0 {
            if some {
                f.write_str(" + ")?;
            }
            write!(f, "{}", self.money)?;
        }
        Ok(())
    }
}

/// Money, with precision down to the 1000th of a cent.
#[derive(
    Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
)]
#[serde(transparent)]
pub struct Money(i64);

impl Money {
    #[inline]
    pub const fn zero() -> Self {
        Money(0)
    }

    #[inline]
    pub const fn millicents(amount: i64) -> Self {
        Self(amount)
    }

    #[inline]
    pub const fn dec_cents(amount: i64) -> Self {
        Self(amount * 100)
    }

    #[inline]
    pub const fn cents(amount: i64) -> Self {
        Self(amount * 1_000)
    }

    #[inline]
    pub const fn dollars(amount: i64) -> Self {
        Self(amount * 100_000)
    }

    /// discard the decimals of cents
    #[inline]
    pub const fn into_cent_precision(self) -> Self {
        Self::cents(self.to_cents())
    }

    /// discard the decimal part
    #[inline]
    pub const fn into_dollar_precision(self) -> Self {
        Self::dollars(self.to_dollars())
    }

    #[inline]
    pub const fn to_cents(self) -> i64 {
        self.0 / 1_000
    }

    #[inline]
    pub const fn to_dollars(self) -> i64 {
        self.0 / 100_000
    }

    #[inline]
    pub const fn to_millicents(self) -> i64 {
        self.0
    }

    /// const method to add two money amounts together
    pub const fn plus(self, other: Self) -> Self {
        Self(self.0 + other.0)
    }
}

impl std::ops::Add for Money {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        self.plus(rhs)
    }
}

impl std::ops::AddAssign for Money {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
    }
}

impl std::ops::Sub for Money {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Money(self.0 - rhs.0)
    }
}

impl std::ops::SubAssign for Money {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
    }
}

impl std::ops::Mul<i32> for Money {
    type Output = Self;

    fn mul(self, rhs: i32) -> Self::Output {
        Money(self.0 * rhs as i64)
    }
}

impl std::ops::Mul<f64> for Money {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Money((self.0 as f64 * rhs) as i64)
    }
}

impl std::iter::Sum for Money {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::zero(), |a, b| a + b)
    }
}

impl fmt::Display for Money {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if *self < Money::zero() {
            f.write_str("-")?;
            return (Money::zero() - *self).fmt(f);
        }

        let dollars = self.0 / 100_000;
        let millicents = self.0 % 100_000;

        if dollars >= 1_000_000 && dollars % 100_000 == 0 && millicents == 0 {
            // no fraction smaller than $100_000, show in millions
            let mdollars = dollars / 1_000_000;
            let rest_dollars = dollars % 1_000_000 / 100_000;
            if rest_dollars == 0 {
                write!(f, "${}M", Separating(mdollars))
            } else {
                write!(f, "${}.{:01}M", Separating(mdollars), rest_dollars)
            }
        } else if millicents == 0 && dollars > 1_000 && dollars % 100 == 0 {
            // no fraction smaller than $100, show in thousands
            let kdollars = dollars / 1_000;
            let rest_dollars = dollars % 1_000 / 100;
            if rest_dollars == 0 {
                write!(f, "${}k", Separating(kdollars))
            } else {
                write!(f, "${}.{:01}k", Separating(kdollars), rest_dollars)
            }
        } else if millicents == 0 {
            write!(f, "${}", Separating(dollars))
        } else {
            if self.0 % 1_000 == 0 {
                // no fraction smaller than cent
                return write!(f, "${}.{:02}", Separating(dollars), millicents / 1_000);
            }
            if self.0 % 100 == 0 {
                // no fraction smaller than 10th of a cent
                return write!(f, "${}.{:03}", Separating(dollars), millicents / 100);
            }
            if self.0 % 10 == 0 {
                // no fraction smaller than 100th of a cent
                return write!(f, "${}.{:04}", Separating(dollars), millicents / 10);
            }
            // maximum precision
            write!(f, "${}.{:05}", Separating(dollars), millicents)
        }
    }
}

/// A count of cloud service operations
#[derive(
    Debug, Default, Copy, Clone, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize,
)]
#[serde(transparent)]
pub struct Ops(pub i64);

impl From<i32> for Ops {
    #[inline]
    fn from(i: i32) -> Self {
        Ops(i as i64)
    }
}

impl From<i64> for Ops {
    #[inline]
    fn from(i: i64) -> Self {
        Ops(i)
    }
}

impl std::ops::Add for Ops {
    type Output = Self;

    #[inline]
    fn add(mut self, rhs: Self) -> Self::Output {
        self += rhs;
        self
    }
}

impl std::ops::AddAssign for Ops {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
    }
}

impl std::ops::Sub for Ops {
    type Output = Self;

    fn sub(mut self, rhs: Self) -> Self::Output {
        self -= rhs;
        self
    }
}

impl std::ops::SubAssign for Ops {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
    }
}

impl std::ops::Mul<i32> for Ops {
    type Output = Self;

    fn mul(self, rhs: i32) -> Self::Output {
        Ops(self.0 * rhs as i64)
    }
}

impl fmt::Display for Ops {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", Separating(self.0))
    }
}

/// A way to present operation counts in a compact way
/// (e.g. in cards)
#[derive(Debug)]
pub struct Compact(Ops);

impl fmt::Display for Compact {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let ops = self.0;
        if ops.0 % 1_000_000_000_000 == 0 {
            write!(f, "{}T", Separating(ops.0 / 1_000_000_000_000))
        } else if ops.0 % 1_000_000_000 == 0 {
            write!(f, "{}G", Separating(ops.0 / 1_000_000_000))
        } else if ops.0 % 1_000_000 == 0 {
            write!(f, "{}M", Separating(ops.0 / 1_000_000))
        } else if ops.0 % 1_000 == 0 {
            write!(f, "{}k", Separating(ops.0 / 1_000))
        } else {
            write!(f, "{}", Separating(ops.0))
        }
    }
}

impl Ops {
    pub fn compact(self) -> Compact {
        Compact(self)
    }
}

/// A memory amounts
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Memory(i64);

impl Memory {
    #[inline]
    pub const fn zero() -> Self {
        Self(0)
    }

    #[inline]
    pub const fn bytes(bytes: i64) -> Self {
        Self(bytes)
    }

    #[inline]
    pub const fn kb(kb: i64) -> Self {
        Self(kb * 1_000)
    }

    #[inline]
    pub const fn mb(mb: i64) -> Self {
        Self(mb * 1_000_000)
    }

    #[inline]
    pub const fn gb(gb: i64) -> Self {
        Self(gb * 1_000_000_000)
    }

    #[inline]
    pub const fn tb(tb: i64) -> Self {
        Self(tb * 1_000_000_000_000)
    }

    #[inline]
    pub fn ratio(self, other: Self) -> f32 {
        self.0 as f32 / other.0 as f32
    }
}

impl From<i32> for Memory {
    #[inline]
    fn from(i: i32) -> Self {
        Memory(i as i64)
    }
}

impl From<i64> for Memory {
    #[inline]
    fn from(i: i64) -> Self {
        Memory(i)
    }
}

impl std::ops::Add for Memory {
    type Output = Self;

    #[inline]
    fn add(mut self, rhs: Self) -> Self::Output {
        self += rhs;
        self
    }
}

impl std::ops::AddAssign for Memory {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
    }
}

impl std::ops::Sub for Memory {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Memory(self.0 - rhs.0)
    }
}

impl std::ops::SubAssign for Memory {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 = self.0.saturating_sub(rhs.0);
    }
}

impl std::ops::Mul<i32> for Memory {
    type Output = Self;

    fn mul(self, rhs: i32) -> Self::Output {
        Memory(self.0 * rhs as i64)
    }
}

impl std::ops::Mul<f32> for Memory {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Memory((self.0 as f32 * rhs) as i64)
    }
}

impl std::ops::Mul<f64> for Memory {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Memory((self.0 as f64 * rhs) as i64)
    }
}

impl std::iter::Sum for Memory {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::bytes(0), |a, b| a + b)
    }
}

impl fmt::Display for Memory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // choose precision and unit based on size
        if self.0 >= 10_000_000_000_000 {
            // >= 10T, prefer TB
            let tb = self.0 / 1_000_000_000_000;
            let gb = (self.0 % 1_000_000_000_000) / 1_000_000_000;
            if gb == 0 {
                write!(f, "{}TB", Separating(tb))
            } else {
                write!(f, "{}.{:02}TB", Separating(tb), gb / 10)
            }
        } else if self.0 >= 10_000_000_000 {
            // >= 10G, prefer GB
            let gb = self.0 / 1_000_000_000;
            let mb = (self.0 % 1_000_000_000) / 1_000_000;
            if mb == 0 {
                write!(f, "{}GB", Separating(gb))
            } else {
                write!(f, "{}.{:02}GB", Separating(gb), mb / 10)
            }
        } else if self.0 >= 10_000_000 {
            // >= 10M, prefer MB
            let mb = self.0 / 1_000_000;
            let kb = (self.0 % 1_000_000) / 1_000;
            if kb == 0 {
                write!(f, "{}MB", Separating(mb))
            } else {
                write!(f, "{}.{:02}MB", Separating(mb), kb / 10)
            }
        } else if self.0 >= 10_000 {
            // >= 10k, prefer KB
            let kb = self.0 / 1_000;
            let b = self.0 % 1_000;
            if b == 0 {
                write!(f, "{}KB", Separating(kb))
            } else {
                write!(f, "{}.{:02}KB", Separating(kb), b / 10)
            }
        } else {
            write!(f, "{}B", Separating(self.0))
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub enum ServiceKind {
    Base,
    Super,
    Epic,
    Awesome,
}

impl fmt::Display for ServiceKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Base => write!(f, "Base"),
            Self::Super => write!(f, "Super"),
            Self::Epic => write!(f, "Epic"),
            Self::Awesome => write!(f, "Awesome"),
        }
    }
}

impl ServiceKind {
    pub fn from_code(code: u8) -> Option<Self> {
        match code {
            0 => Some(Self::Base),
            1 => Some(Self::Super),
            2 => Some(Self::Epic),
            3 => Some(Self::Awesome),
            _ => None,
        }
    }

    pub fn to_code(self) -> u8 {
        match self {
            Self::Base => 0,
            Self::Super => 1,
            Self::Epic => 2,
            Self::Awesome => 3,
        }
    }

    /**
     * The memory required per individual operation of this service tier.
     */
    #[inline]
    pub(crate) fn mem_required(&self) -> Memory {
        match self {
            Self::Base => Memory::kb(512),
            Self::Super => Memory::kb(768),
            Self::Epic => Memory::mb(1),
            Self::Awesome => Memory::mb(4),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Money, Ops};

    #[test]
    fn test_money() {
        let money = Money::cents(123_456_789);
        assert_eq!(money, Money::dollars(1_234_567) + Money::cents(89));
        assert_eq!(money, Money::millicents(123456_789_000));
        assert_eq!(money.into_cent_precision(), Money::cents(123_456_789));
        assert_eq!(money.to_dollars(), 1_234_567);
        assert_eq!(money.to_cents(), 123_456_789);
        assert_eq!(money.into_dollar_precision(), Money::dollars(1_234_567));
        assert_eq!(money.into_cent_precision(), money);
        assert_eq!(money.to_string(), "$1\u{2006}234\u{2006}567.89");

        let money2 = Money::dollars(9_000);
        assert_eq!(money2, Money::cents(900_000));
        assert_eq!(money2, Money::millicents(900_000_000));
        assert_eq!(money2.into_cent_precision(), Money::dollars(9_000));
        assert_eq!(money2.into_dollar_precision(), money2);
        assert_eq!(money2.to_string(), "$9k");
        assert_eq!((money2 + Money::dollars(200)).to_string(), "$9.2k");

        let money3 = Money::millicents(5);
        assert_eq!(money3.to_dollars(), 0);
        assert_eq!(money3.to_cents(), 0);
        assert_eq!(money3.into_cent_precision(), Money::cents(0));
        assert_eq!(money3.into_dollar_precision(), Money::dollars(0));
        assert_eq!(money3.to_string(), "$0.00005");

        let mut money4 = money2 + money3;
        assert_eq!(money4.to_string(), "$9\u{2006}000.00005");

        money4 -= money2;
        assert_eq!(money4, money3);
    }

    #[test]
    fn test_ops() {
        let ops1 = Ops(10_000);
        assert_eq!(ops1.to_string(), "10\u{2006}000");
        assert_eq!(ops1.compact().to_string(), "10k");

        let ops2 = Ops(2_000_000);
        assert_eq!(ops2.to_string(), "2\u{2006}000\u{2006}000");
        assert_eq!(ops2.compact().to_string(), "2M");

        let mut ops3 = ops1 + ops2;
        assert_eq!(ops3.to_string(), "2\u{2006}010\u{2006}000");
        assert_eq!(ops3.compact().to_string(), "2\u{2006}010k");

        ops3 -= ops1;
        assert_eq!(ops3, ops2);
    }
}
