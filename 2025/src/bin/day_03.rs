use advent_of_code::read_file_to_lines;
use std::env;
use std::str::FromStr;

fn main() {
    let mut args = env::args();
    // we don't need the first arg: the bin name
    args.next();
    let input_file: String = args.next().expect("No input file given.");
    let input_file_lines: Vec<String> = read_file_to_lines(input_file);

    let mut max_total_pairs: i64 = 0;
    let mut max_total_dozen: i64 = 0;
    // Loop through each line
    for line in &input_file_lines {
        let bank: PowerBank = line.parse().unwrap();
        max_total_pairs += bank.max_pair() as i64;
        max_total_dozen += bank.max_dozen();
    }
    println!("Max pairs: {:?}", max_total_pairs);
    println!("Max dozens: {:?}", max_total_dozen);
}

#[derive(Debug)]
pub struct PowerBank {
    power_bank: Vec<i32>,
}

impl PowerBank {
    fn max_pair(&self) -> i32 {
        // Find the position of the first occurence of the max digit
        let (_pos1, _dig1) = self.power_bank[0..self.power_bank.len() - 1]
            .iter()
            .enumerate()
            .rev()
            .max_by(|a, b| a.1.cmp(b.1))
            .unwrap();
        // Find the second largest digit sfter the first max one
        let (_pos2, _dig2) = self.power_bank[_pos1 + 1..]
            .iter()
            .enumerate()
            .rev()
            .max_by(|x, y| x.1.cmp(y.1))
            .unwrap();
        _dig1 * 10 + _dig2
    }
}

impl PowerBank {
    fn max_dozen(&self) -> i64 {
        let mut start = 0;
        let mut subsequence: i64 = 0;
        for i in 0..12 {
            let limit = 12 - 1 - i;
            let (position_1, &digit_1) = self.power_bank[start..self.power_bank.len() - limit]
                .iter()
                .enumerate()
                .rev()
                .max_by(|a, b| a.1.cmp(b.1))
                .unwrap();
            start += position_1 + 1;
            subsequence = (subsequence * 10) + digit_1 as i64;
            }
        subsequence
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

// Implement max_pair using nested for loop
//impl PowerBank {
//    fn max_pair(&self) -> i64 {
//        let mut max: i64 = 0;
//        for i in 0..self.power_bank.len() {
//            for j in i+1..self.power_bank.len() {
//                let pair: i64 = (self.power_bank[i] as i64) * 10 + (self.power_bank[j] as i64);
//                if pair > max {
//                    max = pair;
//                }
//            }
//        }
//        max
//    }
//}
