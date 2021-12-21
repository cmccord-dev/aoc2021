use itertools::Itertools;
use std::str::FromStr;

use crate::{DayTrait, ParsingError};
// type Input = u64;
type Output = String;
use nalgebra::Vector2;

type Board = Vec<Vec<bool>>;

#[derive(Debug, Clone)]
enum Fold {
    X(usize),
    Y(usize),
}

#[derive(Debug, Clone)]
pub struct Input {
    board: Board,
    folds: Vec<Fold>,
}

impl FromStr for Input {
    type Err = ParsingError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split("\n\n");
        let points: Vec<Vec<usize>> = parts
            .next()
            .unwrap()
            .lines()
            .map(|x| {
                x.split(",")
                    .map(|y| y.parse().unwrap())
                    .collect::<Vec<usize>>()
            })
            .collect();
        let max: (usize, usize) = points
            .iter()
            .fold((0usize, 0usize), |p, c| (p.0.max(c[0]), p.1.max(c[1])));
        let mut board = vec![vec![false; max.0 + 1]; max.1 + 1];
        points.iter().for_each(|x| board[x[1]][x[0]] = true);
        let folds = parts
            .next()
            .unwrap()
            .lines()
            .map(|x| {
                let p = x
                    .split(' ')
                    .skip(2)
                    .next()
                    .unwrap()
                    .split("=")
                    .collect_vec();
                let v = p[1].parse().unwrap();
                match p[0] {
                    "y" => Fold::Y(v),
                    "x" => Fold::X(v),
                    _ => panic!(),
                }
            })
            .collect_vec();

        Ok(Self { board, folds })
    }
}

fn do_fold(board: &mut Board, fold: &Fold, size: (usize, usize)) -> (usize, usize) {
    match fold {
        Fold::X(v) => {
            for j in 0..*v {
                let end = 2 * v - j;
                for i in 0..size.1 {
                    if end < size.0 {
                        board[i][j] |= board[i][end];
                    }
                }
            }
            (*v, size.1)
        }
        Fold::Y(v) => {
            for i in 0..*v {
                let end = 2 * v - i;
                for j in 0..size.0 {
                    if end < size.1 {
                        board[i][j] |= board[end][j];
                    }
                }
            }
            (size.0, *v)
        }
    }
}

#[derive(Default)]
pub struct Day {}
impl DayTrait<Input, Output> for Day {
    fn get_num(&self) -> usize {
        13
    }

    fn read_input(&self) -> Result<Vec<Input>, <Input as FromStr>::Err> {
        let file = std::fs::read_to_string(format!("input/day{}.in", self.get_num())).unwrap();
        let res = file.parse();
        res.map(|x| vec![x])
    }

    fn part1(&self, input: Vec<Input>) -> Output {
        let input = input.into_iter().next().unwrap();
        let mut board = input.board;
        let folds = input.folds;
        let mut size = (board[0].len(), board.len());
        size = do_fold(&mut board, &folds[0], size);
        // println!("{}",board
        //     .iter()
        //     .take(size.1)
        //     .map(|x| x
        //         .iter()
        //         .take(size.0)
        //         .map(|y| if *y { '#' } else { '.' })
        //         .join(""))
        //     .join("\n"));
        board
            .iter()
            .take(size.1)
            .map(|x| {
                x.iter()
                    .take(size.0)
                    .map(|y| if *y { 1 } else { 0 })
                    .sum::<usize>()
            })
            .sum::<usize>().to_string()
    }

    fn part2(&self, input: Vec<Input>) -> Output {
        let input = input.into_iter().next().unwrap();
        let mut board = input.board;
        let folds = input.folds;
        let mut size = (board[0].len(), board.len());
        for fold in folds.iter() {
            size = do_fold(&mut board, fold, size);
        }

        format!(
            "\n{}",
            board
                .iter()
                .take(size.1)
                .map(|x| x
                    .iter()
                    .take(size.0)
                    .map(|y| if *y { '#' } else { '.' })
                    .join(""))
                .join("\n")
        )
    }

    fn part1_answer(&self) -> Output {
        "".into()
    }

    fn part2_answer(&self) -> Output {
        "".into()
    }
}
