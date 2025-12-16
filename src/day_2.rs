use std::fs;

pub fn part_1() {
    let binding = fs::read_to_string("inputs/day-2.txt").expect("Could not read file");
    let content = binding.trim();

    let mut result: u128 = 0;

    for range_str in content.split(",") {
        let mut r = range_str
            .split("-")
            .map(|s| s.parse::<u128>().expect("Could not parse"));

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

const PRIMES: [usize; 20] = [
    2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71,
];

pub fn part_2() {
    let binding = fs::read_to_string("inputs/day-2.txt").expect("Could not read file");
    let content = binding.trim();

    let mut result: u128 = 0;

    for range_str in content.split(",") {
        let mut r = range_str
            .split("-")
            .map(|s| s.parse::<u128>().expect("Could not parse"));

        let start = r.next().expect("No start");
        let end = r.next().expect("No end");

        for n in start..=end {
            let seq = n.to_string();
            let len = seq.len();

            // Split in half and check equality
            if len % 2 == 0 {
                let (left, right) = seq.split_at(len / 2);

                if left == right {
                    result += n;
                    continue;
                }
            }

            // Chunk in prime numbers and check equality of all chunks.
            for prime in PRIMES {
                if len == prime {
                    let mut chars = seq.chars();
                    let first = chars.next().unwrap();
                    if chars.all(|c| c == first) {
                        result += n;
                    }
                    break;
                }

                if len % prime != 0 {
                    continue;
                }

                let mut chunks = seq.as_bytes().chunks(prime);

                let first = chunks.next().unwrap();
                if chunks.all(|chunk| chunk == first) {
                    result += n;
                    break;
                }
            }
        }
    }

    println!("{}", result);
}
