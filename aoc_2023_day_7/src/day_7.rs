use std::{env, fs::File, io::{BufReader, BufRead}, collections::{HashMap, hash_set}, cmp::Ordering};

pub fn get_file(file_name: &str) -> BufReader<File> {
    let f = File::open(format!(
        "{}/tests/{}",
        &env::current_dir().unwrap().to_string_lossy(),
        file_name
    ))
    .unwrap();

    BufReader::new(f)
}

static STRENGHT: [char; 13] = ['2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A' ];

fn to_hand_set(hand: &str) -> HashMap<char, usize> {
    let mut hand_set = HashMap::new();
    for item in hand.chars() {
        match hand_set.get_mut(&item){
            Some(loc) => *loc += 1,
            None => {
                let _ = hand_set.insert(item, 1);},
        }
    }
    hand_set
}
#[derive(PartialEq, PartialOrd)]
enum HandRank {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind
}
fn get_hand_rank(hand: &str) -> HandRank{
    let hand_set = to_hand_set(&hand);
    let mut amts = hand_set.values().map(|&item| item).collect::<Vec<usize>>();
    amts.sort();
    amts.reverse();
    let largest = *amts.get(0).unwrap();
    match largest {
        5 => return HandRank::FiveOfAKind,
        4 => return HandRank::FourOfAKind,
        3 => {
            match *amts.get(1).unwrap() {
                2 => return HandRank::FullHouse,
                1 => return HandRank::ThreeOfAKind,
                _ => (),
            }
        }
        2 => {
            match *amts.get(1).unwrap() {
                2 => return HandRank::TwoPair,
                1 => return HandRank::OnePair,
                _ => (),
            }
        }
        _ => ()
    }
    HandRank::HighCard
}

fn is_hand_bigger(a: &str, b: &str) -> Ordering {
    let rank_a = get_hand_rank(a);
    let rank_b = get_hand_rank(b);
    if rank_a > rank_b {
        return Ordering::Greater;
    } else if rank_a < rank_b {
        return Ordering::Less;
    } else {
        for (char_a, char_b) in a.chars().zip(b.chars()) {
            if char_a == char_b {
                continue;
            }
            for char in STRENGHT.iter(){
                if *char == char_a{
                    return Ordering::Less;
                } else if *char == char_b {
                    return Ordering::Greater;
                }
            }
        }
    }
    Ordering::Equal
}

fn solve(path: &str) -> usize {
    let reader = get_file(path);
    type Hand = HashMap<char, usize>;
    let mut hands: Vec<(String, usize)> = Vec::new();

    reader.lines().map(Result::unwrap).for_each(|line| {
        let mut items = line.split_whitespace();
        let hand = items.next().unwrap();
        let bid = items.next().unwrap();
        hands.push((hand.to_string(), bid.parse::<usize>().unwrap()));
    });

    // TODO: sort by rank
    hands.sort_by(|a, b| is_hand_bigger(&a.0, &b.0));

    hands.iter().enumerate().map(|(ith, item)| (ith+1)*item.1).sum()
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
        assert_eq!(6440, solve("e_day_7.input"));
    }
    #[test]
    pub fn example2() {
        // assert_eq!(71503, solve2("e_day_7.input"));
    }
    #[test]
    pub fn puzzle() {
        println!("TOTAL = {}", solve("day_7.input"));
    }
    #[test]
    pub fn puzzle2() {
        // println!("TOTAL_2 = {}", solve2("day_7.input"));
    }
}
