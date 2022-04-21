mod collection;
mod errors;
mod game_tree;
mod node;
mod property;

use std::{error::Error, fs, process};

use collection::Collection;

pub fn parse(path: &str) -> Collection {
    let contents = read_file(path).unwrap_or_else(|err| {
        eprintln!("Error while reading file {}: {}", path, err);
        process::exit(1);
    });

    Collection::new(&contents).unwrap_or_else(|err| {
        eprintln!("Error while parsing file {}: {}", path, err);
        process::exit(1);
    })
}

fn read_file(path: &str) -> Result<String, Box<dyn Error>> {
    let contents = fs::read_to_string(path)?;

    Ok(contents)
}

#[cfg(test)]
mod tests {
    use super::read_file;

    #[test]
    fn can_read_file() {
        let contents = read_file("resources/read_file.txt").unwrap();

        let expected = "Everything was beautiful,\nand nothing hurt.";
        assert_eq!(expected, contents);
    }
}
