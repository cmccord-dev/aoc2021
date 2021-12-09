use itertools::Itertools;
use std::str::FromStr;

use crate::{DayTrait, ParsingError};
// type Input = u64;
type Output = usize;

type Board = Vec<Vec<usize>>;

#[derive(Debug, Clone)]
pub struct Input {
    order: Vec<usize>,
    boards: Vec<Board>,
}

impl FromStr for Input {
    type Err = ParsingError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        let order = lines
            .next()
            .unwrap()
            .split(",")
            .map(|x| x.parse::<usize>())
            .collect::<Result<Vec<usize>, _>>()?;
        // let mut boards = Vec::new();

        let boards = lines
            .chunks(6)
            .into_iter()
            .map(|chunk| {
                chunk
                    .skip(1)
                    .map(|x| {
                        x.split(" ")
                            .filter(|x| x.len() > 0)
                            .map(|x| x.parse::<usize>().unwrap())
                            .collect::<Vec<usize>>()
                    })
                    .collect()
            })
            .collect();
        Ok(Self { order, boards })
    }
}

fn find_winning_turn(map: &[usize; 100], board: &Board) -> usize {
    let mut min = 100;
    for i in 0..5 {
        let row_min = board[i].iter().map(|x| map[*x]).max().unwrap();
        min = min.min(row_min);
        let col_min = board.iter().map(|x| map[x[i]]).max().unwrap();
        min = min.min(col_min);
    }
    min
}

#[derive(Default)]
pub struct Day {}
impl DayTrait<Input, Output> for Day {
    fn get_num(&self) -> usize {
        4
    }

    fn read_input(&self) -> Result<Vec<Input>, <Input as FromStr>::Err> {
        let file = std::fs::read_to_string(format!("input/day{}.in", self.get_num())).unwrap();
        let res = file.parse();
        res.map(|x| vec![x])
    }

    fn part1(&self, input: Vec<Input>) -> Output {
        let mut map: [usize; 100] = [101; 100];
        let input = &input[0];
        input.order.iter().enumerate().for_each(|(i, x)| {
            map[*x] = i;
        });
        // dbg!(map);
        let winner = input
            .boards
            .iter()
            .map(|x| (find_winning_turn(&map, x), x))
            .fold1(|p, c| if p.0 < c.0 { p } else { c })
            .unwrap();

        // dbg!(&winner, &input.order[winner.0]);
        let turn = winner.0;
        winner
            .1
            .iter()
            .map(|x| {
                x.iter()
                    .map(|&y| if map[y] <= turn { 0 } else { y })
                    .sum::<usize>()
            })
            .sum::<usize>()
            * input.order[turn]
    }

    fn part2(&self, input: Vec<Input>) -> Output {
        let mut map: [usize; 100] = [101; 100];
        let input = &input[0];
        input.order.iter().enumerate().for_each(|(i, x)| {
            map[*x] = i;
        });
        // dbg!(map);
        let winner = input
            .boards
            .iter()
            .map(|x| (find_winning_turn(&map, x), x))
            .fold1(|p, c| if p.0 > c.0 { p } else { c })
            .unwrap();

        // dbg!(&winner, &input.order[winner.0]);
        let turn = winner.0;
        winner
            .1
            .iter()
            .map(|x| {
                x.iter()
                    .map(|&y| if map[y] <= turn { 0 } else { y })
                    .sum::<usize>()
            })
            .sum::<usize>()
            * input.order[turn]
    }

    fn part1_answer(&self) -> Output {
        65325
    }

    fn part2_answer(&self) -> Output {
        4624
    }
}
