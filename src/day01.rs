use crate::commons::Solution;

pub struct Day01;

pub const DIGITS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

impl Solution for Day01 {
    fn part1(&self, input: &str) -> String {
        let mut result = 0;
    
        for line in input.lines() {
            let mut numeric_chars = line.chars().filter_map(|x| x.to_digit(10));
            let first = numeric_chars.next().unwrap();
            let last = numeric_chars.last().unwrap_or(first);
    
            result += (10 * first) + last;
        }
    
        return result.to_string();
    }
    
    fn part2(&self, input: &str) -> String {
        let update = |mut first: Option<u32>, val: u32| {
            first = first.or(Some(val));
    
            return (first, val);
        };
    
        let mut result: u32 = 0;
        
        for line in input.lines() {
            let mut first: Option<u32> = None;
            let mut last: u32 = 0;
    
            for (i, c) in line.chars().enumerate() {
                if let Some(d) = c.to_digit(10) {
                    (first, last) = update(first, d);
                    continue;
                }

                let word_digit = DIGITS.iter()
                    .enumerate() 
                    .find(|(_, word)| {
                        line[i..].starts_with(**word)
                }); 
                if let Some((j, _)) = word_digit {
                    (first, last) = update(first, (j+1) as u32);
                }
            };
            
            result += (10 * first.unwrap()) + last;
        }
    
        return result.to_string();
    }
}


#[cfg(test)]
mod tests {
    use indoc::indoc;
    use crate::commons::Solution;

    #[test]
    fn first_part() {
        let cal_doc = indoc! {"1abc2
                                    pqr3stu8vwx
                                    a1b2c3d4e5f
                                    treb7uchet"};

        let result = super::Day01.part1(cal_doc);
        assert_eq!(result, "142");
    }

    #[test]
    fn second_part() {
        let cal_doc = indoc! {"two1nine
                                    eightwothree
                                    abcone2threexyz
                                    xtwone3four
                                    4nineeightseven2
                                    zoneight234
                                    7pqrstsixteen"};

        let result = super::Day01.part2(cal_doc);
        assert_eq!(result, "281");
    }
}
