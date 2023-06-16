use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;

fn main() -> io::Result<()> {
    let folder_path = PathBuf::from(".");

    for file_entry in fs::read_dir(folder_path)? {
        let file_path = match file_entry {
            Ok(entry) => entry.path(),
            Err(err) => {
                eprintln!("Error accessing file entry: {}", err);
                continue;
            }
        };

        if let Some(extension) = file_path.extension() {
            if extension != "json" {
                continue;
            }
        } else {
            continue;
        }

        let file_contents = match fs::read_to_string(&file_path) {
            Ok(contents) => contents,
            Err(err) => {
                eprintln!("Error reading file: {}: {}", file_path.display(), err);
                continue;
            }
        };

        let replaced_contents = file_contents
            .replace("[{", "{")
            .replace("}]", "}")
            .replace(r#"},{"#, "}\n{");

        let mut file = match fs::File::create(&file_path) {
            Ok(f) => f,
            Err(err) => {
                eprintln!("Error creating file: {}: {}", file_path.display(), err);
                continue;
            }
        };

        if let Err(err) = file.write_all(replaced_contents.as_bytes()) {
            eprintln!("Error writing to file: {}: {}", file_path.display(), err);
            continue;
        }
    }

    Ok(())
}
