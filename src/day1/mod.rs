use std::str::Lines;

pub fn run() {
    println!("Hello")
}

fn parse_input(lines: Lines) -> Vec<&mut Vec<i32>> {
    let mut result: Vec<&mut Vec<i32>> = Vec::new();
    result.push(&mut Vec::new());
    for line in lines {
        if line.is_empty() {
            result.push(&mut Vec::new())
        } else {
            let calories = result.last_mut().unwrap();
            calories.push(line.parse().unwrap())
        }
    }
    // I have no idea how to coerce this into Vec<Vec<i32>>
    // This language seem
    return result
}

fn get_total_calories_per_elf(calorieEntriesPerElf: Vec<Vec<i32>>) -> Vec<i32> {
    return Vec::new()
}

#[cfg(test)]
mod tests {
    use crate::day1::parse_input;

    #[test]
    fn example1() {
        let input = parse_input("1000\
                2000\
                3000\
                \
                4000\
                \
                6000\
                \
                7000\
                8000\
                9000\
                \
                10000".lines());
        println!("{}", input.len());
        // assert_eq!(df)
    }
}