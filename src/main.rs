use std::io::{BufReader};
use std::time::Duration;
use std::time::Instant;
use std::fs;
use std::env;
mod bruteforce;
mod approx_tsp;
mod cheapest_insertion;
mod utils;
mod christofides;
use bruteforce::bruteforce;
use approx_tsp::approx_tsp;
use christofides::christofides;
use cheapest_insertion::cheapest_insertion;
use utils::read_matrix_from_file;
use utils::write_elapsed_times_to_csv;
use utils::prim;
use utils::print_matrix;

fn main() {
    let args: Vec<_> = env::args().collect();

    let mut file_path: String = "data/".to_string();
    let file_name = match args.len() {
        3 => "report.csv",
        4 => &args[3],
        _ => panic!("Invalid number of arguments!"),
    };
    file_path = format!("{}{}", file_path, file_name);

    let algorithm = match args[1].as_str() {
        "b" => bruteforce,
        "a" => approx_tsp,
        "c" => christofides,
        "ci" => cheapest_insertion,
         _ => panic!("Invalid algorithm name!"),
    };

    let algorithm_name = match args[1].as_str() {
        "b" => "Bruteforce",
        "a" => "Approximation TSP",
        "c" => "Christofides",
        "ci" => "Cheapest Insertion",
         _ => panic!("Invalid algorithm name!"),
    };

    let folder_tsp = "tsp_data";
    let filenames = ["tsp1_253.txt",
                     "tsp2_1248.txt",
                     "tsp3_1194.txt",
                     "tsp4_7013.txt",
                     "tsp5_27603.txt"
                     ];

    let instance = args[2].parse::<usize>().unwrap() - 1;
    if instance > filenames.len() {
        panic!("Invalid file selected!");
    }

                    
    let adjacency_matrix = read_matrix_from_file(&folder_tsp, &filenames[instance]);
    /* let mst = prim(&adjacency_matrix);
    print_matrix(&mst); */
    let mut elapsed_times : Vec<(String, String, u32, Vec<u32>, Duration)> = Vec::new();
    let start_time = Instant::now();
    
    let (path, cost) = algorithm(&adjacency_matrix);
    
    //println!("{}", adjacency_matrix[0][22]);
    let end_time = Instant::now();
    let elapsed_time = end_time - start_time;
    println!("{:?}",path);
    println!("{}", cost);
    println!("{:?}", elapsed_time);


    elapsed_times.push((filenames[instance].to_string(), algorithm_name.to_string(), cost, path, elapsed_time));

    if let Err(err) = write_elapsed_times_to_csv(&elapsed_times, file_path.as_str()) {
        eprintln!("Error writing to CSV: {:?}", err);
        return;
    }
    if let Ok(metadata) = fs::metadata(file_path) {
        println!("File writed successfully. Actual size: {} bytes", metadata.len());
    } else {
        println!("File creation failed or file not found.");
    }
    

}

