pub mod day00;
pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;
pub mod day07;

pub const DAYS : [[fn(bool);2];8] = [
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
    [
        day04::part1,
        day04::part2,
    ],
    [
        day05::part1,
        day05::part2,
    ],
    [
        day06::part1,
        day06::part2,
    ],
    [
        day07::part1,
        day07::part2,
    ],
];