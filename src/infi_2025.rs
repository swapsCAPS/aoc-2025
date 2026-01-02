use std::{collections::HashMap, fmt, fs, ops::Deref};

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum CellType {
    Tree,
    Blank,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Cell {
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

/*
 * Using zip here has the nice benefit
 */
pub fn get_diag_rows_by_pos(
    angle: i16,
    x: usize,
    y: usize,
    columns: &Vec<Vec<Cell>>,
) -> (Vec<((usize, usize), Cell)>, Vec<((usize, usize), Cell)>) {
    let row_count = columns[0].len();
    let (left, right) = match angle {
        0 => (
            (0..=x)
                .rev()
                .zip((0..=y).rev())
                .map(|(ix, iy)| ((ix, iy), columns[ix][iy]))
                .collect(),
            (x..columns.len())
                .zip((0..=y).rev())
                .map(|(ix, iy)| ((ix, iy), columns[ix][iy]))
                .collect(),
        ),
        90 => (
            (x..columns.len())
                .zip((0..=y).rev())
                .map(|(ix, iy)| ((ix, iy), columns[ix][iy]))
                .collect(),
            (x..columns.len())
                .zip(y..row_count)
                .map(|(ix, iy)| ((ix, iy), columns[ix][iy]))
                .collect(),
        ),
        180 => (
            (x..columns.len())
                .zip(y..row_count)
                .map(|(ix, iy)| ((ix, iy), columns[ix][iy]))
                .collect(),
            (0..=x)
                .rev()
                .zip(y..row_count)
                .map(|(ix, iy)| ((ix, iy), columns[ix][iy]))
                .collect(),
        ),
        270 => (
            (0..=x)
                .rev()
                .zip(y..row_count)
                .map(|(ix, iy)| ((ix, iy), columns[ix][iy]))
                .collect(),
            (0..=x)
                .rev()
                .zip((0..=y).rev())
                .map(|(ix, iy)| ((ix, iy), columns[ix][iy]))
                .collect(),
        ),
        _ => panic!("unknown angle"),
    };

    (left, right)
}

pub fn check_fluorescence(
    line: Vec<((usize, usize), Cell)>,
    gets_light_map: &HashMap<(usize, usize), bool>,
) -> bool {
    if line.len() <= 1 {
        return false;
    }

    let gets_light = line
        .iter()
        .enumerate()
        .skip(1)
        .filter(|(_, cell)| *gets_light_map.get(&cell.0).unwrap())
        .any(|(pos, fluor_tree)| {
            line[1..pos].iter().all(|tree| {
                tree.1.height.unwrap_or_default() < fluor_tree.1.height.unwrap_or_default()
            })
        });

    gets_light
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
    for day in 0..8 {
        println!("{} {}", day, angle);
        let prev_state = columns.clone();

        let mut gets_light_map = HashMap::new();

        // Would be nicer to start storing state per cell...
        for x in 0..columns.len() {
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

                gets_light_map.insert((x, y), gets_light);
            }
        }

        for x in 0..columns.len() {
            for y in 0..columns[x].len() {
                if prev_state[x][y].cell_type != CellType::Tree {
                    continue;
                }

                for y in 0..columns.len() {
                    let row = get_row(y, &prev_state);
                    let s = row.iter().map(|c| c.to_string()).collect::<String>();
                    println!("{}", s)
                }

                println!("{} Doing: {}:{} = {}", angle, x, y, prev_state[x][y]);

                let (left, right) = get_diag_rows_by_pos(angle, x, y, &prev_state);

                println!("left");
                for ((x, y), c) in &left {
                    println!("{}:{}, {} {:?}", x, y, c, gets_light_map.get(&(*x, *y)));
                }
                println!("right");
                for ((x, y), c) in &right {
                    println!("{}:{}, {} {:?}", x, y, c, gets_light_map.get(&(*x, *y)));
                }

                let gets_fluoresence = check_fluorescence(left, &gets_light_map)
                    || check_fluorescence(right, &gets_light_map);

                if gets_fluoresence {
                    println!("{}:{} gets_fluoresence", x, y);
                }

                let gets_light = *gets_light_map.get(&(x, y)).unwrap();

                if prev_state[x][y].height.is_none() {
                    let neighbor_count: Vec<Cell> = get_neighbors(x, y, &prev_state)
                        .iter()
                        .filter(|o| o.is_some())
                        .map(|o| o.unwrap())
                        .filter(|c| c.cell_type == CellType::Tree)
                        .filter(|c| c.height.unwrap_or(0) >= 2)
                        .collect();

                    if (gets_fluoresence || gets_light) && neighbor_count.len() >= 2 {
                        columns[x][y].height = Some(0);
                    }
                    continue;
                }

                if gets_light || gets_fluoresence {
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

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::{Cell, CellType, check_fluorescence, get_diag_rows_by_pos};

    const COLUMNS: [&'static [Cell; 4]; 4] = [
        &[
            Cell {
                cell_type: CellType::Tree,
                height: Some(11),
            },
            Cell {
                cell_type: CellType::Tree,
                height: Some(21),
            },
            Cell {
                cell_type: CellType::Tree,
                height: Some(31),
            },
            Cell {
                cell_type: CellType::Tree,
                height: Some(41),
            },
        ],
        &[
            Cell {
                cell_type: CellType::Tree,
                height: Some(12),
            },
            Cell {
                cell_type: CellType::Tree,
                height: Some(22),
            },
            Cell {
                cell_type: CellType::Tree,
                height: Some(32),
            },
            Cell {
                cell_type: CellType::Tree,
                height: Some(42),
            },
        ],
        &[
            Cell {
                cell_type: CellType::Tree,
                height: Some(13),
            },
            Cell {
                cell_type: CellType::Tree,
                height: Some(23),
            },
            Cell {
                cell_type: CellType::Tree,
                height: Some(33),
            },
            Cell {
                cell_type: CellType::Tree,
                height: Some(43),
            },
        ],
        &[
            Cell {
                cell_type: CellType::Tree,
                height: Some(14),
            },
            Cell {
                cell_type: CellType::Tree,
                height: Some(24),
            },
            Cell {
                cell_type: CellType::Tree,
                height: Some(34),
            },
            Cell {
                cell_type: CellType::Tree,
                height: Some(44),
            },
        ],
    ];

    /*
     *   0  1  2  3
     * 0 11 12 13 14
     * 1 21 22 23 24
     * 2 31 32 33 34
     * 3 41 42 43 44
     */

    fn get_height(c: &Cell) -> u8 {
        c.height.unwrap()
    }

    fn parse_result(
        result: (Vec<((usize, usize), Cell)>, Vec<((usize, usize), Cell)>),
    ) -> (Vec<u8>, Vec<u8>) {
        (
            result
                .0
                .iter()
                .map(|(_, cell)| get_height(cell))
                .collect::<Vec<u8>>(),
            result
                .1
                .iter()
                .map(|(_, cell)| get_height(cell))
                .collect::<Vec<u8>>(),
        )
    }

    #[test]
    fn north() {
        let v: Vec<Vec<Cell>> = COLUMNS.to_vec().iter().map(|c| c.to_vec()).collect();

        let result = parse_result(get_diag_rows_by_pos(0, 2, 0, &v));
        assert_eq!(result, (vec![13], vec![13]));

        let result = parse_result(get_diag_rows_by_pos(0, 2, 1, &v));
        assert_eq!(result, (vec![23, 12], vec![23, 14]));

        let result = parse_result(get_diag_rows_by_pos(0, 1, 1, &v));
        assert_eq!(result, (vec![22, 11], vec![22, 13]));

        let result = parse_result(get_diag_rows_by_pos(0, 0, 2, &v));
        assert_eq!(result, (vec![31], vec![31, 22, 13]));

        let result = parse_result(get_diag_rows_by_pos(0, 3, 3, &v));
        assert_eq!(result, (vec![44, 33, 22, 11], vec![44]));

        let result = parse_result(get_diag_rows_by_pos(0, 3, 2, &v));
        assert_eq!(result, (vec![34, 23, 12], vec![34]));
    }

    #[test]
    fn east() {
        let v: Vec<Vec<Cell>> = COLUMNS.to_vec().iter().map(|c| c.to_vec()).collect();

        let result = parse_result(get_diag_rows_by_pos(90, 1, 1, &v));
        assert_eq!(result, (vec![22, 13], vec![22, 33, 44]));

        let result = parse_result(get_diag_rows_by_pos(90, 0, 2, &v));
        assert_eq!(result, (vec![31, 22, 13], vec![31, 42]));

        let result = parse_result(get_diag_rows_by_pos(90, 2, 3, &v));
        assert_eq!(result, (vec![43, 34], vec![43]));

        let result = parse_result(get_diag_rows_by_pos(90, 2, 2, &v));
        assert_eq!(result, (vec![33, 24], vec![33, 44]));
    }

    #[test]
    fn south() {
        let v: Vec<Vec<Cell>> = COLUMNS.to_vec().iter().map(|c| c.to_vec()).collect();

        let result = parse_result(get_diag_rows_by_pos(180, 2, 2, &v));
        assert_eq!(result, (vec![33, 44], vec![33, 42]));

        let result = parse_result(get_diag_rows_by_pos(180, 0, 2, &v));
        assert_eq!(result, (vec![31, 42], vec![31]));

        let result = parse_result(get_diag_rows_by_pos(180, 2, 3, &v));
        assert_eq!(result, (vec![43], vec![43]));

        let result = parse_result(get_diag_rows_by_pos(180, 2, 2, &v));
        assert_eq!(result, (vec![33, 44], vec![33, 42]));
    }

    #[test]
    fn west() {
        let v: Vec<Vec<Cell>> = COLUMNS.to_vec().iter().map(|c| c.to_vec()).collect();

        let result = parse_result(get_diag_rows_by_pos(270, 1, 1, &v));
        assert_eq!(result, (vec![22, 31], vec![22, 11]));

        let result = parse_result(get_diag_rows_by_pos(270, 0, 2, &v));
        assert_eq!(result, (vec![31], vec![31]));

        let result = parse_result(get_diag_rows_by_pos(270, 2, 3, &v));
        assert_eq!(result, (vec![43], vec![43, 32, 21]));

        let result = parse_result(get_diag_rows_by_pos(270, 2, 2, &v));
        assert_eq!(result, (vec![33, 42], vec![33, 22, 11]));

        let result = parse_result(get_diag_rows_by_pos(270, 3, 1, &v));
        assert_eq!(result, (vec![24, 33, 42], vec![24, 13]));
    }

    #[test]
    fn gets_fluorescence() {
        fn fixture(test_line: Vec<(bool, Option<u8>)>) -> bool {
            let line: Vec<((usize, usize), Cell)> = test_line
                .iter()
                .enumerate()
                .map(|(idx, (_, height))| {
                    (
                        (0, idx),
                        Cell {
                            cell_type: CellType::Tree,
                            height: *height,
                        },
                    )
                })
                .collect();

            let gets_light_map: HashMap<(usize, usize), bool> = HashMap::from_iter(
                test_line
                    .iter()
                    .enumerate()
                    .map(|(idx, (gets_light, _))| ((0, idx), *gets_light))
                    .collect::<Vec<((usize, usize), bool)>>(),
            );

            check_fluorescence(line, &gets_light_map)
        }

        assert_eq!(
            fixture(vec![(false, Some(1)), (false, None), (false, None)]),
            false
        );
        assert_eq!(
            fixture(vec![(false, Some(1)), (false, None), (true, Some(1))]),
            true
        );
        assert_eq!(
            fixture(vec![(false, Some(2)), (false, None), (true, Some(1))]),
            true
        );
        assert_eq!(
            fixture(vec![(false, Some(1)), (false, Some(1)), (true, Some(1))]),
            false
        );
        assert_eq!(
            fixture(vec![(false, Some(2)), (false, Some(1)), (true, Some(2))]),
            true
        );
        assert_eq!(
            fixture(vec![(false, Some(3)), (false, Some(1)), (true, Some(2))]),
            true
        );
        assert_eq!(
            fixture(vec![
                (false, Some(4)),
                (false, Some(3)),
                (false, Some(2)),
                (true, Some(2)),
                (true, Some(3)),
                (true, Some(4)),
                (false, Some(4)),
                (true, Some(4)),
            ]),
            true
        );
    }
}
