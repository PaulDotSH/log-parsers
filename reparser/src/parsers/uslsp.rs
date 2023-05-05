// Url space login space pass

use anyhow::Error;
use crate::parsers::common::{Log, LogParser};

pub struct UslspParser;

impl LogParser for UslspParser {
    fn parse_string(tokens: Vec<&str>) -> Result<Log, Error> {
        Ok(Log{
            url: tokens[0].trim().to_string(),
            login: tokens[1].trim().to_string(),
            pass: tokens[2].trim().to_string(),
        })
    }

    fn get_parser_tokens(line: &str) -> Option<Vec<&str>> {
        if !line.contains(" ") || !line.contains(":") || !line.starts_with("http") {
            return None;
        }

        let mut tokens = Vec::new();

        let Some(space_index) = line.find(' ') else {
            return None;
        };
        let url = &line[..space_index];
        let Some(comma_index) = line.rfind(':') else {
            return None;
        };

        let Some(user) = &line.get(url.len()+1..comma_index) else {
            return None;
        };

        let Some(pw) = &line.get(comma_index+1..) else {
            return None;
        };

        tokens.push(url);
        tokens.push(user);
        tokens.push(pw);
        Some(tokens)
    }
}
