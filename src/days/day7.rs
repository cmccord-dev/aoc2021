// use itertools::Itertools;

use std::str::FromStr;

use crate::{parse_list_delim, DayTrait};
type Input = i64;
type Output = i64;

// fn dist(a: i64,b:i64)->i64 {
//     let n = (a-b).abs();


// }

#[derive(Default)]
pub struct Day {}
impl DayTrait<Input, Output> for Day {
    fn get_num(&self) -> usize {
        7
    }

    fn read_input(&self) -> Result<Vec<Input>, <Input as FromStr>::Err> {
        parse_list_delim(&format!("input/day{}.in", self.get_num()), ",")
    }

    fn part1(&self, input: Vec<Input>) -> Output {
        let mut tmp = input;
        tmp.sort();
        dbg!(tmp.len());
        let mid = tmp[tmp.len() / 2];
        tmp.iter().map(|x| (x - mid).abs()).sum()
    }

    fn part2(&self, input: Vec<Input>) -> Output {

        // let start = *input.iter().min().unwrap();
        // let end = input.iter().max().unwrap();
        // let mid = i64::MAX;
        // let min_idx = start;
        // for mid in start..end {
        let mid: i64 = input.iter().sum::<i64>() / (input.len() as i64);


         input
            .iter()
            .map(|&x| (x - mid).abs() * ((x - mid).abs() + 1) / 2)
            .sum()
    }

    fn part1_answer(&self) -> Output {
        1466
    }

    fn part2_answer(&self) -> Output {
        1491
    }
}
