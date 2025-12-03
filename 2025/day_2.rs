//const INPUT: &str = "52-75,71615244-71792700,89451761-89562523,594077-672686,31503-39016,733-976,1-20,400309-479672,458-635,836793365-836858811,3395595155-3395672258,290-391,5168-7482,4545413413-4545538932,65590172-65702074,25-42,221412-256187,873499-1078482,118-154,68597355-68768392,102907-146478,4251706-4487069,64895-87330,8664371543-8664413195,4091-5065,537300-565631,77-115,83892238-83982935,6631446-6694349,1112-1649,7725-9776,1453397-1493799,10240-12328,15873-20410,1925-2744,4362535948-4362554186,3078725-3256936,710512-853550,279817-346202,45515-60928,3240-3952";

const INPUT: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";


fn main() {
    let total: i64 = INPUT
        .split(",")
        .flat_map(|row| {
            let mut range_limits = row.split("-");
            let start = range_limits.next().expect("Expecting something").parse::<i64>().expect("Parse error");
            let end = range_limits.next().expect("Expecting something").parse::<i64>().expect("Parse error");
            start..=end
        })
        .filter(|&i| {
            let input: Input = Input { value: i.to_string(), length: i.to_string().len() };
            input.has_equal_halves() ||
            input.one_repeated_number() || // optional for part 2
            input.several_repeated_patterns() // optional for part 2
        })
        .sum();
    println!("{}", total);
}

struct Input {
    value: String,
    length: usize,
}

impl Input {
    fn has_equal_halves(&self) -> bool {
        /* Input string is made up of 2 identical halves */
        self.length % 2 == 0 &&
        self.value[0..self.length/2] == self.value[self.length/2..self.length]
    }
    fn one_repeated_number(&self) -> bool {
        /* Input string consists of a single number that is repeated N times */
        self.length > 1 &&
        self.value.chars().all(|c| c == self.value.chars().next().unwrap())
    }
    fn several_repeated_patterns(&self) -> bool {
        /* Input string is made of N repeated substrings */
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
        return false;
    }
}
