use std::io::{BufReader};
use std::collections::HashMap;
use std::time::Duration;
use std::time::Instant;
use std::any::type_name;
use std::fs;


mod bruteforce;
mod cheapest_insertion;
mod nearest_neighbor;
mod utils;
use cheapest_insertion::cheapest_insertion;
use nearest_neighbor::nearest_neighbor;
use utils::read_matrix_from_files;
use utils::mst;
use utils::write_elapsed_times_to_csv;
use utils::print_matrix;

fn main() {
    let folder = "tsp_data";
    let filenames = ["tsp1_253.txt",
                    "tsp2_1248.txt",
                    "tsp3_1194.txt",
                    // "tsp4_7013.txt",
                    "tsp5_27603.txt"
                    ];
    
    let file_path = "report2.csv";

    let adjacency_matrix_list: Vec<Vec<Vec<u32>>> = read_matrix_from_files(&folder, &filenames);

    // let min_tree: Vec<Vec<u32>> = mst(&adjacency_matrix_list[0 as usize]);

    // print_matrix(&min_tree);

    let mut report: Vec<Vec<(String, String, u32, Vec<u32>, Duration)>> = Vec::new();

    let algorithms: Vec<fn(&Vec<Vec<u32>>) -> (Vec<u32>, u32)> = vec![cheapest_insertion,nearest_neighbor];
    let algorithm_names = ["Cheapest Insertion", "Nearest Neighbor"];

    for i in 0..500{
        for (index, adjacency_matrix) in adjacency_matrix_list.iter().enumerate() {

            let mut elapsed_times: Vec<(String, String, u32, Vec<u32>, Duration)> = Vec::new();
            // println!("TSP file: {}", filenames[index]);
            
            for (index_algorithm, algorithm) in algorithms.iter().enumerate() {
                
                let start_time = Instant::now();
                let (path, cost) = algorithm(adjacency_matrix);
                let end_time = Instant::now();
                let elapsed_time = end_time - start_time;
                
                // println!("\n{} Algorithm", algorithm_names[index_algorithm]);
                // println!("Path: {:?}", path);
                // println!("Cost: {}", cost);
                // println!("Elapsed time: {:.2?}", elapsed_time);
                
                elapsed_times.push((filenames[index].to_string(), algorithm_names[index_algorithm].to_string(), cost, path, elapsed_time));
                
            }
            
            // println!("=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=");
            report.push(elapsed_times);
        }

        if let Err(err) = write_elapsed_times_to_csv(&report, file_path) {
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

