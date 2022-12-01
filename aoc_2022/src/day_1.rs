use std::io::BufRead;

use crate::tools::get_file;

pub fn max_calories(file_name: &str, top: u32) -> u32 {
    // get data file.
    let reader = get_file(file_name);

    let mut total_calories_to_return: u32 = 0;
    let mut current_elf: u32 = 0;
    let mut current_total_calories = 0;
    let mut calorie_collection = Vec::new();

    // parse each line.
    reader.lines().map(Result::unwrap).for_each(|line| {
        let trimmed_line = line.trim();
        if trimmed_line == "" {
            calorie_collection.push(current_total_calories);
            current_elf += 1;
            current_total_calories = 0;
        } else {
            let calories = line
                .parse::<u32>()
                .expect("Could not parse u32 from string");
            current_total_calories += calories;
        }
    });
    // sort elf calories.
    calorie_collection.sort();
    // largest in the front.
    calorie_collection.reverse();
    for i in 0..top {
        total_calories_to_return += calorie_collection[i as usize];
    }
    total_calories_to_return
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    pub fn example() {
        // top = 1, get max elf.
        assert_eq!(24000, max_calories("example_day_1.input", 1));
    }
    #[test]
    pub fn puzzle() {
        assert_eq!(72478, max_calories("day_1.input", 1));
        assert_eq!(210367, max_calories("day_1.input", 3));
    }
}
