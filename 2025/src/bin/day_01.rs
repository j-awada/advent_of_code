use advent_of_code::read_file_to_lines;
use std::env;

fn main() {
    let mut args = env::args();
    // we don't need the first arg: the bin name
    args.next();
    let input_file: String = args.next().expect("No input file given.");
    let input_file_lines: Vec<String> = read_file_to_lines(input_file);

    let mut part1: InitDial = InitDial { value: 50, zeros: 0 };
    let mut part2: InitDial = InitDial { value: 50, zeros: 0 };
    
    for dial in input_file_lines {
        let (direction, distance) = dial.split_at(1);
        part1.turn_the_dial_part_1(direction, distance);
        part2.turn_the_dial_part_2(direction, distance);
    }
    println!("part1: {}", part1.zeros);
    println!("part2: {}", part2.zeros);
}

struct InitDial {
    value: i32,
    zeros: i32,
}

impl InitDial {
    fn turn_the_dial_part_1(&mut self, direction: &str, distance_str: &str) {
        let distance: i32 = distance_str.parse().unwrap();
        let turn_dial: i32 = match direction {
            "L" => self.value - distance,
            "R" => self.value + distance,
            _ => self.value,
        };

        // The least non-negative remainder
        self.value = turn_dial.rem_euclid(100);
        if self.value == 0 {
            self.zeros += 1;
        }
    }
}

impl InitDial {
    fn turn_the_dial_part_2(&mut self, direction: &str, distance_str: &str) {
        let distance: i32 = distance_str.parse().unwrap();
        let turn_dial: i32 = match direction {
            "L" => -1,
            "R" => 1,
            _ => self.value,
        };

        // The least non-negative remainder
        for _ in 0..distance {
            self.value = (self.value + turn_dial).rem_euclid(100);
            if self.value == 0 {
                self.zeros += 1
            }
        }
    }
}
