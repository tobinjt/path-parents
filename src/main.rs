#![allow(dead_code)]
use clap::Parser;
use std::path::Path;
use std::path::PathBuf;

#[derive(Debug, Default, Parser)]
#[command(
    version,
    about,
    long_about = "Print every parent of the paths provided, e.g. /usr/bin/tail => /usr /usr/bin /usr/bin/tail"
)]
struct Flags {
    // Providing a default value makes it optional.
    #[arg(
        short,
        long,
        default_value_t = 0,
        help = "Do not print the first SKIP components of each path"
    )]
    skip: usize, // TODO: how do I make test coverage realise that I've used this?
    #[arg(help = "If zero paths are provided, reads paths from stdin")]
    paths: Option<Vec<String>>,
}

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

#[cfg(test)]
mod clap_test {
    use super::*;
    use clap::CommandFactory;

    #[test]
    fn verify() {
        Flags::command().debug_assert();
    }

    #[test]
    fn parse_args() {
        // Checks that I've configured the parser correctly.
        let flags = Flags::parse_from(vec!["argv0", "--skip", "3", "/usr/bin/cat"]);
        assert_eq!(3, flags.skip);
        assert_eq!(Some(vec![String::from("/usr/bin/cat")]), flags.paths);
    }
}
