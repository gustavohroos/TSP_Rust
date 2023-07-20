use itertools::Itertools;
use crate::utils::*;
use std::fs::{self, File};
use std::io::Write;

pub fn bruteforce(adjacency_matrix: &Vec<Vec<u32>>) -> (Vec<u32>, u32) {
    let vertices: Vec<u32> = (0..adjacency_matrix.len() as u32).collect();
    let permutations = vertices.iter().permutations(vertices.len());
    //let file_path = "temp/latest_permutation.txt";
    let mut best_cost: u32 = u32::MAX;
    let mut best_path: Vec<u32> = Vec::new();
    let mut counter = 0;

    for permutation in permutations {
        //counter += 1;
        let cost = calculate_cost(&permutation, &adjacency_matrix);
        if cost <= best_cost {
            best_cost = cost;
            best_path = permutation.clone().into_iter().map(|&x| x).collect();
        }
        /* if counter % 100000000 == 0 {
            counter = 0;
            let current_permutation: Vec<u32> = permutation.into_iter().map(|&x| x).collect();
            let mut file = File::create(file_path).expect("Unable to create file");
            file.write_all(format!("{:?},{},{:?}", current_permutation, best_cost, best_path).as_bytes()).expect("Unable to write data");
        } */
    }

   /*  if let Err(e) = fs::remove_file(file_path) {
        // Check if the error is due to the file not existing
        if e.kind() != std::io::ErrorKind::NotFound {
            // If the error is not due to the file not existing, handle the error
            panic!("Unable to delete file: {}", e);
        }
    } else {
        // println!("File deleted successfully.");
    } */
    //println!("Count: {}", counter);
    (best_path, best_cost)
}