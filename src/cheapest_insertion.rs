use crate::utils::*;

pub fn cheapest_insertion(adj_matrix: &Vec<Vec<u32>>) -> (Vec<u32>, u32) {
    let mut path: Vec<u32> = Vec::<u32>::new();
    let mut current_vertex = 0;
    let mut adjacency_matrix = adj_matrix.clone();

    transform_adjacency_matrix(&mut adjacency_matrix);

    while current_vertex < adjacency_matrix.len() {
        let (_cost, index) = insertion_cost(&path, &adjacency_matrix, current_vertex.try_into().unwrap());
        path.insert(index, current_vertex.try_into().unwrap());
        current_vertex += 1;
    }
    let reference_to_vec: &Vec<u32> = &path;
    let transformed_vec: Vec<&u32> = reference_to_vec.iter().collect();
    let best_cost = calculate_cost(&transformed_vec, &adjacency_matrix);

    (path, best_cost)
}

pub fn insertion_cost(path: &Vec<u32>, adjacency_matrix: &Vec<Vec<u32>>, vertex: u32) -> (u32, usize) {
    let mut best_cost = u32::MAX;
    let mut best_index = 0;

    for i in 0..path.len() {
        let mut new_path: Vec<u32> = path.clone();

        new_path.insert(i, vertex);
        if is_valid_path(&new_path, adjacency_matrix) {
            let reference_to_vec: &Vec<u32> = &new_path;
            let transformed_vec: Vec<&u32> = reference_to_vec.iter().collect();
            let cost = calculate_cost(&transformed_vec, adjacency_matrix);
            if cost < best_cost {
                best_cost = cost;
                best_index = i;
            }
        }
    }

    (best_cost, best_index)
}

pub fn is_valid_path(path: &Vec<u32>, adjacency_matrix: &Vec<Vec<u32>>) -> bool {
    for i in 0..path.len() - 1 {
        let from_vertex = path[i];
        let to_vertex = path[i + 1];
        if adjacency_matrix[from_vertex as usize][to_vertex as usize] == 0 {
            return false;
        }
    }
    true
}

pub fn transform_adjacency_matrix(adjacency_matrix: &mut Vec<Vec<u32>>) {
    for i in 0..adjacency_matrix.len(){
        for j in 0..adjacency_matrix.len(){
            if adjacency_matrix[i as usize][j as usize] == 0 {
                adjacency_matrix[i][j] = u32::MAX;
            }
        }
    }
}