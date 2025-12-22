use std::{
    collections::{HashMap, HashSet},
    fs,
};

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

pub fn part_2() {
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

    let mut beams: HashMap<usize, u64> = HashMap::from_iter(vec![(start_position, 1)]);

    for row in iter {
        let splitters: Vec<usize> = row
            .iter()
            .filter(|(pos, c)| *c == '^' && beams.contains_key(pos))
            .map(|(pos, _)| *pos)
            .collect();

        for splitter_pos in &splitters {
            let new_beams = [splitter_pos - 1, splitter_pos + 1];

            let splitter_input = &beams
                .get(&splitter_pos)
                .expect("could not get beam by splitter_pos")
                .clone();

            for beam_pos in new_beams {
                if beams.contains_key(&beam_pos) {
                    let item = beams.get_mut(&beam_pos).expect("could not get_mut");
                    *item += splitter_input;
                } else {
                    beams.insert(beam_pos, *splitter_input);
                }
            }
        }
        for splitter in splitters {
            beams.remove(&splitter);
        }
    }

    println!("{}", beams.iter().fold(0, |acc, b| acc + *b.1));
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
