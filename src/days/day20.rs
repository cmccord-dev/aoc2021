// use itertools::Itertools;
use crate::{DayTrait, ParsingError};
use itertools::Itertools;
use nalgebra::{vector, Vector2};
use std::{collections::HashSet, str::FromStr};
type Output = usize;

#[derive(Debug, Clone)]
pub struct Input {
    key: Vec<bool>,
    initial: HashSet<Vector2<i32>>,
}

impl FromStr for Input {
    type Err = ParsingError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split("\n\n");
        let key = parts
            .next()
            .unwrap()
            .chars()
            .map(|x| if x == '.' { false } else { true })
            .collect_vec();
        assert_eq!(key.len(), 512);
        let mut initial = HashSet::new();
        parts
            .next()
            .unwrap()
            .lines()
            .enumerate()
            .for_each(|(i, x)| {
                x.chars().enumerate().for_each(|(j, c)| {
                    if c == '#' {
                        initial.insert(vector![i as i32, j as i32]);
                    }
                })
            });
        Ok(Self { key, initial })
    }
}

fn print_map(m: &HashSet<Vector2<i32>>) {
    let min = m
        .iter()
        .map(|x| *x)
        .fold1(|p, c| p.zip_map(&c, |a, b| a.min(b)))
        .unwrap();
    let max = m
        .iter()
        .map(|x| *x)
        .fold1(|p, c| p.zip_map(&c, |a, b| a.max(b)))
        .unwrap();
    for i in min[0]..=max[0] {
        println!(
            "{}",
            (min[1]..=max[1])
                .map(|j| if m.contains(&vector![i, j]) { '#' } else { '.' })
                .join("")
        );
    }
}

fn run_for_days(input: Input, days: usize) -> Output {
    let mut next = HashSet::new();
    let mut curr = input.initial.clone();

    let mut min = curr
        .iter()
        .map(|x| *x)
        .fold1(|p, c| p.zip_map(&c, |a, b| a.min(b)))
        .unwrap();
    let mut max = curr
        .iter()
        .map(|x| *x)
        .fold1(|p, c| p.zip_map(&c, |a, b| a.max(b)))
        .unwrap();

    // print_map(&curr);
    for day in 0..days {
        let mut next_min = vector![i32::MAX, i32::MAX];
        let mut next_max = vector![i32::MIN, i32::MIN];
        // println!("Running day {}", day);
        for i in min[0] - 1..=max[0] + 1 {
            for j in min[1] - 1..=max[1] + 1 {
                let mut c = 0;
                for ii in -1..=1 {
                    for jj in -1..=1 {
                        c <<= 1;
                        if curr.contains(&vector![i + ii, j + jj]) {
                            c |= 1;
                        }
                        if input.key[0] && (day & 1) == 1 {
                            if ii + i < min[0]
                                || ii + i > max[0]
                                || jj + j < min[1]
                                || jj + j > max[1]
                            {
                                c |= 1;
                            }
                        }
                    }
                }
                if input.key[c] {
                    next.insert(vector![i, j]);
                    // if i < next_min[0] {
                    //     next_min[0] = i;
                    // }
                    // if i > max[0] {
                    //     next_max[0] = i;
                    // }
                    // if j < min[1] {
                    //     next_min[1] = j;
                    // }
                    // if j > max[1] {
                    //     next_max[1] = j;
                    // }
                }
            }
        }
        // let to_visit: HashSet<Vector2<i32>> = curr
        //     .iter()
        //     .map(|x| (-1..=1).map(move |i| (-1..=1).map(move |j| (x + vector![i, j]))))
        //     .flatten()
        //     .flatten()
        //     .collect();
        // let curr_tmp = &curr;
        // to_visit.iter().for_each(|v| {
        //     let c = (-1..=1)
        //         .map(|i| {
        //             (-1..=1).map(move |j| {
        //                 let t = v + vector![i, j];
        //                 if curr_tmp.contains(&(t)) {
        //                     return true;
        //                 }
        //                 if (day & 1) == 0 {
        //                     return false;
        //                 }
        //                 if t[0] < min[0] || t[0] > max[0] || t[1] < min[1] || t[1] > max[1] {
        //                     return true;
        //                 }
        //                 return false;
        //             })
        //         })
        //         .flatten()
        //         .fold(0, |p, c| (p << 1) | if c { 1 } else { 0 });
        //     if input.key[c as usize] {
        //         next.insert(*v);
        //     }
        // });
        // print_map(&next);
        std::mem::swap(&mut curr, &mut next);
        // min = next_min;
        // max = next_max;
        min = min - vector![1,1];
        max = max + vector![1,1];
        // std::mem::swap(&mut min, &mut next_min);
        // std::mem::swap(&mut max, &mut next_max);
        next.clear();
    }

    curr.len()
}

#[derive(Default)]
pub struct Day {}
impl DayTrait<Input, Output> for Day {
    fn get_num(&self) -> usize {
        20
    }
    fn read_input(&self) -> Result<Vec<Input>, <Input as FromStr>::Err> {
        let file = std::fs::read_to_string(format!("input/day{}.in", self.get_num())).unwrap();
        let res = file.parse();
        res.map(|x| vec![x])
    }

    fn part1(&self, input: Vec<Input>) -> Output {
        let mut input = input;
        run_for_days(input.remove(0), 2)
    }

    fn part2(&self, input: Vec<Input>) -> Output {
        let mut input = input;
        run_for_days(input.remove(0), 50)
    }

    fn part1_answer(&self) -> Output {
        391888
    }

    fn part2_answer(&self) -> Output {
        1754597645339
    }
}
