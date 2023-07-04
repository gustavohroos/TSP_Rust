use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let filename = "tsp_data/tsp1_253.txt";
    let adjacency_matrix = read_matrix_from_file(filename);

    let path = cheapest_insertion(&adjacency_matrix);
    println!("Path cheapest insertion: {:?}", path);

    let (path_nearest_neighbor, cost_nearest_neighbor) = nearest_neighbor(&adjacency_matrix);
    println!("Path nearest neighbor: {:?}", path_nearest_neighbor);
    println!("Cost: {}", cost_nearest_neighbor);

    // let cost = calculate_cost(&adjacency_matrix, &path);
    // println!("Cost: {}", cost);

}

fn calculate_cost(path: &Vec<u32>, adjacency_matrix:&Vec<Vec<u32>>) -> u32 {
    let mut cost = 0;

    for i in 0..(path.len()) {
        let current_vertex = path[i];
        let next_vertex = path[(i + 1) % path.len()];
        cost += adjacency_matrix[current_vertex as usize][next_vertex as usize];
    }

    return cost;
}


fn cheapest_insertion(adjacency_matrix: &Vec<Vec<u32>>) -> Vec<u32> {
    let mut best_path: Vec<u32> = Vec::new();
    let mut best_cost: u32 = 0;

    let mut current_vertex = 0;
    let mut visited: Vec<u32> = Vec::new();
    visited.push(current_vertex);

    let mut all_visited: bool = false;

    let mut unvisited: Vec<usize> = (0..adjacency_matrix.len()).collect();
    
    while unvisited.len() > 0{
        return visited; 

    }
    return visited; 


}

fn nearest_neighbor(adjacency_matrix: &Vec<Vec<u32>>) -> (Vec<u32>, u32) {
    let mut current_vertex = 0;
    let mut visited: Vec<u32> = Vec::new();
    let mut closest_vertex: u32;

    visited.push(current_vertex);
    
    while visited.len() < adjacency_matrix.len() {
        println!("{}, {}", current_vertex, visited.len());
        let row = &adjacency_matrix[current_vertex as usize];
        // print_vector(&row);
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

fn read_matrix_from_file(filename: &str) -> Vec<Vec<u32>> {

    let file = File::open(filename).expect("Não foi possível abrir o arquivo");

    let reader = BufReader::new(file);

    let mut adjacency_matrix: Vec<Vec<u32>> = Vec::new();

    for line in reader.lines() {
        let line = line.expect("Falha ao ler a linha do arquivo");

        let values: Vec<u32> = line
            .split_whitespace()
            .map(|val| val.parse().expect("Falha ao fazer o parse do valor"))
            .collect();

        adjacency_matrix.push(values);
    }
    
    // print_matrix(&adjacency_matrix);
    println!("{}",&adjacency_matrix.len());
    return adjacency_matrix;
}

fn print_matrix(matrix: &Vec<Vec<u32>>) {
    for row in matrix {
        for val in row {
            print!("{} ", val);
        }
        println!();
    }
    println!();
}

fn print_vector(vector: &Vec<u32>) {
    for value in vector {
        print!("{} ", value);
    }
    println!();
}
