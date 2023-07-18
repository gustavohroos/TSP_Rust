use itertools::Itertools;
use rayon::prelude::*;
use std::sync::{Arc, Mutex};
use crate::utils::*;

pub fn parallel_bruteforce(adjacency_matrix: &Vec<Vec<u32>>) -> (Vec<u32>, u32) {
    let vertices: Vec<u32> = (0..adjacency_matrix.len() as u32).collect();
    let permutations = vertices.iter().permutations(vertices.len());

    let best_cost = Arc::new(Mutex::new(u32::MAX));
    let best_path = Arc::new(Mutex::new(Vec::new()));

    // permutations.par_bridge().for_each(|permutation| {
    //     let permutation_vec: Vec<u32> = permutation.into_iter().map(|&x| x).collect();
    //     let cost = calculate_cost(&permutation_vec, &adjacency_matrix);
        
    //     let mut best_cost_lock = best_cost.lock().unwrap();
    //     let mut best_path_lock = best_path.lock().unwrap();

    //     if cost <= *best_cost_lock {
    //         *best_cost_lock = cost;
    //         *best_path_lock = permutation_vec;
    //     }
    // });

    let best_path = Arc::try_unwrap(best_path).unwrap().into_inner().unwrap();
    let best_cost = Arc::try_unwrap(best_cost).unwrap().into_inner().unwrap();
    (best_path, best_cost)
}