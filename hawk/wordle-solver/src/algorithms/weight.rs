use crate::{Correctness, DICTIONARY, Guess, Guesser, get_letter_freq};
use std::borrow::Cow;

pub struct Weight {
    remaining: Vec<(&'static str, usize, [u8; 26])>,
}

struct Candidate {
    word: &'static str,
    goodness: f64,
}

impl Weight {
    pub fn new() -> Self {
        Self {
            remaining: Vec::from_iter(DICTIONARY.lines().map(|line| {
                let (word, count) = line
                    .split_once(' ')
                    .expect("every line is work + space + frequency");
                let count: usize = count.parse().expect("every count is a number");
                (word, count, get_letter_freq(word))
            })),
        }
    }
}

impl Guesser for Weight {
    fn guess(&mut self, history: &[Guess]) -> String {
        if history.is_empty() {
            return "tares".to_string();
        }

        if let Some(last) = history.last() {
            self.remaining.retain(|(word, _, _)| last.matches(word));
        }

        let remaining_count: usize = self.remaining.iter().fold(0, |acc, (_, c, _)| acc + c);

        let mut best: Option<Candidate> = None;
        for (word, count, _) in &self.remaining {
            let mut sum = 0.0;
            for pattern in Correctness::patterns() {
                // considering a world where we _did_ guess `word` and got `pattern` as the
                // correctness. now, compute what _then_ is left.
                let mut in_pattern_total = 0;
                for (candidate, count, letter_freq) in &self.remaining {
                    let g = Guess {
                        word: Cow::Borrowed(word),
                        mask: pattern,
                    };
                    if g.matches_with_letter_freq(candidate, letter_freq) {
                        in_pattern_total += count;
                    }
                }
                if in_pattern_total == 0 {
                    continue;
                }
                // TODO: apply sigmoid
                let p_of_this_pattern = in_pattern_total as f64 / remaining_count as f64;
                sum += p_of_this_pattern * p_of_this_pattern.log2();
            }
            let p_word = *count as f64 / remaining_count as f64;
            let goodness = p_word * -sum;
            if let Some(c) = &best {
                // Is this one better?
                if goodness > c.goodness {
                    best = Some(Candidate { word, goodness })
                }
            } else {
                best = Some(Candidate { word, goodness })
            }
        }
        best.unwrap().word.to_string()
    }
}
