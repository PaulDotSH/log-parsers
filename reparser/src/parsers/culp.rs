// Comma url login pass

use anyhow::Error;
use crate::parsers::common::{Log, LogParser};

pub struct CulpParser;

impl LogParser for CulpParser {
    fn parse_string(tokens: Vec<&str>) -> Result<Log, Error> {
        Ok(Log{
            url: tokens[2].trim().to_string(),
            login: tokens[1].trim().to_string(),
            pass: tokens[0].trim().to_string(),
        })
    }

    fn get_parser_tokens(line: &str) -> Option<Vec<&str>> {
        if !line.starts_with("http") || line.chars().filter(|ch| *ch == ':').count() > 5 {
            return None;
        }
        let tokens: Vec<&str> = line.rsplitn(3, ':').collect();
        if tokens.len() != 3 {
            return None;
        }
        if !tokens[2].contains('.') {
            return None;
        }
        Some(tokens)
    }
}
