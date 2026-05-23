use std::{
    fs::File,
    io::{self, BufRead},
};

#[cfg(test)]
mod tests;

#[derive(Debug)]
struct Dial {
    clicks: u64,
    overclicks: u64,
    pointer: i32,
}

const DIAL_SIZE: i32 = 100;
impl Dial {
    pub fn new(start: i32) -> Self {
        return Dial {
            clicks: 0,
            overclicks: 0,
            pointer: start,
        };
    }

    pub fn rotate(&mut self, direction: Direction, value: i32) {
        let current_pointer = self.pointer;
        let mut overrotation: u64 = u64::try_from(value / 100).unwrap();
        let value = value % 100;

        match direction {
            Direction::Left => {
                self.pointer -= value;
            }
            Direction::Right => {
                self.pointer += value;
            }
        }

        if self.pointer < 0 {
            self.pointer += DIAL_SIZE;
            if self.pointer != 0 && current_pointer != 0 {
                overrotation = overrotation + 1;
            }
        } else if self.pointer >= DIAL_SIZE {
            self.pointer -= DIAL_SIZE;
            if self.pointer != 0 && current_pointer != 0 {
                overrotation = overrotation + 1;
            }
        }

        if overrotation >= 1 {
            self.overclicks += overrotation;
        }

        if self.pointer == 0 {
            self.clicks += 1;
        }
    }
}

enum Direction {
    Left,
    Right,
}

fn parse_dial_instruction(line: String) -> Result<(Direction, i32), String> {
    let direction: Result<Direction, String> = if line.starts_with("L") {
        Ok(Direction::Left)
    } else if line.starts_with("R") {
        Ok(Direction::Right)
    } else {
        Err(format!("Invalid direction {}", line).to_string())
    };
    let direction = direction?;

    let valuestr = &line[1..];
    let value = valuestr
        .parse::<i32>()
        .map_err(|_| format!("Invalid value {}", valuestr).to_string())?;

    return Ok((direction, value));
}

pub fn run(input: &str) -> io::Result<()> {
    let mut dial = Dial::new(50);
    let file = File::open(input)?;
    let lines = io::BufReader::new(file).lines();

    for line in lines.map_while(Result::ok) {
        if let Ok(instruction) = parse_dial_instruction(line) {
            let (direction, value) = instruction;
            dial.rotate(direction, value);
        }
    }

    println!("Dial is pointing at {}", dial.pointer);
    println!("Clicks count: {}", dial.clicks);
    println!("Overclicks count: {}", dial.overclicks);
    println!(
        "Total Clicks for method 0x434C49434B: {}",
        dial.overclicks + dial.clicks
    );

    Ok(())
}
