use std::fs;
use std::io::{BufRead, BufReader};

pub enum  Password{}

impl Password {
    pub fn get_passwords(path: Option<String>) -> Vec<String> {
        match path {
            None => vec!["aaa".into(), "bbb".into(), "ccc".into()],
            Some(fp) => {
                let file = fs::File::open(fp).unwrap();
                let reader = BufReader::new(file);
                reader.lines()
                    .map(|line| line.unwrap())
                    .collect()
            }
        }
    }
}