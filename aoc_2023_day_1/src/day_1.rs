use std::{env, fs::File, io::{BufReader, BufRead}};
use regex::{Regex, NoExpand};

pub fn get_file(file_name: &str) -> BufReader<File> {
    let f = File::open(format!(
        "{}/tests/{}",
        &env::current_dir().unwrap().to_string_lossy(),
        file_name
    ))
    .unwrap();

    BufReader::new(f)
}

fn solve(path: &str) -> usize {
    let reader = get_file(path);
    let mut total: usize = 0;

    reader.lines().map(Result::unwrap).for_each(|line| {
        let chars: Vec<char> = line.chars().filter(|character| character.is_digit(10)).collect();
        let first = *chars.first().unwrap() as usize - 48;
        let last = *chars.last().unwrap() as usize - 48;
        total += first*10 +last;
    });

    return total

}

fn get_first_and_last_char(line: &str ) -> (usize, usize){
    let first: usize; 
    let last: usize;
    let valid_strings = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    let mut indices = Vec::new();
    let mut end_ind = Vec::new();
    for (i, item) in valid_strings.iter().enumerate(){
        let re = Regex::new(item).unwrap();
        match re.find(&line){
            Some(index) => {
                indices.push((index.start(), i + 1));
            }
            None => ()
        };

        match re.find_iter(line).last(){
            Some(index) => end_ind.push((index.start(), i+1)),
            None => ()
        };
    }
    indices.sort();
    end_ind.sort();
    // first num
    let to_replace = indices.first().unwrap_or(&(usize::MAX, 11));
    let num_ind: Vec<(usize, usize)> = line.chars().enumerate().filter(|(_, c)| c.is_numeric()).map(|(i, c)| (i, c as usize - 48)).collect();
    let first_numeric = match num_ind.first() {
        Some((i, c)) => (*i, *c),
        None => (usize::MAX, 0)
    };
    if first_numeric.0 < to_replace.0{
        first = first_numeric.1 as usize ;
    } else {
        first = to_replace.1;
    }

    // last num
    let to_replace = end_ind.last().unwrap_or(&(usize::MIN, 11));
    let last_numeric = match num_ind.last() {
        Some((i, c)) => (*i, *c),
        None => (usize::MIN, 0)
    };
    if to_replace.1 == 11{
        last = last_numeric.1 as usize;
    }
    else if last_numeric.0 > to_replace.0 {
        last = last_numeric.1 as usize ;
    } else {
        last = to_replace.1;
    }

    (first, last)
}

fn solve2(path: &str) -> usize {
    let reader = get_file(path);
    let mut total: usize = 0;

    reader.lines().map(Result::unwrap).for_each(|line| {
        let line = line.to_string();
        // look for closest match and replace.  until no matches can be found.

        let (first, last) = get_first_and_last_char(&line);
        total += first*10 +last;
    });

    return total

}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    pub fn short_test(){
        assert_eq!(get_first_and_last_char("7v"), (7, 7));
    }
    #[test]
    pub fn example() {
        assert_eq!(142, solve("e_day_1.input"));
    }
    #[test]
    pub fn example2() {
        assert_eq!(281, solve2("e2_day_1.input"));
    }
    #[test]
    pub fn puzzle() {
        // println!("TOTAL = {}", solve("day_1.input"));
    }
    #[test]
    pub fn puzzle2() {
        println!("TOTAL = {}", solve2("day_1.input"));
    }
}
