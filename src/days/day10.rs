// use itertools::Itertools;
use std::str::FromStr;

use itertools::Itertools;

use crate::{DayTrait, ParsingError};
type Output = usize;
type Input = String;

#[derive(Default)]
pub struct Day {}
impl DayTrait<Input, Output> for Day {
    fn get_num(&self) -> usize {
        10
    }

    fn part1(&self, input: Vec<Input>) -> Output {
        input
            .into_iter()
            .map(|line| {
                let mut stack = Vec::new();
                line.chars()
                    .find(|c| {
                        match c {
                            '{' => stack.push('}'),
                            '(' => stack.push(')'),
                            '[' => stack.push(']'),
                            '<' => stack.push('>'),
                            _ => {
                                if stack.pop().unwrap() != *c {
                                    return true;
                                }
                            }
                        };
                        false
                    })
                    .map(|x| match x {
                        ')' => 3,
                        ']' => 57,
                        '}' => 1197,
                        '>' => 25137,
                        _ => panic!(),
                    })
                    .unwrap_or(0)
            })
            .sum()
    }

    fn part2(&self, input: Vec<Input>) -> Output {
        let res = input
            .into_iter()
            .map(|line| {
                let mut stack = Vec::new();
                if line
                    .chars()
                    .find(|c| {
                        match c {
                            '{' => stack.push('}'),
                            '(' => stack.push(')'),
                            '[' => stack.push(']'),
                            '<' => stack.push('>'),
                            _ => {
                                if stack.pop().unwrap() != *c {
                                    return true;
                                }
                            }
                        };
                        false
                    })
                    .is_none()
                {
                    // dbg!(stack.iter().join(""));
                    stack
                        .iter().rev()
                        .map(|y| match y {
                            ')' => 1,
                            ']' => 2,
                            '}' => 3,
                            '>' => 4,
                            _ => panic!(),
                        })
                        .fold1(|p, c| p * 5 + c)
                        .unwrap()
                } else {
                    0
                }
            })
            .filter(|x| *x != 0)
            .sorted()
            .collect::<Vec<Output>>();
        // dbg!(&res);
        res[res.len() / 2]
    }

    fn part1_answer(&self) -> Output {
        1670340
    }

    fn part2_answer(&self) -> Output {
        1954293920
    }
}
