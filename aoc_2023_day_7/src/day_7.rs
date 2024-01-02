use std::{env, fs::File, io::{BufReader, BufRead}, collections::HashMap, cmp::Ordering};

pub fn get_file(file_name: &str) -> BufReader<File> {
    let f = File::open(format!(
        "{}/tests/{}",
        &env::current_dir().unwrap().to_string_lossy(),
        file_name
    ))
    .unwrap();

    BufReader::new(f)
}

static STRENGHT: [char; 13] = ['2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A'];
static STRENGHT2: [char; 13] = ['J', '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'Q', 'K', 'A'];

fn to_hand_set2(hand: &str) -> HashMap<char, usize> {
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

#[derive(Debug, PartialEq, PartialOrd)]
enum HandRank {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind
}

fn get_hand_rank(hand: &str, part_2: bool) -> HandRank {
    let mut hand_set = to_hand_set(&hand);
    let mut j_amt = 0;
    if part_2 {
        j_amt = hand_set.remove(&'J').unwrap_or(0);
        // This seems to handle all the cases where J is the second largest set of cards.
        if j_amt >= 4 {
            return HandRank::FiveOfAKind;
        }
    }
    let mut amts = hand_set.values().map(|&item| item).collect::<Vec<usize>>();
    amts.sort();
    amts.reverse();
    let mut largest = *amts.get(0).unwrap();
    if part_2 {
        largest += j_amt;
    }
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

fn comp_strings(a: &str, b: &str, part_2: bool) -> Ordering {
    let strenghts = match part_2{
        false => STRENGHT,
        true => STRENGHT2,
    };
    for (char_a, char_b) in a.chars().zip(b.chars()) {
        if char_a == char_b {
            continue;
        }
        for char in strenghts.iter() {
            if *char == char_a{
                return Ordering::Less;
            } else if *char == char_b {
                return Ordering::Greater;
            }
        }
    }
    Ordering::Equal
}

fn is_hand_bigger(a: &str, b: &str, part_2: bool) -> Ordering {
    let rank_a = get_hand_rank(a, part_2);
    let rank_b = get_hand_rank(b, part_2);
    if rank_a > rank_b {
        return Ordering::Greater;
    } else if rank_a < rank_b {
        return Ordering::Less;
    } else {
        return comp_strings(a, b, part_2);
    }
}

fn solve(path: &str, part_2: bool) -> usize {
    let reader = get_file(path);
    let mut hands: Vec<(String, usize)> = Vec::new();

    reader.lines().map(Result::unwrap).for_each(|line| {
        let mut items = line.split_whitespace();
        let hand = items.next().unwrap();
        let bid = items.next().unwrap();
        hands.push((hand.to_string(), bid.parse::<usize>().unwrap()));
    });

    // TODO: sort by rank
    hands.sort_by(|a, b| is_hand_bigger(&a.0, &b.0, part_2));

    hands.iter().enumerate().map(|(ith, item)| (ith+1)*item.1).sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    pub fn example() {
        assert_eq!(6440, solve("e_day_7.input", false));
    }
    #[test]
    pub fn example2() {
        assert_eq!(5905, solve("e_day_7.input", true));
    }
    #[test]
    pub fn puzzle() {
        println!("TOTAL = {}", solve("day_7.input", false));
    }
    #[test]
    pub fn puzzle2() {
        println!("TOTAL_2 = {}", solve("day_7.input", true));
    }
    #[test]
    pub fn comparison() {
        assert_eq!(Ordering::Less, is_hand_bigger("JKKK2", "QQQQ2", true))
    }
    #[test]
    pub fn comparison2() {
        assert_eq!(Ordering::Less, is_hand_bigger("T55J5", "QQQJA", true))
    }
    #[test]
    pub fn hand_size_test() {
        assert_eq!(HandRank::ThreeOfAKind, get_hand_rank("JJ3Q4", true))
    }
}
