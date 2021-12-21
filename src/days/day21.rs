// use itertools::Itertools;
use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

use crate::{DayTrait, ParsingError};
type Output = usize;

#[derive(Debug, Clone)]
pub struct Input {
    start: usize,
}

impl FromStr for Input {
    type Err = ParsingError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split(": ").collect::<Vec<&str>>();
        let start = parts[1].parse()?;
        Ok(Self { start })
    }
}

trait Dice {
    fn get_count(&self) -> usize;
    fn roll(&mut self) -> usize;
}

struct DeterministicDice {
    state: usize,
    rolls: usize,
}
impl DeterministicDice {
    pub fn new() -> Self {
        Self { state: 0, rolls: 0 }
    }
}
impl Dice for DeterministicDice {
    fn get_count(&self) -> usize {
        self.rolls
    }

    fn roll(&mut self) -> usize {
        self.rolls += 1;
        let ret = self.state + 1;
        self.state = (ret) % 100;
        ret
    }
}

fn play(dice: &mut dyn Dice, players: Vec<Input>) -> usize {
    let mut a = players[0].start - 1;
    let mut b = players[1].start - 1;
    let mut a_score = 0;
    let mut b_score = 0;
    let mut turn = 0;
    while a_score < 1000 && b_score < 1000 {
        let v = dice.roll() + dice.roll() + dice.roll();
        if turn < 10 {
            println!(
                "Turn {}.  a {} score {} b {} score {} roll {}",
                turn, a, a_score, b, b_score, v
            );
        }
        if (turn & 1) == 0 {
            a = (a + v) % 10;
            a_score += a + 1;
        } else {
            b = (b + v) % 10;
            b_score += b + 1;
        }
        turn += 1;
    }
    let s = if a_score < b_score { a_score } else { b_score };
    dbg!(dice.get_count(), s, a_score, b_score);
    s * dice.get_count()
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
struct BoardState {
    a: usize,
    b: usize,
    a_score: usize,
    b_score: usize,
}
#[derive(Default)]
pub struct Day {}
impl DayTrait<Input, Output> for Day {
    fn get_num(&self) -> usize {
        21
    }

    fn part1(&self, input: Vec<Input>) -> Output {
        let mut dice = DeterministicDice::new();
        play(&mut dice, input)
    }

    fn part2(&self, input: Vec<Input>) -> Output {
        let mut board_states: HashMap<BoardState, usize> = HashMap::new();
        // let mut a_map: HashSet<(usize, usize)> = HashSet::new();
        // let mut b_map: HashSet<(usize, usize)> = HashSet::new();
        // a_map.insert((input[0].start, 0));
        // b_map.insert((input[1].start, 0));
        board_states.insert(
            BoardState {
                a: input[0].start,
                b: input[1].start,
                a_score: 0,
                b_score: 0,
            },
            1,
        );

        let mut a_wins = 0;
        let mut b_wins = 0;
        let mut turn = 0;
        let rolls = vec![
            (3usize, 1usize),
            (4, 3),
            (5, 6),
            (6, 7),
            (7, 6),
            (8, 3),
            (9, 1),
        ];
        while board_states.len() > 0 {
            let mut next_states = HashMap::new();
            board_states.iter().for_each(|state| {
                for roll in &rolls {
                    let mut new_state = state.0.clone();
                    if (turn & 1) == 0 {
                        new_state.a += roll.0;
                        if new_state.a > 10 {
                            new_state.a -= 10;
                        }
                        new_state.a_score += new_state.a;
                        if new_state.a_score >= 21 {
                            a_wins += roll.1 * state.1;
                            continue;
                        }
                    } else {
                        new_state.b += roll.0;
                        if new_state.b > 10 {
                            new_state.b -= 10;
                        }
                        new_state.b_score += new_state.b;

                        if new_state.b_score >= 21 {
                            b_wins += roll.1 * state.1;
                            continue;
                        }
                    }
                    *next_states.entry(new_state).or_insert(0) += roll.1 * state.1;
                }
            });
            turn += 1;
            std::mem::swap(&mut board_states, &mut next_states);
        }
        // while a_map.len() > 0 && b_map.len() > 0 {}
        a_wins.max(b_wins)
    }

    fn part1_answer(&self) -> Output {
        1670340
    }

    fn part2_answer(&self) -> Output {
        1954293920
    }
}
