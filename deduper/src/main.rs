use std::collections::HashSet;
use std::fs;
use std::path::PathBuf;
use serde::{Serialize, Deserialize};
use walkdir::WalkDir;

#[derive(Debug, Serialize, Deserialize, Eq, Hash, PartialEq)]
pub struct Log {
    pub url: String,
    pub login: String,
    pub pass: String
}

pub fn get_all_json(path: &PathBuf) -> Vec<PathBuf> {
    WalkDir::new(path).into_iter()
        .map(|a| a.unwrap())
        .filter(|a| a.file_type().is_file())
        .filter(|a| match a.path().extension() {
            Some(extension) => {
                let extension = extension.to_str().unwrap_or("");
                extension == "json"
            },
            None => false,
        })
        .map(|a| a.into_path())
        .collect()
}

fn main() {
    let starting = PathBuf::from(".");
    let files = get_all_json(&starting);

    let mut items: HashSet<Log> = HashSet::new();
    let files_size = files.len();
    let mut i = 1;
    for file in files {
        println!("Reading file {:?} (number {} out of {})", file, i, files_size);
        i+=1;
        let content = fs::read_to_string(&file).expect("Couldn't read file");
        let logs: Vec<Log> = serde_json::from_str(&content).expect("Couldn't parse json");
        for log in logs {
            items.insert(log);
        }
    }

    let items: Vec<Log> = items.into_iter().collect();

    let logs_json: String = serde_json::to_string(&items).unwrap();
    fs::write(
        PathBuf::from("Deduped.json"),
        logs_json.as_bytes()
    ).unwrap();

}
