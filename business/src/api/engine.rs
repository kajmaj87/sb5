use crate::api::command::Command;
use crate::scripting::script_engine::ScriptEngine;

pub struct Engine { 
    script_engine: ScriptEngine
}


impl Engine {
    pub fn new() -> Self {
        Self {
            script_engine: ScriptEngine::new()
        }
    }
    pub fn execute(&mut self, command: Command) -> Result<(), String> {
        println!("Executing command: {:#?}", command);
        match command {
            Command::LoadScript { name, script } => {
                self.script_engine.load_script(name, script);
                Ok(())
            }
        }
    }
}