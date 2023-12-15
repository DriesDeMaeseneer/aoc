use std::{env, fs::File, io::{BufReader, BufRead}};

pub fn get_file(file_name: &str) -> BufReader<File> {
    let f = File::open(format!(
        "{}/tests/{}",
        &env::current_dir().unwrap().to_string_lossy(),
        file_name
    ))
    .unwrap();

    BufReader::new(f)
}

fn hash(str: &str) -> usize {
    let mut current_value = 0;
    for chr in str.chars(){
        current_value += chr as usize;
        current_value *= 17;
        current_value %= 256;
    }
    current_value
}

fn solve(path: &str) -> usize {
    let reader = get_file(path);
    let mut total = 0;

    reader.lines().map(Result::unwrap).for_each(|line| {
        total = line.split(',').map(|item| hash(item)).sum();
    });
    total
}
fn solve2(path: &str) -> usize {
    let reader = get_file(path);

    reader.lines().map(Result::unwrap).for_each(|line| {
    });

    0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    pub fn example() {
        assert_eq!(1320, solve("e_day_15.input", ));
    }
    #[test]
    pub fn example2() {
        // assert_eq!(71503, solve2("e_day_15.input"));
    }
    #[test]
    pub fn puzzle() {
        println!("TOTAL = {}", solve("day_15.input"));
    }
    #[test]
    pub fn puzzle2() {
        // println!("TOTAL_2 = {}", solve2("day_15.input"));
    }
}
