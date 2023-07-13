use::crate::utils::*;

pub fn approximation(adjacency_matrix: &Vec<Vec<u32>>) -> (Vec<u32>, u32) {
    let mst = mst(adjacency_matrix);
    let mut path: Vec<u32> = Vec::<u32>::new();

    println!("MST: {:?}", mst);
}