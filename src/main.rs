#![allow(dead_code)]
use clap::Parser;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Read;
use std::path::Path;
use std::path::PathBuf;

#[derive(Debug, Parser)]
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
        help = "Do not print the first SKIP components of each path"
    )]
    // This is Option<usize> rather than usize because code coverage shows this as covered when I
    // use Option.
    skip: Option<usize>,

    #[arg(help = "If zero paths are provided, reads paths from stdin")]
    paths: Option<Vec<String>>,
}

struct Options {
    stdin_reader: Box<dyn Read>,
}

impl Options {
    fn new() -> Self {
        Self {
            stdin_reader: Box::new(std::io::stdin()),
        }
    }
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

fn realmain(options: Options, flags: Flags) -> String {
    let paths = match flags.paths {
        None => BufReader::new(options.stdin_reader)
            .lines()
            .map(|line| line.unwrap())
            .collect::<Vec<String>>(),
        Some(paths) => paths,
    };
    paths
        .iter()
        .flat_map(|path| parents_of_filename(path, flags.skip.unwrap_or_default()))
        .collect::<Vec<String>>()
        .join("\n")
}

fn main() {
    println!("{}", realmain(Options::new(), Flags::parse()));
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
        assert_eq!(Some(3), flags.skip);
        assert_eq!(Some(vec![String::from("/usr/bin/cat")]), flags.paths);

        assert!(
            Flags::command()
                .try_get_matches_from(vec!["argv0", "--skip", "/usr/bin/cat"])
                .is_err()
        );
    }
}

#[cfg(test)]
mod options {
    use super::*;

    #[test]
    fn new() {
        // What can I check here?
        let _options = Options::new();
    }
}

#[cfg(test)]
mod realmain {
    use super::*;

    #[test]
    fn paths_given() {
        let flags = Flags {
            paths: Some(vec![
                String::from("/usr/bin/tail"),
                String::from("/tmp/foo/bar"),
            ]),
            skip: Some(1),
        };
        let expected = String::from("/usr/bin\n/usr/bin/tail\n/tmp/foo\n/tmp/foo/bar");
        assert_eq!(expected, realmain(Options::new(), flags));
    }

    #[test]
    fn no_paths_given() {
        use std::io::Cursor;
        let flags = Flags {
            paths: None,
            skip: Some(1),
        };
        let expected = String::from("/var/run\n/var/run/asdf\n/tmp/foo\n/tmp/foo/bar");
        let cursor = Cursor::new(String::from("/var/run/asdf\n/tmp/foo/bar"));
        let options = Options {
            stdin_reader: Box::new(cursor),
        };
        assert_eq!(expected, realmain(options, flags));
    }
}
