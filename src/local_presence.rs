use std::{
    path::Path,
    sync::mpsc::{channel, Sender},
    time::Duration,
};

use notify::{RecursiveMode, Result};
use notify_debouncer_mini::new_debouncer;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tracing::{error, info};

#[derive(Debug, Deserialize, Serialize)]
pub struct LocalFiles {
    pub files: Vec<LocalFile>,
}

impl LocalFiles {
    pub fn new(files: Vec<LocalFile>) -> Self {
        Self { files }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LocalFile {
    pub file: String,
    pub contents: Value,
}

impl LocalFile {
    pub fn new(file_path: String) -> Option<Self> {
        let contents = std::fs::read_to_string(&file_path).unwrap();

        // Get the file from the path (e.g /path/to/file.js -> file.js).
        let file = Path::new(&file_path)
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();

        match () {
            _ if file.ends_with(".json") => Some(Self {
                file,
                contents: serde_json::from_str(&contents).unwrap(),
            }),
            _ if file.ends_with(".js") => Some(Self {
                file,
                contents: Value::String(contents),
            }),
            _ => None,
        }
    }
}

pub struct LocalPresence;

impl LocalPresence {
    pub fn new() -> Self {
        Self {}
    }

    pub fn watch_files(&self, path: &Path, handler: Sender<Vec<LocalFile>>) -> Result<()> {
        let (tx, rx) = channel();

        let mut debouncer = new_debouncer(Duration::from_secs(1), None, tx).unwrap();

        debouncer
            .watcher()
            .watch(path, RecursiveMode::Recursive)
            .unwrap();

        for result in rx {
            match result {
                Ok(events) => {
                    info!("watch {events:?}");
                    let files = events
                        .iter()
                        .filter_map(|event| {
                            LocalFile::new(event.path.to_str().unwrap().to_string())
                        })
                        .collect();
                    handler.send(files).unwrap();
                }
                Err(errors) => errors
                    .iter()
                    .for_each(|error| error!("watch error {error:?}")),
            }
        }

        Ok(())
    }
}
