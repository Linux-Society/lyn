use std::{
    fs::{self, read_to_string},
    path::PathBuf,
};

use home::home_dir;
use lazy_static::lazy_static;
use mlua::Lua;

use crate::config::config_types::Config;

pub mod config_types;

static CONFIG_LOCATION: &str = ".config/lyn/config.lua";
static DEFAULT_CONFIG: &str = include_str!("default.lua");

// Load config into a lazy static
lazy_static! {
    pub static ref CONFIG: Config = {
        let mut home = home_dir().unwrap();
        home.push(CONFIG_LOCATION);

        // Create the config file if it doesnt exist into the config location
        create_config_file(&home);

        // Load lua config
        let lua = Lua::new();
        let config = Config::default();

        let globals = lua.globals();

        globals.set("config", config).unwrap();

        // Read config content into a string to be parsed/exec by lua
        let config_content = read_to_string(home).unwrap();

        // Execute the lua config file (should set/write config values.)
        lua.load(config_content).exec().unwrap();

        // Unwrap on config fail
        globals.get("config").unwrap()
    };
}

fn create_config_file(location: &PathBuf) {
    // Check to make sure location doesnt exist
    if !location.exists() {
        // Create file/directories and all to location and unwrap
        // Any config things, we can just unwrap since it's pretty important
        // that none of it fails.
        fs::create_dir_all(location.parent().unwrap()).unwrap();
        fs::write(location, DEFAULT_CONFIG).unwrap();
    }
}
