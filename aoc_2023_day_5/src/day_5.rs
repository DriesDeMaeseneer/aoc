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

type Range = (u64, u64);

fn solve2(path: &str) -> usize {
    let reader = get_file(path);
    let mut ranges: Vec<Range> = Vec::new();

    let mut new_ranges: Vec<Range> = Vec::new();
    reader.lines().map(Result::unwrap).for_each(|line| {
        if line.chars().next().is_none() {
        } else if line.chars().nth(0).unwrap().is_numeric(){
            let mut new_line_ranges: Vec<Range> = Vec::new();
            let cur_line_split: Vec<u64> = line.split_whitespace().map(|item| item.parse::<u64>().unwrap()).collect();

            let output_start: u64 = *cur_line_split.get(0).unwrap();
            let input_start: u64 = *cur_line_split.get(1).unwrap();
            let range_amt: u64 = *cur_line_split.get(2).unwrap();

            for (start, amt) in ranges.iter() {

                println!("{start}, {amt}, {:?}", ranges);
                println!("{input_start}, {range_amt}\n");

                let mut start = *start;
                let mut amt = *amt;
                if (start > input_start + range_amt) || (start + amt < input_start){
                    println!("skipped");
                    new_line_ranges.push((start, amt));
                    continue; // there is no overlap
                } 
                let val: i64 = input_start as i64 - start as i64;
                println!("checking left, {val}");
                if val > 0 {
                    println!("added left range: {}, {}", start, val);
                    new_line_ranges.push((start, val as u64));
                    start = input_start;
                    amt -= val as u64;
                    println!("new_cur_range: {}, {}", start, amt);
                }
                let val: i64 = (start + amt) as i64 - (input_start + range_amt) as i64;
                println!("checking right, {val}");
                if val > 0 {
                    println!("added right range: {}, {}", input_start + range_amt, val);
                    new_line_ranges.push((input_start+range_amt, val as u64));
                    start = start;
                    amt -= val as u64;
                    println!("new_cur_range: {}, {}", start, amt);
                }
                if amt == 0 {
                    continue;
                }
                println!("substituted range: {}, {}", output_start + (start - input_start), amt);
                new_line_ranges.push((output_start + (start - input_start), amt));
            }
            ranges = new_line_ranges.clone();
        } else if line.starts_with("seeds: "){
            ranges = line
                .split(": ")
                .nth(1)
                .unwrap()
                .split_whitespace()
                .map(|item| item.parse::<u64>().expect("could not parse number of seeds"))
                .collect::<Vec<u64>>()
                .chunks(2)
                .map(|item| (item[0], item[1]))
                .collect();
            println!("{:?}", ranges);
        } else {
            if !new_ranges.is_empty() {
                ranges = new_ranges.clone();
                new_ranges.clear();
            }
            new_ranges.clear();
        }
    });
    ranges.iter().map(|item| item.0).min().unwrap() as usize
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    pub fn example() {
        // assert_eq!(35, solve("e_day_5.input"));
    }
    #[test]
    pub fn example2() {
        assert_eq!(46, solve2("e_day_5.input"));
    }
    #[test]
    pub fn puzzle() {
        // println!("TOTAL = {}", solve("day_5.input"));
    }
    #[test]
    pub fn puzzle2() {
        // println!("TOTAL_2 = {}", solve2("day_5.input"));
    }
}
