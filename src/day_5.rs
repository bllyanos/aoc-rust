use std::{
    fmt,
    fs::File,
    io::{self, BufRead},
    vec,
};

#[derive(Clone, Debug)]
struct Range {
    start: u64,
    end: u64,
}

impl fmt::Display for Range {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "R({}-{})", self.start, self.end)
    }
}

impl Range {
    pub fn from(input: &str) -> Self {
        let pair: Vec<&str> = input.split("-").collect();
        let start: u64 = pair[0].parse().unwrap();
        let end: u64 = pair[1].parse().unwrap();
        Range { start, end }
    }

    pub fn has(&self, challenge: u64) -> bool {
        challenge >= self.start && challenge <= self.end
    }

    pub fn count(&self) -> u64 {
        (self.end + 1) - self.start
    }

    pub fn intersecting(&self, other: &Range) -> bool {
        return self.has(other.start)
            || self.has(other.end)
            || other.has(self.start)
            || other.has(self.end);
    }

    pub fn merge(&self, other: &Range) -> Option<Range> {
        if self.intersecting(other) {
            let new_start = self.start.min(other.start);
            let new_end = self.end.max(other.end);

            return Some(Range {
                start: new_start,
                end: new_end,
            });
        } else {
            return None;
        }
    }
}

pub fn run(input: &str) -> io::Result<()> {
    let file = File::open(input)?;
    let lines = io::BufReader::new(file).lines();

    let mut is_reading_ranges = true;
    let mut ranges: Vec<Range> = vec![];
    let mut challenges: Vec<u64> = vec![];
    for line in lines.map_while(Result::ok) {
        if line.len() <= 0 {
            is_reading_ranges = false;
            continue;
        }

        if is_reading_ranges {
            ranges.push(Range::from(&line));
        } else {
            let value: u64 = line.parse().unwrap();
            challenges.push(value);
        }
    }

    let mut counter: u64 = 0;
    for c in challenges {
        for r in ranges.iter() {
            if r.has(c) {
                counter += 1;
                break;
            }
        }
    }

    println!("total fresh ingredient are: {}", counter);

    ranges.sort_by(|range, other| range.start.cmp(&other.start));
    let mut ranges: Vec<Option<Range>> = ranges.into_iter().map(|v| Some(v)).collect();
    for i in 0..ranges.len() {
        if let None = &ranges[i] {
            continue;
        }
        for j in i + 1..ranges.len() {
            if let Some(range) = &ranges[i]
                && let Some(other) = &ranges[j]
                && let Some(merged) = range.merge(other)
            {
                ranges[i] = Some(merged);
                ranges[j] = None;
            }
        }
    }

    let mut total_count: u64 = 0;
    for r in ranges.iter() {
        if let Some(range) = r {
            let count = range.count();
            // println!("proceeding with {} with count {}", r, count);
            total_count += count;
        }
    }

    println!(
        "total fresh ingredient based on range only: {}",
        total_count
    );

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_range_creation() {
        let r = Range::from("1-2");
        assert_eq!(r.start, 1);
        assert_eq!(r.end, 2);
        assert_eq!(r.count(), 2);

        let r = Range::from("2404-2404");
        assert_eq!(r.start, 2404);
        assert_eq!(r.end, 2404);
        assert_eq!(r.count(), 1);
    }

    #[test]
    fn test_range_merging() {
        let test_cases: Vec<(&str, &str, Option<Range>)> = vec![
            ("1-5", "4-6", Some(Range::from("1-6"))),
            ("1-5", "1-4", Some(Range::from("1-5"))),
            ("2-5", "1-5", Some(Range::from("1-5"))),
            ("2-4", "5-10", None),
        ];

        for (r1_material, r2_material, expect) in test_cases {
            let r = Range::from(r1_material);
            let r2 = Range::from(r2_material);

            if let Some(r3) = &r.merge(&r2) {
                println!("expecting {} and {} merged into {}", r, r2, r3);
                let challenge = expect.unwrap();
                assert!(r3.start == challenge.start);
                assert!(r3.end == challenge.end);
            } else {
                assert!(matches!(expect, None))
            }
        }
    }
}
