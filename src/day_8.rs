use std::{collections::HashSet, fs};

type Point = (i128, i128, i128);

pub fn part_1() {
    let binding = fs::read_to_string("inputs/day-8.txt").expect("Could not read file");
    let content = binding.trim();

    let points: Vec<Point> = content
        .lines()
        .map(|l| l.trim())
        .map(|l| l.split(','))
        .map(|v| v.map(|s| s.parse::<i128>().expect("could not parse")))
        .map(|v| v.collect::<Vec<i128>>())
        .map(|v| (v[0], v[1], v[2]))
        .collect();

    fn get_dist(p: &Point, q: &Point) -> i128 {
        // Removed sqrt as per Reddit hint
        let d = (p.0 - q.0).pow(2) + (p.1 - q.1).pow(2) + (p.2 - q.2).pow(2);
        d
    }

    let mut distances: Vec<(Point, Point, i128)> = points
        .iter()
        .flat_map(|op| {
            points
                .iter()
                .filter(move |ip| op != *ip)
                .map(|ip| (*op, *ip, get_dist(op, ip)))
        })
        .collect();

    distances.sort_by(|a, b| a.2.cmp(&b.2));

    distances = distances
        .iter()
        .enumerate()
        .filter(|(i, _)| i % 2 == 0)
        .map(|(_, d)| *d)
        .collect();

    for d in distances.iter().take(20) {
        println!("{:?}", d);
    }

    let mut circuits: Vec<HashSet<Point>> = Vec::new();
    for (i, d) in distances.iter().enumerate().take(10) {
        println!("{} {:?}", i + 1, d);

        let existing_idx = circuits
            .iter()
            .position(|c| c.contains(&d.0) || c.contains(&d.1));

        if let Some(idx) = existing_idx {
            circuits[idx].insert(d.0);
            circuits[idx].insert(d.1);
        } else {
            circuits.push(HashSet::from_iter(vec![d.0, d.1]))
        }
        for c in &circuits {
            println!("{:?}", c)
        }
        println!("");
    }

    println!("");
    for c in &circuits {
        println!("{:?}", c)
    }
}
