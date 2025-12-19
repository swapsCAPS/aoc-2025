use std::{fs, rc::Rc};

pub fn part_1() {
    let binding = fs::read_to_string("inputs/day-4.txt").expect("Could not read file");
    let content = binding.trim();

    let mut grid: Vec<Vec<bool>> = Vec::new();

    for line in content.lines() {
        let row = line.chars().map(|c| c == '@').collect();
        grid.push(row);
    }

    let mut result = 0;

    for y in 0..grid.len() {
        let row_above = if y > 0 { Some(&grid[y - 1]) } else { None };
        let row = &grid[y];
        let row_below = if y < grid.len() - 1 {
            Some(&grid[y + 1])
        } else {
            None
        };

        for (x, _) in row.iter().enumerate().filter(|(_, b)| **b) {
            let mut count = 0;
            if let Some(row) = row_above {
                if x > 0 && row[x - 1] {
                    count += 1
                }
                if row[x] {
                    count += 1
                }
                if x < row.len() - 1 && row[x + 1] {
                    count += 1
                }
            }

            if x > 0 && row[x - 1] {
                count += 1
            }
            if x < row.len() - 1 && row[x + 1] {
                count += 1
            }

            if let Some(row) = row_below {
                if x > 0 && row[x - 1] {
                    count += 1
                }
                if row[x] {
                    count += 1
                }
                if x < row.len() - 1 && row[x + 1] {
                    count += 1
                }
            }

            if count < 4 {
                result += 1;
            }
        }
    }

    println!("{}", result)
}

pub fn part_2() {
    let binding = fs::read_to_string("inputs/day-4.txt").expect("Could not read file");
    let content = binding.trim();

    let mut grid: Vec<Vec<bool>> = Vec::new();

    for line in content.lines() {
        let row = line.chars().map(|c| c == '@').collect();
        grid.push(row);
    }

    let mut total = 0;

    loop {
        let mut result = 0;
        for y in 0..grid.len() {
            for x in 0..grid[y].len() {
                if grid[y][x] == false {
                    continue;
                }

                let mut count = 0;
                if y > 0 {
                    if x > 0 && grid[y - 1][x - 1] {
                        count += 1
                    }
                    if grid[y - 1][x] {
                        count += 1
                    }
                    if x < grid[y - 1].len() - 1 && grid[y - 1][x + 1] {
                        count += 1
                    }
                }

                if x > 0 && grid[y][x - 1] {
                    count += 1
                }
                if x < grid[y].len() - 1 && grid[y][x + 1] {
                    count += 1
                }

                if y < grid.len() - 1 {
                    if x > 0 && grid[y + 1][x - 1] {
                        count += 1
                    }
                    if grid[y + 1][x] {
                        count += 1
                    }
                    if x < grid[y + 1].len() - 1 && grid[y + 1][x + 1] {
                        count += 1
                    }
                }

                if count < 4 {
                    println!("{}:{}: {}", y, x, grid[y][x]);
                    grid[y][x] = false;
                    result += 1;
                }
            }
        }

        if result == 0 {
            break;
        }

        total += result
    }

    println!("{}", total)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let mut v: Vec<Vec<bool>> = Vec::new();
        let mut new_v: Vec<bool> = Vec::new();
        new_v.push(false);
        new_v.push(true);
        v.push(new_v);
        print!("{:?}", v);
    }
}
