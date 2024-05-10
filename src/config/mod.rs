use std::{
    fs::{self, read_to_string},
    path::PathBuf,
};

use home::home_dir;
use lazy_static::lazy_static;
use mlua::{Lua, LuaSerdeExt};
use tracing::info;

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

        globals.set("config", lua.to_value(&config).unwrap()).unwrap();

        // Read config content into a string to be parsed/exec by lua
        let config_content = read_to_string(home).unwrap();

        // Execute the lua config file (should set/write config values.)
        lua.load(config_content).exec().unwrap();

        // Convert from lua value (gotten from globals under "config") to our Config
        lua.from_value::<Config>(
            globals.get::<_, mlua::Value>("config")
            .unwrap()
        ).unwrap()
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
