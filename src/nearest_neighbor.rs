use crate::utils::calculate_cost;

pub fn nearest_neighbor(adjacency_matrix: &Vec<Vec<u32>>) -> (Vec<u32>, u32) {
    let mut current_vertex = 0;
    let mut visited: Vec<u32> = Vec::new();
    let mut closest_vertex: u32;

    visited.push(current_vertex);
    
    while visited.len() < adjacency_matrix.len() {
        let row = &adjacency_matrix[current_vertex as usize];
        let (min_index, min_value) = row.iter()
            .enumerate()
            .filter(|&(_, &value)| value != 0)
            .filter(|&(index, _)| !visited.contains(&(index as u32)))
            .min_by_key(|(_, &value)| value)
            .unwrap();
        closest_vertex = min_index as u32;
        current_vertex = closest_vertex;
        visited.push(current_vertex);
    }

    let cost = calculate_cost(&visited, adjacency_matrix);

    return (visited, cost);
}