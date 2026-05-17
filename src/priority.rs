#[derive(Clone, Debug, Eq, Hash, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Priority(u8);

impl Priority {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn from(priority: u8) -> Self {
        Self(priority)
    }

    #[must_use]
    pub fn lowest() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn is_lowest(&self) -> bool {
        self == &Self::lowest()
    }
}

impl Default for Priority {
    fn default() -> Self {
        26.into()
    }
}

impl From<Priority> for char {
    fn from(priority: Priority) -> Self {
        (priority.0 + b'A') as char
    }
}

impl TryFrom<char> for Priority {
    type Error = crate::Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        let c = value.to_ascii_uppercase() as u8;

        if c.is_ascii_uppercase() {
            Ok(Self(c - b'A'))
        } else {
            Err(crate::Error::InvalidPriority(value))
        }
    }
}

impl std::cmp::PartialEq<char> for Priority {
    fn eq(&self, other: &char) -> bool {
        let c = char::from(self.clone());

        c.eq_ignore_ascii_case(other)
    }
}

impl From<Priority> for u8 {
    fn from(priority: Priority) -> Self {
        priority.0
    }
}

impl From<u8> for Priority {
    fn from(c: u8) -> Self {
        Self::from(c)
    }
}

impl std::cmp::PartialEq<u8> for Priority {
    fn eq(&self, other: &u8) -> bool {
        self.0 == *other
    }
}

impl std::fmt::Display for Priority {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_lowest() {
            Ok(())
        } else {
            write!(f, "{}", (self.0 + b'A') as char)
        }
    }
}

impl std::cmp::PartialOrd for Priority {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl std::cmp::Ord for Priority {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.cmp(&other.0).reverse()
    }
}

impl std::ops::Add<u8> for Priority {
    type Output = Priority;

    fn add(self, rhs: u8) -> Self::Output {
        Self(self.0.saturating_sub(rhs))
    }
}

impl std::ops::AddAssign<u8> for Priority {
    fn add_assign(&mut self, rhs: u8) {
        self.0 = self.0.saturating_sub(rhs);
    }
}

#[allow(clippy::suspicious_arithmetic_impl)]
impl std::ops::Sub<u8> for Priority {
    type Output = Priority;

    fn sub(self, rhs: u8) -> Self::Output {
        Self(u8::min(self.0 + rhs, 25))
    }
}

#[allow(clippy::suspicious_op_assign_impl)]
impl std::ops::SubAssign<u8> for Priority {
    fn sub_assign(&mut self, rhs: u8) {
        self.0 = u8::min(self.0 + rhs, 25);
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn from_char() {
        assert_eq!(crate::Priority::from(1), 'B');
    }

    #[test]
    fn from_u8() {
        assert_eq!(crate::Priority::from(1), 1);
    }

    #[test]
    fn lowest() {
        let priority = crate::Priority::default();

        assert!(priority.is_lowest());
        assert_eq!(priority, 26);
    }

    #[test]
    fn display() {
        let priority = crate::Priority::from(1);
        assert_eq!(priority.to_string(), "B");

        let priority = crate::Priority::default();
        assert_eq!(priority.to_string(), "");
    }

    #[test]
    fn ord() {
        let a = crate::Priority::from(1);
        let b = crate::Priority::from(2);

        assert!(a > b);
    }

    #[test]
    fn eq() {
        let a = crate::Priority::from(0);

        assert_eq!(a, 'a');
        assert_eq!(a, 'A');
    }

    #[test]
    fn add() {
        let mut b = crate::Priority::from(1);
        assert_eq!(b, 'b');

        b += 1;
        assert_eq!(b, 'a');

        assert_eq!(b + 1, 'a');
    }

    #[test]
    fn sub() {
        let mut y = crate::Priority::from(24);
        assert_eq!(y, 'y');

        y -= 1;
        assert_eq!(y, 'z');

        assert_eq!(y - 1, 'z');
    }
}
