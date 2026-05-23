use super::*;

use Pattern::*;

#[test]
fn test_is_repeating() {
    let test_cases: Vec<(Pattern, Pattern)> = vec![
        (is_repeating(101), NotRepeating),
        (is_repeating(1010), Repeating(10)),
        (is_repeating(10101010), Repeating(1010)),
    ];
    for (left, right) in test_cases {
        assert_eq!(left, right);
    }
}

#[test]
fn test_is_repeating_rec() {
    println!("begin");
    let test_cases: Vec<(Pattern, Pattern)> = vec![
        (is_repeating_rec(111), Repeating(1)),
        (is_repeating_rec(212121), Repeating(21)),
        (is_repeating_rec(10101010), Repeating(1010)),
        (is_repeating_rec(10101011), NotRepeating),
    ];
    for (left, right) in test_cases {
        assert_eq!(left, right);
    }
}
