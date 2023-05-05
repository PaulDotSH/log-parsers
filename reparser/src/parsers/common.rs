use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Log {
    pub url: String,
    pub login: String,
    pub pass: String
}

pub(crate) trait LogParser {
    fn parse_string(tokens: Vec<&str>) -> Result<Log, anyhow::Error>;
    fn get_parser_tokens(line: &str) -> Option<Vec<&str>>;
}

pub fn clean_log_to_file(content: &str, output: &PathBuf) {
    println!("{:?}", &output);
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .open(output)
        .unwrap();
    let mut bad_lines = 0;
    for line in content.split("\n") {
        let bytes = line.as_bytes();
        if std::str::from_utf8(&bytes).is_ok() {
            writeln!(file, "{}", line);
        } else {
            bad_lines += 1;
        }
    }
    println!("Cleaned file with {} bad lines", bad_lines);

    // std::str::from_utf8(&bytes).is_ok();
}

// UNIQ
pub fn parse_log(content: &str) -> (Vec<Log>, usize) {
    let mut i=0;
    let mut j = 0;
    let mut logs = Vec::new();
    for line in content.split("\n") {
        // i+=1;
        if line.len() < 5 {
            continue
        }
        let line = line.replace(';', ":");

        if !line.contains("http") && !line.contains(".com") && !line.starts_with("ftp")
            && !line.starts_with("android"){
            // println!("{}. {:?} - not supported protocol ", i, line);
            j+=1;
            continue
        }
        // One of the 2 formats for UNIQ, either "url user:pw" or "url:user:pw
        let colon_count = line.chars().filter(|c| *c == ':').count();
        if line.contains(' ') && colon_count == 2 {
            let Some(url) = line.find(' ') else { continue; };
            let url = &line[..url];
            let items: Vec<&str> = line[url.len()..].trim().split(':').take(2).collect();
            if items.len() < 2 {
                continue
            }
            logs.push(Log{
                url: url.to_string(),
                login: items[0].to_string(),
                pass: items[1].to_string(),
            });
        } else if colon_count == 3 {
            let items: Vec<&str> = line.split(':').take(3).collect();
            logs.push(Log{
                url: items[0].to_string(),
                login: items[1].to_string(),
                pass: items[2].to_string(),
            });
        }
    }
    (logs, j)
}

// This function will panic the program if the line isn't valid, since it should serialize the string, and the STD expects that every string is a valid utf8 string
pub fn check_log_validity(content: &str) -> Vec<Log> {
    for line in content.split("\n") {
        println!("{:?} {}", line, line);
    }
    Vec::new()
}