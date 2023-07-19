use std::time::Instant;
use crate::utils::*;
use itertools::Itertools;

pub fn christofides(adjacency_matrix: &Vec<Vec<u32>>) -> (Vec<u32>, u32) {
    // -> Vec<Vec<u32>>
    let mut start_time = Instant::now();
    let mut mst: Vec<Vec<u32>> = prim(adjacency_matrix);
    println!("MST");
    print_matrix(&mst);

    let vertices_with_odd_degree = vertices_with_odd_degree(&mst);
    println!("Vertices with odd degree");
    println!("{:?}", vertices_with_odd_degree);

    let mut subgraph_with_odd_degree_vertices = subgraph_with_odd_degree_vertices(&vertices_with_odd_degree, &adjacency_matrix);
    println!("Subgraph with odd degree vertices");
    print_matrix(&subgraph_with_odd_degree_vertices);

    // let full_matching = vec![vec![8,7], vec![6,9]]; // tsp1_253 NetworkX python script
    // let full_matching = vec![vec![0,2]]; // tsp2_1248
    let full_matching = find_minimum_weight_perfect_matching(&subgraph_with_odd_degree_vertices);
    println!("Full matching");
    println!("{:?}", full_matching);
    let mut eulerian_multigraph = create_eulerian_multigraph(mst, full_matching);
    println!("Eulerian multigraph");
    print_matrix(&eulerian_multigraph);

    let euler_tour = find_eulerian_cycle(&mut eulerian_multigraph);
	println!("Euler tour \n{:?}", euler_tour);

    let mut hamiltonian_path: Vec<u32> = Vec::new();
    for vertex in euler_tour {
        if !hamiltonian_path.contains(&vertex) {
            hamiltonian_path.push(vertex);
        }
    }

    // Reference to the original vector
    let reference_to_vec: &Vec<u32> = &hamiltonian_path;

    // New vector containing references to the elements in the original vector
    let transformed_vec: Vec<&u32> = reference_to_vec.iter().collect();

    let cost = calculate_cost(&transformed_vec, &adjacency_matrix);

    let mut end_time = Instant::now();
    let mut elapsed_time = end_time - start_time;
    println!("Christofides : {:?}", elapsed_time);
    println!("Cost: {}", cost);
    println!("Path: {:?}", hamiltonian_path);

    (hamiltonian_path, cost)
}

type AdjacencyMatrix = Vec<Vec<u32>>;

// Function to find the minimum-weight perfect matching using the Hungarian algorithm
fn find_minimum_weight_perfect_matching(adj_matrix: &AdjacencyMatrix) -> Vec<(usize, usize)> {
    // Number of vertices on each side of the bipartite graph
    let n = adj_matrix.len();
    let m = adj_matrix[0].len();

    // Function to find the minimum-weight perfect matching using the Hungarian algorithm
    fn hungarian_algorithm(adj_matrix: &AdjacencyMatrix) -> Vec<Option<usize>> {
        let n = adj_matrix.len();
        let m = adj_matrix[0].len();

        // Step 1: Subtract the minimum value of each row from each element in that row
        let mut reduced_matrix = adj_matrix.clone();
        for i in 0..n {
            let min_val = *reduced_matrix[i].iter().min().unwrap();
            for j in 0..m {
                reduced_matrix[i][j] -= min_val;
            }
        }

        // Step 2: Subtract the minimum value of each column from each element in that column
        for j in 0..m {
            let min_val = (0..n).map(|i| reduced_matrix[i][j]).min().unwrap();
            for i in 0..n {
                reduced_matrix[i][j] -= min_val;
            }
        }

        // Step 3: Construct the cover matrix
        let mut cover_matrix = vec![vec![false; m]; n];
        let mut row_cover = vec![false; n];
        let mut col_cover = vec![false; m];

        // Step 4: Iterate until a minimum-weight perfect matching is found
        let mut num_covered_rows = 0;
        while num_covered_rows < n {
            // Find an uncovered zero in the reduced matrix
            let (mut zero_row, mut zero_col) = (0, 0);
            let mut found_zero = false;
            for i in 0..n {
                if !row_cover[i] {
                    for j in 0..m {
                        if !col_cover[j] && reduced_matrix[i][j] == 0 {
                            zero_row = i;
                            zero_col = j;
                            found_zero = true;
                            break;
                        }
                    }
                }
                if found_zero {
                    break;
                }
            }

            if !found_zero {
                // Step 6: No uncovered zero found. Proceed to Step 6
                let min_uncovered_value = (0..n)
                    .filter(|&i| !row_cover[i])
                    .map(|i| {
                        (0..m)
                            .filter(|&j| !col_cover[j])
                            .map(|j| reduced_matrix[i][j])
                            .min()
                            .unwrap()
                    })
                    .min()
                    .unwrap();

                // Step 6: Modify the cover matrix
                for i in 0..n {
                    if row_cover[i] {
                        for j in 0..m {
                            reduced_matrix[i][j] += min_uncovered_value;
                        }
                    }
                }
                for j in 0..m {
                    if !col_cover[j] {
                        for i in 0..n {
                            reduced_matrix[i][j] -= min_uncovered_value;
                        }
                    }
                }
            } else {
                // Step 5: Modify the cover matrix
                cover_matrix[zero_row][zero_col] = true;
                row_cover[zero_row] = true;
                col_cover[zero_col] = true;
                num_covered_rows += 1;
            }
        }

        // Step 7: Extract the minimum-weight perfect matching from the cover matrix
        let mut matching = vec![None; n];
        for i in 0..n {
            for j in 0..m {
                if cover_matrix[i][j] {
                    matching[i] = Some(j);
                }
            }
        }
        
        matching
    }

    // Find the minimum-weight perfect matching using the Hungarian algorithm
    let matching = hungarian_algorithm(adj_matrix);

    let mut full_matching = Vec::new();
    for (i, &opt_j) in matching.iter().enumerate() {
        if let Some(j) = opt_j {
            full_matching.push((i, j));
        }
    }

    full_matching
}


pub fn create_eulerian_multigraph(mst : Vec<Vec<u32>>, full_matching: Vec<(usize, usize)>) -> Vec<Vec<u32>> {
    let mut multigraph: Vec<Vec<u32>> = vec![vec![0; mst.len()]; mst.len()];
    for i in 0..mst.len() {
       for j in 0..mst.len() {
            if mst[i as usize][j as usize] > 0 {
                multigraph[i as usize][j as usize] += 1;
            }
       }
    }
    for (u, v) in full_matching {
        multigraph[u][v] += 1;
        multigraph[v][u] += 1;
    }

    (multigraph)
}

pub fn vertices_with_odd_degree(adjacency_matrix: &Vec<Vec<u32>>) -> Vec<u32> {
    let mut vertices_with_odd_degree: Vec<u32> = Vec::new();
    for vertex in 0..adjacency_matrix.len() {
        let row = &adjacency_matrix[vertex as usize];
        let degree = row.iter().filter(|&value| *value != 0).count();
        if degree % 2 != 0 {vertices_with_odd_degree.push(vertex as u32)};
    }
    (vertices_with_odd_degree)
}

pub fn prim(adjacency_matrix: &Vec<Vec<u32>>) -> Vec<Vec<u32>> {
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
        mst[b][a] = min_value;
        visited[b] = true;
    }
    (mst)
}

pub fn subgraph_with_odd_degree_vertices(vertices_with_odd_degree: &Vec<u32>, adjacency_matrix: &Vec<Vec<u32>>) -> Vec<Vec<u32>> {
    let mut subgraph: Vec<Vec<u32>> = vec![vec![0; adjacency_matrix.len()]; adjacency_matrix.len()];
    for pair in vertices_with_odd_degree.iter().combinations(2) {
        let i = *pair[0] as usize;
        let j = *pair[1] as usize;
        subgraph[i][j] = adjacency_matrix[i][j];
        subgraph[j][i] = adjacency_matrix[j][i];
    }
    (subgraph)
}

fn find_eulerian_cycle(adjacency_matrix: &mut Vec<Vec<u32>>) -> Vec<u32> {
    let mut stack: Vec<u32> = Vec::new();
    let mut cycle: Vec<u32> = Vec::new();
    let mut current_vertex = 0;
    stack.push(current_vertex);

    while !stack.is_empty() {
        if adjacency_matrix[current_vertex as usize].iter().any(|&v| v != 0) {
            let mut next_vertex = 0;
            for i in 0..adjacency_matrix.len() {
                if adjacency_matrix[current_vertex as usize][i] != 0 {
                    next_vertex = i as u32;
                    break;
                }
            }
            stack.push(next_vertex);
            adjacency_matrix[current_vertex as usize][next_vertex as usize] -= 1;
            adjacency_matrix[next_vertex as usize][current_vertex as usize] -= 1;
            current_vertex = next_vertex;
        } else {
            current_vertex = stack.pop().unwrap();
            cycle.push(current_vertex);
        }
    }
    cycle.reverse();
    (cycle)
}