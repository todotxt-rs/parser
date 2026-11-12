#[cfg(all(test, feature = "extended"))]
mod test {
    use std::collections::BTreeMap;

    #[test]
    #[cfg(feature = "serde")]
    fn test_extra_deserialize() {
        let json = r#"{
            "subject": "Test"
        }"#;

        let actual: todo_txt::task::Extended = serde_json::from_str(json).unwrap();

        let expected = todo_txt::task::Extended {
            inner: todo_txt::task::Simple {
                subject: "Test".to_string(),
                priority: 26.into(),
                create_date: None,
                finish_date: None,
                contexts: Vec::new(),
                threshold_date: None,
                due_date: None,
                hashtags: Vec::new(),
                finished: false,
                projects: Vec::new(),
                tags: BTreeMap::new(),
            },
            flagged: false,
            note: todo_txt::task::Note::None,
            recurrence: None,
            hidden: false,
        };

        assert_eq!(actual, expected);
    }

    #[test]
    #[cfg(feature = "serde")]
    fn test_note_deserialize() {
        let json = r#"{
            "subject": "Test",
            "note": "A note"
        }"#;

        let actual: todo_txt::task::Extended = serde_json::from_str(json).unwrap();

        let expected = todo_txt::task::Extended {
            inner: todo_txt::task::Simple {
                subject: "Test".to_string(),

                ..todo_txt::task::Simple::default()
            },
            note: todo_txt::task::Note::Short("A note".to_string()),

            ..todo_txt::task::Extended::default()
        };

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_from_task() {
        let task = todo_txt::task::Simple {
            subject: "Subject".to_string(),
            tags: {
                let mut map = BTreeMap::new();
                map.insert("f".to_string(), "1".to_string());

                map
            },

            ..todo_txt::task::Simple::default()
        };

        let extra: todo_txt::task::Extended = task.into();

        assert!(extra.flagged);
    }

    #[test]
    fn test_to_string() {
        let task = todo_txt::task::Extended {
            inner: todo_txt::task::Simple {
                subject: "Subject".to_string(),

                ..Default::default()
            },
            flagged: true,
            hidden: true,

            ..Default::default()
        };

        assert_eq!(task.to_string(), "Subject f:1 h:1");
    }
}
