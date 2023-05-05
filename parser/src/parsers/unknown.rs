use anyhow::anyhow;
use crate::parsers::common::{Log, LogParser};

pub struct UnknownParser;

impl LogParser for UnknownParser {
    fn parse_string(content: &str) -> Result<Vec<Log>, anyhow::Error> {
        let mut logs = Vec::new();
        let url_str = "Host: ";
        let user_str = "Login: ";
        let pw_str = "Password: ";

        let content = content.replace("\r\n", "\n");
        let content = content.trim();
        for line in content.split("\n\n") {
            let line = line.trim();
            let line = line.replace("URL: ", url_str);
            let line = line.replace("Url: ", url_str);
            let line = line.replace("USER: ", user_str);
            let line = line.replace("HOST: ", url_str);
            let line = line.replace("PASS: ", pw_str);
            let line = line.replace("Hostname: ", url_str);
            let line = line.replace("Username: ", user_str);
            let line = line.replace("| Site: ", url_str);
            let line = line.replace("| Pass: ", pw_str);
            let line = line.replace("| Login: ", user_str);
            let pw = line.find(pw_str);
            if pw.is_none() {
                continue
            }
            let pw = pw.ok_or(anyhow!("Unknown parser error"))?;
            let pw_line = &line[pw + pw_str.len()..].trim();

            let url = line.find(url_str);
            if url.is_none() {
                continue
            }
            let url = url.ok_or(anyhow!("Unknown parser error"))?;
            let url_line = &line[url + url_str.len()..].trim();
            let url_line = &url_line[..url_line.find('\n')
                .ok_or(anyhow!("Unknown parser error"))?].trim();

            let user = line.find(user_str);
            if user.is_none() {
                continue
            }
            let user = user.ok_or(anyhow!("Unknown parser error"))?;
            let user_line = &line[user + user_str.len()..];
            let user_line = &user_line[..user_line.find('\n')
                .ok_or(anyhow!("Unknown parser error"))?].trim();


            if user_line.len() < 2 || pw_line.len() < 2 || url_line.len() < 2 {
                continue
            }
            logs.push(Log {
                url: url_line.to_string(),
                login: user_line.to_string(),
                pass: pw_line.to_string(),
            })
        }
        Ok(logs)
    }
    fn is_this_parser(content: &str) -> bool {
        (content.contains("Soft: ") && content.contains("Host: ") && content.contains("Login: ")
            && content.contains("Password: ")) ||
            (content.contains("SOFT: ") && content.contains("HOST: ") && content.contains("USER: ")
                && content.contains("PASS: ")) ||
            (content.contains("Hostname: ") && content.contains("Username: ") && content.contains("Password: ")) ||
            (content.contains("URL: ") && content.contains("USER: ") && content.contains("PASS: ")) ||
            (content.contains("| Site: ") && content.contains("| Login: ") && content.contains("| Pass: ")) |
                (content.contains("Url: ") && content.contains("Username: ") && content.contains("Password: "))
    }
}