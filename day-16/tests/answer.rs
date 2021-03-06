const INPUT: &str = include_str!("input.txt");

#[test]
fn part_1() {
    assert_eq!(day_16::part_1(INPUT), 904);
}

#[test]
fn part_2() {
    assert_eq!(day_16::part_2(INPUT), 200476472872);
}
