#[cfg(test)]
mod test {
    use std::collections::BTreeMap;

    #[test]
    fn simple_task() {
        let line = "Email SoAndSo at soandso@example.com".to_string();
        let task = todo_txt::task::Simple {
            subject: "Email SoAndSo at soandso@example.com".to_string(),
            ..Default::default()
        };

        assert_eq!(todo_txt::parser::task(&line), task);
    }

    #[test]
    fn finished() {
        let line = "x done".to_string();
        let task = todo_txt::task::Simple {
            subject: "done".to_string(),
            finished: true,
            ..Default::default()
        };

        assert_eq!(todo_txt::parser::task(&line), task);
    }

    #[test]
    fn created() {
        let line = "x 2017-11-25 subject".to_string();
        let task = todo_txt::task::Simple {
            subject: "subject".to_string(),
            create_date: todo_txt::Date::from_ymd_opt(2017, 11, 25),
            finished: true,
            ..Default::default()
        };

        assert_eq!(todo_txt::parser::task(&line), task);
    }

    #[test]
    fn invalid_date() {
        let line = "2017-02-30 subject".to_string();
        let task = todo_txt::task::Simple {
            subject: "2017-02-30 subject".to_string(),

            ..Default::default()
        };

        assert_eq!(todo_txt::parser::task(&line), task);
    }

    #[test]
    fn completed() {
        let line = "x 2017-11-26 2017-11-25 subject".to_string();
        let task = todo_txt::task::Simple {
            subject: "subject".to_string(),
            create_date: todo_txt::Date::from_ymd_opt(2017, 11, 25),
            finish_date: todo_txt::Date::from_ymd_opt(2017, 11, 26),
            finished: true,
            ..Default::default()
        };

        assert_eq!(todo_txt::parser::task(&line), task);
    }

    #[test]
    fn priority() {
        let line = "x (A) 2017-11-26 2017-11-25 subject".to_string();
        let task = todo_txt::task::Simple {
            subject: "subject".to_string(),
            priority: 0.into(),
            create_date: todo_txt::Date::from_ymd_opt(2017, 11, 25),
            finish_date: todo_txt::Date::from_ymd_opt(2017, 11, 26),
            finished: true,
            ..Default::default()
        };

        assert_eq!(todo_txt::parser::task(&line), task);
    }

    #[test]
    fn contexts() {
        let line = "Email SoAndSo at soandso@example.com @context1 @context2".to_owned();
        let task = todo_txt::task::Simple {
            subject: "Email SoAndSo at soandso@example.com @context1 @context2".to_owned(),
            contexts: vec!["context1".to_owned(), "context2".to_owned()],
            ..Default::default()
        };

        assert_eq!(todo_txt::parser::task(&line), task);
    }

    #[test]
    fn deplucate_contexts() {
        let line = "Email SoAndSo at soandso@example.com @context1 @context2 @context1".to_owned();
        let task = todo_txt::task::Simple {
            subject: "Email SoAndSo at soandso@example.com @context1 @context2 @context1"
                .to_owned(),
            contexts: vec!["context1".to_owned(), "context2".to_owned()],
            ..Default::default()
        };

        assert_eq!(todo_txt::parser::task(&line), task);
    }

    #[test]
    fn projects() {
        let line = "Email SoAndSo at soandso@example.com +project1 +project1\\subject1 @context2"
            .to_owned();
        let task = todo_txt::task::Simple {
            subject: "Email SoAndSo at soandso@example.com +project1 +project1\\subject1 @context2"
                .to_owned(),
            contexts: vec!["context2".to_owned()],
            projects: vec!["project1".to_owned(), "project1\\subject1".to_owned()],
            ..Default::default()
        };

        assert_eq!(todo_txt::parser::task(&line), task);
    }

    #[test]
    fn empty_tag() {
        let line = "Email SoAndSo at soandso@example.com + @ #".to_string();
        let task = todo_txt::task::Simple {
            subject: "Email SoAndSo at soandso@example.com + @ #".to_string(),

            ..Default::default()
        };

        assert_eq!(todo_txt::parser::task(&line), task);
    }

    #[test]
    fn case_sensitive_tag() {
        let line = "Email SoAndSo at soandso@example.com +Project1".to_owned();
        let task = todo_txt::task::Simple {
            subject: "Email SoAndSo at soandso@example.com +Project1".to_owned(),
            projects: vec!["Project1".to_string()],

            ..Default::default()
        };

        assert_eq!(todo_txt::parser::task(&line), task);
    }

    #[test]
    fn start_with_tag() {
        let line = "+Project1".to_owned();
        let task = todo_txt::task::Simple {
            subject: "+Project1".to_owned(),
            projects: vec!["Project1".to_string()],

            ..Default::default()
        };

        assert_eq!(todo_txt::parser::task(&line), task);
    }

    #[test]
    fn url() {
        let line = "Participer à https://contributopia.org".to_string();
        let task = todo_txt::task::Simple {
            subject: line.clone(),

            ..Default::default()
        };

        assert_eq!(todo_txt::parser::task(&line), task);
    }

    #[test]
    fn hashtags() {
        let line = "Email SoAndSo at soandso@example.com +project1 #tag @context2".to_owned();
        let task = todo_txt::task::Simple {
            subject: "Email SoAndSo at soandso@example.com +project1 #tag @context2".to_owned(),
            contexts: vec!["context2".to_owned()],
            projects: vec!["project1".to_owned()],
            hashtags: vec!["tag".to_owned()],
            ..Default::default()
        };

        assert_eq!(todo_txt::parser::task(&line), task);
    }

    #[test]
    fn keywords() {
        let mut keywords = BTreeMap::new();
        keywords.insert("key1".to_string(), "2018-01-01".to_string());
        keywords.insert("key2".to_string(), "value".to_string());

        let line = "Email SoAndSo at soandso@example.com key1:2018-01-01 key2:value".to_string();
        let task = todo_txt::task::Simple {
            subject: "Email SoAndSo at soandso@example.com".to_string(),
            tags: keywords,
            ..Default::default()
        };

        assert_eq!(todo_txt::parser::task(&line), task);
    }

    #[test]
    fn due() {
        let line = "Email SoAndSo at soandso@example.com due:2018-01-01".to_string();
        let task = todo_txt::task::Simple {
            subject: "Email SoAndSo at soandso@example.com".to_string(),
            due_date: todo_txt::Date::from_ymd_opt(2018, 1, 1),

            ..Default::default()
        };

        assert_eq!(todo_txt::parser::task(&line), task);
    }

    #[test]
    fn threshold() {
        let line = "Email SoAndSo at soandso@example.com t:2018-01-01".to_string();
        let task = todo_txt::task::Simple {
            subject: "Email SoAndSo at soandso@example.com".to_string(),
            threshold_date: todo_txt::Date::from_ymd_opt(2018, 1, 1),

            ..Default::default()
        };

        assert_eq!(todo_txt::parser::task(&line), task);
    }

    #[test]
    fn begin_with_keyword() {
        let line = "(C) t:2018-04-03 Open issue on todo-txt parser".to_string();

        let task = todo_txt::task::Simple {
            subject: "Open issue on todo-txt parser".to_string(),
            threshold_date: todo_txt::Date::from_ymd_opt(2018, 4, 3),
            priority: 2.into(),

            ..Default::default()
        };

        assert_eq!(todo_txt::parser::task(&line), task);
    }

    #[test]
    fn url_in_tags() {
        let mut keywords = BTreeMap::new();
        keywords.insert("url".to_string(), "http://example.org".to_string());

        let line = "2018-03-26 test url:http://example.org".to_string();
        let task = todo_txt::task::Simple {
            subject: "test".to_string(),
            create_date: todo_txt::Date::from_ymd_opt(2018, 3, 26),
            tags: keywords,

            ..Default::default()
        };

        assert_eq!(todo_txt::parser::task(&line), task);
    }

    #[test]
    fn relative_date() {
        let line = "relative due due:6months t:-7d".to_string();
        let task = todo_txt::parser::task(&line);

        let due_date = todo_txt::date::today() + chrono::TimeDelta::days((6. * 30.5) as i64);
        let threshold_date = todo_txt::date::today() - chrono::TimeDelta::weeks(1);

        assert_eq!(task.due_date, Some(due_date));
        assert_eq!(task.threshold_date, Some(threshold_date));
    }
}
