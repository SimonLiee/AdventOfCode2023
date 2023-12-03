pub mod day00;
pub mod day01;
pub mod day02;
pub mod day03;

pub const DAYS : [[fn(bool);2];4] = [
    [
        day00::part1,
        day00::part2,
    ],
    [
        day01::part1,
        day01::part2,
    ],
    [
        day02::part1,
        day02::part2,
    ],
    [
        day03::part1,
        day03::part2,
    ],
];