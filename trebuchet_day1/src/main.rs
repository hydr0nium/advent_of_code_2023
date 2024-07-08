use std::fs;

fn main() {
    solution1();
    solution2();
}

fn solution2() {
    let input: String = fs::read_to_string("./input.txt").expect("Could not read from input file!");
    let results: Vec<_> = input
        .lines()
        .into_iter()
        .map(|line| replace_ascci_num(line))
        .map(|line| line.chars().filter(|c| c.is_numeric()).collect::<String>())
        .flat_map(|line| {
            line.chars().next().map(|f| {
                line.chars().next_back().map(|l| {
                    (f.to_string() + &(l.to_string()))
                        .parse::<u32>()
                        .expect("Could not form a int from the two substrings!")
                })
            })
        })
        .flatten()
        .collect();
    dbg!(results.iter().sum::<u32>());
}

fn replace_ascci_num(line: &str) -> String {
    if line.len() == 0 {
        return String::from("");
    }
    let nums = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    for (num, s) in nums.iter().enumerate() {
        if line.starts_with(*s) {
            let r = (num + 1).to_string() + &(replace_ascci_num(&line[1..]));
            return r;
        }
    }
    let r = String::from(&line[0..1]) + &(replace_ascci_num(&line[1..]));
    return r;
}

fn solution1() {
    let input: String = fs::read_to_string("./input.txt").expect("Could not open file");
    let ints: Vec<_> = input
        .lines()
        .into_iter()
        .map(|val| val.chars().filter(|c| c.is_numeric()))
        .map(|n| n.collect::<String>())
        .collect();

    let res: u32 = ints
        .into_iter()
        .map(|s| {
            s.chars()
                .next()
                .map(|c| {
                    s.chars().next_back().map(|k| {
                        (c.to_string() + &(k.to_string()))
                            .parse::<u32>()
                            .expect("Could not form a int from the two substrings!")
                    })
                })
                .flatten()
        })
        .flatten()
        .sum::<u32>();
    dbg!(res);
}
