use mlua::FromLua;

#[derive(Default, Clone, FromLua)]
pub struct Config {
    // Directory of which log files will go to
    pub log_dir: String,

    // Base name of log files
    pub log_file: String,
}

impl mlua::UserData for Config {
    fn add_fields<'lua, F: mlua::UserDataFields<'lua, Self>>(fields: &mut F) {
        // Getters
        fields.add_field_method_get("log_dir", |_, this| Ok(this.log_dir.clone()));
        fields.add_field_method_get("log_file", |_, this| Ok(this.log_file.clone()));

        // Setters
        fields.add_field_method_set("log_dir", |_, this, log_dir| {
            this.log_dir = log_dir;
            Ok(())
        });
        fields.add_field_method_set("log_file", |_, this, log_file| {
            this.log_file = log_file;
            Ok(())
        });
    }
}
