use itertools::Itertools;
use crate::utils::*;

pub fn bruteforce(adjacency_matrix: &Vec<Vec<u32>>) -> (Vec<u32>, u32) {
    let vertices: Vec<u32> = (0..adjacency_matrix.len() as u32).collect();
    let permutations = vertices.iter().permutations(vertices.len());
    let mut best_cost: u32 = u32::MAX;
    let mut best_path: Vec<u32> = Vec::new();

    for permutation in permutations {
        let cost = calculate_cost(&permutation, &adjacency_matrix);
        if cost <= best_cost {
            best_cost = cost;
            best_path = permutation.into_iter().map(|&x| x).collect();
        }
    }

    (best_path, best_cost)
}