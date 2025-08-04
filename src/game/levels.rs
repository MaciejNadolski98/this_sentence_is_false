use rand::{random, seq::SliceRandom, thread_rng};
use bevy::prelude::*;

use crate::game::sentence::{Sentence, Value};

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<CurrentLevel>();
    }
}

#[derive(Clone)]
pub struct Level {
    pub sentences: Vec<SentenceDescription>,
}

impl Level {
    pub fn get(_level_id: u32) -> Self {
        return generate_level();
    }

    pub fn take_apart(&self) -> (Vec<Sentence>, Vec<Value>, Vec<Value>, Vec<Value>) {
        let mut sentences = Vec::new();
        let mut ids = Vec::new();
        let mut numbers = Vec::new();
        let mut bools = Vec::new();

        for sentence in self.sentences.iter() {
            sentences.push(sentence.sentence);
            for &value in sentence.values.iter() {
                match value {
                    Value::Id(_) => ids.push(value),
                    Value::Number(_) => numbers.push(value),
                    Value::Bool(_) => bools.push(value),
                }
            }
        }

        (sentences, ids, numbers, bools)
    }

    pub fn shuffle(&self) -> Self {
        let (sentences, mut ids, mut numbers, mut bools) = self.take_apart();

        ids.shuffle(&mut thread_rng());
        numbers.shuffle(&mut thread_rng());
        bools.shuffle(&mut thread_rng());

        let mut shuffled_sentences = Vec::new();

        for sentence in sentences {
            match sentence {
                Sentence::IdSentenceIsBool => {
                    let id = ids.pop().unwrap();
                    let truth = bools.pop().unwrap();
                    shuffled_sentences.push(SentenceDescription {
                        sentence,
                        values: vec![id, truth],
                    });
                },
                Sentence::ThereAreNumberOfBoolSentences => {
                    let number = numbers.pop().unwrap();
                    let truth = bools.pop().unwrap();
                    shuffled_sentences.push(SentenceDescription {
                        sentence,
                        values: vec![number, truth],
                    });
                },
                Sentence::ClosestBoolIsNumberAway => {
                    let truth = bools.pop().unwrap();
                    let number = numbers.pop().unwrap();
                    shuffled_sentences.push(SentenceDescription {
                        sentence,
                        values: vec![truth, number],
                    });
                },
                Sentence::ThereAreNumberOfAlternatingGroups => {
                    let number = numbers.pop().unwrap();
                    shuffled_sentences.push(SentenceDescription {
                        sentence,
                        values: vec![number],
                    });
                },
                Sentence::IdSentenceAndIdSentenceAreTheSame | Sentence::IdSentenceAndIdSentenceAreDifferent => {
                    let id1 = ids.pop().unwrap();
                    let id2 = ids.pop().unwrap();
                    shuffled_sentences.push(SentenceDescription {
                        sentence,
                        values: vec![id1, id2],
                    });
                },
            }
        }

        Self {
            sentences: shuffled_sentences,
        }
    }
}

#[derive(Resource, Default)]
pub struct CurrentLevel(pub u32);

#[derive(Clone)]
pub struct SentenceDescription {
    pub sentence: Sentence,
    pub values: Vec<Value>,
}

impl SentenceDescription {
    pub fn generate(is_true: bool, position: usize, truths: &Vec<bool>) -> Self {
        match random::<u32>() % 6 {
            0 => {
                let sentence = Sentence::IdSentenceIsBool;
                let id = random::<u32>() as usize % truths.len();
                let truth = if id != position {
                    truths[id] ^ !is_true
                } else {
                    true
                };
                SentenceDescription {
                    sentence: sentence,
                    values: vec![Value::Id(id as u32 + 1), Value::Bool(truth)],
                }
            },
            1 => {
                let sentence = Sentence::ThereAreNumberOfBoolSentences;
                let truth = random::<bool>();
                let count = truths.iter().filter(|&&t| t == truth).count() as u32;

                let number = if is_true {
                    count
                } else {
                    random_range_other_than(1, truths.len() as u32, count)
                };
                SentenceDescription {
                    sentence: sentence,
                    values: vec![Value::Number(number), Value::Bool(truth)],
                }
            },
            2 => {
                let sentence = Sentence::ClosestBoolIsNumberAway;
                let truth = random::<bool>();
                let mut number = 0;
                for i in 1..truths.len() {
                    if position + i < truths.len() && truths[position + i] == truth {
                        number = i as u32;
                        break;
                    } else if position >= i && truths[position - i] == truth {
                        number = i as u32;
                        break;
                    }
                }

                let (truth, number) = if is_true {
                    if number == 0 {
                        (!truth, 1)
                    } else {
                        (truth, number)
                    }
                } else {
                    if number == 0 {
                        (truth, random::<u32>() % (truths.len() as u32) + 1)
                    } else {
                        (truth, random_range_other_than(1, truths.len() as u32, number))
                    }
                };
                
                SentenceDescription {
                    sentence: sentence,
                    values: vec![Value::Bool(truth), Value::Number(number)],
                }
            },
            3 => {
                let sentence = Sentence::ThereAreNumberOfAlternatingGroups;
                let mut groups = 1;
                for i in 1..truths.len() {
                    if truths[i] != truths[i - 1] {
                        groups += 1;
                    }
                }
                
                let number = if is_true {
                    groups
                } else {
                    random_range_other_than(1, truths.len() as u32, groups as u32)
                };
                SentenceDescription {
                    sentence: sentence,
                    values: vec![Value::Number(number)],
                }
            }
            4 => {
                let mut sentence = Sentence::IdSentenceAndIdSentenceAreTheSame;
                let id1 = random::<u32>() as usize % truths.len();
                let look_for = if is_true {
                    truths[id1]
                } else {
                    !truths[id1]
                };
                let id2 = {
                    let options: Vec<usize> = truths.iter()
                        .enumerate()
                        .filter(|&(i, &truth)| truth == look_for && i != id1)
                        .map(|(i, _)| i)
                        .collect();
                    if options.len() != 0 {
                        options[random::<u32>() as usize % options.len()]
                    } else {
                        sentence = Sentence::IdSentenceAndIdSentenceAreDifferent;
                        random_range_other_than(0, truths.len() as u32 - 1, id1 as u32) as usize
                    }
                };

                SentenceDescription {
                    sentence: sentence,
                    values: vec![Value::Id(id1 as u32 + 1), Value::Id(id2 as u32 + 1)],
                }
            },
            _ => {
                let mut sentence = Sentence::IdSentenceAndIdSentenceAreDifferent;
                let id1 = random::<u32>() as usize % truths.len();
                let look_for = if is_true {
                    !truths[id1]
                } else {
                    truths[id1]
                };
                let id2 = {
                    let options: Vec<usize> = truths.iter()
                        .enumerate()
                        .filter(|&(id, &truth)| truth == look_for && id != id1)
                        .map(|(i, _)| i)
                        .collect();
                    if options.len() != 0 {
                        options[random::<u32>() as usize % options.len()]
                    } else {
                        sentence = Sentence::IdSentenceAndIdSentenceAreTheSame;
                        random_range_other_than(0, truths.len() as u32 - 1, id1 as u32) as usize
                    }
                };

                SentenceDescription {
                    sentence: sentence,
                    values: vec![Value::Id(id1 as u32 + 1), Value::Id(id2 as u32 + 1)],
                }
            }
        }
    }
}

fn random_range_other_than(start: u32, stop: u32, other_than: u32) -> u32 {
    let mut result = random::<u32>() % (stop - start) + start;
    if result >= other_than {
        result += 1;
    }
    result
}

fn generate_level() -> Level {
    let n = (random::<u32>() % 4 + 3) as usize;
    let mut solution_truths = Vec::<bool>::new();
    for _ in 0..n {
        solution_truths.push(random::<bool>());
    }

    let mut solution_sentences = Vec::<SentenceDescription>::new();
    for i in 0..n {
        solution_sentences.push(SentenceDescription::generate(solution_truths[i], i, &solution_truths));
    }

    (Level {
        sentences: solution_sentences,
    }).shuffle()
}
