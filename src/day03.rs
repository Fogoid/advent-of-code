use std::{collections::HashMap, vec};

use regex::Regex;

use crate::commons::Solution;

pub struct Day03;

impl Solution for Day03 {
    fn part1(&self, input: &str) -> String {
        let re = Regex::new(r"(\d+)").unwrap();    
        let input: Vec<&str> = input.lines().collect();
        let mut result = 0;

        for i in 0..input.len() {
            let line = *input.get(i).unwrap();    
            let line_chars: Vec<char> = line.chars().collect();
            let line_val = re.find_iter(line).filter_map(|m|{
                let n: i32 = m.as_str().parse().unwrap();
                let start = m.start() as i32;
                let left_bound = m.start() - (if start == 0 {0} else {1});
                let right_bound = m.end() - (if m.end() < line.len() {0} else {1});

                if left_bound != m.start() && line_chars.get(left_bound).is_some_and(|c| c.is_ascii_punctuation() && *c != '.') {
                    return Some(n);
                } 
                if right_bound != line.len() && line_chars.get(right_bound).is_some_and(|c| c.is_ascii_punctuation() && *c != '.') {
                    return Some(n);
                } 
                if i > 0 && 
                    input.get(i-1).unwrap().chars().enumerate()
                        .any(|(j, c)| j >= left_bound && j <= right_bound && c.is_ascii_punctuation() && c != '.') {
                    return Some(n);
                }
                if i < line.len()-1 && 
                    input.get(i + 1).unwrap().chars().enumerate()
                        .any(|(j, c)| j >= left_bound && j <= right_bound && c.is_ascii_punctuation() && c != '.') {
                    return Some(n);
                }
                
                return None;
            }).reduce(|x, y| x + y);

            if let Some(x) = line_val {
                result += x;
            }
        }

        return result.to_string();
    }

    fn part2(&self, input: &str) -> String {
        let re = Regex::new(r"(\d+)").unwrap();    
        let input: Vec<&str> = input.lines().collect();
        let mut gears: HashMap<String, Gear> = HashMap::new();

        for i in 0..input.len() {
            let line = *input.get(i).unwrap();    
            let line_chars: Vec<char> = line.chars().collect();
            re.find_iter(line).for_each(|m|{
                let n: i32 = m.as_str().parse().unwrap();
                let start = m.start() as i32;
                let left_bound = m.start() - (if start == 0 {0} else {1});
                let right_bound = m.end() - (if m.end() < line.len() {0} else {1});

                if left_bound != m.start() && line_chars.get(left_bound).is_some_and(|c| *c == '*') {
                    let key = format!("{}:{}", i, left_bound);
                    add_gear_count(&mut gears, key, n);
                } 
                if right_bound != line.len() && line_chars.get(right_bound).is_some_and(|c| *c == '*') {
                    let key = format!("{}:{}", i, right_bound);
                    add_gear_count(&mut gears, key, n);
                } 
                if i > 0 { 
                    input.get(i-1).unwrap().chars().enumerate()
                        .filter(|(j, c)| *j >= left_bound && *j <= right_bound && *c == '*')
                        .for_each(|(j, _)| {
                            let key = format!("{}:{}", i-1, j);
                            add_gear_count(&mut gears, key, n);
                        }); 
                }
                if i < line.len()-1 { 
                    input.get(i + 1).unwrap().chars().enumerate()
                        .filter(|(j, c)| *j >= left_bound && *j <= right_bound && *c == '*')
                        .for_each(|(j, _)| {
                            let key = format!("{}:{}", i+1, j);
                            add_gear_count(&mut gears, key, n);
                        }); 
                }
            });

        }

        return gears.into_iter()
            .filter_map(|(_, g)| {
                match g.n {
                    2 => Some(g.joints.into_iter().reduce(|x, y| x*y)).unwrap(),
                    _ => None
                }
            }).reduce(|x,y| x+y).unwrap_or(0)
            .to_string();

    }

}

fn add_gear_count(gears_map: &mut HashMap<String, Gear>, key: String, value: i32) {
    let gear = gears_map.get_mut(&key);
    match gear {
        Some(x) => {(*x).add(value);},
        None => {gears_map.insert(key, Gear::new(value));}
    }
}

struct Gear {
    n: usize,
    joints: Vec<i32>,
}

impl Gear {
    pub fn new(joint: i32) -> Gear {
        Gear { n: 1, joints: vec![joint] }
    }

    pub fn add(&mut self, joint: i32) {
        self.n += 1;
        self.joints.extend_from_slice(&[joint]);
    }
}

#[cfg(test)]
mod tests {
    use indoc::indoc;
    use crate::commons::Solution;

    const INPUT_ONE: &str = indoc! {
        "467..114..
        ...*......
        ..35..633.
        ......#...
        617*......
        .....+.58.
        ..592.....
        ......755.
        ...$.*....
        .664.598.."
    };

    #[test]
    fn part_one_test()  {
        let result = super::Day03.part1(INPUT_ONE);
        assert_eq!(result, "4361")
    }

    #[test]
    fn part_two_test()  {
        let result = super::Day03.part2(INPUT_ONE);
        assert_eq!(result, "467835")
    }
}