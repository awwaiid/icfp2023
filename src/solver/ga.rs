use std::collections::HashSet;
use rand::Rng;
use crate::icfp::*;
use crate::scorer::*;

// // , Serialize, Deserialize)]
#[derive(Debug, Clone)]
struct Individual {
    solution: Solution,
    score: f32
}

fn generate_random_individual(problem: &Problem) -> Individual {
    let mut rng = rand::thread_rng();
    let mut players: Vec<Position> = Vec::new();

    let mut used_locations = HashSet::new();

    let stage_x_min = problem.stage_bottom_left[0];
    let stage_y_min = problem.stage_bottom_left[1];

    let stage_cols = ((problem.stage_width - 20.0) / 10.0).ceil() as u32;
    let stage_rows = ((problem.stage_height - 20.0) / 10.0).ceil() as u32;

    for _n in 0..(problem.musicians.len()) {
        // eprintln!("Placing musician {}", n);
        let mut found_spot = false;
        while !found_spot {
            let col = rng.gen_range(0..stage_cols);
            let row = rng.gen_range(0..stage_rows);
            if !used_locations.contains(&(col, row)) {
                used_locations.insert((col, row));
                found_spot = true;
                players.push(Position {
                    x: stage_x_min + (col as f32 * 10.0) + 10.0,
                    y: stage_y_min + (row as f32 * 10.0) + 10.0,
                });
            }
        }
    }

    let solution = Solution {
        placements: players,
    };

    let score = scorer(&problem, &solution);

    Individual {
        solution: solution,
        score: score
    }
}

fn mutate_swap_placements(placements: Placements) -> Placements {
    let mut placements = placements.clone();
    let mut rng = rand::thread_rng();
    let placement_1_n = rng.gen_range(0..placements.len());
    let placement_1_v = placements[placement_1_n].clone();
    let placement_2_n = rng.gen_range(0..placements.len());
    let placement_2_v = placements[placement_2_n].clone();
    placements[placement_1_n] = placement_2_v;
    placements[placement_2_n] = placement_1_v;
    placements
}

fn generate_offspring(problem: &Problem, individual_a: &Individual, individual_b: &Individual) -> Individual {

    let mut rng = rand::thread_rng();

    // Swap two placements
    if rng.gen_ratio(10, 100) {
        let placements = individual_a.solution.placements.clone();
        let placements = mutate_swap_placements(placements.clone());
        let solution = Solution { placements: placements };
        let score = scorer(&problem, &solution);
        return Individual { solution: solution, score: score };
    }

    if rng.gen_ratio(10, 100) {
        return individual_a.clone();
    }

    if rng.gen_ratio(10, 100) {
        if individual_a.score > individual_b.score {
            return individual_a.clone();
        } else {
            return individual_b.clone();
        }
    }

    // Generate an entirely new individual
    let mut random_individual = generate_random_individual(problem);
    let score = scorer(&problem, &random_individual.solution);
    random_individual.score = score;
    random_individual
}

pub fn solve(problem: &Problem) -> Solution {
    let mut rng = rand::thread_rng();
    let mut population: Vec<Individual> = Vec::new();

    // Set up the initial population
    for _ in 0..100 {
        let random_individual = generate_random_individual(problem);
        population.push(random_individual);
    }

    // Start with a sorted population
    population.sort_by(|b, a| a.score.partial_cmp(&b.score).unwrap());

    for generation in 0..100 {
        // Select top X
        population.truncate(20);

        // Fill up new population
        while population.len() < 100 {
            let individual_a = &population[rng.gen_range(0..population.len())];
            let individual_b = &population[rng.gen_range(0..population.len())];
            let new_individual = generate_offspring(problem, individual_a, individual_b);
            population.push(new_individual);
        }

        // Sort
        population.sort_by(|b, a| a.score.partial_cmp(&b.score).unwrap());

        // Stats!
        let sum: f32 = population.iter().map(|i| i.score).sum();
        let avg = sum / population.len() as f32;
        eprintln!("Gen {}\tBest {}\tWorst {}\tAvg {}", generation, population[0].score, population.last().unwrap().score, avg);
    }

    // Return the best of the best
    let solution = population[0].solution.clone();
    solution
}

