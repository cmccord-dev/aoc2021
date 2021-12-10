// use itertools::Itertools;
use std::str::FromStr;

use itertools::Itertools;

use crate::input_struct;
use crate::{DayTrait, ParsingError};
type Output = i32;

input_struct! {Input, Vec<u8>}

// #[derive(Debug, Clone)]
// pub struct Input {
//     data:Vec<u8>
// }

impl FromStr for Input {
    type Err = ParsingError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(
            s.trim().bytes().map(|x| x - 0x30).collect::<Vec<u8>>(),
        ))
    }
}

fn fill_basin(basin: &mut Vec<Vec<bool>>, input: &Vec<Input>, (i, j): (i32, i32)) -> usize {
    if i >= 0 && j >= 0 && (i as usize) < input.len() && (j as usize) < input[0].len() {
        if basin[i as usize][j as usize] {
            0
        } else {
            basin[i as usize][j as usize] = true;
            if input[i as usize][j as usize] != 9 {
                1 + fill_basin(basin, input, (i - 1, j))
                    + fill_basin(basin, input, (i + 1, j))
                    + fill_basin(basin, input, (i, j + 1))
                    + fill_basin(basin, input, (i, j - 1))
            } else {
                0
            }
        }
    } else {
        0
    }
}

#[derive(Default)]
pub struct Day {}
impl DayTrait<Input, Output> for Day {
    fn get_num(&self) -> usize {
        9
    }

    fn part1(&self, input: Vec<Input>) -> Output {
        let width = input[0].len();
        let height = input.len();
        let mut lowest_sum = 0;
        let mut count = 0;
        for i in 0..height {
            for j in 0..width {
                let curr = input[i][j];
                if (i == 0 || input[i - 1][j] > curr)
                    && (i == height - 1 || input[i + 1][j] > curr)
                    && (j == 0 || input[i][j - 1] > curr)
                    && (j == width - 1 || input[i][j + 1] > curr)
                {
                    lowest_sum += 1 + curr as Output;
                    count += 1;
                }
            }
        }
        dbg!(count);
        lowest_sum
    }

    fn part2(&self, input: Vec<Input>) -> Output {
        let mut basins = vec![];

        let width = input[0].len();
        let height = input.len();
        let mut basin: Vec<Vec<bool>> = vec![vec![false; width]; height];
        for i in 0..height {
            for j in 0..width {
                if !basin[i][j] {
                    basins.push((fill_basin(&mut basin, &input, (i as i32, j as i32)) as i32))
                }
            }
        }
        basins
            .into_iter()
            .sorted()
            .rev()
            .take(3)
            .map(|x| (x))
            .fold1(|p, c| p * c)
            .unwrap()
    }

    fn part1_answer(&self) -> Output {
        1670340
    }

    fn part2_answer(&self) -> Output {
        1954293920
    }
}
