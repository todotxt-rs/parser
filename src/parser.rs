use nom::Parser as _;
use nom::bytes::complete::{tag, take};
use nom::combinator::{complete, opt, rest};
use std::collections::BTreeMap;

macro_rules! return_error (
    ($num:expr) => {
        return Err(
            nom::Err::Error(
                nom::error::make_error(
                    "",
                    nom::error::ErrorKind::Tag
                )
            )
        )
    }
);

fn date(input: &str) -> nom::IResult<&str, crate::Date> {
    let (input, (year, _, month, _, day, _)) = (
        take(4usize),
        tag("-"),
        take(2usize),
        tag("-"),
        take(2usize),
        tag(" "),
    )
        .parse(input)?;

    let year = match year.parse() {
        Ok(year) => year,
        Err(_) => return_error!(1),
    };

    let month = match month.parse() {
        Ok(month) => month,
        Err(_) => return_error!(2),
    };

    let day = match day.parse() {
        Ok(day) => day,
        Err(_) => return_error!(3),
    };

    let date = match crate::Date::from_ymd_opt(year, month, day) {
        Some(date) => date,
        None => return_error!(4),
    };

    Ok((input, date))
}

fn priority(input: &str) -> nom::IResult<&str, crate::Priority> {
    let (input, (_, priority, ..)) = (tag("("), take(1usize), tag(") ")).parse(input)?;

    let priority = match priority.chars().next() {
        Some(c) => c.try_into().unwrap_or_default(),
        None => crate::Priority::default(),
    };

    Ok((input, priority))
}

fn get_tags(regex: &::regex::Regex, subject: &str) -> Vec<String> {
    let mut tags = regex
        .captures_iter(subject)
        .map(|x| x["tag"].to_string())
        .filter(|x| !x.is_empty())
        .collect::<Vec<_>>();

    tags.sort();
    tags.dedup();

    tags
}

macro_rules! regex_tags_shared {
    () => {
        "(?P<space>^|[\\s]){}(?P<tag>[\\w\\\\-]+)"
    };
}

fn get_contexts(subject: &str) -> Vec<String> {
    static REGEX: std::sync::LazyLock<regex::Regex> = std::sync::LazyLock::new(|| {
        regex::Regex::new(&format!(regex_tags_shared!(), crate::task::Tag::Context)).unwrap()
    });

    get_tags(&REGEX, subject)
}

fn get_projects(subject: &str) -> Vec<String> {
    static REGEX: std::sync::LazyLock<regex::Regex> = std::sync::LazyLock::new(|| {
        regex::Regex::new(&format!(
            regex_tags_shared!(),
            regex::escape(&crate::task::Tag::Project.to_string())
        ))
        .unwrap()
    });

    get_tags(&REGEX, subject)
}

fn get_hashtags(subject: &str) -> Vec<String> {
    static REGEX: std::sync::LazyLock<regex::Regex> = std::sync::LazyLock::new(|| {
        regex::Regex::new(&format!(regex_tags_shared!(), crate::task::Tag::Hashtag)).unwrap()
    });

    get_tags(&REGEX, subject)
}

fn get_keywords(subject: &str) -> (String, BTreeMap<String, String>) {
    static REGEX: std::sync::LazyLock<regex::Regex> = std::sync::LazyLock::new(|| {
        regex::Regex::new(r"(\s+|^)(?P<key>[^\s]+?):(?P<value>[^\s]+)").unwrap()
    });

    let mut tags = BTreeMap::new();

    let new_subject = REGEX.replace_all(subject, |caps: &regex::Captures<'_>| {
        let key = caps.name("key").unwrap().as_str();
        let value = caps.name("value").unwrap().as_str();

        if value.starts_with('/') {
            format!(" {key}:{value}")
        } else {
            tags.insert(key.to_string(), value.to_string());

            String::new()
        }
    });

    (new_subject.trim().to_string(), tags)
}

fn parse(input: &str) -> nom::IResult<&str, crate::task::Simple> {
    let (input, (finished, priority, finish_date, create_date, rest)) = (
        opt(complete(tag("x "))),
        opt(complete(priority)),
        opt(complete(date)),
        opt(complete(date)),
        rest,
    )
        .parse(input)?;

    let mut task = crate::task::Simple {
        priority: priority.unwrap_or_default(),
        create_date: create_date.or(finish_date),
        finish_date: create_date.and(finish_date),
        finished: finished.is_some(),
        contexts: get_contexts(rest),
        projects: get_projects(rest),
        hashtags: get_hashtags(rest),

        ..crate::task::Simple::default()
    };

    let (subject, mut tags) = get_keywords(rest);
    task.subject = subject;

    if let Some(due) = tags.remove("due") {
        task.due_date = crate::Date::parse_from_str(&due, "%Y-%m-%d").ok();
    }

    if let Some(t) = tags.remove("t") {
        task.threshold_date = crate::Date::parse_from_str(&t, "%Y-%m-%d").ok();
    }

    task.tags = tags;

    Ok((input, task))
}

#[must_use]
pub fn task(line: &str) -> crate::task::Simple {
    parse(line).unwrap().1
}
