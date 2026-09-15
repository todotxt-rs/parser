#[cfg(all(test, feature = "extended"))]
mod test {
    use std::str::FromStr;
    use todo_txt::task::Recurrence;

    #[test]
    fn from_invalid() {
        use std::str::FromStr;

        for p in &["1", "+1"] {
            assert!(Recurrence::from_str(p).is_err());
        }
    }

    #[test]
    fn from_valid() {
        use std::str::FromStr;

        for p in &["+4y", "+4m", "+4w", "+4d", "12m"] {
            assert!(Recurrence::from_str(p).is_ok());
        }
    }

    #[test]
    fn add_years() {
        let current =
            Recurrence::from_str("4y").unwrap() + jiff::civil::date(1998, 1, 1);
        let expected = jiff::civil::date(2002, 1, 1);

        assert_eq!(current, expected);
    }

    #[test]
    fn add_months() {
        let current =
            Recurrence::from_str("4m").unwrap() + jiff::civil::date(1999, 11, 1);
        let expected = jiff::civil::date(2000, 3, 1);

        assert_eq!(current, expected);
    }

    #[test]
    fn add_months_extra() {
        let current =
            Recurrence::from_str("2m").unwrap() + jiff::civil::date(2009, 12, 31);
        let expected = jiff::civil::date(2010, 2, 28);

        assert_eq!(current, expected);
    }

    #[test]
    fn add_months_sticky() {
        let current =
            Recurrence::from_str("3m").unwrap() + jiff::civil::date(2010, 2, 28);
        let expected = jiff::civil::date(2010, 5, 31);

        assert_eq!(current, expected);
    }
}
