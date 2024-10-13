use mlua::Lua;
pub struct ScriptEngine {
    // ...
    lua: Lua,
}

impl ScriptEngine {
    pub fn new() -> Self {
        let lua = Lua::new();
        Self { lua }
    }

    pub fn load_script(&mut self, name: String, script: String) {
        self.lua.load(&script).set_name(name).exec().unwrap();
    }
}
