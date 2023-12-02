use std::{env, fs::File, io::{BufReader, BufRead}};
use std::collections::HashMap;

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
    let max_colors = HashMap::from([("red", 12), ("green", 13), ("blue", 14)] );

    reader.lines().map(Result::unwrap).enumerate().for_each(|(i, line)| {
        // Game 1: (3, blue, 4, red); (1, red, 2, green, 6, blue); (2, green)
        let game_splits: Vec<&str> = line.split_once(": ").unwrap().1.split("; ").collect();

        let mut all_smaller = true;
        for item in game_splits.iter() {
            let mut current_game: HashMap<&str, usize> = HashMap::from([("red", 0), ("green", 0), ("blue", 0)]);
            for items in item.split(' ').collect::<Vec<&str>>().chunks(2){
                let amt = items[0].parse::<usize>().expect("Not an usize");
                let col: &str = &items[1].replace(",", "");
                *current_game.get_mut(col).unwrap() += amt;
            }
            for key in current_game.keys(){
                if max_colors.get(key).unwrap() < current_game.get(key).unwrap(){
                    all_smaller = false;
                }
            }
        }
        total += (i+1)*all_smaller as usize;
    });

    return total
}

fn solve2(path: &str) -> usize {
    let reader = get_file(path);
    let mut total: usize = 0;

    reader.lines().map(Result::unwrap).enumerate().for_each(|(i, line)| {
        // Game 1: (3, blue, 4, red); (1, red, 2, green, 6, blue); (2, green)
        let game_splits: Vec<&str> = line.split_once(": ").unwrap().1.split("; ").collect();

        let mut current_game: HashMap<&str, usize> = HashMap::from([("red", 0), ("green", 0), ("blue", 0)]);
        for item in game_splits.iter() {
            for items in item.split(' ').collect::<Vec<&str>>().chunks(2){
                let amt = items[0].parse::<usize>().expect("Not an usize");
                let col: &str = &items[1].replace(",", "");
                *current_game.get_mut(col).unwrap() = amt.max(*current_game.get(col).unwrap());
            }
        }
        let mut local_total = 1;
        for val in current_game.values(){
            local_total *= val;
        }
        total += local_total;
    });

    return total
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    pub fn example() {
        assert_eq!(8, solve("e_day_2.input"));
    }
    #[test]
    pub fn example2() {
        assert_eq!(2286, solve2("e_day_2.input"));
    }
    #[test]
    pub fn puzzle() {
        println!("TOTAL = {}", solve("day_2.input"));
    }
    #[test]
    pub fn puzzle2() {
        println!("TOTAL_2 = {}", solve2("day_2.input"));
    }
}
