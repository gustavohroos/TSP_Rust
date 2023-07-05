use std::fs::File;
use std::io::{BufRead, BufReader};
use itertools::Itertools;
use std::collections::HashMap;
use std::io::prelude::*;
use std::time::Duration;
use humantime::format_duration;
use std::time::Instant;

   

fn main() {
    let filenames = ["tsp_data/tsp1_253.txt",
                    "tsp_data/tsp2_1248.txt",
                    "tsp_data/tsp3_1194.txt",
                    // "tsp_data/tsp4_7013.txt",
                    // "tsp_data/tsp5_27603.txt"
                    ];

    let adjacency_matrix_list = read_matrix_from_files(&filenames);

    let mut i = 1;

    let min_tree: Vec<Vec<u32>> = mst(&adjacency_matrix_list[0 as usize]);

    print_matrix(&min_tree);

    let mut elapsed_time_list: Vec<HashMap<String, Duration>> = Vec::new();

    for adjacency_matrix in adjacency_matrix_list {
        let mut elapsed_times: HashMap<String, Duration> = HashMap::new();
        println!("TSP file {}", i);
        i += 1;
        let mut start_time = Instant::now();     
        let (path_cheapest_insertion, cost_cheapest_insertion) = cheapest_insertion(&adjacency_matrix);
        let mut end_time = Instant::now();
        let mut elapsed_time = end_time - start_time;
        println!("Path cheapest insertion: {:?}", path_cheapest_insertion);
        println!("Cost: {}", cost_cheapest_insertion);
        println!("Elapsed time: {:.2?}", elapsed_time);
        elapsed_times.insert(String::from("elapsed_time_cheapest_insertion"), elapsed_time);


        let mut start_time = Instant::now();
        let (path_nearest_neighbor, cost_nearest_neighbor) = nearest_neighbor(&adjacency_matrix);
        let mut end_time = Instant::now();
        let mut elapsed_time = end_time - start_time;
        println!("Path nearest neighbor: {:?}", path_nearest_neighbor);
        println!("Cost: {}", cost_nearest_neighbor);
        println!("Elapsed time: {:.2?}", elapsed_time);
        elapsed_times.insert(String::from("elapsed_time_nearest_neighbor"), elapsed_time);

        let mut start_time = Instant::now();
        let (path_bruteforce, cost_bruteforce) = bruteforce(&adjacency_matrix);
        let mut end_time = Instant::now();
        let mut elapsed_time = end_time - start_time;
        println!("Path bruteforce: {:?}", path_bruteforce);
        println!("Cost: {}", cost_bruteforce);
        println!("Elapsed time: {:.2?}", elapsed_time);
        elapsed_times.insert(String::from("elapsed_time_bruteforce"), elapsed_time);

        elapsed_time_list.push(elapsed_times);
    }

    write_elapsed_times_to_file(&elapsed_time_list, "report.txt").unwrap();

}

fn cheapest_insertion(adjacency_matrix: &Vec<Vec<u32>>) -> (Vec<u32>, u32) {
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


fn insertion_cost(path: &Vec<u32>, adjacency_matrix: &Vec<Vec<u32>>, vertex: u32) -> (u32, usize) {
    let mut best_cost = u32::MAX;
    let mut best_index = 0;

    for i in 0..path.len() {
        let mut new_path: Vec<u32> = path.clone();
        new_path.insert(i, vertex);
        let cost = calculate_cost(&new_path, adjacency_matrix);

        if cost < best_cost {
            best_cost = cost;
            best_index = i;
        }
    }

    (best_cost, best_index)
}

fn calculate_cost(path: &Vec<u32>, adjacency_matrix: &Vec<Vec<u32>>) -> u32 {
    let mut cost = 0;

    for i in 0..(path.len()) {
        let current_vertex = path[i];
        let next_vertex = path[(i + 1) % path.len()];
        cost += adjacency_matrix[current_vertex as usize][next_vertex as usize];
    }

    return cost;
}

fn nearest_neighbor(adjacency_matrix: &Vec<Vec<u32>>) -> (Vec<u32>, u32) {
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

fn mst(adjacency_matrix: &Vec<Vec<u32>>) -> Vec<Vec<u32>> {
    let mut mst: Vec<Vec<u32>> = vec![vec![0; adjacency_matrix.len()]; adjacency_matrix.len()];
    let mut visited: Vec<u32> = Vec::new();
    visited.push(0);
    let mut a = 0;
    let mut b = 0;
    let mut k = u32::MAX;
    while visited.len() < adjacency_matrix.len() {
        for i in 0..visited.len() {
            k = u32::MAX;
            let row = &adjacency_matrix[visited[i] as usize];
            let (min_index, min_value) = row.iter()
                .enumerate()
                .filter(|&(_, &value)| value != 0)
                .filter(|&(index, _)| !visited.contains(&(index as u32)))
                .min_by_key(|(_, &value)| value)
                .unwrap();
            if *min_value < k {
                a = visited[i];
                b = min_index;
                k = *min_value;    
            }
        }
        mst[a as usize][b as usize] = k;
        mst[b as usize][a as usize] = k;
        visited.push(b as u32);    
    }
    return mst;
}

fn bruteforce(adjacency_matrix: &Vec<Vec<u32>>) -> (Vec<u32>, u32) {
    let vertexes: Vec<u32> = (0..adjacency_matrix.len() as u32).collect();
    let permutations = vertexes.iter().permutations(vertexes.len());
    let mut best_cost: u32 = u32::MAX;
    let mut best_path: Vec<u32> = Vec::new();

    for permutation in permutations {
        let permutation_vec: Vec<u32> = permutation.into_iter().map(|&x| x).collect();
        let cost = calculate_cost(&permutation_vec, &adjacency_matrix);
        if cost <= best_cost {
            best_cost = cost;
            best_path = permutation_vec;
        }
    }

    (best_path, best_cost)
}

fn read_matrix_from_files(filenames: &[&str]) -> Vec<Vec<Vec<u32>>> {

    let mut adjacency_matrix_list: Vec<Vec<Vec<u32>>> = Vec::new();

    for filename in filenames {
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
        adjacency_matrix_list.push(adjacency_matrix);
    }

    return adjacency_matrix_list;
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

fn write_elapsed_times_to_file(
    elapsed_time_list: &Vec<HashMap<String, Duration>>,
    file_path: &str,
) -> std::io::Result<()> {
    // Create a file for writing
    let mut file = File::create(file_path)?;

    for i in 0..elapsed_time_list.len() {
        let line = format!("TSP: File {}\n", i + 1);
        file.write_all(line.as_bytes())?;
        for (key, value) in &elapsed_time_list[i as usize] {
            let formatted_duration = format_duration(*value);
            let line = format!("{}: {}\n", key, formatted_duration);
            file.write_all(line.as_bytes())?;
        }
    }

    Ok(())
}