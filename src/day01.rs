use std::vec;

pub fn part1(input_lines: Vec<String>) -> u32 {
    let mut result = 0;

    for line in input_lines {
        let mut numeric_chars = line.chars().filter_map(|x| x.to_digit(10));
        let first = numeric_chars.next().unwrap();
        let last = numeric_chars.last().unwrap_or(first);

        result += (10 * first) + last;
    }

    return result;
}

pub fn part2(input_lines: Vec<String>) -> u32 {
    let digits: Vec<&str> = vec![
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    let update = |mut first: Option<u32>, val: u32| {
        first = first.or(Some(val));

        return (first, val);
    };

    let mut result: u32 = 0;
    
    for line in input_lines {
        let mut first: Option<u32> = None;
        let mut last: u32 = 0;
        let mut illegal_until: i32 = -1;

        for (i, c) in line.chars().enumerate() {
            if illegal_until >= i as i32 {
                continue;
            }

            if let Some(d) = c.to_digit(10) {
                (first, last) = update(first, d);
                continue;
            }

            let word_digit = digits.iter()
                .enumerate() 
                .find(|(i, word)| {
                    println!("word {}", **word);
                    line[(*i)..].starts_with(**word)
            }); 
            if let Some((j, w)) = word_digit {
                (first, last) = update(first, (j+1) as u32);
                illegal_until = (i + (*w).len()) as i32;
            }
        };
        
        result += (10 * first.unwrap()) + last;
    }

    return result;
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    #[test]
    fn first_part() {
        let cal_doc = indoc! {"1abc2
                                    pqr3stu8vwx
                                    a1b2c3d4e5f
                                    treb7uchet"};

        let mut input_lines: Vec<String> = vec![];
        cal_doc
            .lines()
            .for_each(|x| input_lines.extend_from_slice(&[x.to_string()]));

        let result = super::part1(input_lines);
        assert_eq!(result, 142);
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

        let mut input_lines: Vec<String> = vec![];
        cal_doc
            .lines()
            .for_each(|x| input_lines.extend_from_slice(&[x.to_string()]));

        let result = super::part2(input_lines);
        assert_eq!(result, 281);
    }
}
