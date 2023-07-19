use std::io::{BufReader};
use std::time::Duration;
use std::time::Instant;
use std::fs;

mod bruteforce;
use bruteforce::bruteforce;
//mod cheapest_insertion;
//mod nearest_neighbor;
mod parallel_bruteforce;
use parallel_bruteforce::parallel_bruteforce;
mod utils;
mod christofides;
use christofides::christofides;
//use cheapest_insertion::cheapest_insertion;
//use nearest_neighbor::nearest_neighbor;
use utils::read_matrix_from_files;
use utils::write_elapsed_times_to_csv;
use crate::utils::calculate_cost;

fn main() {
    let folder_tsp = "tsp_data";
    let filenames = ["tsp1_253.txt",
                    "tsp2_1248.txt",
                    "tsp3_1194.txt",
                    // "tsp4_7013.txt",
                    // "tsp4_2.txt",
                    "tsp5_27603.txt"
                    ];
    
    let file_path = "data/report_bruteforce.csv";

    let path1248 = vec![0, 2, 3, 4, 5, 1, 0];
    let path253 = vec![0, 7, 4, 3, 5, 9, 6, 1, 10, 2, 8, 0];
    let path1194 = vec![0, 1, 9, 10, 11, 12, 13, 14, 8, 6, 5, 7, 4, 3, 2, 0];
    let path27603 = vec![ 0, 10,  9, 11, 14, 18, 17, 23, 26, 24, 15, 19, 25, 27, 28, 22, 20,
        21, 16, 13, 12,  7,  3,  6,  8,  2,  4,  5,  1,  0];

    let adjacency_matrix_list: Vec<Vec<Vec<u32>>> = read_matrix_from_files(&folder_tsp, &filenames);
    
    let algorithms: Vec<fn(&Vec<Vec<u32>>) -> (Vec<u32>, u32)> = vec![christofides];
    // let algorithm_names = ["Bruteforce", "Parallel Bruteforce"];
    // let algorithm_names = ["Bruteforce"];
    let algorithm_names = ["Christofides"];


    for i in 0..1{
        let mut elapsed_times : Vec<(String, String, u32, Vec<u32>, Duration)> = Vec::new();
        for (index, adjacency_matrix) in adjacency_matrix_list.iter().enumerate() {
            for (index_algorithm, algorithm) in algorithms.iter().enumerate() {
                let start_time = Instant::now();
                let (path, cost) = algorithm(adjacency_matrix);
                let end_time = Instant::now();
                let elapsed_time = end_time - start_time;
                elapsed_times.push((filenames[index].to_string(), algorithm_names[index_algorithm].to_string(), cost, path, elapsed_time));
            }
        }
        if let Err(err) = write_elapsed_times_to_csv(&elapsed_times, file_path) {
            eprintln!("Error writing to CSV: {:?}", err);
            return;
        }
        if let Ok(metadata) = fs::metadata(file_path) {
            // println!("File writed successfully. Actual size: {} bytes", metadata.len());
            continue;
        } else {
            println!("File creation failed or file not found.");
        }
    }
}

