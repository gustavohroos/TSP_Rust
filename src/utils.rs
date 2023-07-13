use std::fs::{self, File, OpenOptions};
use std::io::{self, Write};
use std::io::BufRead;
use std::path::Path;
use crate::Duration;
use crate::HashMap;
use crate::BufReader;


pub fn calculate_cost(path: &Vec<u32>, adjacency_matrix: &Vec<Vec<u32>>) -> u32 {
    let mut cost = 0;

    for i in 0..(path.len()) {
        let current_vertex = path[i];
        let next_vertex = path[(i + 1) % path.len()];
        cost += adjacency_matrix[current_vertex as usize][next_vertex as usize];
    }

    return cost;
}

pub fn read_matrix_from_files(folder: &str, filenames: &[&str]) -> Vec<Vec<Vec<u32>>> {

    let mut adjacency_matrix_list: Vec<Vec<Vec<u32>>> = Vec::new();

    for filename in filenames {
        let path = format!("{}/{}", folder, filename);
        let file = File::open(path).expect("Não foi possível abrir o arquivo");

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

pub fn print_matrix(matrix: &Vec<Vec<u32>>) {
    for row in matrix {
        for val in row {
            print!("{} ", val);
        }
        println!();
    }
    println!();
}

pub fn write_elapsed_times_to_csv(
    report: &Vec<Vec<(String, String, u32, Vec<u32>, Duration)>>,
    file_path: &str,
) -> io::Result<()> {
    let should_write_header = !Path::new(file_path).exists();

    let mut file = if should_write_header {
        File::create(file_path)?
    } else {
        OpenOptions::new()
            .append(true)
            .open(file_path)
            .expect("Não foi possível abrir o arquivo")
    };

    if should_write_header {
        file.write_all(b"tsp_file,algorithm,cost,elapsed_time\n")?;
    }

    for i in 0..report.len() {
        for (filename, algorithm, cost, path, time) in &report[i] {
            let formatted_duration = time.as_secs_f64();
            let formatted_algorithm_name = algorithm.to_lowercase().replace(" ", "_");
            let line = format!(
                "{},{},{},{}\n",
                filename.replace(".txt", ""),
                formatted_algorithm_name,
                cost,
                // path.iter().map(|&x| x.to_string()).collect::<Vec<String>>().join(""),
                formatted_duration
            );
            file.write_all(line.as_bytes())?;
        }
    }

    Ok(())
}

pub fn mst(adjacency_matrix: &Vec<Vec<u32>>) -> Vec<Vec<u32>> {
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