use rand::prelude::*;
use std::fmt::Display;
use Strategy::{Smart, Random};

const PRISONER_COUNT: usize = 100;
const CUBES_TO_OPEN: usize = PRISONER_COUNT / 2;
const SIMULATION_COUNT: usize = 10000;
const STRATEGY: Strategy = Random;
const DEBUG: bool = false;

enum Strategy {
    Smart,
    Random
}

fn log<S: Display + Into<String>>(s: S) {
    if DEBUG {
        println!("{s}");
    }
}

/// Set up the boxes with random numbers inside each of them
fn place_cubes(rng: &mut ThreadRng) -> [usize; PRISONER_COUNT] {
    let mut cubes: [usize; PRISONER_COUNT] = [0; PRISONER_COUNT];
    for (i, cube) in cubes.iter_mut().enumerate() {
        *cube = i;
    }
    cubes.shuffle(rng);
    cubes
}

/// Runs the simulation with the naive strategy
fn attempt_random_strategy(rng: &mut ThreadRng) -> bool {
    let cubes = place_cubes(rng);
    let mut success_count: usize = 0;
    // Each prisoner
    for i in 0..PRISONER_COUNT {
        // Checking 50 boxes
        // let mut cubes_for_this_prisoner = cubes.into_iter().collect::<Vec<_>>();
        let mut visited: [usize;CUBES_TO_OPEN] = [usize::MAX;CUBES_TO_OPEN];
        // log(format!("\nPrisoner #{i} steps into the room"));
        for j in 0..CUBES_TO_OPEN {
            // Deciding which box to open
            let mut number_in_cube = cubes.choose(rng).unwrap();
            while visited.contains(&number_in_cube) {
                number_in_cube = cubes.choose(rng).unwrap();
            }
            // log(format!(
            //     "Prisoner #{i}'s {j}th choice was a random box. The box contained {number_in_cube}"
            // ));
            // If the box has their number, then they succeed
            if number_in_cube == &i {
                success_count += 1;
                // log(format!(
                //     "Prisoner #{i} found their number after opening {} boxes",
                //     j + 1
                // ));
                // log(format!("WIN!!"));
                break;
            }
            // If it doesn't have their number, make sure they won't try to open it again
            // cubes_for_this_prisoner.retain(|&number| number != number_in_cube);
            visited[j] = *number_in_cube;
            // println!("{:?}", cubes_for_this_prisoner);
        }
    }
    success_count == PRISONER_COUNT
}

/// Runs the simulation with the smartest strategy
fn attempt_smart_strategy(rng: &mut ThreadRng) -> bool {
    let cubes = place_cubes(rng);
    let mut success_count: usize = 0;
    // Each prisoner
    for i in 0..PRISONER_COUNT {
        // Checking 50 boxes
        // Deciding which box to open first
        // log(format!("\nPrisoner #{i} steps into the room"));
        let mut decision = i;
        let first_decision = decision;

        for j in 0..CUBES_TO_OPEN {
            // If the box has their number, then they succeed
            // log(format!( "Prisoner #{i}'s {j}th choice is box #{decision}. The box contained {}", cubes[decision]));
            if cubes[decision] == i {
                success_count += 1;
                // log(format!( "Prisoner #{i} found their number after opening {} boxes", j + 1));
                // log(format!("WIN!!"));
                break;
            // If it doesn't their next decision will be the number within the box they just checked
            } else {
                decision = cubes[decision];
                // Don't continue if you've already closed the loop
                if decision == first_decision {
                    break;
                }
            }
        }
    }
    // log(format!("\n\n{success_count}/{PRISONER_COUNT} prisoners found their number!"));
    // println!("{success_count}/{PRISONER_COUNT} prisoners found their number!");
    success_count == PRISONER_COUNT
}

fn main() {
    let mut rng = rand::thread_rng();
    // let mut results: [Option<usize>; SIMULATION_COUNT] = [None; SIMULATION_COUNT];
    let mut success_count = 0;
    for i in 0..SIMULATION_COUNT {
        log("\n\n\n================ NEW SIMULATION!========================\n\n\n");
        let result = match STRATEGY {
            Smart => attempt_smart_strategy(&mut rng),
            Random => attempt_random_strategy(&mut rng)
        };

        if result {
            success_count += 1;
        }
    }

    // let total_escaped: usize = results.iter().filter_map(|&t| t).sum();

    let rate = (success_count as f64)/ (SIMULATION_COUNT as f64) * 100.;

    println!("\n\n\nSuccess rate: {rate}%");
}
