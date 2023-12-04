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
    let mut card_wins: HashMap<usize, usize> = HashMap::new(); // maps card nr to amt of winnings.
    let mut last_card = 0;

    reader.lines().map(Result::unwrap).enumerate().for_each(|(ith, line)| {
        let card_id = ith + 1;
        last_card = card_id;
        if !card_wins.contains_key(&card_id) {
            card_wins.insert(card_id, 1);
        } else {
            *card_wins.get_mut(&card_id).unwrap() += 1;
        }
        let mut local_total = 0;
        let mut nums = line.split(": ").nth(1).unwrap().split("|");
        let winning = nums.next().unwrap().split_whitespace();
        let own_nums = nums.next().unwrap().split_whitespace();
        for own in own_nums {
            for win in winning.clone(){
                local_total += (own == win) as usize
            }
        }
        let multiplier = *card_wins.get(&card_id).unwrap();
        for i in card_id+1..card_id+1+local_total{
            if let Some(amt) = card_wins.get_mut(&i) {
                *amt+=1*multiplier;
            } else{
                card_wins.insert(i, 1 * multiplier);
            }
        }
    });
    for keys in card_wins.keys(){
        if *keys <= last_card{
            total += card_wins.get(keys).unwrap();
        }
    }

    total
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
