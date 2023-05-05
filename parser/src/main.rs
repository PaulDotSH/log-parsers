mod extractor;
mod parsers;

use std::{fs, io, string::String};
use std::ffi::OsStr;
use std::io::{Read, Write};
use std::path::PathBuf;
use anyhow::anyhow;
use walkdir::WalkDir;
use crate::extractor::{get_all_archives, try_extract_archives};
use crate::parsers::common::{Log, LogParser};
use rand::{Rng, distributions::Alphanumeric};
use std::env;
use std::fs::OpenOptions;

fn pause() {
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();

    // We want the cursor to stay at the end of the line, so we print without a newline and flush manually.
    write!(stdout, "Press any key to continue...").unwrap();
    stdout.flush().unwrap();

    // Read a single byte and discard
    let _ = stdin.read(&mut [0u8]).unwrap();
}

fn get_all_files_with_name(path: &PathBuf, file_name: &str) -> Vec<PathBuf>{
    let file_name = file_name.to_lowercase();
    WalkDir::new(path).into_iter()
        .map(|a| a.unwrap())
        .filter(|a| match a.path().file_name() {
            Some(name) => {
                let name = name.to_str().unwrap_or("").to_lowercase();
                name == file_name
            },
            None => false,
        })
        .map(|a| a.into_path())
        .collect()
}

fn create_random_str(len: usize) -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(len)
        .map(char::from)
        .collect()
}

pub fn clean_log_to_file(content: &str, output: &PathBuf) -> Result<(), anyhow::Error> {
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .open(output)?;
    let mut bad_lines = 0;
    for line in content.split("\n") {
        let bytes = line.as_bytes();
        if std::str::from_utf8(&bytes).is_ok() {
            writeln!(file, "{}", line);
        } else {
            bad_lines += 1;
        }
    }
    if bad_lines != 0 {
        println!("Cleaned file with {} bad lines", bad_lines);
    }
    Ok(())
}

fn clean_file(file: &PathBuf) -> Result<(), anyhow::Error> {
    let res = fs::read(&file)?;

    // The data inside the text files something is kinda corrupted, so we need to use from_utf8_unchecked
    let content: String;
    unsafe { content = String::from_utf8_unchecked(res); }
    fs::remove_file(&file)?;
    clean_log_to_file(&content, &file)?;
    Ok(())
}

fn main() {
    // let path = PathBuf::from("passwords.txt");
    // println!("{:?}", clean_file(&path));
    // let content = fs::read_to_string(&path).unwrap();
    // println!("{:?}", parsers::unknown::UnknownParser::is_this_parser(&content));
    // println!("{:?}", parsers::unknown::UnknownParser::parse_string(&content));
    // return;


    let pw = vec!["joker", "123", "password"];
    let starting = PathBuf::from(".");

    // println!("{:?}", curr_logs);
    // return;
    let args = env::args().collect::<Vec<String>>();
    if args.len() > 1 && args[1] == "pre" {
        println!("Searching for logs in case archive already extracted");
        let mut logs: Vec<Log> = Vec::new();
        parse_passwords(&PathBuf::from("."), &mut logs);
        if !logs.is_empty() {
            let logs_json: String = serde_json::to_string(&logs).unwrap();
            fs::write(
                PathBuf::from("already_found.json"),
                logs_json.as_bytes()
            ).unwrap();
        }
    }

    let archives = get_all_archives(&starting);
    println!("Archives found: {:?}", archives);
    for archive in archives {
        let archive_extraction = PathBuf::from(format!("./{}",create_random_str(8)));

        let mut logs: Vec<Log> = Vec::new();
        println!("{:?}", &archive);
        let result = try_extract_archives(&archive,  &archive_extraction, &pw);
        if result.is_err() {
            println!("Error {:?}", result);
        }

        let mut counter = 0;
        loop {
            println!("Getting all subarchives");
            let subarchives = get_all_archives(&archive_extraction);
            counter += 1;
            if subarchives.is_empty() || counter == 10 {
                break
            }
            println!("The dest has some archives...");
            println!("{:?}", subarchives);
            for (i, subarchive) in (&subarchives).into_iter().enumerate() {
                let path = subarchive.to_str().unwrap().to_lowercase();
                if !path.contains("telegram desktop") && (path.contains("desktop") || path.contains("documents")) || path.contains("filegrabber") {
                    fs::remove_file(subarchive);
                    continue
                }

                let Some(filename) = subarchive.file_name() else {
                    fs::remove_file(subarchive);
                    continue;
                };

                let Some(filename) = filename.to_str() else {
                    fs::remove_file(subarchive);
                    continue;
                };
                let filename = filename.to_lowercase();
                if filename == "default.zip" || filename == "adobeinstalledcodecs.zip"
                    || filename == "sssdf.zip" || filename == "local.zip" || filename == "userprofile.zip" ||
                    filename=="def.zip" || filename == "usb.zip" || filename == "doc.zip" || filename == "11111.zip" {
                    fs::remove_file(subarchive);
                    continue;
                }

                println!("Extracting subarchive {:?} {}/{}", subarchive, i, subarchives.len());
                let path = archive_extraction.join(subarchive.file_name().unwrap_or(OsStr::new(create_random_str(10).as_str())));
                let result = try_extract_archives(&subarchive,  &path, &pw);
                if result.is_err() {
                    println!("Error {:?}", result);
                }
                fs::remove_file(subarchive);
            }
        }


        println!("Searching for passwords files...");
        parse_passwords(&archive_extraction, &mut logs);

        println!("Found a total of {} logs in the archive", logs.len());
        if !logs.is_empty() {
            let logs_json: String = serde_json::to_string(&logs).unwrap();
            fs::write(
                PathBuf::from(format!("{}.json", archive.file_name().unwrap().to_str().unwrap())),
                logs_json.as_bytes()
            ).unwrap();
        }


        if fs::remove_dir_all(&archive_extraction).is_err() {
            eprintln!("Couldn't remove {:?}", &archive_extraction);
            // pause();
        }
    }

}

fn parse_passwords(path: &PathBuf, logs: &mut Vec<Log>) {
    println!("Searching for passwords files");
    let mut passwords_files = get_all_files_with_name(&path, "Passwords.txt");
    let mut other_files = get_all_files_with_name(&path, "_AllPasswords_list.txt");
    passwords_files.append(&mut other_files);
    drop(other_files);
    println!("Found {} passwords.txt files", passwords_files.len());
    println!("Parsing...");
    for file in passwords_files {
        let path = file.to_str().unwrap().to_lowercase();
        if path.contains("desktop") || path.contains("documents") {
            continue
        }
        clean_file(&file);
        let content = fs::read_to_string(&file).unwrap_or(String::new());
        let result = if parsers::unknown::UnknownParser::is_this_parser(&content) {
            parsers::unknown::UnknownParser::parse_string(&content)
        } else if parsers::redline::RedlineParser::is_this_parser(&content) {
            parsers::redline::RedlineParser::parse_string(&content)
        } else {
            Err(anyhow!("No supported parser"))
        };
        if result.is_err() {
            println!("Error {:?} while parsing file {:?}", result, file.as_path());
        } else {
            logs.append(&mut result.unwrap());
        }
    }
}