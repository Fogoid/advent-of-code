use std::{fs::File, io::Read};

pub fn read_file_to_lines(file_path: &String) -> String {
    let f_res = File::open(file_path);
    
    let mut f = match f_res {
        Ok(x) => x,
        Err(e) => panic!("Could not open file in given path. {}", e)
    };
    
    let mut buf = String::new();
    match f.read_to_string(&mut buf) {
        Err(e) => panic!("Could not read file. {}", e),
        Ok(_) => buf
    }
}

pub trait Solution {
    fn part1(&self, input: &str) -> String; 
    fn part2(&self, input: &str) -> String; 
}
