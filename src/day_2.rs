use std::fs;

pub fn run() {
    let binding = fs::read_to_string("inputs/day-2.txt").expect("Could not read file");
    let content = binding.trim();

    let mut result: u64 = 0;

    for range_str in content.split(",") {
        let mut r = range_str
            .split("-")
            .map(|s| s.parse::<u64>().expect("Could not parse"));

        let start = r.next().expect("No start");
        let end = r.next().expect("No end");

        for n in start..=end {
            let seq = n.to_string();
            let len = seq.len();

            if len % 2 != 0 {
                continue;
            }

            let (left, right) = seq.split_at(len / 2);

            if left == right {
                result += n
            }
        }
    }

    println!("{}", result)
}
