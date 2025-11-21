use clap::{Parser, ValueEnum};
use wordle_solver::Guesser;

const GAMES: &str = include_str!("../answers.txt");

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, value_enum)]
    implementation: Implementation,

    #[arg(short, long)]
    max: Option<usize>,
}

#[derive(ValueEnum, Debug, Clone)]
enum Implementation {
    Naive,
    Allocs,
    Cache,
    Vecrem,
    Weight,
    Prune,
}

fn main() {
    let args = Args::parse();

    match args.implementation {
        Implementation::Naive => play(wordle_solver::algorithms::Naive::new, args.max),
        Implementation::Allocs => play(wordle_solver::algorithms::Allocs::new, args.max),
        Implementation::Cache => play(wordle_solver::algorithms::Cache::new, args.max),
        Implementation::Vecrem => play(wordle_solver::algorithms::Vecrem::new, args.max),
        Implementation::Weight => play(wordle_solver::algorithms::Weight::new, args.max),
        Implementation::Prune => play(wordle_solver::algorithms::Prune::new, args.max),
    }
}

fn play<G>(mut mk: impl FnMut() -> G, max: Option<usize>)
where
    G: Guesser,
{
    let wordle = wordle_solver::Wordle::new();
    let mut score = 0;
    let mut games = 0;
    for answer in GAMES.split_whitespace().take(max.unwrap_or(usize::MAX)) {
        let guesser = (mk)();
        match wordle.play(answer, guesser) {
            Some(s) => {
                score += s;
                games += 1;
                println!("guessed {} in {}", answer, s);
            }
            None => {
                eprintln!("failed to guess {}", answer);
            }
        }
    }
    if games != 0 {
        println!("average score: {:.2}", score as f64 / games as f64);
    }
}
