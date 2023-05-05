mod parsers;

#[macro_use] extern crate log;
extern crate simplelog;

use std::fs;
use std::fs::File;
use std::path::PathBuf;
use walkdir::WalkDir;
use simplelog::*;
use crate::parsers::common::{Log, LogParser};

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

fn parse_logs(content: &str) -> Vec<Log> {
    let mut logs = Vec::new();
    for line in content.split("\n") {
        if line.len() < 5 {
            continue
        }
        let tokens = parsers::uwlpc::UwlpcParser::get_parser_tokens(line);
        if tokens.is_some() {
            let token = parsers::uwlpc::UwlpcParser::parse_string(tokens.unwrap());
            if token.is_err() {
                info!("Line {} cannot be parsed error {:?}", line, token);
                continue;
            }
            logs.push(token.unwrap());
            continue;
        }

        let tokens = parsers::uslsp::UslspParser::get_parser_tokens(line);
        if tokens.is_some() {
            let token = parsers::uslsp::UslspParser::parse_string(tokens.unwrap());
            if token.is_err() {
                info!("Line {} cannot be parsed error {:?}", line, token);
                continue;
            }
            logs.push(token.unwrap());
            continue;
        }

        let tokens = parsers::uslsp::UslspParser::get_parser_tokens(line);
        if tokens.is_some() {
            let token = parsers::uslsp::UslspParser::parse_string(tokens.unwrap());
            if token.is_err() {
                info!("Line {} cannot be parsed error {:?}", line, token);
                continue;
            }
            logs.push(token.unwrap());
            continue;
        }

        let tokens = parsers::csv::CsvParser::get_parser_tokens(line);
        if tokens.is_some() {
            let token = parsers::csv::CsvParser::parse_string(tokens.unwrap());
            if token.is_err() {
                info!("Line {} cannot be parsed error {:?}", line, token);
                continue;
            }
            logs.push(token.unwrap());
            continue;
        }

        let tokens = parsers::culp::CulpParser::get_parser_tokens(line);
        if tokens.is_some() {
            let token = parsers::culp::CulpParser::parse_string(tokens.unwrap());
            if token.is_err() {
                info!("Line {} cannot be parsed error {:?}", line, token);
                continue;
            }
            logs.push(token.unwrap());
            continue;
        }

        let tokens = parsers::clpu::ClpuParser::get_parser_tokens(line);
        if tokens.is_some() {
            let token = parsers::clpu::ClpuParser::parse_string(tokens.unwrap());
            if token.is_err() {
                info!("Line {} cannot be parsed error {:?}", line, token);
                continue;
            }
            logs.push(token.unwrap());
            continue;
        }

        let tokens = parsers::ssv::SsvParser::get_parser_tokens(line);
        if tokens.is_some() {
            let token = parsers::ssv::SsvParser::parse_string(tokens.unwrap());
            if token.is_err() {
                info!("Line {} cannot be parsed error {:?}", line, token);
                continue;
            }
            logs.push(token.unwrap());
            continue;
        }

        error!("Line {} doesn't have any parser", line);
    }
    logs
}

fn main() {


    WriteLogger::init(
        LevelFilter::Info,
        Config::default(),
        File::create("reparser.log").unwrap()
    ).unwrap();

    let files = get_all_text_files(&PathBuf::from("."));
    println!("Will try to parse files {:?}", files);

    for file in files {
        let res = fs::read(&file).unwrap();
        let content: String;
        unsafe { content = String::from_utf8_unchecked(res); }
        // let (logs, bad_lines) = parse_log(&content);
        let logs = parse_logs(&content);
        let logs_json: String = serde_json::to_string(&logs).unwrap();
        drop(content);
        fs::write(
            PathBuf::from(format!("{}.json", file.file_name().unwrap().to_str().unwrap())),
            logs_json.as_bytes()
        );

    }

}
