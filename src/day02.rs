use std::collections::HashMap;

use crate::commons::Solution;

pub struct Day02;

impl Solution for Day02 {
    fn part1(&self, input: &str) -> String {
        let available_cubes: HashMap<&str, i32> = HashMap::from([
            ("red", 12),
            ("green", 13),
            ("blue", 14),
        ]);
        let mut result = 0; 

        for line in input.lines() { 
            let mut record = line.split(":");
            let game_id: i32 = record.next().unwrap()
                .split(" ").last().unwrap()
                .parse().unwrap();

            let game_rounds = record.next().unwrap().trim_start();
            let is_valid = game_rounds.split(";").map(|round| {
                return round.trim_start().split(",")
                    .map(|cube| {
                        let mut c = cube.trim_start().split(" ");
                        let n = c.next().unwrap().parse::<i32>().unwrap();
                        let color = c.next().unwrap();
                        (color, n)
                    }).all(|(color, n)| *available_cubes.get(color).unwrap() >= n); 
            }).all(|round| round);

            if is_valid {
                result += game_id; 
            }
        }
        return result.to_string();
    }

    fn part2(&self, input: &str) -> String {
        let mut result = 0; 
        
        for line in input.lines() { 
            let mut minimal_cubes = HashMap::from([
                ("blue", 0),
                ("red", 0),
                ("green", 0),
            ]);

            let game_record = line.split(":").last().unwrap().trim_start();
            let rounds = game_record.split(";");
            for round in rounds {
                for cube in round.trim_start().split(",") {
                    let mut cube = cube.trim_start().split(" ");
                    let taken = (cube.next().unwrap().parse::<i32>().unwrap(),
                        cube.next().unwrap()); 
                    let color_min = minimal_cubes.get_mut(taken.1).unwrap();
                    if *color_min < taken.0 {*color_min = taken.0}
                };
                
            }
            
            result += minimal_cubes.iter().map(|x| *(x.1)).reduce(|x, y| x * y).unwrap(); 
        }
        return result.to_string();
    }
}

#[cfg(test)]
mod tests {
    use indoc::indoc;
    use crate::commons::Solution;

    const PUZZLE_INPUT: &'static str = indoc! {
        "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green" };

    #[test]
    fn first_part() {
        let result = super::Day02.part1(PUZZLE_INPUT);
        assert_eq!(result, "8");
    }

    #[test]
    fn second_part() {
        let result = super::Day02.part2(PUZZLE_INPUT);
        assert_eq!(result, "2286");
    }
}