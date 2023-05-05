use anyhow::anyhow;
use crate::parsers::common::{Log, LogParser};

pub struct RedlineParser;

impl LogParser for RedlineParser {
    fn parse_string(content: &str) -> Result<Vec<Log>, anyhow::Error> {
        let mut logs = Vec::new();
        let stars = "***********************************************";
        let url_str = "URL: ";
        let pw_str = "Password: ";
        let user_str = "Username: ";

        let content = if content.rfind(stars).is_some() {
            &content[(content.rfind(stars)
                .ok_or(anyhow!("Unknown parser error"))?)+stars.len()..]
        } else {
            &content[(content.rfind("#======================================================#")
                .ok_or(anyhow!("Unknown parser error"))?)+stars.len()..]
        };



        let content = content.trim();
        for line in content.split("===============") {
            if !(line.contains(url_str) && line.contains(pw_str) && line.contains(user_str)) {
                continue
            }
            let line = line.trim();

            let url = line.find(url_str).ok_or(anyhow!("Unknown parser error"))?;
            let user = line.find(user_str).ok_or(anyhow!("Unknown parser error"))?;
            let pw = line.find(pw_str).ok_or(anyhow!("Unknown parser error"))?;

            let pw_line = &line[pw+pw_str.len()..];
            let pw_line = &pw_line[..pw_line.find('\n')
                .ok_or(anyhow!("Unknown parser error"))?].trim();

            let url_line = &line[url+url_str.len()..];
            let url_line = &url_line[..url_line.find('\n')
                .ok_or(anyhow!("Unknown parser error"))?].trim();

            let user_line =  &line[user+user_str.len()..];
            let user_line = &user_line[..user_line.find('\n')
                .ok_or(anyhow!("Unknown parser error"))?].trim();

            logs.push(Log{
                url: url_line.to_string(),
                login: user_line.to_string(),
                pass: pw_line.to_string(),
            })
        }
        Ok(logs)
    }
    fn is_this_parser(content: &str) -> bool {
        content.contains("____  _____ ____  _     ___ _   _ _____") ||
            content.contains("( M | E | T | A )") ||
            content.contains("( F | A | T | E ") ||
            content.contains("#======================================================#")
    }
}