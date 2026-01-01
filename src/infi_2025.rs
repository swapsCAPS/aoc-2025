use std::{fmt, fs, ops::Deref};

#[derive(Debug, Copy, Clone, PartialEq)]
enum CellType {
    Tree,
    Blank,
}

#[derive(Debug, Copy, Clone)]
struct Cell {
    cell_type: CellType,
    height: Option<u8>,
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.cell_type == CellType::Blank {
            return write!(f, "_");
        }
        if let Some(h) = self.height {
            write!(f, "{}", h)
        } else {
            write!(f, "x")
        }
    }
}

pub fn part_1() {
    let binding = fs::read_to_string("inputs/infi-2025.txt").expect("Could not read file");
    let content = binding.trim();

    let second_line = content.lines().skip(1).take(1).collect::<Vec<&str>>()[0];
    let lines_amount = content.lines().count();

    let column_count = second_line.chars().skip(1).step_by(3).count();

    let mut columns: Vec<Vec<Cell>> = (0..column_count).map(|_| vec![]).collect();

    // Having "blank" cells is a slight waste of resources, but it makes row checking easier
    fn parse_cell(c: char) -> Cell {
        if c.is_digit(10) {
            Cell {
                cell_type: CellType::Tree,
                height: Some(c.to_string().parse::<u8>().unwrap()),
            }
        } else if c.is_ascii_whitespace() {
            Cell {
                cell_type: CellType::Tree,
                height: None,
            }
        } else if c == '_' {
            Cell {
                cell_type: CellType::Blank,
                height: None,
            }
        } else {
            panic!("auw")
        }
    }

    // Nothing on first row and nothing on last
    for l in content.lines().take(lines_amount - 1).skip(1) {
        for (ux, c) in l.chars().skip(1).step_by(3).enumerate() {
            columns[ux].push(parse_cell(c));
        }
    }

    let row_count = columns[0].len();

    fn get_row(y: usize, columns: &Vec<Vec<Cell>>) -> Vec<Cell> {
        columns.iter().map(|c| c[y]).collect()
    }

    fn get_neighbors(x: usize, y: usize, columns: &Vec<Vec<Cell>>) -> [Option<Cell>; 6] {
        let column = &columns.get(x).unwrap();

        let has_top = y > 0;
        let has_bottom = y < column.len() - 1;
        let has_left = x > 0;
        let has_right = x < columns.len() - 1;

        // Note offset of `2` due to hex grid
        let top = if y > 1 { Some(column[y - 2]) } else { None };
        let bottom = if y < column.len() - 2 {
            Some(column[y + 2])
        } else {
            None
        };

        let tl = if has_top && has_left {
            Some(columns[x - 1][y - 1])
        } else {
            None
        };
        let tr = if has_top && has_right {
            Some(columns[x + 1][y - 1])
        } else {
            None
        };
        let bl = if has_bottom && has_left {
            Some(columns[x - 1][y + 1])
        } else {
            None
        };
        let br = if has_bottom && has_right {
            Some(columns[x + 1][y + 1])
        } else {
            None
        };

        [top, bottom, tl, tr, bl, br]

        // It's such a PITA I can't use negative indexes. Would be much more elegant
        // let top = column.get(y - 2);
        // let bottom = column.get(y + 2);
        // let tl = columns.get(x - 1).map_or(None, |c| c.get(y - 1));
        // let tr = columns.get(x + 1).map_or(None, |c| c.get(y - 1));
        // let bl = columns.get(x - 1).map_or(None, |c| c.get(y + 1));
        // let br = columns.get(x + 1).map_or(None, |c| c.get(y + 1));
    }

    for y in 0..columns.len() {
        let row = get_row(y, &mut columns);
        let s = row.iter().map(|c| c.to_string()).collect::<String>();
        println!("{}", s)
    }

    let mut cut_down_trees = 0;

    let mut angle = 0_i16;
    for day in 0..256 {
        println!("{} {}", day, angle);
        let prev_state = columns.clone();

        for x in 0..columns.len() {
            // for (x, column) in columns.iter().map(|c|).enumerate() {
            for y in 0..columns[x].len() {
                if prev_state[x][y].cell_type != CellType::Tree {
                    continue;
                }

                let (pos, tree_line): (usize, &Vec<Cell>) = match angle {
                    0 => (y, &prev_state[x]),
                    90 => (
                        column_count - 1 - x,
                        &get_row(y, &prev_state).into_iter().rev().collect(),
                    ),
                    180 => (
                        row_count - 1 - y,
                        &prev_state[x].iter().map(|v| *v).rev().collect(),
                    ),
                    270 => (x, &get_row(y, &prev_state)),
                    _ => panic!("unknown angle"),
                };

                let gets_light = tree_line.iter().take(pos.max(1) - 1).all(|c| {
                    let ch = c.height.unwrap_or_default();
                    ch == 0 || ch < prev_state[x][y].height.unwrap_or_default()
                });

                if prev_state[x][y].height.is_none() {
                    let neighbor_count: Vec<Cell> = get_neighbors(x, y, &prev_state)
                        .iter()
                        .filter(|o| o.is_some())
                        .map(|o| o.unwrap())
                        .filter(|c| c.cell_type == CellType::Tree)
                        .filter(|c| c.height.unwrap_or(0) >= 2)
                        .collect();

                    if gets_light && neighbor_count.len() >= 2 {
                        println!("{}:{} sprouted", x, y);
                        columns[x][y].height = Some(0);
                    }
                    continue;
                }

                if gets_light {
                    columns[x][y].height = Some(prev_state[x][y].height.unwrap() + 1)
                }

                if columns[x][y].height.expect("height should be Some now") >= 5 {
                    println!("{}:{} doei boompie", x, y);
                    cut_down_trees += 1;
                    columns[x][y].height = None;
                    continue;
                }
            }
        }

        angle += 90;
        if angle >= 360 {
            angle = 0;
        }

        for y in 0..columns.len() {
            let row = get_row(y, &columns);
            let s = row.iter().map(|c| c.to_string()).collect::<String>();
            println!("{}", s)
        }

        println!("{}", cut_down_trees);
    }
}
