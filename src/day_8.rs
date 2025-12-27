use std::{collections::HashSet, fs};

type Point = (i128, i128, i128);

fn get_dist(p: &Point, q: &Point) -> f64 {
    let d = (p.0 - q.0).pow(2) + (p.1 - q.1).pow(2) + (p.2 - q.2).pow(2);
    (d as f64).sqrt()
}

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

    let mut distances: Vec<(Point, Point, f64)> = points
        .iter()
        .flat_map(|op| {
            points
                .iter()
                .filter(move |ip| op != *ip)
                .map(|ip| (*op, *ip, get_dist(op, ip)))
        })
        .collect();

    distances.sort_by(|a, b| a.2.total_cmp(&b.2));

    distances = distances
        .iter()
        .enumerate()
        .filter(|(i, _)| i % 2 == 0)
        .map(|(_, d)| *d)
        .collect();

    let mut circuits: Vec<HashSet<Point>> = Vec::new();

    for d in distances.iter().take(1000) {
        let c = circuits.clone();

        let existing_circuits: Vec<(usize, &HashSet<Point>)> = c
            .iter()
            .enumerate()
            .filter(|(_, c)| c.contains(&d.0) || c.contains(&d.1))
            .collect();

        if existing_circuits.len() > 0 {
            circuits[existing_circuits[0].0].insert(d.0);
            circuits[existing_circuits[0].0].insert(d.1);

            // Join circuits if needed!
            // o boy have I been stuck on this :')
            if existing_circuits.len() > 1 {
                for (idx, points) in existing_circuits.iter().skip(1) {
                    for p in points.iter() {
                        circuits[existing_circuits[0].0].insert(*p);
                    }
                    circuits.remove(*idx);
                }
            }
        } else {
            circuits.push(HashSet::from_iter(vec![d.0, d.1]));
        }
    }

    circuits.sort_by(|a, b| a.len().cmp(&b.len()));
    circuits.reverse();

    println!("");
    for c in &circuits {
        println!("{} {:?}", c.len(), c)
    }

    let result = circuits
        .iter()
        .skip(1)
        .take(2)
        .fold(circuits[0].len(), |acc, c| acc * c.len());

    println!("{}", result)
}

pub fn part_2() {
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

    let mut distances: Vec<(Point, Point, f64)> = points
        .iter()
        .flat_map(|op| {
            points
                .iter()
                .filter(move |ip| op != *ip)
                .map(|ip| (*op, *ip, get_dist(op, ip)))
        })
        .collect();

    distances.sort_by(|a, b| a.2.total_cmp(&b.2));

    distances = distances
        .iter()
        .enumerate()
        .filter(|(i, _)| i % 2 == 0)
        .map(|(_, d)| *d)
        .collect();

    let mut circuits: Vec<HashSet<Point>> = Vec::new();

    let mut i = 0;
    let result = loop {
        let c = circuits.clone();
        let d = distances[i];

        let existing_circuits: Vec<(usize, &HashSet<Point>)> = c
            .iter()
            .enumerate()
            .filter(|(_, c)| c.contains(&d.0) || c.contains(&d.1))
            .collect();

        if existing_circuits.len() > 0 {
            circuits[existing_circuits[0].0].insert(d.0);
            circuits[existing_circuits[0].0].insert(d.1);

            if existing_circuits.len() > 1 {
                for (idx, points) in existing_circuits.iter().skip(1) {
                    for p in points.iter() {
                        circuits[existing_circuits[0].0].insert(*p);
                    }
                    circuits.remove(*idx);
                }
            }
        } else {
            circuits.push(HashSet::from_iter(vec![d.0, d.1]));
        }

        i += 1;

        if circuits[0].len() == 1000 {
            break d.0.0 * d.1.0;
        }
    };

    println!("{}", result)
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    #[test]
    fn it_works() {
        let d = super::get_dist(&(0, 0, 0), &(99999, 0, 0));

        assert_eq!(d, 99999.0)
    }

    #[test]
    fn it_works_2() {
        let d = super::get_dist(&(99999, 0, 0), &(99999, 0, 0));

        assert_eq!(d, 0.0)
    }

    #[test]
    fn it_works_3() {
        let d = super::get_dist(&(99999, 0, 99999), &(0, 0, 0));
        let e = super::get_dist(&(99999, 99999, 0), &(0, 0, 0));

        assert_eq!(d, 141419.94202374713);
        assert_eq!(e, 141419.94202374713);
    }

    #[test]
    fn it_works_4() {
        let mut set = HashSet::new();

        set.insert((99999, 99999, 0));
        set.insert((99999, 99999, 0));
        set.insert((99999, 99999, 1));

        assert_eq!(set.len(), 2);
    }
}
