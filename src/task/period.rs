#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Period {
    Day,
    Week,
    Month,
    Year,
}

impl Period {
    fn is_leap_year(year: i32) -> bool {
        year % 4 == 0 && (year % 100 != 0 || year % 400 == 0)
    }

    pub fn days_in_month(month: u32, year: i32) -> u32 {
        if month == 2 {
            if Self::is_leap_year(year) { 29 } else { 28 }
        } else if [1, 3, 5, 7, 8, 10, 12].contains(&month) {
            31
        } else {
            30
        }
    }
}

impl std::str::FromStr for Period {
    type Err = crate::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Period::*;

        match s {
            "d" => Ok(Day),
            "w" => Ok(Week),
            "m" => Ok(Month),
            "y" => Ok(Year),
            _ => Err(crate::Error::InvalidPeriod(s.to_string())),
        }
    }
}

impl std::fmt::Display for Period {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Period::*;

        let s = match *self {
            Day => "d",
            Week => "w",
            Month => "m",
            Year => "y",
        };

        f.write_str(s)?;

        Ok(())
    }
}

impl std::ops::Add<chrono::NaiveDate> for Period {
    type Output = chrono::NaiveDate;

    fn add(self, rhs: Self::Output) -> Self::Output {
        let rec = super::Recurrence {
            num: 1,
            period: self,
            strict: true,
        };
        rec + rhs
    }
}
