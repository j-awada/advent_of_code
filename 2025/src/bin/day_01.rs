struct InitDial {
    value: i32,
    zeros: i32,
}

impl InitDial {
    fn turn_the_dial_part_1(&mut self, direction: &str, distance_str: &str) {
        let distance: i32 = distance_str.parse().unwrap();
        let temp: i32 = match direction {
            "L" => self.value - distance,
            "R" => self.value + distance,
            _ => self.value,
        };

        // Count zeros crossed
        //let divid: i32 = temp / 100;
        //if divid.abs() > 0 {
        //    self.zeros += divid.abs();//.ceil();
        //}

        self.value = temp.rem_euclid(100);
        if self.value == 0 {
            self.zeros += 1;
        }
    }
}

fn main() {
    let mut start: InitDial = InitDial { value: 50, zeros: 0 };
    let dials: Vec<&str> = vec!["L68", "L30", "R48", "L5", "R60", "L55", "L1", "L99", "R14", "L82"];
    
    for dial in &dials {
        let (direction, distance) = dial.split_at(1);
        start.turn_the_dial_part_1(direction, distance);
    }
    println!("part1: {}", start.zeros);
    //println!("part2: {}", part_2(&dials));
}
