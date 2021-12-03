use crate::{DayTrait, ParsingError};
type Input = String;
type Output = i32;
use std::i32;

// #[derive(Debug, Clone)]
// pub struct Input {
//     dir: i32,
//     count: i32,
// }

// impl FromStr for Input {
//     type Err = ParsingError;
//     fn from_str(s: &str) -> Result<Self, Self::Err> {
//         let parts = s.split(' ').collect::<Vec<&str>>();
//         let dir = match parts[0] {
//             "down" => 1,
//             "up" => -1,
//             _ => 0,
//         };
//         let count = parts[1].parse()?;
//         Ok(Self { dir, count })
//     }
// }

fn find_most_common_in_pos(input: &Vec<String>, pos: usize) -> i32 {
    if 2 * input.iter().fold(0, |total, curr| {
        total
            + if curr.as_bytes()[pos] == '1' as u8 {
                1
            } else {
                0
            }
    }) >= input.len() as i32
    {
        1
    } else {
        0
    }
}

#[derive(Default)]
pub struct Day {}
impl DayTrait<Input, Output> for Day {
    fn get_num(&self) -> usize {
        3
    }

    fn part1(&self, input: Vec<Input>) -> Output {
        let len = input[0].len();
        let mut counts: Vec<i32> = Vec::with_capacity(len);
        let mut gamma = 0;
        for i in 0..len {
            gamma = gamma * 2 + find_most_common_in_pos(&input, i);
        }
        // input.iter().for_each(|n| {
        //     n.chars()
        //         .enumerate()
        //         .for_each(|(i, c)| counts[i] += if c == '1' { 1 } else { 0 })
        // });
        // let gamma = counts.iter().fold(0, |p, &c| {
        //     2 * p + if c >= (input.len() as i32 / 2) { 1 } else { 0 }
        // });
        let epsilon = gamma ^ ((1 << len) - 1);
        gamma * epsilon
    }

    fn part2(&self, input: Vec<Input>) -> Output {
        let len = input[0].len();
        let mut oxygen = input.clone();
        let mut i = 0;
        while oxygen.len() > 1 {
            let desired = if find_most_common_in_pos(&oxygen, i) == 1 {
                '1'
            } else {
                '0'
            } as u8;
            oxygen = oxygen
                .into_iter()
                .filter(|s| s.as_bytes()[i] == desired)
                .collect();
            i += 1;
        }
        let mut co2 = input;
        i = 0;
        while co2.len() > 1 {
            let desired = if find_most_common_in_pos(&co2, i) == 1  {
                '0'
            } else {
                '1'
            } as u8;
            co2 = co2
                .into_iter()
                .filter(|s| s.as_bytes()[i] == desired)
                .collect();
            i += 1;
        }
        // dbg!(&oxygen, &co2);
        i32::from_str_radix(&co2[0], 2).unwrap() * i32::from_str_radix(&oxygen[0], 2).unwrap()
    }

    fn part1_answer(&self) -> Output {
        4160394
    }

    fn part2_answer(&self) -> Output {
        0
    }
}
