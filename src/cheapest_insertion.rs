use crate::utils::*;

pub fn cheapest_insertion(adjacency_matrix: &Vec<Vec<u32>>) -> (Vec<u32>, u32) {
    let mut path: Vec<u32> = Vec::<u32>::new();
    let mut current_vertex = 0;
    let mut visited: Vec<u32> = Vec::new();

    while visited.len() < adjacency_matrix.len() {
        let (cost, index) = insertion_cost(&path, adjacency_matrix, current_vertex);
        path.insert(index, current_vertex);
        visited.push(current_vertex);
        current_vertex += 1;
    }

    let best_cost = calculate_cost(&path, adjacency_matrix);

    (path, best_cost)
}


pub fn insertion_cost(path: &Vec<u32>, adjacency_matrix: &Vec<Vec<u32>>, vertex: u32) -> (u32, usize) {
    let mut best_cost = u32::MAX;
    let mut best_index = 0;

    for i in 0..path.len() {
        let mut new_path: Vec<u32> = path.clone();
        new_path.insert(i, vertex);
        let cost = calculate_cost(&new_path, adjacency_matrix);

        if cost <= best_cost {
            best_cost = cost;
            best_index = i;
        }

        println!("{:?} ", new_path);
    }

    (best_cost, best_index)
}