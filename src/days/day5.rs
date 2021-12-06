// use itertools::Itertools;
use std::{collections::HashMap, str::FromStr};

use crate::{DayTrait, ParsingError};
use itertools::Itertools;
use nalgebra::vector;
use nalgebra::Vector2;
type Output = i32;

#[derive(Debug, Clone)]
pub struct Input {
    start: Vector2<i32>,
    end: Vector2<i32>,
}

impl FromStr for Input {
    type Err = ParsingError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split(" -> ").collect::<Vec<&str>>();

        let res: Vec<Vec<i32>> = parts
            .into_iter()
            .map(|x| x.split(',').map(|x| x.parse::<i32>().unwrap()).collect())
            .collect();
        Ok(Self {
            start: vector!(res[0][0], res[0][1]),
            end: vector!(res[1][0], res[1][1]),
        })
    }
}

impl Input {
    // fn get_len(&self) -> f32 {
    //     Vector2::new(
    //         (self.end[0] - self.start[0]) as f32,
    //         (self.end[1] - self.start[1]) as f32,
    //     )
    //     .magnitude()
    // }
    fn is_horizontal_or_perpendicular(&self) -> bool {
        self.end[0] == self.start[0] || self.end[1] == self.start[1]
    }
    fn line(&self) -> Vec<Vector2<i32>> {
        let count = (self.end - self.start).abs().max();
        let start: Vector2<f32> =
            vector![self.start[0] as f32, self.start[1] as f32] + vector![0.5, 0.5];
        // let len = self.get_len();
        let delta: Vector2<f32> = vector![
            (self.end[0] - self.start[0]) as f32,
            (self.end[1] - self.start[1]) as f32
        ] / (count) as f32;
        (0..count + 1)
            .map(|x| start + delta * (x as f32))
            .map(|x| vector![x[0] as i32, x[1] as i32])
            .collect()
    }
}

#[derive(Default)]
pub struct Day {}
impl DayTrait<Input, Output> for Day {
    fn get_num(&self) -> usize {
        5
    }

    fn part1(&self, input: Vec<Input>) -> Output {
        let mut map: HashMap<Vector2<i32>, usize> = HashMap::new();
        input
            .into_iter()
            .filter(|x| x.is_horizontal_or_perpendicular())
            .for_each(|x| {
                x.line().into_iter().for_each(|y| {
                    if map.contains_key(&y) {
                        *map.get_mut(&y).unwrap() += 1;
                    } else {
                        map.insert(y, 1);
                    }
                })
            });
        map.values().filter(|x| **x > 1).count() as i32
    }

    fn part2(&self, input: Vec<Input>) -> Output {
        let mut map: HashMap<Vector2<i32>, usize> = HashMap::new();
        input
            .into_iter()
            // .filter(|x| x.is_horizontal_or_perpendicular())
            .for_each(|x| {
                x.line().into_iter().for_each(|y| {
                    if map.contains_key(&y) {
                        *map.get_mut(&y).unwrap() += 1;
                    } else {
                        map.insert(y, 1);
                    }
                })
            });
        map.values().filter(|x| **x > 1).count() as i32
    }

    fn part1_answer(&self) -> Output {
        1670340
    }

    fn part2_answer(&self) -> Output {
        1954293920
    }
}
