// Comma login pass url

use anyhow::Error;
use crate::parsers::common::{Log, LogParser};

pub struct ClpuParser;

impl LogParser for ClpuParser {
    fn parse_string(tokens: Vec<&str>) -> Result<Log, Error> {
        Ok(Log{
            url: tokens[2].trim().to_string(),
            login: tokens[0].trim().to_string(),
            pass: tokens[1].trim().to_string(),
        })
    }

    fn get_parser_tokens(line: &str) -> Option<Vec<&str>> {
        let tokens: Vec<&str> = line.splitn(3,':').collect();

        if tokens.len() != 3 {
            return None;
        }
        if !tokens[2].contains('.') {
            return None;
        }
        Some(tokens)
    }
}
