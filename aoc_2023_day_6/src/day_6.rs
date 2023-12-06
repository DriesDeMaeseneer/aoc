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

fn combinations_to_beat(time: usize, dist: usize) -> usize {
    let mut combinations = 0;
    for num_of_milis in 0..=time {
        let travel_time = time - num_of_milis;
        let travel_distance = travel_time * num_of_milis;
        if travel_distance > dist{
            combinations += 1;
        }
    }
    combinations
}

fn solve(path: &str) -> usize {
    let reader = get_file(path);
    let mut total = 1;
    let mut races: Vec<(usize, usize)> = Vec::new();

    reader.lines().map(Result::unwrap).for_each(|line| {
        let mut items =  line.split_whitespace();
        let str = items.next().expect("No first item in the string.");

        if str == "Time:" {
            items
                .map(|item| item.parse::<usize>())
                .map(|item| item.unwrap())
                .for_each(|time| races.push((time, 0)));
        } else {
            items
                .map(|item| item.parse::<usize>())
                .map(|item| item.unwrap())
                .enumerate()
                .for_each(|(ith, distance)| *races.get_mut(ith).unwrap() = (races.get(ith).unwrap().0, distance));
        }
    });

    for (time, dist) in races.iter() {
        let combinations = combinations_to_beat(*time, *dist);
        total *= combinations;
    }

    total
}
fn solve2(path: &str) -> usize {
    let reader = get_file(path);
    let mut time = 0;
    let mut dist = 0;

    reader.lines().map(Result::unwrap).for_each(|line| {
        let mut items =  line.split_whitespace();
        let str = items.next().expect("No first item in the string.");

        let mut total_time:String = "".to_string();
        for str in items {
            total_time.push_str(str);
        }
        let local_item =total_time.parse::<usize>().unwrap();
        if str == "Time:" {
            time = local_item;
        } else {
            dist = local_item;
        }
    });

    combinations_to_beat(time, dist)
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    pub fn example() {
        assert_eq!(288, solve("e_day_6.input"));
    }
    #[test]
    pub fn example2() {
        assert_eq!(71503, solve2("e_day_6.input"));
    }
    #[test]
    pub fn puzzle() {
        println!("TOTAL = {}", solve("day_6.input"));
    }
    #[test]
    pub fn puzzle2() {
        println!("TOTAL_2 = {}", solve2("day_6.input"));
    }
}
