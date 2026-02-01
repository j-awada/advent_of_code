use advent_of_code::read_file_to_lines;
use std::env;

fn main() {
    let mut args = env::args();
    // we don't need the first arg: the bin name
    args.next();
    let input_file: String = args.next().expect("No input file given.");
    let input_file_lines: Vec<String> = read_file_to_lines(input_file);

    let total: i64 = input_file_lines[0]
        .split(",")
        .flat_map(|row| {
            let mut range_limits = row.split("-");
            let start = range_limits.next().expect("Expecting something").parse::<i64>().expect("Parse error");
            let end = range_limits.next().expect("Expecting something").parse::<i64>().expect("Parse error");
            start..=end
        })
        .filter(|&i| {
            let input: Range = Range { value: i.to_string(), length: i.to_string().len() };
            input.has_equal_halves() ||
            input.one_repeated_number() || // optional for part 2
            input.several_repeated_patterns() // optional for part 2
        })
        .sum();
    println!("{}", total);
}

struct Range {
    value: String,
    length: usize,
}

impl Range {
    fn has_equal_halves(&self) -> bool {
        /* Range string is made up of 2 identical halves */
        self.length.is_multiple_of(2) &&
        self.value[0..self.length/2] == self.value[self.length/2..self.length]
    }
    fn one_repeated_number(&self) -> bool {
        /* Range string consists of a single number that is repeated N times */
        self.length > 1 &&
        self.value.chars().all(|c| c == self.value.chars().next().unwrap())
    }
    fn several_repeated_patterns(&self) -> bool {
        /* Range string is made of N repeated substrings */
        let max_limit: usize = self.length/2;
        for j in 2..=max_limit {
            let substrings: Vec<&str> = self.value
                .as_bytes()
                .chunks(j)
                .map(|chunk| std::str::from_utf8(chunk).unwrap())
                .collect();
            if substrings.iter().all(|p| p == &substrings[0]) {
                return true;
            }
        }
        false
    }
}
