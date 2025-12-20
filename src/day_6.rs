use std::fs;

pub fn part_1() {
    let binding = fs::read_to_string("inputs/day-6.txt").expect("Could not read file");
    let content = binding.trim();

    let input: Vec<Vec<&str>> = content
        .lines()
        .map(|l| l.split_whitespace().collect::<Vec<&str>>())
        .collect();

    let nums: Vec<Vec<u64>> = input
        .iter()
        .take(input.len() - 1)
        .map(|l| {
            l.iter()
                .map(|s| s.parse::<u64>().unwrap())
                .collect::<Vec<u64>>()
        })
        .collect();

    let ops = input.last().expect("Could not get last row");

    let mut result = 0;
    for i in 0..ops.len() {
        let op = ops[i];

        let mut n = Vec::new();
        for row in &nums {
            n.push(row[i]);
        }

        result += match op {
            "+" => n.iter().fold(0, |acc, d| acc + d),
            "-" => n.iter().fold(0, |acc, d| acc - d),
            "*" => n.iter().skip(1).fold(*n.first().unwrap(), |acc, d| acc * d),
            _ => panic!("Dafuq"),
        };
    }

    println!("{}", result)
}
