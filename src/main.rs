#![allow(dead_code)]
use std::path::Path;
use std::path::PathBuf;

fn parents_of_filename(filename: &str, skip: usize) -> Vec<String> {
    let mut result: Vec<String> = vec![];
    let mut path = PathBuf::new();
    for (i, component) in Path::new(&filename).components().enumerate() {
        path.push(component);
        if i > skip {
            result.push(path.as_path().to_str().unwrap().to_string());
        }
    }
    result
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod parents_of_filename {
    use super::*;

    #[test]
    fn all_parents() {
        let expected = vec![
            String::from("/usr"),
            String::from("/usr/bin"),
            String::from("/usr/bin/cat"),
        ];
        assert_eq!(expected, parents_of_filename("/usr/bin/cat", 0));
    }

    #[test]
    fn skipping() {
        let expected = vec![String::from("/usr/bin"), String::from("/usr/bin/cat")];
        assert_eq!(expected, parents_of_filename("/usr/bin/cat", 1));
    }
}
