use std::{collections::HashSet, fs};

pub fn part_1() {
    let binding = fs::read_to_string("inputs/day-7.txt").expect("Could not read file");
    let content = binding.trim();

    let grid: Vec<Vec<(usize, char)>> = content
        .lines()
        .map(|l| {
            l.chars()
                .enumerate()
                .filter(|c| c.1 == '^' || c.1 == 'S')
                .collect::<Vec<(usize, char)>>()
        })
        .filter(|l| !l.is_empty())
        .collect();

    let mut iter = grid.iter();

    let first_line = iter.next().expect("No first line");

    let start_position = first_line
        .iter()
        .find(|(_, c)| *c == 'S')
        .expect("No start char")
        .0;

    let mut beam_positions: HashSet<usize> = HashSet::from_iter(vec![start_position]);
    let mut result = 0;

    for row in iter {
        let splitters: Vec<usize> = row
            .iter()
            .filter(|(pos, c)| *c == '^' && beam_positions.contains(pos))
            .map(|(pos, _)| *pos)
            .collect();

        result += splitters.len();

        let new_beam_positions: Vec<usize> = splitters
            .iter()
            .flat_map(|pos| vec![pos - 1, pos + 1])
            .collect();

        for beam_pos in new_beam_positions {
            beam_positions.insert(beam_pos);
        }
        for splitter in splitters {
            beam_positions.remove(&splitter);
        }
    }

    println!("{}", result);
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let mut v: Vec<bool> = Vec::new();
        v.push(false);
        v.push(true);

        let mut iter = v.iter();

        iter.next();

        let mut result = Vec::new();
        for b in iter {
            result.push(b);
        }

        assert_eq!(result.len(), 1);
    }
}
