use std::fs;

pub fn part_1() {
    let binding = fs::read_to_string("inputs/day-5.txt").expect("Could not read file");
    let content = binding.trim();

    let mut ranges = Vec::new();
    let mut ids = Vec::new();

    for line in content.lines().map(|l| l.trim()).filter(|l| !l.is_empty()) {
        if line.contains("-") {
            let range = line
                .split("-")
                .map(|s| s.parse::<u128>().expect("could not parse range"))
                .collect::<Vec<u128>>();
            ranges.push(range[0]..range[1])
        } else if line.len() > 0 {
            let id = line.parse::<u128>().expect("could not parse id");
            ids.push(id);
        } else {
            panic!("don't know what to do with line")
        }
    }

    let result: u128 = ids.iter().fold(0, |acc, id| {
        if ranges.iter().any(|r| r.contains(id)) {
            acc + 1
        } else {
            acc
        }
    });

    println!("{}", result)
}
