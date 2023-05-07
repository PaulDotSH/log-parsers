use std::fs;
use std::path::PathBuf;
use walkdir::WalkDir;
use serde::Deserialize;
use sqlx::postgres::PgPoolOptions;

pub fn get_all_json_files(path: &PathBuf) -> Vec<PathBuf> {
    WalkDir::new(path).into_iter()
        .map(|a| a.unwrap())
        .filter(|a| a.file_type().is_file())
        .filter(|a| match a.path().extension() {
            Some(extension) => {
                let extension = extension.to_str().unwrap_or("");
                extension == "json"
            },
            None => false,
        })
        .map(|a| a.into_path())
        .collect()
}

#[derive(Debug, Deserialize)]
pub struct Log {
    pub url: String,
    pub login: String,
    pub pass: String
}

#[tokio::main]
async fn main() {
    let pool = PgPoolOptions::new()
        .max_connections(16)
        .connect("postgres://postgres:parolanebunapostgres@localhost/coco")
        .await
        .expect("can't connect to database");

    for file in get_all_json_files(&PathBuf::from(".")) {
        println!("Reading file {:?}", &file);
        let content = fs::read_to_string(&file).unwrap();
        let logs: Vec<Log> = serde_json::from_str(&content).unwrap();
        println!("{}", logs.len());
        drop(content);
        println!("Inserting file {:?} into the db", &file);
        for log in logs{
            let resp = sqlx::query!(
                "INSERT INTO log(url, login, pw) VALUES ($1, $2, $3);",
                log.url,
                log.login,
                log.pass,
            ).execute(&pool).await;
        }
    }


}
