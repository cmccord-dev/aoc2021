// use itertools::Itertools;

use crate::DayTrait;
type Input = u64;
type Output = usize;




#[derive(Default)]
pub struct Day {}
impl DayTrait<Input, Output> for Day {
    fn get_num(&self) -> usize {
        1
    }

    fn part1(&self, input: Vec<u64>) -> Output {
        input.windows(2).filter(|&n| n[0] < n[1]).count()
    }

    fn part2(&self, input: Vec<u64>) -> Output {
        input.windows(4).filter(|&n| n[0] < n[3]).count()
    }

    fn part1_answer(&self) -> Output {
        1466
    }

    fn part2_answer(&self) -> Output {
        1491
    }
}