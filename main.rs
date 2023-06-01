use std::fs::{File};
use std::io::{self, BufReader, BufRead};

fn main() {
    let mut max_sum = 0;
    let mut cur_sum = 0;
    for line in read_lines(String::from("input.txt")) {
        let line = line.expect("Could not read line");
        if line.trim().is_empty() {
            max_sum = std::cmp::max(max_sum, cur_sum);
            cur_sum = 0;
            continue;
        }
        cur_sum += line.parse::<i32>().expect("Could not parse line to integer");
    }
    println!("{:?}", max_sum);
}

fn read_lines(filename: String) -> io::Lines<BufReader<File>> {
    let file = File::open(filename).expect("Could not open file"); 
    return io::BufReader::new(file).lines(); 
}
