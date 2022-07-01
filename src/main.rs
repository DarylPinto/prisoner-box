use rand::prelude::*;
use std::thread;

const PRISONER_COUNT: usize = 100;
const SIMULATION_COUNT: usize = 10000;

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
fn attempt_random_strategy(rng: &mut ThreadRng) -> bool {
    let cubes = place_cubes(rng);
    let mut success_count: usize = 0;
    for i in 0..PRISONER_COUNT {
        let mut visited: [usize; CUBES_TO_OPEN] = [usize::MAX; CUBES_TO_OPEN];
        for j in 0..CUBES_TO_OPEN {
            let mut number_in_cube = cubes.choose(rng).unwrap();
            while visited.contains(number_in_cube) {
                number_in_cube = cubes.choose(rng).unwrap();
            }
            if number_in_cube == &i {
                success_count += 1;
                break;
            }
            visited[j] = *number_in_cube;
        }
    }
    success_count == PRISONER_COUNT
}

/// Runs the simulation with the smartest strategy
fn attempt_smart_strategy(rng: &mut ThreadRng) -> bool {
    let cubes = place_cubes(rng);
    let mut success_count: usize = 0;
    for i in 0..PRISONER_COUNT {
        let mut decision = i;
        let first_decision = decision;
        for _ in 0..CUBES_TO_OPEN {
            if cubes[decision] == i {
                success_count += 1;
                break;
            } else {
                decision = cubes[decision];
                if decision == first_decision {
                    break;
                }
            }
        }
    }
    success_count == PRISONER_COUNT
}

fn main() {
    let smart_strategy = thread::spawn(|| {
        let mut rng = rand::thread_rng();
        let mut success_count = 0;
        for _ in 0..SIMULATION_COUNT {
            if attempt_smart_strategy(&mut rng) {
                success_count += 1;
            }
        }

        let rate = (success_count as f64) / (SIMULATION_COUNT as f64) * 100.;
        println!("Smart Strategy Success rate: {rate}%");
    });

    let random_strategy = thread::spawn(|| {
        let mut rng = rand::thread_rng();
        let mut success_count = 0;
        for _ in 0..SIMULATION_COUNT {
            if attempt_random_strategy(&mut rng) {
                success_count += 1;
            }
        }

        let rate = (success_count as f64) / (SIMULATION_COUNT as f64) * 100.;
        println!("Random Success rate: {rate}%");
    });

    smart_strategy
        .join()
        .expect("Failed to join on smart_stragety thread");

    random_strategy
        .join()
        .expect("Failed to join on smart_stragety thread");
}
