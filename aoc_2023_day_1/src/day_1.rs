use std::{env, fs::File, io::BufReader};

pub fn get_file(file_name: &str) -> BufReader<File> {
    let f = File::open(format!(
        "{}/tests/{}",
        &env::current_dir().unwrap().to_string_lossy(),
        file_name
    ))
    .unwrap();

    BufReader::new(f)
}

fn solve(path: &str) -> () {
    reader = get_file(path);

    reader.lines().map(Result::unwrap).for_each(|line| {

    });

}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    pub fn example() {
        assert_eq!(24000, solve("example_day_1.input", 1));
    }
    #[test]
    pub fn puzzle() {
        assert_eq!(72478, max_calories("day_1.input", 1));
        assert_eq!(210367, max_calories("day_1.input", 3));
    }
}
