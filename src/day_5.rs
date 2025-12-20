use std::{collections::HashSet, fs, ops::RangeInclusive};

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
            ranges.push(range[0]..=range[1]);
        } else if line.len() > 0 {
            let id = line.parse::<u128>().expect("could not parse id");
            ids.push(id);
        } else {
            panic!("don't know what to do with line");
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

pub fn part_2() {
    let binding = fs::read_to_string("inputs/day-5.txt").expect("Could not read file");
    let content = binding.trim();

    let mut ranges: Vec<(bool, RangeInclusive<u128>)> = content
        .lines()
        .map(|l| l.trim())
        .filter(|l| l.contains("-"))
        .map(|l| {
            let range = l
                .split("-")
                .map(|s| s.parse::<u128>().expect("could not parse range"))
                .collect::<Vec<u128>>();
            return (false, range[0]..=range[1]);
        })
        .collect();

    ranges.sort_by(|a, b| a.1.start().cmp(b.1.start()));

    for i in 0..ranges.len() {
        if ranges[i].0 == true {
            continue;
        }

        loop {
            let right = ranges.iter().position(|r| {
                ranges[i].1.end() < r.1.end()
                    && (ranges[i].1.start() == r.1.start() || r.1.start() <= ranges[i].1.end())
            });
            let left = ranges.iter().position(|r| {
                ranges[i].1.start() > r.1.start() && r.1.end() >= ranges[i].1.start()
            });

            let mut done1 = false;
            let mut done2 = false;
            if let Some(next) = right {
                ranges[i].1 = *ranges[i].1.start()..=*ranges[next].1.end();
                ranges[next].0 = true
            } else {
                // We are no longer able to find any ranges on our right to extend with
                done1 = true;
            }
            if let Some(prev) = left {
                ranges[i].1 = *ranges[prev].1.start()..=*ranges[i].1.end();
                ranges[prev].0 = true
            } else {
                // We are no longer able to find any ranges on our left to extend with
                done2 = true;
            }

            // yuck
            if done1 && done2 {
                break;
            }
        }
    }

    println!("ranges:");
    for r in ranges.iter().filter(|r| r.0 == false) {
        println!("{} {}-{}", r.0, r.1.start(), r.1.end());
    }

    let ranges = ranges.iter().filter(|r| r.0 == false).map(|r| r.1.clone());

    // Handle dupes. There must be a nicer way to do this?
    let set: HashSet<RangeInclusive<u128>> = HashSet::from_iter(ranges);

    println!("set:");
    for r in &set {
        println!("{}-{}", r.start(), r.end());
    }

    let result = set.iter().fold(0, |acc, r| acc + (r.end() + 1 - r.start()));

    println!("{}", result);
}

#[cfg(test)]
mod tests {
    use std::{collections::HashSet, ops::RangeInclusive};

    #[test]
    fn it_works() {
        let mut s: HashSet<RangeInclusive<u128>> = HashSet::new();

        s.insert(0..=1);
        s.insert(0..=1);

        assert_eq!(s.len(), 1);
    }
}
