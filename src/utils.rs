use std::fs::{File, OpenOptions};
use std::io::{self, Write};
use std::io::BufRead;
use std::path::Path;
use crate::Duration;
use crate::BufReader;


pub fn calculate_cost(path: &Vec<&u32>, adjacency_matrix: &Vec<Vec<u32>>) -> u32 {
    let mut cost = 0;

    for i in 0..(path.len()) {
        let current_vertex = path[i];
        let next_vertex = path[(i + 1) % path.len()];
        cost += adjacency_matrix[*current_vertex as usize][*next_vertex as usize];
    }

    return cost;
}

pub fn read_matrix_from_file(folder: &str, filename: &str) -> Vec<Vec<u32>> {

    let mut adjacency_matrix_list: Vec<Vec<u32>> = Vec::new();

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


    return adjacency_matrix;
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
    report: &Vec<(String, String, u32, Vec<u32>, Duration)>,
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
        //for (filename, algorithm, cost, path, time) in &report[i] {
        let (filename, algorithm, cost, path, time) = &report[i];
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

    Ok(())
}

pub fn is_symmetric (adjacency_matrix: &Vec<Vec<u32>>) -> bool {
    for i in 0..adjacency_matrix.len() {
        for j in 0..adjacency_matrix.len() {
            if adjacency_matrix[i][j] != adjacency_matrix[j][i] {
                return false
            }
        }
    }
    return true;
}

pub fn prim(adjacency_matrix: &Vec<Vec<u32>>) -> Vec<Vec<u32>> {
    let symmetric : bool = is_symmetric(&adjacency_matrix);
    let mut mst: Vec<Vec<u32>> = vec![vec![0; adjacency_matrix.len()]; adjacency_matrix.len()];
    let mut visited: Vec<bool> = vec![false; adjacency_matrix.len()];
    visited[0] = true;

    while visited.iter().any(|&v| !v) {
        let mut a = 0;
        let mut b = 0;
        let mut min_value = u32::MAX;

        for i in 0..adjacency_matrix.len() {
            if visited[i] {
                let row = &adjacency_matrix[i];
                for j in 0..row.len() {
                    if !visited[j] && row[j] != 0 && row[j] < min_value {
                        min_value = row[j];
                        a = i;
                        b = j;
                    }
                }
            }
        }

        mst[a][b] = min_value;
        if symmetric {
            mst[b][a] = min_value;
        }
        visited[b] = true;
    }
    mst
}