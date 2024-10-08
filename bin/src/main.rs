use clap::Parser;

use cuberithm::solver;
use cuberithm::{cube::CubeState, solution::Solution};
use std::str::FromStr;
use std::{collections::BTreeSet, time::Instant};

#[cfg(unix)]
#[global_allocator]
static GLOBAL: jemallocator::Jemalloc = jemallocator::Jemalloc;

/// Simple algorithm generator for a 3x3x3 Rubik's Cube
///
/// Format of states passed in arguments is a 54 character long string composed of:
/// characters: Y (yellow), B (blue), G (green), R (red), W (white), O (orange)
/// arranged from left to right, bottom to top, in the order of faces:
/// white -> orange -> green -> red -> blue -> yellow
/// example: WWWWWWWWWOOOOOOOOOGGGGGGGGGRRRRRRRRRBBBBBBBBBYYYYYYYYY (solved cube)
///          WWWWWWWWWOOOOOOGGGGGGGGGRRRRRRRRRBBBBBBBBBOOOYYYYYYYYY (after U move)
///          WWWWWWWWWOOOOOOOOOGGGGGGGRRRRRRRRBGGBBBBBBRBBYYYYYYYYY (after J-Perm)
#[derive(Parser)]
#[command(version, about, verbatim_doc_comment)]
struct Args {
    /// Initial Cube state, right->left bottom->top green on front white on top
    #[arg(short, long)]
    initial_state: String,
    /// Desired Cube state
    #[arg(short, long)]
    desired_state: String,
    /// Min moves for algorithms to be generated
    #[arg(long)]
    min_moves: u8,
    /// Max moves for algorithms to be generated
    #[arg(long)]
    max_moves: u8,
    // Max difference between shortest algorithm and the longest
    #[arg(short, long)]
    threshold: u8,
}

fn main() {
    let args = Args::parse();

    let initial_state = CubeState::from_str(&args.initial_state).unwrap();
    let desired_state = CubeState::from_str(&args.desired_state).unwrap();
    let min_moves = args.min_moves;
    let max_moves = args.max_moves;
    let threshold = args.threshold;

    let initial_time = Instant::now();

    let mut solutions: BTreeSet<Solution> = BTreeSet::new();

    let mut since_found = 0;
    for i in min_moves..=max_moves {
        let found_solutions = solver::solve(initial_state, desired_state, i, true);

        solutions.extend(found_solutions);

        if since_found > 0 || !solutions.is_empty() {
            since_found += 1;
        }

        if since_found >= threshold {
            break;
        }
    }

    let elapsed_time = Instant::now().duration_since(initial_time);

    for (idx, solution) in (0_u16..).zip(solutions.iter()) {
        print!("Solution {}: ", idx);
        for rot in solution {
            print!("{} ", rot);
        }
        println!();
    }
    println!("\nDone.");

    println!("Elapsed Time: {:.3}s", elapsed_time.as_secs_f64());
    println!("Solutions Found: {}", solutions.len());
}
