use std::str::FromStr;
use std::env;
use advent_of_code::read_file_to_lines;

#[derive(Debug)]
pub struct PowerBank {
    power_bank: Vec<i32>,
}

impl PowerBank {
    fn max_pair(&self) -> i64 {
        let mut max: i64 = 0;
        for i in 0..self.power_bank.len() {
            for j in i+1..self.power_bank.len() {
                let pair: i64 = (self.power_bank[i] as i64) * 10 + (self.power_bank[j] as i64);
                if pair > max {
                    max = pair;
                }
            }
        }
        max
    }
}

impl FromStr for PowerBank {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(PowerBank {
            power_bank: s.chars().map(|s| s.to_digit(10).unwrap() as i32).collect(),
        })
    }
}

fn main() {
    let mut args = env::args();
    // we don't need the first arg: the bin name
    args.next();
    let input_file: String = args.next().expect("No input file given.");

    let input_file_lines: Vec<String> = read_file_to_lines(input_file);

    let mut max_total_pairs: i64 = 0;
    for line in input_file_lines {
        let bank: PowerBank = line.parse().unwrap();
        max_total_pairs += bank.max_pair();
    }

    println!("Max pairs: {:?}", max_total_pairs);
}