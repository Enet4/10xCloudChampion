use std::fmt;

/// A formatting utility that adds a half-width space for each 3 digits
pub struct Separating(pub i64);

impl fmt::Display for Separating {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut n = self.0;
        if n == 0 {
            f.write_str("0")?;
            return Ok(());
        }
        if n < 0 {
            f.write_str("-")?;
            n = -n;
        }

        let mut parts = vec![];

        while n > 0 {
            let part = n % 1000;
            n /= 1000;
            parts.push(part);
        }

        // traverse in reverse order
        let mut first = true;
        for (i, part) in parts.iter().rev().enumerate() {
            if !first {
                // half-width space
                f.write_str("\u{2006}")?;
            }
            if i == 0 {
                write!(f, "{part}")?;
            } else {
                write!(f, "{part:03}")?;
            }
            first = false;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::Separating;

    #[test]
    fn test_separating() {
        assert_eq!(Separating(0).to_string(), "0",);

        assert_eq!(Separating(1).to_string(), "1",);

        assert_eq!(Separating(1_000).to_string(), "1 000",);

        assert_eq!(Separating(435).to_string(), "435",);

        assert_eq!(Separating(499_999_999).to_string(), "499 999 999",);

        assert_eq!(Separating(-45_300).to_string(), "-45 300",);
    }
}
