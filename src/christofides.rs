use std::time::Instant;
use crate::utils::*;
use itertools::Itertools;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::{File, OpenOptions};
use std::io::{self, Write};

type Vertex = usize;

pub fn christofides(adjacency_matrix: &Vec<Vec<u32>>) -> (Vec<u32>, u32) {
    let mut start_time = Instant::now();
    let mut mst: Vec<Vec<u32>> = prim(adjacency_matrix);

    let vertices_with_odd_degree = vertices_with_odd_degree(&mst);

    let mut subgraph_with_odd_degree_vertices = subgraph_with_odd_degree_vertices(&vertices_with_odd_degree, &adjacency_matrix);

    let full_matching = minimum_weight_matching(&mst, &subgraph_with_odd_degree_vertices, &vertices_with_odd_degree);
    println!("Full matching");
    println!("{:?}", full_matching);
    

    let mut eulerian_multigraph = create_eulerian_multigraph(mst, full_matching);

    let euler_tour = find_eulerian_cycle(&mut eulerian_multigraph);

    let mut hamiltonian_path: Vec<u32> = Vec::new();
    for vertex in euler_tour {
        if !hamiltonian_path.contains(&vertex) {
            hamiltonian_path.push(vertex);
        }
    }

    let reference_to_vec: &Vec<u32> = &hamiltonian_path;
    let transformed_vec: Vec<&u32> = reference_to_vec.iter().collect();
    let cost = calculate_cost(&transformed_vec, &adjacency_matrix);

    let mut end_time = Instant::now();
    let mut elapsed_time = end_time - start_time;
    println!("Nosso christofides : {:?}", elapsed_time);
    println!("Cost: {}", cost);

    (hamiltonian_path, cost)
}

fn minimum_weight_matching(mst: &Vec<Vec<u32>>, adjacency_matrix: &Vec<Vec<u32>>, odd_vertices: &Vec<u32>) -> Vec<(usize, usize)> {
    let mut full_matching: Vec<(usize, usize)> = Vec::new();
    let mut vertices: Vec<u32> = odd_vertices.clone();
    let mut pairs: Vec<(usize, usize, usize)> = Vec::new();
    let mut lowest_weight = u32::MAX;
    let num_vertices = vertices.len() / 2;
    let mut result_combination = Vec::new();
    let mut lowest_weight = u32::MAX;
    
    for combination in vertices.iter().combinations(2) {
        let i = *combination[0] as usize;
        let j = *combination[1] as usize;
        let weight = adjacency_matrix[i][j];
        pairs.push((i, j, weight as usize));
    }

    result_combination = find_lowest_weight_combination(
        &pairs,
        num_vertices
    );

    println!("result combination: {:?}", result_combination);

    result_combination
}

fn find_lowest_weight_combination(
        pairs: &Vec<(usize, usize, usize)>,
        num_vertices: usize,
    ) -> Vec<(usize, usize)> {
    let mut result_combination: Vec<(usize, usize)> = Vec::new();
    let mut stack: Vec<(
        Vec<(usize, usize, usize)>,
        HashSet<usize>,
        Vec<(usize, usize, usize)>,
        u32,
    )> = Vec::new();
    let mut lowest_weight = u32::MAX;

    stack.push((pairs.clone(), HashSet::new(), Vec::new(), 0));

    while !stack.is_empty() {
        let (pairs, used_vertices, current_combination, weight_so_far) = stack.pop().unwrap();

        if current_combination.len() == num_vertices {
            if weight_so_far < lowest_weight {
                lowest_weight = weight_so_far;
                result_combination.clear();
                result_combination.extend(current_combination.iter().map(|&(u, v, _)| (u, v)));
            }
            continue;
        }

        for &(u, v, weight) in pairs.iter() {
            if !used_vertices.contains(&u) && !used_vertices.contains(&v) {
                let mut new_used_vertices = used_vertices.clone();
                new_used_vertices.insert(u);
                new_used_vertices.insert(v);

                let mut new_combination = current_combination.clone();
                new_combination.push((u, v, weight));

                let new_weight = weight_so_far + weight as u32;

                stack.push((pairs.clone(), new_used_vertices, new_combination, new_weight));
            }
        }
    }

    result_combination
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

    multigraph
}

pub fn vertices_with_odd_degree(adjacency_matrix: &Vec<Vec<u32>>) -> Vec<u32> {
    let mut vertices_with_odd_degree: Vec<u32> = Vec::new();
    for vertex in 0..adjacency_matrix.len() {
        let row = &adjacency_matrix[vertex as usize];
        let degree = row.iter().filter(|&value| *value != 0).count();
        if degree % 2 != 0 {vertices_with_odd_degree.push(vertex as u32)};
    }
    vertices_with_odd_degree
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
    mst
}

pub fn subgraph_with_odd_degree_vertices(vertices_with_odd_degree: &Vec<u32>, adjacency_matrix: &Vec<Vec<u32>>) -> Vec<Vec<u32>> {
    let mut subgraph: Vec<Vec<u32>> = vec![vec![0; adjacency_matrix.len()]; adjacency_matrix.len()];
    for pair in vertices_with_odd_degree.iter().combinations(2) {
        let i = *pair[0] as usize;
        let j = *pair[1] as usize;
        subgraph[i][j] = adjacency_matrix[i][j];
        subgraph[j][i] = adjacency_matrix[j][i];
    }
    subgraph
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
    cycle
}