use std::fs;

pub fn part_1() {
    let binding = fs::read_to_string("inputs/day-9.txt").expect("Could not read file");
    let content = binding.trim();

    let points: Vec<(i64, i64)> = content
        .lines()
        .map(|l| l.trim())
        .map(|l| l.split(","))
        .map(|l| l.map(|s| s.parse::<i64>().expect("could not parse")))
        .map(|l| l.collect::<Vec<i64>>())
        .map(|l| (l[0], l[1]))
        .collect();

    let result = points.iter().fold(0, |acc, (y, x)| {
        let max = points
            .iter()
            .map(|(iy, ix)| ((iy - y).abs() + 1) * ((ix - x).abs() + 1))
            .max()
            .expect("could not get max");

        if max > acc {
            return max;
        }

        return acc;
    });

    println!("{}", result)
}
