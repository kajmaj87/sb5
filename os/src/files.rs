use notify::{Event, RecommendedWatcher, RecursiveMode, Watcher, Config};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::time::{Duration, Instant};
use std::{fs, thread};
use std::collections::HashMap;
use std::io;
use std::path::{Path, PathBuf};
use std::sync::Mutex;
use std::thread::sleep;

pub struct FileWatcher {
    pub rx: Receiver<(String, String)>,
    _watcher: RecommendedWatcher,
}

impl FileWatcher {    pub fn new(directory: &Path) -> io::Result<Self> {
        let (event_tx, event_rx) = channel();
        let (tx, rx) = channel();

        // Create a watcher object
        let mut watcher: RecommendedWatcher = Watcher::new(
            event_tx,
            Config::default().with_poll_interval(Duration::from_secs(2)),
        )
        .map_err(|e| {
            io::Error::new(io::ErrorKind::Other, format!("Failed to create watcher: {}", e))
        })?;

        // Watch the directory recursively
        watcher.watch(directory, RecursiveMode::Recursive).map_err(|e| {
            io::Error::new(io::ErrorKind::Other, format!("Failed to watch directory: {}", e))
        })?;

        // Shared state for debouncing
        let debounce_map = Mutex::new(HashMap::new());

        // Spawn a thread to process events and read file contents
        thread::spawn(move || {
            for event in event_rx {
                if let Ok(Event { paths, .. }) = event {
                    for path in paths {
                        let path_str = path.to_string_lossy().into_owned();
                        let mut map = debounce_map.lock().unwrap();
                        let now = Instant::now();
                        let should_send = match map.get(&path_str) {
                            Some(&last_event) => now.duration_since(last_event) > Duration::from_secs(2),
                            None => true,
                        };
                        if should_send {
                            map.insert(path_str.clone(), now);
                            // Add a short delay to ensure the file is fully written
                            sleep(Duration::from_millis(100));
                            if let Ok(contents) = fs::read_to_string(&path) {
                                if tx.send((path_str, contents)).is_err() {
                                    eprintln!("Failed to send file contents");
                                }
                            }
                        }
                    }
                }
            }
        });

        Ok(FileWatcher {
            rx,
            _watcher: watcher,
        })
    }

    pub fn watch_files(&self) -> io::Result<()> {
        // This function can be kept empty if no additional logic is needed
        Ok(())
    }
}