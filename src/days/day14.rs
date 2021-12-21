use itertools::Itertools;
use reqwest::header::Iter;
use std::{collections::HashMap, str::FromStr};

use crate::{DayTrait, ParsingError};
// type Input = u64;
type Output = usize;
use nalgebra::Vector2;

type Board = Vec<Vec<bool>>;

#[derive(Debug, Clone)]
enum Fold {
    X(usize),
    Y(usize),
}

#[derive(Debug, Clone)]
pub struct Input {
    dat: String,
    map: HashMap<(char, char), char>,
}

impl FromStr for Input {
    type Err = ParsingError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split("\n\n");
        let dat = parts.next().unwrap().into();
        let mut map = HashMap::new();
        parts.next().unwrap().lines().for_each(|x| {
            let mut v = x.split(" -> ");
            let mut pair = v.next().unwrap().chars();
            map.insert(
                (pair.next().unwrap(), pair.next().unwrap()),
                v.next().unwrap().chars().next().unwrap(),
            );
        });
        Ok(Self { dat, map })
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

        0
    }

    fn part2(&self, input: Vec<Input>) -> Output {
        let input = input.into_iter().next().unwrap();
        let mut count_map: HashMap<(char, char), Vec<(char, char)>> = HashMap::new();
        let mut counts: HashMap<(char, char), usize> = HashMap::new();
        // input.dat.chars().tuple_windows().for_each(|x:(char,char)| {
        //     // counts.entry(key)
        // });
        0
    }

    fn part1_answer(&self) -> Output {
        0
    }

    fn part2_answer(&self) -> Output {
        0
    }
}
