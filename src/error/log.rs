use chrono::prelude::*;
use std::error::Error;

#[derive()]
pub struct LogError {
    pub date: NaiveDateTime,
    pub content: String,
}

impl LogError {
    pub fn new(err: String) -> Self {
        let date: NaiveDateTime = Utc::now().naive_utc();
        let content = err.to_string();
        LogError { date, content }
    }

    pub fn log(&self) {
        let mut file = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open("./log/server.log")
            .unwrap();
        use std::io::Write;
        writeln!(file, "{} - {}", self.date, self.content).unwrap();
    }
}
