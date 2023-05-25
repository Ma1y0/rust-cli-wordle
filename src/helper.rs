use std::fmt::Display;

use colored::{ColoredString, Colorize};
use random_word::{all, gen_len};

pub fn get_random_world() -> Option<&'static str> {
    gen_len(5)
}

pub fn validate(word: &str) -> bool {
    all().iter().any(|&x| x == word.to_lowercase().trim()) && word.trim().len() == 5
}

pub enum LatterState {
    Right(char),
    Place(char),
    Not(char),
}

pub enum GuessResult {
    Correct,
    Incorrect(ColorOutput)
}

pub fn check(word: &str, ans: &str) -> GuessResult {
    let word_chars = word.trim().chars();
    let ans_chars = ans.trim().chars();
    let mut a: Vec<LatterState> = Vec::new();

    if word .trim() == ans.trim() {
        return GuessResult::Correct;
    }

    for (i, ch) in word_chars.enumerate() {
        if let Some(index) = ans_chars.clone().position(|x| x == ch) {
            if index == i {
                a.push(LatterState::Right(ch))
            } else {
                a.push(LatterState::Place(ch))
            }
        } else {
            a.push(LatterState::Not(ch))
        }
    }

    GuessResult::Incorrect(ColorOutput(a))
}

pub struct ColorOutput(Vec<LatterState>);

impl Display for ColorOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in &self.0 {
            match i {
                LatterState::Right(letter) => write!(f, "{}", letter.to_string().green())?,
                LatterState::Place(latter) => write!(f, "{}", latter.to_string().bright_yellow())?,
                LatterState::Not(latter) => write!(f, "{}", latter.to_string().bright_black())?,
            };
        }

        Ok(())
    }
}
