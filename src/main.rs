use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let filename = "tsp_data/tsp1_253.txt";
    let adjacency_matrix = read_matrix_from_file(filename);

    let path = cheapest_insertion(&adjacency_matrix);
    println!("Path: {:?}", path);

    // let cost = calculate_cost(&adjacency_matrix, &path);
    // println!("Cost: {}", cost);

}

fn cheapest_insertion(adjacency_matrix: &Vec<Vec<u32>>) -> Vec<usize> {
    let mut best_path: Vec<u32> = Vec::new();
    let mut best_cost: u32 = 0;

    let mut current_vertex = 0;
    let mut visited: Vec<u32> = Vec::new();
    visited.push(current_vertex);

    let mut all_visited: bool = false;

    let mut unvisited: Vec<usize> = (0..adjacency_matrix.len()).collect();
    
    while unvisited.len() < 0:
    

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

    print_matrix(&adjacency_matrix);

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
