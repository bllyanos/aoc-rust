use std::{
    fs::File,
    io::{self, Read},
};

#[cfg(test)]
mod tests;

struct IdRange {
    end: u64,
    current: u64,
}

impl IdRange {
    pub fn from(range_str: &str) -> Self {
        let range_str = range_str.trim();
        let range_points: Vec<&str> = range_str.split("-").collect();
        let start: u64 = range_points.get(0).unwrap().parse().unwrap();
        let end: u64 = range_points.get(1).unwrap().parse().unwrap();
        IdRange {
            end: end,
            current: start,
        }
    }
}

impl Iterator for IdRange {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current <= self.end {
            let current = self.current.clone();
            self.current += 1;
            Some(current)
        } else {
            None
        }
    }
}

#[derive(Debug, PartialEq)]
enum Pattern {
    Repeating(u64),
    NotRepeating,
}

fn is_repeating(num: u64) -> Pattern {
    let numstr = num.to_string();

    if numstr.len() % 2 != 0 {
        return Pattern::NotRepeating;
    }

    let half_size = numstr.len() / 2;
    let first_half = &numstr[..half_size];
    let second_half = &numstr[half_size..];

    if first_half != second_half {
        return Pattern::NotRepeating;
    }

    let repeating_value: u64 = first_half.parse().unwrap();
    return Pattern::Repeating(repeating_value);
}

fn is_repeating_rec(num: u64) -> Pattern {
    let numstr = num.to_string();
    let mut window_size = numstr.len() / 2;

    while window_size >= 1 {
        if numstr.len() % window_size == 0 {
            let loop_count = numstr.len() / window_size;
            if loop_count <= 1 {
                window_size -= 1;
                continue;
            }

            let first_chunk = &numstr[..window_size];
            let mut remaining_chunks = &numstr[window_size..];

            let mut is_repeating = true;

            loop {
                if remaining_chunks.len() < window_size {
                    break;
                }
                let next_chunk = &remaining_chunks[..window_size];
                remaining_chunks = &remaining_chunks[window_size..];
                if first_chunk != next_chunk {
                    is_repeating = false;
                    break;
                }
            }

            if !is_repeating {
                window_size -= 1;
                continue;
            }

            let repeat_pattern = first_chunk.parse::<u64>().unwrap();
            return Pattern::Repeating(repeat_pattern);
        }

        window_size -= 1;
    }

    return Pattern::NotRepeating;
}

pub fn run(input: &str) -> io::Result<()> {
    let mut input_content: String = String::new();
    File::open(input)
        .unwrap()
        .read_to_string(&mut input_content)
        .unwrap();

    let mut count: u64 = 0;
    let mut invalid_id_sum: u64 = 0;
    let mut count_rec: u64 = 0;
    let mut invalid_id_sum_rec: u64 = 0;
    for part in input_content.split(",") {
        let range = IdRange::from(part);
        for range_item in range {
            match is_repeating(range_item) {
                Pattern::Repeating(_data) => {
                    // println!("found invalid id {}", range_item);
                    count += 1;
                    invalid_id_sum += range_item;
                }
                _ => (),
            }

            match is_repeating_rec(range_item) {
                Pattern::Repeating(_data) => {
                    // println!("found invalid id {}", range_item);
                    count_rec += 1;
                    invalid_id_sum_rec += range_item;
                }
                _ => (),
            }
        }
    }
    println!("total repeating data is: {}", count);
    println!("sum of invalid id: {}", invalid_id_sum);
    println!("total repeating data rec is: {}", count_rec);
    println!("sum of invalid id rec: {}", invalid_id_sum_rec);
    Ok(())
}
