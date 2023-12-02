pub mod day00;
pub mod day01;
pub mod day02;

pub const DAYS : [[fn(bool);2];3] = [
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
];