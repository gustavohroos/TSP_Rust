use std::io::{BufReader};
use std::collections::HashMap;
use std::time::Duration;
use std::time::Instant;

mod bruteforce;
mod cheapest_insertion;
mod nearest_neighbor;
mod utils;
use cheapest_insertion::cheapest_insertion;
use nearest_neighbor::nearest_neighbor;
use utils::read_matrix_from_files;
use utils::mst;
use utils::write_elapsed_times_to_file;
use utils::print_matrix;

fn main() {
    let filenames = ["tsp_data/tsp1_253.txt",
                    "tsp_data/tsp2_1248.txt",
                    "tsp_data/tsp3_1194.txt",
                    // "tsp_data/tsp4_7013.txt",
                    "tsp_data/tsp5_27603.txt"
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
        let start_time = Instant::now();     
        let (path_cheapest_insertion, cost_cheapest_insertion) = cheapest_insertion(&adjacency_matrix);
        let end_time = Instant::now();
        let elapsed_time = end_time - start_time;
        println!("Path cheapest insertion: {:?}", path_cheapest_insertion);
        println!("Cost: {}", cost_cheapest_insertion);
        println!("Elapsed time: {:.2?}", elapsed_time);
        elapsed_times.insert(String::from("elapsed_time_cheapest_insertion"), elapsed_time);

        let start_time = Instant::now();
        let (path_nearest_neighbor, cost_nearest_neighbor) = nearest_neighbor(&adjacency_matrix);
        let end_time = Instant::now();
        let elapsed_time = end_time - start_time;
        println!("Path nearest neighbor: {:?}", path_nearest_neighbor);
        println!("Cost: {}", cost_nearest_neighbor);
        println!("Elapsed time: {:.2?}", elapsed_time);
        elapsed_times.insert(String::from("elapsed_time_nearest_neighbor"), elapsed_time);

        // let mut start_time = Instant::now();
        // let (path_bruteforce, cost_bruteforce) = bruteforce(&adjacency_matrix);
        // let mut end_time = Instant::now();
        // let mut elapsed_time = end_time - start_time;
        // println!("Path bruteforce: {:?}", path_bruteforce);
        // println!("Cost: {}", cost_bruteforce);
        // println!("Elapsed time: {:.2?}", elapsed_time);
        // elapsed_times.insert(String::from("elapsed_time_bruteforce"), elapsed_time);

        elapsed_time_list.push(elapsed_times);
    }

    write_elapsed_times_to_file(&elapsed_time_list, "report.txt").unwrap();

}