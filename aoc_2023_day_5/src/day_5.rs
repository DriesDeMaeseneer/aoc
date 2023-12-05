use std::{env, fs::File, io::{BufReader, BufRead}, collections::HashMap};

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
    let mut current_locs = Vec::new();
    let mut pre_current_locs = Vec::new();

    reader.lines().map(Result::unwrap).for_each(|line| {
        if line.chars().next().is_none(){
        } else if line.chars().nth(0).unwrap().is_numeric(){
            let cur_line_split: Vec<u64> = line.split_whitespace().map(|item| item.parse::<u64>().unwrap()).collect();
            let output_start:u64 = *cur_line_split.get(0).unwrap();
            let input_start: u64 = *cur_line_split.get(1).unwrap();
            let range_amt:u64 = *cur_line_split.get(2).unwrap();
            for (ith, cur_loc) in current_locs.iter().enumerate() {
                if *cur_loc >= input_start && *cur_loc <= input_start+range_amt {
                    if let Some(loc) = pre_current_locs.get_mut(ith) {
                        *loc = output_start + (*cur_loc - input_start);
                    }
                }
            }
        } else if line.starts_with("seeds: "){
            current_locs = line.split(": ").nth(1).unwrap().split_whitespace().map(|item| item.parse::<u64>().expect("could not parse number of seeds")).collect();
            pre_current_locs = current_locs.clone();
        } else {
            current_locs = pre_current_locs.clone();
        }
    });

    return *pre_current_locs.iter().min().unwrap() as usize;
}

fn solve2(path: &str) -> usize {
    let reader = get_file(path);
    let mut total: usize = 0;

    reader.lines().map(Result::unwrap).enumerate().for_each(|(ith, line)| {
    });

    total
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    pub fn example() {
        assert_eq!(35, solve("e_day_5.input"));
    }
    #[test]
    pub fn example2() {
        // assert_eq!(30, solve2("e_day_5.input"));
    }
    #[test]
    pub fn puzzle() {
        println!("TOTAL = {}", solve("day_5.input"));
    }
    #[test]
    pub fn puzzle2() {
        // println!("TOTAL_2 = {}", solve2("day_5.input"));
    }
}
