// use itertools::Itertools;
use std::str::FromStr;

use crate::{DayTrait, ParsingError};
type Output = i32;

#[derive(Debug, Clone)]
pub struct Input {
    dir: i32,
    count: i32,
}

impl FromStr for Input {
    type Err = ParsingError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split(' ').collect::<Vec<&str>>();
        let dir = match parts[0] {
            "down" => 1,
            "up" => -1,
            _ => 0,
        };
        let count = parts[1].parse()?;
        Ok(Self { dir, count })
    }
}

#[derive(Default)]
pub struct Day {}
impl DayTrait<Input, Output> for Day {
    fn get_num(&self) -> usize {
        2
    }

    fn part1(&self, input: Vec<Input>) -> Output {
        let mut x = 0;
        let mut depth = 0;
        input.iter().for_each(|f| match f.dir {
            0 => x += f.count,
            _ => depth += f.dir * f.count,
        });
        x * depth
    }

    fn part2(&self, input: Vec<Input>) -> Output {
        let mut x = 0;
        let mut aim = 0;
        let mut depth = 0;
        input.iter().for_each(|f| match f.dir {
            0 => {
                x += f.count;
                depth += f.count * aim
            }
            _ => aim += f.dir * f.count,
        });
        x * depth
    }

    fn part1_answer(&self) -> Output {
        1670340
    }

    fn part2_answer(&self) -> Output {
        1954293920
    }
}
