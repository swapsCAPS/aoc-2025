use std::fs;

pub fn deserialize(input: &str) -> (char, i16) {
    let mut iter = input.chars();
    let dir = iter.next().expect("Could not get first char");
    let string_num: String = iter.collect();
    let num = string_num.parse::<i16>().expect("Could not parse int");

    (dir, num)
}

pub fn part_1() {
    let content = fs::read_to_string("inputs/day-1.txt").expect("Could not read file");

    let mut current: i16 = 50;
    let mut password = 0;

    for line in content.lines() {
        let (dir, num) = deserialize(line);

        // Accomodate multiple spins
        if dir == 'L' {
            current = current - num % 100;
        } else {
            current = current + num % 100;
        }

        // Cap results
        if current < 0 {
            current = 100 + current
        }
        if current > 99 {
            current = current - 100
        }

        if current == 0 {
            password += 1;
        }
    }

    println!("Password: {}", password)
}

pub fn part_2() {
    let content = fs::read_to_string("inputs/day-1.txt").expect("Could not read file");

    let mut current: i16 = 50;
    let mut prev_cur: i16 = current;
    let mut password = 0;

    for line in content.lines() {
        let (dir, num) = deserialize(line);

        // Accomodate multiple spins
        if dir == 'L' {
            current = current - num % 100;
        } else {
            current = current + num % 100;
        }

        if num >= 100 {
            password += (num as f32 / 100.0).floor() as i16;
        }

        if current == 0 {
            password += 1;
        } else if current < 0 {
            current = current + 100;
            if prev_cur != 0 {
                password += 1;
            }
        } else if current > 99 {
            current = current - 100;
            password += 1;
        }

        prev_cur = current;

        println!(
            "dir: {}, num: {}, cur: {}, pass: {}",
            dir, num, current, password
        );
    }

    println!("Password: {}", password)
}
