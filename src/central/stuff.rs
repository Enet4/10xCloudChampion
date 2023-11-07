//! Module containing an assortment of base data structures
//! for the game.
use std::fmt;

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
    pub const fn nothing() -> Self {
        Self {
            money: Money(0),
            base_ops: Ops(0),
            super_ops: Ops(0),
            epic_ops: Ops(0),
            awesome_ops: Ops(0),
        }
    }

    pub const fn money(money: Money) -> Self {
        Self {
            money,
            base_ops: Ops(0),
            super_ops: Ops(0),
            epic_ops: Ops(0),
            awesome_ops: Ops(0),
        }
    }
    pub const fn cents(money_cents: i64) -> Self {
        Self::money(Money::cents(money_cents))
    }

    pub const fn dollars(money_cents: i64) -> Self {
        Self::money(Money::dollars(money_cents))
    }

    pub const fn base_ops(base_ops: i32) -> Self {
        Self {
            money: Money(0),
            base_ops: Ops(base_ops as i64),
            super_ops: Ops(0),
            epic_ops: Ops(0),
            awesome_ops: Ops(0),
        }
    }

    pub const fn super_ops(super_ops: i32) -> Self {
        Self {
            money: Money(0),
            base_ops: Ops(0),
            super_ops: Ops(super_ops as i64),
            epic_ops: Ops(0),
            awesome_ops: Ops(0),
        }
    }

    pub const fn epic_ops(epic_ops: i32) -> Self {
        Self {
            money: Money(0),
            base_ops: Ops(0),
            super_ops: Ops(0),
            epic_ops: Ops(epic_ops as i64),
            awesome_ops: Ops(0),
        }
    }

    pub const fn awesome_ops(awesome_ops: i32) -> Self {
        Self {
            money: Money(0),
            base_ops: Ops(0),
            super_ops: Ops(0),
            epic_ops: Ops(0),
            awesome_ops: Ops(awesome_ops as i64),
        }
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

impl fmt::Display for Cost {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut some = false;
        if self.money.0 != 0 {
            write!(f, "{}", self.money)?;
            some = true;
        }
        if self.base_ops != Ops(0) {
            if some {
                f.write_str(" + ")?;
            }
            write!(f, "{} base ops", self.base_ops)?;
            some = true;
        }
        if self.super_ops != Ops(0) {
            if some {
                f.write_str(" + ")?;
            }
            write!(f, "{} super ops", self.super_ops)?;
        }
        Ok(())
    }
}

/// Money, with precision down to the 1000th of a cent.
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
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
    pub const fn into_cent(self) -> Self {
        Self(self.0 / 1_000 * 1_000)
    }

    /// discard the decimal part
    #[inline]
    pub const fn into_dollars(self) -> Self {
        Self(self.0 / 100_000 * 100_000)
    }
}

impl std::ops::Add for Money {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Money(self.0 + rhs.0)
    }
}

impl std::ops::Sub for Money {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Money(self.0 - rhs.0)
    }
}

impl std::ops::Mul<i32> for Money {
    type Output = Self;

    fn mul(self, rhs: i32) -> Self::Output {
        Money(self.0 * rhs as i64)
    }
}

impl fmt::Display for Money {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let dollars = self.0 / 100_000;
        let millicents = self.0 % 100_000;
        if millicents == 0 {
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

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct Ops(pub i64);

impl From<i32> for Ops {
    fn from(i: i32) -> Self {
        Ops(i as i64)
    }
}

impl From<i64> for Ops {
    fn from(i: i64) -> Self {
        Ops(i)
    }
}

impl std::ops::Add for Ops {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Ops(self.0 + rhs.0)
    }
}

impl std::ops::Sub for Ops {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Ops(self.0 - rhs.0)
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

/// A memory amounts
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct Memory(i64);

impl Memory {
    pub fn bytes(bytes: i64) -> Self {
        Self(bytes)
    }

    pub fn kb(kb: i64) -> Self {
        Self(kb * 1_000)
    }

    pub fn mb(mb: i64) -> Self {
        Self(mb * 1_000_000)
    }

    pub fn gb(gb: i64) -> Self {
        Self(gb * 1_000_000_000)
    }
}

impl From<i32> for Memory {
    fn from(i: i32) -> Self {
        Memory(i as i64)
    }
}

impl From<i64> for Memory {
    fn from(i: i64) -> Self {
        Memory(i)
    }
}

impl std::ops::Add for Memory {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Memory(self.0 + rhs.0)
    }
}

impl std::ops::Sub for Memory {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Memory(self.0 - rhs.0)
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

impl fmt::Display for Memory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // choose precision and unit based on size
        if self.0 >= 10_000_000_000 {
            // >= 10G, prefer GB
            let gb = self.0 / 1_000_000_000;
            let mb = (self.0 % 1_000_000_000) / 1_000_000;
            if mb == 0 {
                write!(f, "{}GB", Separating(gb))
            } else {
                write!(f, "{}.{:02}GB", Separating(gb), mb)
            }
        } else if self.0 >= 10_000_000 {
            // >= 10M, prefer MB
            let mb = self.0 / 1_000_000;
            let kb = (self.0 % 1_000_000) / 1_000;
            if kb == 0 {
                write!(f, "{}MB", Separating(mb))
            } else {
                write!(f, "{}.{:02}MB", Separating(mb), kb)
            }
        } else if self.0 >= 10_000 {
            // >= 10k, prefer KB
            let kb = self.0 / 1_000;
            let b = self.0 % 1_000;
            if b == 0 {
                write!(f, "{}KB", Separating(kb))
            } else {
                write!(f, "{}.{:02}KB", Separating(kb), b)
            }
        } else {
            write!(f, "{}B", Separating(self.0))
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
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
}

#[cfg(test)]
mod tests {
    use crate::Money;

    #[test]
    fn test_money() {
        let money = Money::cents(123_456_789);
        assert_eq!(money.into_cent(), Money::cents(123_456_789));
        assert_eq!(money.into_dollars(), Money::dollars(1_234_567));
        assert_eq!(money.to_string(), "$1\u{2006}234\u{2006}567.89");

        let money2 = Money::dollars(9_000);
        assert_eq!(money2.into_cent(), Money::dollars(9_000));
        assert_eq!(money2.into_dollars(), Money::dollars(9_000));
        assert_eq!(money2.to_string(), "$9\u{2006}000");

        let money3 = Money::millicents(5);
        assert_eq!(money3.into_cent(), Money::cents(0));
        assert_eq!(money3.into_dollars(), Money::dollars(0));
        assert_eq!(money3.to_string(), "$0.00005");

        let money4 = money2 + money3;
        assert_eq!(money4.to_string(), "$9\u{2006}000.00005");
    }
}
