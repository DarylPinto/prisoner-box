use rand::prelude::*;
use std::thread;

// Configuration
const PRISONER_COUNT: usize = 100;
const SIMULATION_COUNT: usize = 1_000_000;

const CUBES_TO_OPEN: usize = PRISONER_COUNT / 2;

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
/// Returns true if all prisoners find their number
fn attempt_random_strategy(rng: &mut ThreadRng) -> bool {
    let cubes = place_cubes(rng);
    let mut success_count: usize = 0;

    // For each prisoner
    for i in 0..PRISONER_COUNT {
        // Keep track of which boxes they've opened (none to begin with)
        let mut cubes = cubes.clone();
        let len = PRISONER_COUNT;

        // Open 50 boxes
        for j in 0..CUBES_TO_OPEN {
            // Randomly select a box that they haven't opened yet
            let idx = rng.gen_range(0..(len - j));
            let number_in_cube = cubes[idx];
            // If they've found their number, they leave the room
            if number_in_cube == i {
                success_count += 1;
                break;
            }
            // If not, mark the box as opened so it won't be randomly chosen again
            cubes.swap(idx, len - j - 1);
        }
    }

    success_count == PRISONER_COUNT
}

/// Runs the simulation with the smartest strategy
/// Returns true if all prisoners find their number
fn attempt_loop_strategy(rng: &mut ThreadRng) -> bool {
    let cubes = place_cubes(rng);
    let mut success_count: usize = 0;

    // For each prisoner
    for i in 0..PRISONER_COUNT {
        // First box to open is the box that matches their number
        let mut decision = i;
        // Open 50 Boxes
        for _ in 0..CUBES_TO_OPEN {
            // If they've found their number, they leave the room
            if cubes[decision] == i {
                success_count += 1;
                break;
            }
            // If they didn't find their number, they go to the box pointed
            // to by the slip within the one they just opened
            decision = cubes[decision];
        }
    }

    success_count == PRISONER_COUNT
}

fn main() {
    // Run smart strategy in one thread
    let loop_strategy = thread::spawn(|| {
        let mut rng = rand::thread_rng();
        let mut success_count = 0;

        for _ in 0..SIMULATION_COUNT {
            if attempt_loop_strategy(&mut rng) {
                success_count += 1;
            }
        }

        let rate = (success_count as f64) / (SIMULATION_COUNT as f64) * 100.;
        println!("Loop strategy success rate: {rate}%");
    });

    // Run naive strategy on another thread
    let random_strategy = thread::spawn(|| {
        let mut rng = rand::thread_rng();
        let mut success_count = 0;

        for _ in 0..SIMULATION_COUNT {
            if attempt_random_strategy(&mut rng) {
                success_count += 1;
            }
        }

        let rate = (success_count as f64) / (SIMULATION_COUNT as f64) * 100.;
        println!("Random strategy success rate: {rate}%");
    });

    // Wait for threads to finish
    loop_strategy
        .join()
        .expect("Failed to join on loop_strategy thread");
    random_strategy
        .join()
        .expect("Failed to join on smart_strategy thread");
}
