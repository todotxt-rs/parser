#[derive(Clone, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum Note {
    #[default]
    None,
    Short(String),
    Long {
        filename: String,
        content: String,
    },
}

impl Note {
    pub fn from_file(filename: &str) -> Self {
        use std::io::Read;

        if filename.is_empty() {
            return Note::None;
        }

        let note_file = match Self::note_file(filename) {
            Ok(note_file) => note_file,
            Err(err) => {
                log::error!("{err}");
                return Note::Short(filename.to_string());
            }
        };

        let file = match std::fs::File::open(note_file.clone()) {
            Ok(file) => file,
            Err(_) => {
                log::error!("Unable to open {note_file:?}");
                return Note::Short(filename.to_string());
            }
        };

        let mut buffer = std::io::BufReader::new(file);
        let mut content = String::new();

        match buffer.read_to_string(&mut content) {
            Ok(_) => (),
            Err(_) => {
                log::error!("Unable to read {note_file:?}");
                return Note::Short(filename.to_string());
            }
        };

        Note::Long {
            filename: filename.to_string(),
            content,
        }
    }

    pub fn content(&self) -> Option<String> {
        match *self {
            Note::None => None,
            Note::Short(ref content) | Note::Long { ref content, .. } => Some(content.clone()),
        }
    }

    pub fn write(&mut self) -> crate::Result {
        if self == &Note::None {
            return Ok(());
        }

        if let Note::Short(ref content) = *self {
            *self = Note::Long {
                filename: Self::new_filename(),
                content: content.clone(),
            }
        }

        if let Note::Long {
            ref filename,
            ref content,
        } = *self
        {
            use std::io::Write;

            let note_file = Self::note_file(filename)?;

            if let Some(note_dir) = note_file.parent() {
                if !note_dir.exists() {
                    std::fs::create_dir_all(note_dir).map_err(crate::Error::Note)?;
                }
            }

            let mut f = std::fs::File::create(note_file).map_err(crate::Error::Note)?;
            f.write(content.as_bytes()).map_err(crate::Error::Note)?;
        }

        Ok(())
    }

    pub fn delete(&mut self) -> crate::Result {
        if let Self::Long { filename, .. } = self {
            std::fs::remove_file(filename).map_err(crate::Error::Note)?;
        }

        *self = Self::None;

        Ok(())
    }

    fn new_filename() -> String {
        let ext = match std::env::var("TODO_NOTE_EXT") {
            Ok(ext) => ext,
            Err(_) => ".txt".to_string(),
        };

        let name = Self::new_note_id();

        format!("{name}{ext}")
    }

    fn new_note_id() -> String {
        use rand::Rng as _;

        rand::rng()
            .sample_iter(&rand::distr::Alphanumeric)
            .map(char::from)
            .take(3)
            .collect()
    }

    fn note_file(filename: &str) -> crate::Result<std::path::PathBuf> {
        let todo_dir = match std::env::var("TODO_DIR") {
            Ok(todo_dir) => todo_dir,
            Err(_) => return Err(crate::Error::Env),
        };

        let note_dir = match std::env::var("TODO_NOTES_DIR") {
            Ok(note_dir) => note_dir,
            Err(_) => format!("{todo_dir}/notes"),
        };

        let path = format!("{note_dir}/{filename}");

        Ok(path.into())
    }
}

impl std::fmt::Display for Note {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let tag = match std::env::var("TODO_NOTE_TAG") {
            Ok(tag) => tag,
            Err(_) => "note".to_string(),
        };

        let tag = match *self {
            Note::None => String::new(),
            Note::Short(ref content) => format!("{tag}:{content}"),
            Note::Long { ref filename, .. } => format!("{tag}:{filename}"),
        };

        f.write_str(&tag)
    }
}

impl From<String> for Note {
    fn from(value: String) -> Self {
        Self::Short(value)
    }
}

impl std::str::FromStr for Note {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::from(s.to_string()))
    }
}
