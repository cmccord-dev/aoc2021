// use itertools::Itertools;
use std::{collections::HashMap, str::FromStr};

use itertools::Itertools;

use crate::{DayTrait, ParsingError};
type Output = usize;

#[derive(Debug, Clone)]
pub struct Input {
    input: Vec<u8>,
    output: Vec<u8>,
}

impl FromStr for Input {
    type Err = ParsingError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split('|').collect::<Vec<&str>>();
        let input = parts[0]
            .split(' ').filter(|x|x.len()>0).sorted_by_key(|x|x.len())
            .map(|x| str_to_byte(x))
            .collect();
        let output = parts[1]
            .split(' ').filter(|x|x.len()>0)
            .map(|x| str_to_byte(x))
            .collect();

        Ok(Self { input, output })
    }
}

fn str_to_byte(x: &str) -> u8 {
    x.chars()
        .map(|y| match y {
            'a' => 1,
            'b' => 2,
            'c' => 4,
            'd' => 8,
            'e' => 16,
            'f' => 32,
            'g' => 64,
            _ => panic!(),
        })
        .sum()
}

#[derive(Default)]
pub struct Day {}
impl DayTrait<Input, Output> for Day {
    fn get_num(&self) -> usize {
        8
    }

    fn part1(&self, input: Vec<Input>) -> Output {
        input
            .into_iter()
            .map(|x| {
                x.output
                    .into_iter()
                    .filter_map(|y| match y.count_ones() {
                        2 => Some(1),
                        4 => Some(4),
                        3 => Some(7),
                        7 => Some(8),
                        _ => None,
                    })
                    .count()
            })
            .sum()
    }

    fn part2(&self, input: Vec<Input>) -> Output {
        input
            .into_iter()
            .map(|mut x| {
                let mut dig_map: [u8; 256] = [0;256];
                let one = *&x.input[0];
                let seven = *&x.input[1];
                let four = *&x.input[2];
                let eight = *&x.input[9];
                let mut nine: u8=0;
                let mut zero: u8=0;
                let mut six: u8=0;
                let mut two: u8=0;
                let mut five: u8=0;
                let mut three: u8=0;
                dig_map[one as usize] = 1;
                dig_map[seven as usize] = 7;
                dig_map[four as usize] = 4;
                dig_map[eight as usize] = 8;
                for i in 6..=8 {
                    let y = x.input[i];
                    if(y & four) == four {
                        dig_map[y as usize] = 9;
                        nine = y;
                    }else if(y&one) == one {
                        dig_map[y as usize] = 0;
                        zero = y;
                    }else {
                        dig_map[y as usize] = 6;
                        six = y;
                    }
                }
                for i in 3..=5 {
                    let y = x.input[i];
                    if(y & six) == y {
                        dig_map[y as usize] = 5;
                        five = y;
                    }else if(y&one) == one {
                        dig_map[y as usize] = 3;
                        three = y;
                    }else {
                        dig_map[y as usize] = 2;
                        two = y;
                    }
                }
                // let nine = 
                //     x.input[6..=8]
                //         .iter()
                //         .find(|y| (str_to_byte(y) & four) == four)
                //         .unwrap(),
                // ;
                // let zero = str_to_byte(
                //     x.input[6..=8]
                //         .iter()
                //         .find(|y| (str_to_byte(y) != nine) && (str_to_byte(y) & seven) == seven)
                //         .unwrap(),
                // );
                // let six = str_to_byte(
                //     x.input[6..=8]
                //         .iter()
                //         .find(|y| str_to_byte(y) != nine && str_to_byte(y) != zero)
                //         .unwrap(),
                // );
                // let five = str_to_byte(
                //     x.input[3..=5]
                //         .iter()
                //         .find(|y| {
                //             let v = str_to_byte(y);
                //             (v & six) == v
                //         })
                //         .unwrap(),
                // );
                // dig_map.insert(five, 5);
                // let three = str_to_byte(
                //     x.input[3..=5]
                //         .iter()
                //         .find(|y| {
                //             let v = str_to_byte(y);
                //             (v & one) == one
                //         })
                //         .unwrap(),
                // );
                // dig_map.insert(three, 3);
                // let two = str_to_byte(
                //     x.input[3..=5]
                //         .iter()
                //         .find(|y| {
                //             let v = str_to_byte(y);
                //             v != five && v != three
                //         })
                //         .unwrap(),
                // );
                // dig_map.insert(two, 2);
                // dig_map.insert(one, 1);
                // dig_map.insert(seven, 7);
                // dig_map.insert(four, 4);
                // dig_map.insert(eight, 8);
                // dig_map.insert(nine, 9);
                // dig_map.insert(zero, 0);
                // dig_map.insert(six, 6);

                dbg!(x.output
                    .iter()
                    .map(|x| dig_map[*x as usize] as Output)
                    .fold1(|p, c| 10 * p + c)
                    .unwrap())
            })
            .sum()
    }

    fn part1_answer(&self) -> Output {
        469
    }

    fn part2_answer(&self) -> Output {
        1070957
    }
}
