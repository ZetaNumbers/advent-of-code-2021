const INPUT: &str = include_str!("example.txt");

#[test]
fn part_1() {
    assert_eq!(day_15::part_1(INPUT), 40)
}

#[test]
fn part_2() {
    assert_eq!(day_15::part_2(INPUT), 315)
}
