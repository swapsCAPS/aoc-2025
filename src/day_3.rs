use std::fs;

pub fn part_1() {
    let binding = fs::read_to_string("inputs/day-3.txt").expect("Could not read file");
    let content = binding.trim();

    let mut result: u64 = 0;

    for line in content.lines() {
        let binding = line
            .trim()
            .chars()
            .map(|c| c.to_string().parse::<u8>().unwrap())
            .enumerate()
            // Need double ended iterator
            .collect::<Vec<(usize, u8)>>();

        let (index, left) = binding
            .iter()
            // Reversing as max_by returns _last_ max, we want first max
            .rev()
            // We need 2 digits, skip the rightmost digit
            .skip(1)
            .max_by(|(_, n), (_, o)| n.cmp(o))
            .expect("Could not find max");

        // Now we have the max from the left with its index

        let rest = binding.iter().skip(*index + 1);

        let (_, right) = rest
            .max_by(|(_, n), (_, o)| n.cmp(o))
            .expect("Could not find max");

        let joltage = (left.to_string() + &right.to_string())
            .parse::<u8>()
            .expect("Could not parse result to number");

        result += joltage as u64;

        println!("{} {} {}", line, joltage, result);
    }
}

pub fn part_2() {
    let binding = fs::read_to_string("inputs/day-3.txt").expect("Could not read file");
    let content = binding.trim();

    let mut result: u64 = 0;

    for line in content.lines() {
        let binding = line
            .trim()
            .chars()
            .map(|c| c.to_string().parse::<u8>().unwrap())
            .enumerate()
            // Need double ended iterator
            .collect::<Vec<(usize, u8)>>();

        let mut joltages: Vec<u8> = Vec::new();

        let (index, first) = binding
            .iter()
            // Reversing as max_by returns _last_ max, we want first max
            .rev()
            // We need 12 digits, skip 11 rightmost digits
            .skip(11)
            .max_by(|(_, n), (_, o)| n.cmp(o))
            .expect("Could not find max");

        joltages.push(*first);

        // Now we have the max from the left with its index

        let mut last_index = index;
        for i in (0..=10).rev() {
            let search_range = binding.iter().skip(*last_index + 1).rev().skip(i);

            let (index, next) = search_range
                .clone()
                .max_by(|(_, n), (_, o)| n.cmp(o))
                .expect("Could not find max");

            last_index = index;

            joltages.push(*next);
        }

        let joltage = joltages
            .iter()
            .map(|i| i.to_string())
            .collect::<Vec<String>>()
            .concat()
            .parse::<u64>()
            .expect("Could not parse final joltage to number");

        result += joltage;

        println!("final {} {} {:?}", line, joltage, result);
    }
}
