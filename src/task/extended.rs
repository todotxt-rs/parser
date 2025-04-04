#[derive(Clone, Debug, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Extended {
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub inner: super::Simple,
    #[cfg_attr(feature = "serde", serde(default))]
    pub note: super::Note,
    pub recurrence: Option<super::Recurrence>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub flagged: bool,
    #[cfg_attr(feature = "serde", serde(default))]
    pub hidden: bool,
}

impl Extended {
    pub fn has_note(&self) -> bool {
        self.note != super::Note::None
    }

    fn note(task: &super::Simple) -> super::Note {
        if let Some(file) = task.tags.get(&Self::tag_name()) {
            super::Note::from_file(file)
        } else {
            super::Note::None
        }
    }

    fn tag_name() -> String {
        match std::env::var("TODO_NOTE_TAG") {
            Ok(tag) => tag,
            Err(_) => "note".to_string(),
        }
    }
}

impl std::convert::From<super::Simple> for Extended {
    fn from(task: super::Simple) -> Self {
        use std::str::FromStr;

        let mut inner = task;

        let note = Self::note(&inner);
        inner.tags.remove(&Self::tag_name());

        let mut recurrence = None;

        if let Some(rec) = inner.tags.get("rec") {
            recurrence = super::Recurrence::from_str(rec).ok();
        }
        inner.tags.remove("rec");

        let flagged = inner.tags.contains_key("f");
        inner.tags.remove("f");

        let hidden = inner.tags.contains_key("h");
        inner.tags.remove("h");

        Self {
            inner,
            note,
            recurrence,
            flagged,
            hidden,
        }
    }
}

impl std::str::FromStr for Extended {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        super::Simple::from_str(s).map(|x| x.into())
    }
}

impl From<String> for Extended {
    fn from(value: String) -> Self {
        value.parse().unwrap()
    }
}

impl std::ops::Deref for Extended {
    type Target = super::Simple;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl std::ops::DerefMut for Extended {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl std::fmt::Display for Extended {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use std::ops::Deref;

        f.write_str(&format!("{}", self.deref()))?;

        if self.note != super::Note::None {
            f.write_str(&format!(" {}", self.note))?;
        }

        if let Some(ref recurrence) = self.recurrence {
            f.write_str(&format!(" rec:{recurrence}"))?;
        }

        if self.flagged {
            f.write_str(" f:1")?;
        }

        if self.hidden {
            f.write_str(" h:1")?;
        }

        Ok(())
    }
}

impl std::cmp::PartialOrd for Extended {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl std::cmp::Ord for Extended {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.inner.cmp(&other.inner)
    }
}

impl AsRef<super::Simple> for Extended {
    fn as_ref(&self) -> &super::Simple {
        &self.inner
    }
}
