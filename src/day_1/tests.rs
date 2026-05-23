use super::*;

#[test]
fn test_dial() {
    let mut dial = Dial::new(0);
    dial.rotate(Direction::Left, 1);
    assert!(dial.pointer == 99);

    dial.rotate(Direction::Right, 100);
    assert!(dial.pointer == 99);

    dial.rotate(Direction::Left, 99);
    assert!(dial.pointer == 0);

    dial.rotate(Direction::Left, 501);
    assert!(dial.pointer == 99);
}

#[test]
fn test_dial_over() {
    let mut dial = Dial::new(50);
    dial.rotate(Direction::Left, 68);
    assert_eq!(dial.overclicks, 1);
    dial.rotate(Direction::Left, 30);
    dial.rotate(Direction::Right, 48);
    assert_eq!(dial.overclicks, 1);
    assert_eq!(dial.pointer, 0);
    dial.rotate(Direction::Left, 5);
    // error here
    assert_eq!(dial.overclicks, 1);
    dial.rotate(Direction::Right, 60);
    assert_eq!(dial.overclicks, 2);
    dial.rotate(Direction::Left, 55);
    assert_eq!(dial.pointer, 0);
    dial.rotate(Direction::Left, 1);
    dial.rotate(Direction::Left, 99);
    assert_eq!(dial.pointer, 0);
    assert_eq!(dial.overclicks, 2);
    dial.rotate(Direction::Right, 14);
    dial.rotate(Direction::Left, 82);
    assert_eq!(dial.clicks, 3);
    assert_eq!(dial.overclicks, 3);
    assert_eq!(dial.pointer, 32);

    dial.rotate(Direction::Right, 1000);
    assert_eq!(dial.overclicks, 13);
}
