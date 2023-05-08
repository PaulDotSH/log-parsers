use std::fs;
use std::io::Write;
use std::path::PathBuf;

fn main() -> std::io::Result<()> {
    // Get the folder path from the curr directory
    let folder_path = PathBuf::from(".");

    // Iterate over each file in the folder
    for file_entry in fs::read_dir(folder_path)? {
        let file_path = file_entry?.path();

        // Check if the file is a JSON file
        if let Some(extension) = file_path.extension() {
            if extension != "json" {
                continue;
            }
        } else {
            continue;
        }

        // Read the file contents
        let file_contents = fs::read_to_string(&file_path)?;

        // Perform the replacements
        let replaced_contents = file_contents
            .replace("[{", "{")
            .replace("}]", "}")
            .replace(r#"},{"#, "}\n{");

        // Write the modified contents back to the file
        let mut file = fs::File::create(&file_path)?;
        file.write_all(replaced_contents.as_bytes())?;
    }

    Ok(())
}
