use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Log {
    pub url: String,
    pub login: String,
    pub pass: String
}

pub(crate) trait LogParser {
    fn parse_string(content: &str) -> Result<Vec<Log>, anyhow::Error>;
    fn is_this_parser(content: &str) -> bool;
}