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
    let mut total = 0;
    let mut map: Vec<u8> = Vec::new();
    let mut width = 0;
    let mut height = 0;

    reader.lines().map(Result::unwrap).for_each(|line| {
        width = line.len();
        for character in line.chars(){
            match character {
                '.' => map.push(0),
                'O' => map.push(1),
                '#' => map.push(2),
                _ => continue,
            }
        }
        height += 1;

    });
    let map_size: usize = width*height;

    for ith in 0..map_size {
        let item = map.get(ith).unwrap();
        if *item == 1 {
            let x = ith % width;
            let y = ith / width;
            // propagate up.
            let mut new_coord: usize = ith;
            for new_y in (0..y).rev() {
                let new_loc_item = map.get(x+new_y*width).unwrap();
                if *new_loc_item == 0 {
                    new_coord = x+new_y*width;
                } else {
                    break;
                }
            }
            let old_loc = map.get_mut(ith).unwrap(); 
            *old_loc = 0;
            let new_loc = map.get_mut(new_coord).unwrap();
            *new_loc = 1;
            
            // scoring the total load.
            let new_y = new_coord as usize / width;
            let height_load = height - new_y;
            total += height_load;
        }
    }

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
        assert_eq!(136, solve("e_day_14.input", ));
    }
    #[test]
    pub fn example2() {
        // assert_eq!(71503, solve2("e_day_14.input"));
    }
    #[test]
    pub fn puzzle() {
        println!("TOTAL = {}", solve("day_14.input"));
    }
    #[test]
    pub fn puzzle2() {
        // println!("TOTAL_2 = {}", solve2("day_14.input"));
    }
}
