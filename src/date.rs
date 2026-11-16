pub use jiff::civil::Date;

#[must_use]
pub fn today() -> Date {
    jiff::ZonedDateTime::now().date()
}

pub fn parse(s: &str) -> Option<Date> {
    use jiff::ToSpan as _;

    static REGEX: std::sync::OnceLock<regex::Regex> = std::sync::OnceLock::new();

    if let Ok(date) = crate::Date::strptime("%Y-%m-%d", s) {
        return Some(date);
    }

    let regex = REGEX.get_or_init(|| {
        regex::Regex::new(
            "(?P<sign>\\+|-)?(?P<nth>\\d+)(?P<unit>m(onths?)?|d(ays?)?|w(eeks?)?|y(ears?)?)",
        )
        .unwrap()
    });

    if let Some(captures) = regex.captures(s) {
        let Ok(nth) = captures["nth"].parse() else {
            return None;
        };

        let mut days = match captures["unit"].chars().next() {
            Some('d') => nth,
            Some('w') => nth * 7,
            Some('m') => (nth as f32 * 30.5) as i64,
            Some('y') => (nth as f32 * 365.25) as i64,
            _ => return None,
        };

        let sign = captures.name("sign").as_ref().map(regex::Match::as_str);
        if sign == Some("-") {
            days *= -1;
        }

        let delta = days.days();

        return Some(today() + delta);
    }

    None
}
