#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Recurrence {
    pub num: i64,
    pub period: super::Period,
    #[cfg_attr(feature = "serde", serde(default))]
    pub strict: bool,
}

impl std::str::FromStr for Recurrence {
    type Err = crate::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut s = String::from(s);

        let strict = if s.get(0..1) == Some("+") {
            s.remove(0);
            true
        } else {
            false
        };

        let period = match s.pop() {
            Some(c) => super::Period::from_str(&c.to_string())?,
            None => return Err(crate::Error::InvalidRecurrence(s.to_string())),
        };

        let num = s
            .parse()
            .map_err(|_| crate::Error::InvalidRecurrence(s.to_string()))?;

        Ok(Self {
            num,
            period,
            strict,
        })
    }
}

impl std::fmt::Display for Recurrence {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.strict {
            f.write_str("+")?;
        }

        f.write_str(&format!("{}{}", self.num, self.period))?;

        Ok(())
    }
}

impl std::ops::Add<jiff::civil::Date> for Recurrence {
    type Output = jiff::civil::Date;

    fn add(self, rhs: Self::Output) -> Self::Output {
        use super::Period::{self, *};
        use jiff::ToSpan as _;

        let delta_months = match self.period {
            #[allow(clippy::suspicious_arithmetic_impl)]
            Year => 12 * self.num as u32,
            Month => self.num as u32,
            Week => return rhs + self.num.weeks(),
            Day => return rhs + self.num.days(),
        };

        let mut y = rhs.year() as i32;
        let mut m = rhs.month() as u32;
        let mut d = rhs.day() as u32;

        // Semantics taken from
        //  https://github.com/dbeniamine/todo.txt-vim/blob/259125d9efe93f69582f50ef68c17e20fd1e963a/autoload/todo.vim#L531-L538
        let was_last_day = d == Period::days_in_month(m, y);

        m += delta_months;
        y += ((m - 1) / 12) as i32;
        m = (m - 1) % 12 + 1;
        if was_last_day || d > Period::days_in_month(m, y) {
            d = Period::days_in_month(m, y);
        }

        jiff::civil::date(y as i16, m as i8, d as i8)
    }
}
