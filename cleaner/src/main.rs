use std::fs;
use std::fs::OpenOptions;
use std::io::{BufWriter, Write};
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

pub fn clean_log_to_file(content: &str, output: &PathBuf) {
    let file = OpenOptions::new()
        .create(true)
        .write(true)
        .open(output)
        .unwrap();
    let mut stream = BufWriter::new(file);
    let mut bad_lines = 0;
    for line in content.split("\n") {
        let bytes = line.as_bytes();
        if std::str::from_utf8(&bytes).is_ok() {
            stream.write_all(line.as_bytes()).unwrap();
        } else {
            bad_lines += 1;
        }
    }
    println!("Cleaned file with {} bad lines", bad_lines);
}




pub fn get_all_text_files(path: &PathBuf) -> Vec<PathBuf> {
    WalkDir::new(path).into_iter()
        .map(|a| a.unwrap())
        .filter(|a| a.file_type().is_file())
        .filter(|a| match a.path().extension() {
            Some(extension) => {
                let extension = extension.to_str().unwrap_or("");
                extension == "txt"
            },
            None => false,
        })
        .map(|a| a.into_path())
        .collect()
}


fn main() {
    for file in get_all_text_files(&PathBuf::from(".")) {
        let res = fs::read(&file).unwrap();
        println!("Read file data");
        // The data inside the text files something is kinda corrupted, so we need to use from_utf8_unchecked
        let content: String;
        unsafe { content = String::from_utf8_unchecked(res); }
        println!("Converted into a string");
        let file = Path::new(".").join(format!("{} - cleaned.txt", file.file_name().unwrap().to_str().unwrap()));
        clean_log_to_file(&content, &file);
    }

}
