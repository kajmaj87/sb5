use std::path::Path;
use business::api::command::Command;
use business::api::engine::Engine;
use os::files::FileWatcher;

fn main() {
    println!("Hello, world from UI!");
    let mut engine = Engine::new();
    let watcher = FileWatcher::new(Path::new("scripts")).unwrap();
    engine.execute(Command::LoadScript {
        name: "test".to_string(),
        script: "a=1; function f() return a end; print('Hello, world from Lua!'.. f())".to_string(),
    }).unwrap();
    engine.execute(Command::LoadScript {
        name: "test".to_string(),
        script: "a=2; print('Hello, world from Lua!'.. f())".to_string(),
    }).unwrap();
    engine.execute(Command::LoadScript {
        name: "test".to_string(),
        script: "a=1; print('Hello, world from Lua!'.. a)".to_string(),
    }).unwrap();
    
        loop {
        match watcher.rx.recv() {
            Ok((path, contents)) => {
                engine.execute(Command::LoadScript {
                    name: path.to_string(),
                    script: contents,
                }).unwrap();
            }
            Err(e) => {
                eprintln!("Error receiving file change: {}", e);
                break;
            }
        }
    }
}
