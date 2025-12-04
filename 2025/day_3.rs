const INPUT: &str = "
987654321111111
811111111111119
234234234234278
818181911112111";

fn main() {
    let lines: Vec<&str> = INPUT.lines().collect();
    let mut max_joltage: i32 = 0;
    for line in lines {
        let line_arr: Vec<i32> = line
            .chars()
            .map(|s| s.to_digit(10).unwrap() as i32)
            .collect();
        max_joltage += max_pair(&line_arr);
    }
    println!("{}", max_joltage);
}

fn max_pair(arr: &Vec<i32>) -> i32 {
    let mut max: i32 = 0;
    for i in 0..arr.len() {
        for j in i+1..arr.len()  {
            let pair: i32 = arr[i] * 10 + arr[j];
            if pair > max {
                max = pair;
            }
        }
    }
    max
}
