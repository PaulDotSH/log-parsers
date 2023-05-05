// Url without Http Login Pass Comma

use anyhow::Error;
use crate::parsers::common::{Log, LogParser};

pub struct UwlpcParser;

impl LogParser for UwlpcParser {
    fn parse_string(tokens: Vec<&str>) -> Result<Log, Error> {
        Ok(Log{
            url: tokens[0].trim().to_string(),
            login: tokens[1].trim().to_string(),
            pass: tokens[2].trim().to_string(),
        })
    }

    fn get_parser_tokens(line: &str) -> Option<Vec<&str>> {
        let tokens: Vec<&str> = line.split(", ").collect();
        if tokens.len() != 3 {
            return None;
        }
        if !tokens[0].contains(".") {
            return None;
        }
        Some(tokens)
    }
}
