use std::{borrow::Cow, collections::HashSet};

pub mod algorithms;

const DICTIONARY: &str = include_str!("../dictionary.txt");

pub struct Wordle {
    dictionary: HashSet<&'static str>,
}

impl Wordle {
    pub fn new() -> Self {
        Self {
            dictionary: HashSet::from_iter(DICTIONARY.lines().map(|line| {
                line.split_once(' ')
                    .expect("every line is word + space + frequency")
                    .0
            })),
        }
    }

    pub fn play<G: Guesser>(&self, answer: &'static str, mut guesser: G) -> Option<usize> {
        let mut history = Vec::new();
        // Wordle only allows six guesses.
        // We allow more to avoid chopping off the score distrubution for stats purposes.
        for i in 1..=32 {
            let guess = guesser.guess(&history);
            if guess == answer {
                return Some(i);
            }
            assert!(self.dictionary.contains(&guess[..]));
            let correctness = Correctness::compute(answer, &guess);
            history.push(Guess {
                word: Cow::Owned(guess),
                mask: correctness,
            })
        }
        None
    }
}

fn get_letter_freq(s: &str) -> [u8; 26] {
    let mut result = [0u8; 26];
    for c in s.bytes() {
        result[(c - b'a') as usize] += 1;
    }
    result
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd)]
pub enum Correctness {
    /// Green
    Correct,
    /// Yellow
    Misplaced,
    /// Gray
    Wrong,
}

impl Correctness {
    fn compute(answer: &str, guess: &str) -> [Self; 5] {
        assert_eq!(answer.len(), 5);
        assert_eq!(guess.len(), 5);
        let mut c = [Self::Wrong; 5];
        let mut freq = get_letter_freq(answer);
        for (i, (a, g)) in answer.bytes().zip(guess.bytes()).enumerate() {
            if a == g {
                c[i] = Self::Correct;
                freq[(a - b'a') as usize] -= 1;
            }
        }
        for (i, g) in guess.bytes().enumerate() {
            if c[i] == Self::Correct {
                // Already marked as green.
                continue;
            }
            let index = (g - b'a') as usize;
            if freq[index] > 0 {
                freq[index] -= 1;
                c[i] = Self::Misplaced;
            }
        }
        c
    }

    pub fn patterns() -> impl Iterator<Item = [Self; 5]> {
        itertools::iproduct!(
            [Self::Correct, Self::Misplaced, Self::Wrong],
            [Self::Correct, Self::Misplaced, Self::Wrong],
            [Self::Correct, Self::Misplaced, Self::Wrong],
            [Self::Correct, Self::Misplaced, Self::Wrong],
            [Self::Correct, Self::Misplaced, Self::Wrong],
        )
        .map(|(a, b, c, d, e)| [a, b, c, d, e])
    }
}

pub struct Guess<'a> {
    pub word: Cow<'a, str>,
    pub mask: [Correctness; 5],
}

impl<'a> Guess<'a> {
    pub fn matches_with_letter_freq(&self, word: &str, letter_freq: &[u8; 26]) -> bool {
        assert_eq!(self.word.len(), 5);
        assert_eq!(word.len(), 5);
        let mut freq = letter_freq.clone();
        // First, check with two fingers and consume greens
        for ((g, &m), w) in self.word.bytes().zip(&self.mask).zip(word.bytes()) {
            match g == w {
                true => match m {
                    Correctness::Correct => freq[(g - b'a') as usize] -= 1,
                    _ => return false,
                },
                false => match m {
                    Correctness::Correct => return false,
                    _ => (),
                },
            }
        }
        // Second, check yellow characters exist and consume yellows
        for (g, &m) in self.word.bytes().zip(&self.mask) {
            if m == Correctness::Misplaced {
                let i = (g - b'a') as usize;
                if freq[i] == 0 {
                    return false;
                }
                freq[i] -= 1;
            }
        }
        // Last, check grays
        for (g, &m) in self.word.bytes().zip(&self.mask) {
            if m == Correctness::Wrong {
                if freq[(g - b'a') as usize] != 0 {
                    return false;
                }
            }
        }
        true
    }

    pub fn matches(&self, word: &str) -> bool {
        assert_eq!(self.word.len(), 5);
        assert_eq!(word.len(), 5);
        let mut freq = get_letter_freq(word);
        // First, check with two fingers and consume greens
        for ((g, &m), w) in self.word.bytes().zip(&self.mask).zip(word.bytes()) {
            match g == w {
                true => match m {
                    Correctness::Correct => freq[(g - b'a') as usize] -= 1,
                    _ => return false,
                },
                false => match m {
                    Correctness::Correct => return false,
                    _ => (),
                },
            }
        }
        // Second, check yellow characters exist and consume yellows
        for (g, &m) in self.word.bytes().zip(&self.mask) {
            if m == Correctness::Misplaced {
                let i = (g - b'a') as usize;
                if freq[i] == 0 {
                    return false;
                }
                freq[i] -= 1;
            }
        }
        // Last, check grays
        for (g, &m) in self.word.bytes().zip(&self.mask) {
            if m == Correctness::Wrong {
                if freq[(g - b'a') as usize] != 0 {
                    return false;
                }
            }
        }
        true
    }
}

pub trait Guesser {
    fn guess(&mut self, history: &[Guess]) -> String;
}

#[cfg(test)]
macro_rules! mask {
    (C) => {{ Correctness::Correct }};
    (M) => {{ Correctness::Misplaced }};
    (W) => {{ Correctness::Wrong }};
    ($($c:tt)+) => {{[ $(mask!($c)),+ ]}}
}

#[cfg(test)]
mod tests {

    mod game {
        use crate::{Guess, Guesser, Wordle};

        macro_rules! guesser {
            (|$history:ident| $impl:block) => {{
                struct G;
                impl Guesser for G {
                    fn guess(&mut self, $history: &[Guess]) -> String {
                        $impl
                    }
                }
                G
            }};
        }

        #[test]
        fn genius() {
            let w = Wordle::new();
            let guesser = guesser!(|_history| { "right".to_string() });
            assert_eq!(w.play("right", guesser), Some(1));
        }

        #[test]
        fn magnificent() {
            let w = Wordle::new();
            let guesser = guesser!(|history| {
                if history.len() == 1 {
                    return "right".to_string();
                }
                "wrong".to_string()
            });
            assert_eq!(w.play("right", guesser), Some(2));
        }

        #[test]
        fn impressive() {
            let w = Wordle::new();
            let guesser = guesser!(|history| {
                if history.len() == 2 {
                    return "right".to_string();
                }
                "wrong".to_string()
            });
            assert_eq!(w.play("right", guesser), Some(3));
        }

        #[test]
        fn splendid() {
            let w = Wordle::new();
            let guesser = guesser!(|history| {
                if history.len() == 3 {
                    return "right".to_string();
                }
                "wrong".to_string()
            });
            assert_eq!(w.play("right", guesser), Some(4));
        }

        #[test]
        fn great() {
            let w = Wordle::new();
            let guesser = guesser!(|history| {
                if history.len() == 4 {
                    return "right".to_string();
                }
                "wrong".to_string()
            });
            assert_eq!(w.play("right", guesser), Some(5));
        }

        #[test]
        fn phew() {
            let w = Wordle::new();
            let guesser = guesser!(|history| {
                if history.len() == 5 {
                    return "right".to_string();
                }
                "wrong".to_string()
            });
            assert_eq!(w.play("right", guesser), Some(6));
        }

        #[test]
        fn oops() {
            let w = Wordle::new();
            let guesser = guesser!(|_history| { "wrong".to_string() });
            assert_eq!(w.play("right", guesser), None);
        }
    }

    mod compute {
        use crate::Correctness;

        #[test]
        fn all_green() {
            assert_eq!(Correctness::compute("abcde", "abcde"), mask!(C C C C C));
        }

        #[test]
        fn all_yellow() {
            assert_eq!(Correctness::compute("abcde", "eabcd"), mask!(M M M M M));
        }

        #[test]
        fn all_grey() {
            assert_eq!(Correctness::compute("abcde", "fghij"), mask!(W W W W W));
        }

        #[test]
        fn repeat_green() {
            assert_eq!(Correctness::compute("aabbb", "aaccc"), mask!(C C W W W));
        }

        #[test]
        fn repeat_yellow() {
            assert_eq!(Correctness::compute("aabbb", "cccaa"), mask!(W W W M M));
        }

        #[test]
        fn repeat_some_green() {
            assert_eq!(Correctness::compute("aabbb", "accca"), mask!(C W W W M));
        }

        #[test]
        fn already_taken() {
            assert_eq!(Correctness::compute("aabbb", "ccaaa"), mask!(W W M M W));
        }

        #[test]
        fn should_not_compete_with_correct_ones() {
            assert_eq!(Correctness::compute("babbb", "aaccc"), mask!(W C W W W));
        }
    }

    mod matches {
        use crate::{Correctness, Guess};
        use std::borrow::Cow;

        #[test]
        fn should_caught_by_the_fisrt_iteration() {
            // answer is aabbc
            let guess = Guess {
                word: Cow::Borrowed("bacdb"),
                mask: mask!(M C W W M),
            };
            assert_eq!(guess.matches("bwxyz"), false, "the 1st char can't be 'b'");
            assert_eq!(guess.matches("uwxyz"), false, "the 2nd char should be 'a'");
            assert_eq!(guess.matches("uwcyz"), false, "the 3rd char can't be 'c'");
            assert_eq!(guess.matches("uwxdz"), false, "the 4th char can't be 'd'");
            assert_eq!(guess.matches("uwxyb"), false, "the 4th char can't be 'b'");
        }

        #[test]
        fn should_caught_by_the_second_iteration() {
            // answer is aabbc
            let guess = Guess {
                word: Cow::Borrowed("bacdb"),
                mask: mask!(M C W W M),
            };
            assert_eq!(guess.matches("xabxx"), false, "require two 'b'");
            assert_eq!(guess.matches("xaxbx"), false, "require two 'b'");
        }

        #[test]
        fn should_caught_by_the_last_iteration() {
            // answer is aabbc
            let guess = Guess {
                word: Cow::Borrowed("bacdb"),
                mask: mask!(M C W W M),
            };
            assert_eq!(guess.matches("cabbx"), false, "can't have 'c'");
            assert_eq!(guess.matches("xabbc"), false, "can't have 'c'");
            assert_eq!(guess.matches("dabbx"), false, "can't have 'd'");
            assert_eq!(guess.matches("xabbd"), false, "can't have 'd'");
        }

        #[test]
        fn possible_guesses() {
            // answer is aabbc
            let guess = Guess {
                word: Cow::Borrowed("bacdb"),
                mask: mask!(M C W W M),
            };
            assert_eq!(guess.matches("xabbx"), true);
            assert_eq!(guess.matches("xabby"), true);
        }
    }
}
