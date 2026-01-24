use notify::{Event, RecursiveMode, Watcher};
use std::path::PathBuf;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;
use tao::event_loop::EventLoopProxy;

#[derive(Debug)]
pub enum UserEvent {
    ConfigChanged,
}

pub struct ConfigWatcher {
    _watcher: notify::RecommendedWatcher,
}

impl ConfigWatcher {
    pub fn new(paths: Vec<PathBuf>, proxy: EventLoopProxy<UserEvent>) -> anyhow::Result<Self> {
        let (tx, rx) = mpsc::channel();

        // notify 6.0 uses a more complex setup, but simplified here:
        let mut watcher = notify::recommended_watcher(tx)?;

        for path in &paths {
            if path.exists() {
                watcher.watch(path, RecursiveMode::NonRecursive)?;
            } else {
                // If file doesn't exist yet, we can watch parent, but for now let's just log
                eprintln!("Warning: Config file not found for watching: {:?}", path);
            }
        }

        thread::spawn(move || {
            // Debounce logic could go here, but for MVP we'll just pass through
            let mut last_event = std::time::Instant::now();

            while let Ok(res) = rx.recv() {
                match res {
                    Ok(Event { kind, .. }) => {
                        if kind.is_modify() {
                            // Simple debounce
                            if last_event.elapsed() > Duration::from_millis(100) {
                                let _ = proxy.send_event(UserEvent::ConfigChanged);
                                last_event = std::time::Instant::now();
                            }
                        }
                    }
                    Err(e) => eprintln!("watch error: {:?}", e),
                }
            }
        });

        Ok(Self { _watcher: watcher })
    }
}
