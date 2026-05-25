use std::{
    fs::File,
    io::{self, Read},
    vec,
};

#[derive(Default, Clone)]
struct Grid {
    size: usize,
    elements: Vec<Vec<bool>>,
}

const ROLL_CHAR: &str = "@";

impl Grid {
    fn from(input: &String) -> Self {
        let mut grid = Grid::default();
        let mut y = 0;
        for line in input.lines() {
            if line.len() > 0 {
                grid.size = line.len();
                if let None = grid.elements.get(y) {
                    let mut vector = vec![];
                    vector.resize(grid.size, false);
                    grid.elements.push(vector);
                }
                for x in 0..grid.size {
                    let char = &line[x..x + 1];
                    grid.elements[y][x] = ROLL_CHAR == char;
                }
                y += 1;
            }
        }

        grid
    }

    #[allow(dead_code)]
    pub fn print(&self) {
        for y in 0..self.size {
            for x in 0..self.size {
                let element = self.elements[y][x];
                let element = if element { "@" } else { "." };
                print!("{}", element);
            }
            print!("\n")
        }
    }

    fn get_adjacent_coordinates_of(size: usize, x: usize, y: usize) -> Vec<(usize, usize)> {
        let mut coordinates: Vec<(usize, usize)> = vec![];

        let first_idx = 0;
        let last_idx = size - 1;

        if x > first_idx && y > first_idx {
            coordinates.push((y - 1, x - 1));
        }

        if x > first_idx {
            coordinates.push((y, x - 1));
        }

        if x > first_idx && y < last_idx {
            coordinates.push((y + 1, x - 1));
        }

        if y > first_idx {
            coordinates.push((y - 1, x));
        }

        if y < last_idx {
            coordinates.push((y + 1, x));
        }

        if x < last_idx && y > first_idx {
            coordinates.push((y - 1, x + 1));
        }

        if x < last_idx {
            coordinates.push((y, x + 1));
        }

        if x < last_idx && y < last_idx {
            coordinates.push((y + 1, x + 1));
        }

        coordinates
    }

    pub fn count_adjacent_of(&self, x: usize, y: usize) -> u8 {
        let mut count: u8 = 0;
        let adjacents = Grid::get_adjacent_coordinates_of(self.size, x, y);
        for (y2, x2) in adjacents {
            if self.elements[y2][x2] {
                count += 1;
            }
        }

        count
    }
}

pub fn run(input: &str) -> io::Result<()> {
    let mut file_content = String::new();
    File::open(input)
        .unwrap()
        .read_to_string(&mut file_content)
        .unwrap();

    let mut grid = Grid::from(&file_content);
    let mut next_grid;

    let mut all_total = 0;
    let mut total: Option<usize> = None;
    loop {
        if let Some(0) = total {
            break;
        }

        total = Some(0);

        next_grid = grid.clone();

        for y in 0..grid.size {
            for x in 0..grid.size {
                let pointer = grid.elements[y][x];
                if !pointer {
                    continue;
                }
                let count = grid.count_adjacent_of(x, y);
                if pointer && count < 4 {
                    next_grid.elements[y][x] = false;
                    if let Some(t) = total {
                        total = Some(t + 1)
                    } else {
                        total = Some(1);
                    }
                }
            }
        }

        if let Some(t) = total {
            all_total += t;
        }

        // println!("total {:?}", total);

        // for part 1
        // break;

        // thread::sleep(Duration::from_millis(200));
        // clearscreen::clear().unwrap();
        // grid.print();
        grid = next_grid.clone();
    }

    // println!("\n\n");
    // grid.print();

    println!("total accessible rolls of paper is: {}", all_total);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_adjacent_coordinates_of() {
        let result = Grid::get_adjacent_coordinates_of(10, 0, 0);
        assert_eq!(result.len(), 3, "{:#?}", result);

        let result = Grid::get_adjacent_coordinates_of(10, 9, 9);
        assert_eq!(result.len(), 3, "{:#?}", result);

        let result = Grid::get_adjacent_coordinates_of(10, 0, 9);
        assert_eq!(result.len(), 3, "{:#?}", result);

        let result = Grid::get_adjacent_coordinates_of(10, 9, 0);
        assert_eq!(result.len(), 3, "{:#?}", result);

        let result = Grid::get_adjacent_coordinates_of(10, 1, 6);
        assert_eq!(result.len(), 8, "{:#?}", result);
    }
}
