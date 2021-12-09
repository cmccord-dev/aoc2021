// use itertools::Itertools;
use std::str::FromStr;

use crate::{parse_list_delim, DayTrait, ParsingError};
type Output = usize;
type Input = usize;

fn run_for_days(input: Vec<Input>, days: usize) -> Output {
    let mut arr: [usize; 9] = [0; 9];
    input.iter().for_each(|x| arr[*x] += 1);
    for _ in 0..days {
        let new_fish = arr[0];
        for i in 1..=8 {
            arr[i - 1] = arr[i];
        }
        arr[8] = new_fish;
        arr[6] += new_fish;
    }
    arr.into_iter().sum()
}

#[derive(Default)]
pub struct Day {}
impl DayTrait<Input, Output> for Day {
    fn get_num(&self) -> usize {
        6
    }
    fn read_input(&self) -> Result<Vec<Input>, <Input as FromStr>::Err> {
        parse_list_delim(&format!("input/day{}.in", self.get_num()), ",")
    }

    fn part1(&self, input: Vec<Input>) -> Output {
        run_for_days(input, 80)
    }

    fn part2(&self, input: Vec<Input>) -> Output {
        run_for_days(input, 256)
    }

    fn part1_answer(&self) -> Output {
        391888
    }

    fn part2_answer(&self) -> Output {
        1754597645339
    }
}
