use std::{
    fs::File,
    io::{self, BufRead},
};

const LARGE_WINDOW_SIZE: usize = 12;

struct BatteryPair(u8, u8);

impl BatteryPair {
    pub fn value(self) -> u8 {
        let num_value: u8 = format!("{}{}", self.0, self.1).parse().unwrap();
        num_value
    }
}

#[derive(Default)]
struct BatterySet {
    batteries: [u8; LARGE_WINDOW_SIZE],
}

impl BatterySet {
    pub fn value(self) -> u64 {
        let mut value_str = String::new();
        for i in 0..LARGE_WINDOW_SIZE {
            let v = self.batteries[i];
            value_str.push_str(&format!("{}", v));
        }

        let value: u64 = value_str.parse().unwrap();
        value
    }
}

struct Bank {
    batteries: Vec<u8>,
}

impl Bank {
    pub fn from(line: String) -> Result<Self, String> {
        let cap = line.len();

        if cap == 0 {
            return Err("Invalid banks".to_string());
        }

        let mut batteries: Vec<u8> = Vec::with_capacity(cap);
        for i in 0..cap {
            let char = &line[i..i + 1];
            let num: u8 = char.parse().unwrap();
            batteries.push(num);
        }
        Ok(Bank {
            batteries: batteries,
        })
    }

    pub fn find_largest(&self) -> BatteryPair {
        let len = self.batteries.len();

        let mut first_digit = 0;
        let mut first_digit_location: Option<usize> = None;
        for i in 0..len - 1 {
            let pointer = self.batteries[i];
            if pointer > first_digit {
                first_digit = pointer;
                first_digit_location = Some(i + 1);
            }
        }

        let mut second_digit = 0;
        for i in first_digit_location.unwrap()..len {
            let pointer = self.batteries[i];
            if pointer > second_digit {
                second_digit = pointer;
            }
        }

        BatteryPair(first_digit, second_digit)
    }

    pub fn find_largest_twelve(&self) -> BatterySet {
        let mut battery_set = BatterySet::default();
        let len = self.batteries.len();
        let mut finding = 1;
        let mut previous_location: Option<usize> = None;
        while finding <= LARGE_WINDOW_SIZE {
            let mut largest = 0;
            let ending_point = len - (LARGE_WINDOW_SIZE - finding);
            let starting_point = match previous_location {
                Some(prev) => prev + 1,
                None => 0,
            };

            for i in starting_point..ending_point {
                let pointer = self.batteries[i];
                if pointer > largest {
                    largest = pointer;
                    previous_location = Some(i);
                }
            }
            battery_set.batteries[finding - 1] = largest;
            finding += 1;
        }

        battery_set
    }
}

pub fn run(input: &str) -> io::Result<()> {
    let file = File::open(input)?;
    let lines = io::BufReader::new(file).lines();

    let mut sum: u64 = 0;
    let mut sum_twelve: u64 = 0;
    for line in lines.map_while(Result::ok) {
        if let Ok(bank) = Bank::from(line) {
            let pair = bank.find_largest();
            let value = pair.value();
            sum += u64::from(value);
            let twelve = bank.find_largest_twelve();
            let value = twelve.value();
            sum_twelve += value;
        }
    }

    println!("total joltage: {}", sum);
    println!("total joltage twelve: {}", sum_twelve);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_largest() {
        let test_cases: Vec<(&str, u8, u64)> = vec![
            ("987654321111111", 98, 987654321111),
            ("811111111111119", 89, 811111111119),
            ("234234234234278", 78, 434234234278),
            ("818181911112111", 92, 888911112111),
        ];

        for tc in test_cases {
            let (inp, expected, expected_twelve) = tc;
            let bank = Bank::from(inp.to_string()).unwrap();
            let largest = bank.find_largest().value();
            assert_eq!(largest, expected);

            let largest_twelve = bank.find_largest_twelve().value();
            assert_eq!(largest_twelve, expected_twelve);
        }
    }
}
