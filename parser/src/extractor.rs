use std::fs::FileType;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use walkdir::WalkDir;
use anyhow::{anyhow, Error};

pub fn get_all_archives(path: &PathBuf) -> Vec<PathBuf> {
        WalkDir::new(path).into_iter()
            .filter(|a| a.is_ok())
        .map(|a| a.unwrap())
        .filter(|a| a.file_type().is_file())
        .filter(|a| match a.path().extension() {
            Some(extension) => {
                let extension = extension.to_str().unwrap_or("");
                extension == "7z" || extension == "zip" || extension == "rar"
            },
            None => false,
        })
        .map(|a| a.into_path())
        .collect()
}

pub fn extract_archive_to_folder(archive: &PathBuf, destination: &PathBuf, pass: &str) -> Result<(), Error> {
    let proc = Command::new("7z")
        .args(["x", archive.to_str().ok_or(anyhow!("Unknown error"))?, "-aoa", "-y", "-ir!*.7z", "-ir!*.txt", "-ir!*.zip", "-ir!*.rar", "-ir!*.7zip",
            format!("-o{}", destination.to_str().ok_or(anyhow!("Unknown error"))?).as_str(), format!("-p{}", pass).as_str()])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()?;
    // let stdout = String::from_utf8(proc.stdout).unwrap();
    let stderr = String::from_utf8(proc.stderr)?;
    if stderr.contains("ERROR: Wrong password") {
        return Err(anyhow::anyhow!("Wrong password"));
    }
    if stderr.len() > 5 {
        eprintln!("Unknown error happened when extracting archive {:?}, error: {}", archive, stderr);
        return Err(anyhow::anyhow!("Unknown error"));
    }
    // println!("Out: {}", stdout);
    // println!("Err: {}", stderr);
    Ok(())
}

pub fn try_extract_archives(archive: &PathBuf, destination: &PathBuf, passwords: &Vec<&str>) -> Result<(), Error> {
    for pass in passwords {
        let result = extract_archive_to_folder(&archive, &destination, pass);
        if result.is_ok() {
            return Ok(())
        }
    }
    Err(anyhow!(format!("No correct password for archive {:?}", archive)))
}