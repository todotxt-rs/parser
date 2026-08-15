#[derive(Clone, Debug, Default)]
pub enum Color {
    Colored(owo_colors::AnsiColors),
    #[default]
    None,
    Raw(String),
}

impl Color {
    #[must_use]
    pub fn colorize(&self, s: &str) -> String {
        use owo_colors::OwoColorize;

        s.if_supports_color(owo_colors::Stream::Stdout, |text| match self {
            Self::Colored(color) => text.color(*color).to_string(),
            Self::None => text.to_string(),
            Self::Raw(color) => format!("{color}{text}\x1B[0m"),
        })
        .to_string()
    }
}

impl From<&str> for Color {
    fn from(s: &str) -> Self {
        s.parse().unwrap_or_default()
    }
}

impl std::str::FromStr for Color {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let ansi_color = s.into();

        let color = if ansi_color == owo_colors::AnsiColors::White {
            Self::Raw(s.replace("\\\\033", "\x1B"))
        } else {
            Self::Colored(ansi_color)
        };

        Ok(color)
    }
}

#[allow(clippy::to_string_trait_impl)]
impl ToString for Color {
    fn to_string(&self) -> String {
        match self {
            Self::Colored(color) => format!("{color:?}").to_lowercase(),
            Self::None => String::new(),
            Self::Raw(color) => color.clone(),
        }
    }
}
