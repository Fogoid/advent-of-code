use crate::commons::Solution;

pub struct Day04;

impl Day04 {
    fn map_filter_numbers(numbers: &str) -> Vec<i32> {
        numbers.trim().split(" ")
            .filter_map(|n| { match n {
                    "" => None,
                    x => Some(x.trim().parse().unwrap())
                }
            }).collect()
    }
}

impl Solution for Day04 {
    fn part1(&self, input: &str) -> String {
        let val: i32 = input.lines().filter_map(|line| {
            let (_, line) = line.split_once(":").unwrap();
            let (winning, mine) = line.split_once("|").unwrap();
            let winning: Vec<i32> = Day04::map_filter_numbers(winning);
            let mine: Vec<i32> = Day04::map_filter_numbers(mine);

            let mut exponent: u32 = 0;
            winning.into_iter()
                .filter(|w| mine.contains(w))
                .for_each(|_| exponent = exponent + 1);

            match exponent {
                0 => None,
                x => Some(2_i32.pow(x - 1))
            }
        }).sum();

        val.to_string()
    }

    fn part2(&self, input: &str) -> String {
        return input.to_string();
    }
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use crate::commons::Solution;

    const PUZZLE_INPUT: &str = indoc!{
        "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
        Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
        Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
        Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
        Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
        Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"
    };

    #[test]
    fn part_one_test() {
        let result = super::Day04.part1(PUZZLE_INPUT);
        assert_eq!(result, "13");
    }
}