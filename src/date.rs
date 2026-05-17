pub use chrono::NaiveDate as Date;

#[must_use]
pub fn today() -> Date {
    chrono::offset::Local::now().date_naive()
}

pub fn parse(s: &str) -> Option<Date> {
    if let Ok(date) = crate::Date::parse_from_str(s, "%Y-%m-%d") {
        return Some(date);
    }

    static REGEX: std::sync::OnceLock<regex::Regex> = std::sync::OnceLock::new();

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

        let delta = chrono::TimeDelta::days(days);

        return Some(today() + delta);
    }

    None
}
