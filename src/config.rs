use std::collections::HashMap;

#[derive(Clone, Debug, envir::Deserialize, envir::Serialize)]
#[envir(prefix = "COLOR_")]
pub struct Colors {
    #[envir(default)]
    pub context: crate::Color,
    #[envir(default)]
    pub done: crate::Color,
    #[envir(default)]
    pub date: crate::Color,
    #[envir(skip)]
    pub none: crate::Color,
    #[envir(default)]
    pub meta: crate::Color,
    #[envir(default)]
    pub number: crate::Color,
    #[envir(default)]
    pub project: crate::Color,
    #[envir(load_with = "priorities_load", export_with = "priorities_export")]
    priorities: HashMap<String, crate::Color>,
}

fn priorities_load(env: &HashMap<String, String>) -> envir::Result<HashMap<String, crate::Color>> {
    let mut priorities = HashMap::new();
    priorities.insert("PRI_A".to_string(), "yellow".into());
    priorities.insert("PRI_B".to_string(), "green".into());
    priorities.insert("PRI_C".to_string(), "bright blue".into());
    priorities.insert("PRI_X".to_string(), "white".into());

    for (k, v) in env {
        if k.starts_with("PRI_") {
            priorities.insert(k.clone(), v.parse().unwrap_or_default());
        }
    }

    Ok(priorities)
}

fn priorities_export(priorities: &HashMap<String, crate::Color>) -> HashMap<String, String> {
    priorities
        .iter()
        .map(|(k, v)| (k.clone(), v.to_string()))
        .collect()
}

impl Colors {
    pub fn for_pri(&self, pri: &crate::Priority) -> crate::Color {
        self.priorities
            .get(&format!("PRI_{pri}"))
            .unwrap_or_else(|| self.priorities.get("PRI_X").unwrap())
            .clone()
    }
}

#[derive(envir::Deserialize, envir::Serialize)]
#[envir(prefix = "TODOTXT_")]
pub struct Config {
    #[envir(noprefix, name = "TODO_ACTIONS_DIR", default = "${HOME}/.todo/actions")]
    pub action_dir: String,
    #[envir(default = "true")]
    pub auto_archive: bool,
    #[envir(nested)]
    pub colors: Colors,
    #[envir(default)]
    pub date_on_add: bool,
    #[envir(default = "help")]
    pub default_action: String,
    #[envir(default)]
    pub disable_filter: bool,
    #[envir(default = "cat")]
    pub final_filter: String,
    #[envir(default)]
    pub force: bool,
    #[envir(
        noprefix,
        name = "TODO_NOTE_ARCHIVE",
        default = "${TODO_DIR}/notes/archive.txt"
    )]
    pub note_archive: String,
    #[envir(noprefix, name = "TODO_NOTES_DIR", default = "${TODO_DIR}/notes")]
    pub notes_dir: String,
    #[envir(noprefix, name = "TODO_NOTE_FILTER", default = "cat")]
    pub note_filter: String,
    #[envir(default)]
    pub plain: bool,
    #[envir(default = "true")]
    pub preserve_line_numbers: bool,
    pub priority_on_add: Option<char>,
    #[envir(default = "env LC_COLLATE=C sort -f -k2")]
    pub sort_command: String,
    #[envir(noprefix)]
    pub todo_dir: String,
    #[envir(noprefix, default = "${TODO_DIR}/todo.txt")]
    pub todo_file: String,
    #[envir(noprefix, default = "${TODO_DIR}/done.txt")]
    pub done_file: String,
    #[envir(noprefix, default = "${TODO_DIR}/report.txt")]
    pub report_file: String,
    #[envir(default = "1")]
    pub verbose: u8,
}

impl Config {
    #[must_use]
    pub fn from_env() -> Self {
        Self::load_env();

        envir::from_env().unwrap()
    }

    pub fn load_env() {
        envir::set("NONE", "");
        envir::set("BLACK", "\x1B[0;30m");
        envir::set("RED", "\x1B[0;31m");
        envir::set("GREEN", "\x1B[0;32m");
        envir::set("BROWN", "\x1B[0;33m");
        envir::set("BLUE", "\x1B[0;34m");
        envir::set("PURPLE", "\x1B[0;35m");
        envir::set("CYAN", "\x1B[0;36m");
        envir::set("LIGHT_GREY", "\x1B[0;37m");
        envir::set("DARK_GREY", "\x1B[1;30m");
        envir::set("LIGHT_RED", "\x1B[1;31m");
        envir::set("LIGHT_GREEN", "\x1B[1;32m");
        envir::set("YELLOW", "\x1B[1;33m");
        envir::set("LIGHT_BLUE", "\x1B[1;34m");
        envir::set("LIGHT_PURPLE", "\x1B[1;35m");
        envir::set("LIGHT_CYAN", "\x1B[1;36m");
        envir::set("WHITE", "\x1B[1;37m");
        envir::set("DEFAULT", "\x1B[0m");

        Self::load_config_file();
    }

    #[cfg(not(debug_assertions))]
    fn load_config_file() {
        let home = envir::get("HOME").unwrap();

        let configs: Vec<Box<dyn Fn() -> String>> = vec![
            Box::new(|| format!("{home}/.todo/config")),
            Box::new(|| format!("{home}/todo.cfg")),
            Box::new(|| format!("{home}/.todo.cfg")),
            Box::new(|| format!("{home}/.config/todo/config")),
            Box::new(|| {
                let filename = std::env::args().next().unwrap();
                let mut path = std::path::PathBuf::from(filename);
                path.pop();

                format!("{}/todo.cfg", path.display())
            }),
            Box::new(|| {
                envir::get("TODOTXT_GLOBAL_CFG_FILE")
                    .unwrap_or_else(|_| "/etc/todo/config".to_string())
            }),
        ];

        for config in configs {
            if envir::from_path(config()).is_ok() {
                break;
            }
        }
    }

    #[cfg(debug_assertions)]
    fn load_config_file() {
        envir::dotenv();
    }
}
