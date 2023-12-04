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

fn solve(path: &str) -> usize {
    let reader = get_file(path);
    let mut total: usize = 0;

    reader.lines().map(Result::unwrap).for_each(|line| {
        let mut local_total = 0;
        let mut nums = line.split(": ").nth(1).unwrap().split("|");
        let winning = nums.next().unwrap().split_whitespace();
        let own_nums = nums.next().unwrap().split_whitespace();
        for own in own_nums {
            for win in winning.clone(){
                local_total += (own == win) as usize
            }
        }
        let power: i32 = 2;
        if local_total != 0 {
            total += power.pow((local_total - 1)as u32) as usize;
        }
    });

    return total

}

fn solve2(path: &str) -> usize {
    let reader = get_file(path);
    let mut total: usize = 0;

    reader.lines().map(Result::unwrap).for_each(|line| {
    });

    return total

}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    pub fn example() {
        assert_eq!(13, solve("e_day_4.input"));
    }
    #[test]
    pub fn example2() {
        assert_eq!(30, solve2("e_day_4.input"));
    }
    #[test]
    pub fn puzzle() {
        println!("TOTAL = {}", solve("day_4.input"));
    }
    #[test]
    pub fn puzzle2() {
        println!("TOTAL_2 = {}", solve2("day_4.input"));
    }
}
