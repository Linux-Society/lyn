use mlua::FromLua;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Clone, FromLua, Debug)]
pub struct Config {
    // Directory of which log files will go to
    pub log_dir: String,

    // Base name of log files
    pub log_file: String,
}