use std::{
    collections::{HashMap, HashSet},
    fs,
    hash::{BuildHasherDefault, DefaultHasher},
    ops::RangeInclusive,
};

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

// pub fn draw(points: &Vec<(i64, i64)>) {
//     let (size_x, size_y) = points.iter().fold((0_i64, 0_i64), |(sx, sy), (x, y)| {
//         (if x > &sx { *x } else { sx }, if y > &sy { *y } else { sy })
//     });
//     println!("size: ({}, {})", size_x, size_y);
//
//     let pts: HashMap<i64, HashSet<i64>> = points.iter().fold(HashMap::new(), |mut acc, (x, y)| {
//         if acc.contains_key(&y) {
//             acc.get_mut(y).expect("get mut").insert(*x);
//         } else {
//             acc.insert(*y, HashSet::from_iter(vec![*x]));
//         }
//         return acc;
//     });
//
//     println!("{:?}", pts);
//
//     let mut file = fs::OpenOptions::new()
//         .write(true)
//         .append(true)
//         .create(true)
//         .open("drawing.txt")
//         .unwrap();
//
//     for y in 0..=size_y {
//         if !pts.contains_key(&y) {
//             continue;
//         }
//         let line = (0..=size_x)
//             .map(|x| {
//                 if pts.get(&y).unwrap().contains(&x) {
//                     '#'
//                 } else {
//                     '.'
//                 }
//             })
//             .collect::<String>();
//         write!(file, "{}\n", line).unwrap();
//     }
// }

pub fn part_2() {
    let binding = fs::read_to_string("inputs/day-9.txt").expect("Could not read file");
    let content = binding.trim();

    let mut points: Vec<(i64, i64)> = content
        .lines()
        .map(|l| l.trim())
        .map(|l| l.split(","))
        .map(|l| l.map(|s| s.parse::<i64>().expect("could not parse")))
        .map(|l| l.collect::<Vec<i64>>())
        .map(|l| (l[0], l[1]))
        .collect();

    points.sort_by(|a, b| a.1.cmp(&b.1));

    // Too lazy for single fold
    let min_y = points.iter().map(|(_, y)| y).min().unwrap();
    let max_y = points.iter().map(|(_, y)| y).max().unwrap();

    let (xs, _ys): (HashMap<i64, HashSet<i64>>, HashMap<i64, HashSet<i64>>) =
        points
            .iter()
            .fold((HashMap::new(), HashMap::new()), |mut acc, (x, y)| {
                if acc.0.contains_key(&x) {
                    acc.0.get_mut(x).expect("get mut x ouch").insert(*y);
                } else {
                    acc.0.insert(*x, HashSet::from_iter(vec![*y]));
                }
                // // deadcode now
                // if acc.1.contains_key(&y) {
                //     acc.1.get_mut(y).expect("get mut y ouch").insert(*x);
                // } else {
                //     acc.1.insert(*y, HashSet::from_iter(vec![*x]));
                // }
                return acc;
            });

    let walls_vert: HashMap<i64, RangeInclusive<i64>, BuildHasherDefault<DefaultHasher>> =
        HashMap::from_iter(xs.iter().map(|(k, v)| {
            let mut pts: Vec<&i64> = v.iter().collect();
            pts.sort();
            (*k, *pts[0]..=*pts[1])
        }));

    /*
     * Scan through Y and create ranges. We allow for multiple ranges on one line. I.e. "towers",
     * or "bridges", but in hindsight these doesn't seem to exist in the test data
     * For example:
     *
     *         #XXXX#         1 range
     * #XXXX#  XXXXXX         2 ranges
     * XXXXXX  XXXXXX  #XXXX# 3 ranges
     * XXXXX#XX#XXXXX  XXXXXX 2 ranges
     * XXXXXXXXXXXXX#XX#XXXXX 1 range
     * #XXXXXXXXXXXXXXXXXXXX# 1 range
     *
     * ¯\_(ツ)_/¯
     */
    let green_tiles = (*min_y..=*max_y)
        .map(|oy| {
            let mut ranges: Vec<i64> = walls_vert
                .iter()
                .filter(|(_, r)| r.contains(&oy))
                .map(|p| *p.0)
                .collect();
            ranges.sort();
            let value = ranges
                .windows(2)
                .map(|w| w[0]..=w[1])
                .collect::<Vec<RangeInclusive<i64>>>();
            (oy, value)
        })
        .map(|(oy, v)| {
            let joined: Vec<RangeInclusive<i64>> = v.iter().fold(Vec::new(), |acc, r| {
                if acc.len() > 0 && acc.last().unwrap().end() == r.start() {
                    let new_range = *acc.last().unwrap().start()..=*r.end();
                    return [
                        // rly?
                        acc.iter().take(acc.len() - 1).map(|r| r.clone()).collect(),
                        vec![new_range],
                    ]
                    .concat();
                }
                if acc.is_empty() {
                    return [acc, vec![r.clone()]].concat();
                }
                acc
            });
            (oy, joined)
        });

    let green_rows: HashMap<i64, Vec<RangeInclusive<i64>>, BuildHasherDefault<DefaultHasher>> =
        HashMap::from_iter(green_tiles);

    let result = points.iter().fold(0, |acc, (x, y)| {
        let max = points
            .iter()
            .filter(|(_, iy)| iy >= y)
            .filter(|(ix, iy)| {
                let y_range: RangeInclusive<i64> = *y..=*iy; // iy is always larger than y

                y_range.into_iter().all(|yr| {
                    let allowed_tiles = green_rows.get(&yr).expect("no row found in green tiles");
                    allowed_tiles
                        .iter()
                        .any(|green_range| green_range.contains(x) && green_range.contains(ix))
                })
            })
            .map(|(ix, iy)| {
                let result = ((iy - y).abs() + 1) * ((ix - x).abs() + 1);

                result
            })
            .max();

        if let Some(max) = max {
            if max > acc {
                return max;
            }
        }

        return acc;
    });

    println!("{}", result);
}

/*
 * This approach was an attempt to do a form of ray casting, but I got stuck and decided to use
 * full range checking instead. I'm fairly certain the first approach would have been much faster
 * though.
 *
 * This approach assumes a "blob" with no towers/bridges/gaps
 * For example:
 *
 *    #XX#
 *    X  X
 *    X  #XX#
 *  #X#     X
 *  X     #X#
 *  #XX#  X
 *     X  X
 *     #XX#
 */
pub fn part_2_doing_way_too_moeilijk() {
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

    // We only have two X points per Y and two Y points per X!

    let (xs, ys): (HashMap<i64, HashSet<i64>>, HashMap<i64, HashSet<i64>>) =
        points
            .iter()
            .fold((HashMap::new(), HashMap::new()), |mut acc, (x, y)| {
                if acc.0.contains_key(&x) {
                    acc.0.get_mut(x).expect("get mut x ouch").insert(*y);
                } else {
                    acc.0.insert(*x, HashSet::from_iter(vec![*y]));
                }
                if acc.1.contains_key(&y) {
                    acc.1.get_mut(y).expect("get mut y ouch").insert(*x);
                } else {
                    acc.1.insert(*y, HashSet::from_iter(vec![*x]));
                }
                return acc;
            });

    let walls_hori: HashMap<i64, RangeInclusive<i64>, BuildHasherDefault<DefaultHasher>> =
        HashMap::from_iter(ys.iter().map(|(k, v)| {
            let mut pts: Vec<&i64> = v.iter().collect();
            pts.sort_by(|a, b| b.cmp(a));
            (*k, *pts[0]..=*pts[1])
        }));
    let walls_vert: HashMap<i64, RangeInclusive<i64>, BuildHasherDefault<DefaultHasher>> =
        HashMap::from_iter(xs.iter().map(|(k, v)| {
            let mut pts: Vec<&i64> = v.iter().collect();
            pts.sort_by(|a, b| b.cmp(a));
            (*k, *pts[0]..=*pts[1])
        }));

    let result = points.iter().fold(0, |acc, (x, y)| {
        let points_down: Vec<&(i64, i64)> = points.iter().filter(|(_, iy)| iy >= y).collect();

        let below = points_down.iter().find(|(iix, iiy)| iix == x && iiy > y);

        // This is wrong
        if below.is_none() {
            return acc;
        }

        let below = below.unwrap();

        let left = points_down.iter().find(|(iix, iiy)| iiy == y && iix < x);
        let right = points_down.iter().find(|(iix, iiy)| iiy == y && iix > x);

        let walls_right = walls_vert
            .iter()
            .filter(|(k, r)| k > &x && r.contains(y))
            .map(|(k, v)| k);

        let points_down: Vec<&(i64, i64)> = points.iter().filter(|(_, iy)| iy >= y).collect();

        let below = points_down.iter().find(|(iix, iiy)| iix == x && iiy > y);

        if below.is_none() {
            return acc;
        }

        let below = below.unwrap();

        let left = points_down.iter().find(|(iix, iiy)| iiy == y && iix < x);
        let right = points_down.iter().find(|(iix, iiy)| iiy == y && iix > x);
        let max = [
            // down left
            points_down
                .iter()
                .find(|(iix, iiy)| iix < &below.0 && iiy == &below.1),
            // down right
            points_down
                .iter()
                .find(|(iix, iiy)| iix > &below.0 && iiy == &below.1),
            // left down
            if left.is_some() {
                let l = left.unwrap();
                let left_down = points_down
                    .iter()
                    .find(|(iix, iiy)| iix == &l.0 && iiy > &l.1);
                if let Some(ld) = left_down {
                    if ld.1 > below.1 { None } else { left_down }
                } else {
                    None
                }
            } else {
                None
            },
            // right down
            if right.is_some() {
                let r = right.unwrap();
                let right_down = points_down
                    .iter()
                    .find(|(iix, iiy)| iix == &r.0 && iiy > &r.1);
                if let Some(rd) = right_down {
                    if rd.1 > below.1 { None } else { right_down }
                } else {
                    None
                }
            } else {
                None
            },
        ]
        .iter()
        .filter(|item| item.is_some())
        .map(|item| item.unwrap())
        .map(|(ix, iy)| {
            let result = ((ix - x).abs() + 1) * ((iy - y).abs() + 1);
            println!("{}:{} {}:{} = {}", x, y, ix, iy, result);
            result
        })
        .max()
        .expect("could not get max");

        println!("{}:{} {}", y, x, max);

        if max > acc {
            return max;
        }

        return acc;
    });

    println!("{}", result)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_doesnt_do_what_i_want() {
        let result: Vec<(usize, u8)> = (3..6).enumerate().collect();

        assert_eq!(result, vec![(0, 3), (1, 4), (2, 5),])
    }
}
