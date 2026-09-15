#[cfg(all(test, feature = "extended"))]
mod test {
    use todo_txt::task::Period::*;

    #[test]
    fn add_year() {
        let current = Year + jiff::civil::date(1999, 1, 1);
        let expected = jiff::civil::date(2000, 1, 1);

        assert_eq!(current, expected);
    }

    #[test]
    fn add_month() {
        let current = Month + jiff::civil::date(1999, 1, 1);
        let expected = jiff::civil::date(1999, 2, 1);

        assert_eq!(current, expected);
    }

    #[test]
    fn add_month_extra() {
        let current = Month + jiff::civil::date(1999, 12, 1);
        let expected = jiff::civil::date(2000, 1, 1);

        assert_eq!(current, expected);
    }

    #[test]
    fn add_week() {
        let current = Week + jiff::civil::date(1999, 1, 1);
        let expected = jiff::civil::date(1999, 1, 8);

        assert_eq!(current, expected);
    }

    #[test]
    fn add_day() {
        let current = Day + jiff::civil::date(1999, 1, 1);
        let expected = jiff::civil::date(1999, 1, 2);

        assert_eq!(current, expected);
    }

    #[test]
    fn add_day_extra() {
        let current = Day + jiff::civil::date(1999, 1, 31);
        let expected = jiff::civil::date(1999, 2, 1);

        assert_eq!(current, expected);

        let current = Day + jiff::civil::date(1999, 4, 30);
        let expected = jiff::civil::date(1999, 5, 1);

        assert_eq!(current, expected);

        let current = Day + jiff::civil::date(1999, 2, 28);
        let expected = jiff::civil::date(1999, 3, 1);

        assert_eq!(current, expected);

        let current = Day + jiff::civil::date(2000, 2, 28);
        let expected = jiff::civil::date(2000, 2, 29);

        assert_eq!(current, expected);

        let current = Day + jiff::civil::date(2000, 2, 29);
        let expected = jiff::civil::date(2000, 3, 1);

        assert_eq!(current, expected);
    }
}
