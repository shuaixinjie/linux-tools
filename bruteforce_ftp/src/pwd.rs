use std::fs;
use std::io::{BufRead, BufReader};


pub fn get_pwd(path: Option<String>) -> Vec<String> {
    match path {
        Some(fp) => {
            let file = fs::File::open(fp).unwrap();
            let reader = BufReader::new(file);
            reader.lines()
                .map(|line| line.unwrap())
                .collect()
        }
        None => vec!["aaa".into(), "bbb".into(), "ccc".into()],
    }
}
