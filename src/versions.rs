extern crate toml;

use std::path::PathBuf;
use std::collections::HashMap;
use std::fs;
use config;

const VERSIONS_FILE_NAME: &'static str = ".alt.toml";
const DEFS_FILE_NAME: &'static str = "defs.toml";

pub fn find_use_file(mut dir: PathBuf) -> Option<PathBuf> {
    loop {
        let file = dir.join(VERSIONS_FILE_NAME);

        if file.is_file() {
            return Some(file);
        } else {
            if dir.parent().is_none() {
                return None;
            } else {
                dir.pop();
            }
        }
    }
}

pub type CommandVersions = HashMap<String, String>;
pub type CommandDefs = HashMap<String, CommandVersions>;

fn command_versions(raw_defs: String, command: &str) -> CommandVersions {
    let all_defs: CommandDefs = toml::from_str(&raw_defs)
        .expect("failed to parse definitions toml");

    all_defs.get(command)
        .map(|r| r.clone())
        .unwrap_or_else(|| CommandVersions::new())
}

pub fn load_def_for(command: &str) -> CommandVersions {
    let def_file_path = config::home_dir().join(DEFS_FILE_NAME);
    match fs::read_to_string(def_file_path) {
        Ok(contents) => command_versions(contents, command),
        Err(_) => CommandVersions::new(),
    }
}
