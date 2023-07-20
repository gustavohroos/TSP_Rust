use crate::utils::calculate_cost;
use crate::utils::prim;
use crate::utils::print_matrix;
use std::collections::HashSet;

fn preorder_traversal(
    adjacency_matrix: &Vec<Vec<u32>>,
    start: usize,
    visited: &mut HashSet<usize>,
) {
    visited.insert(start);

    for (neighbor, &weight) in adjacency_matrix[start].iter().enumerate() {
        if weight != 0 && !visited.contains(&neighbor) {
            preorder_traversal(adjacency_matrix, neighbor, visited);
        }
    }
}

pub fn approx_tsp(adjacency_matrix: &Vec<Vec<u32>>) -> (Vec<u32>, u32) {
    let root = 0;
    let mst = prim(adjacency_matrix);
    println!("MST:");
    print_matrix(&mst);
    let mut visited = HashSet::new();

    preorder_traversal(&mst, root, &mut visited);

    let mut path: Vec<u32> = visited.into_iter().map(|x| x as u32).collect();

    let reference_to_vec: &Vec<u32> = &path;
    let transformed_vec: Vec<&u32> = reference_to_vec.iter().collect();
    let cost = calculate_cost(&transformed_vec, &adjacency_matrix);

    (path, cost)
}