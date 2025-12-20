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

pub fn part_2() {
    let binding = fs::read_to_string("inputs/day-6.txt").expect("Could not read file");
    let content = binding.trim();

    let input: Vec<&str> = content.lines().collect();

    let ops: Vec<char> = input
        .last()
        .expect("Could not get last row")
        .chars()
        .collect();

    let nums: Vec<Vec<char>> = input
        .iter()
        .take(input.len() - 1)
        .map(|l| l.chars().collect())
        .collect();

    let mut result = 0;

    for i in 0..ops.len() {
        if !ops[i].is_whitespace() {
            let mut block_width = 0;

            for row in &nums {
                let width = row.iter().skip(i).take_while(|c| c.is_digit(10)).count();
                if width > block_width {
                    block_width = width;
                }
            }

            let mut vert_nums = Vec::new();
            for w in (i..block_width + i).rev() {
                let mut str_num = String::from("");
                for row in &nums {
                    str_num += &row[w].to_string();
                }
                vert_nums.push(
                    str_num
                        .trim()
                        .parse::<u64>()
                        .expect("could not parse vert num"),
                );
            }

            result += match ops[i] {
                '+' => vert_nums.iter().fold(0, |acc, d| acc + d),
                '-' => vert_nums.iter().fold(0, |acc, d| acc - d),
                '*' => vert_nums
                    .iter()
                    .skip(1)
                    .fold(*vert_nums.first().unwrap(), |acc, d| acc * d),
                _ => panic!("Dafuq"),
            };
        }
    }

    println!("{}", result);
}
