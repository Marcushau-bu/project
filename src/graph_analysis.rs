// graph_analysis.rs
use std::collections::HashSet;
use std::collections::HashMap;

// Calculate average degree centrality for each category for all nodes
pub fn compute_average_degree_centrality_all_nodes(
    graph: &HashMap<String, Vec<String>>,
    node_info: &HashMap<String, (String, String)>,
) -> HashMap<String, f64> {
    let mut category_degrees: HashMap<String, (usize, usize)> = HashMap::new();

    // Calculate degree centrality for each node and accumulate it by category
    for (node, neighbors) in graph.iter() {
        let (_, category) = node_info.get(node).unwrap();
        let degree = neighbors.len();

        let (total_degree, node_count) = category_degrees.entry(category.clone()).or_insert((0, 0));
        *total_degree += degree;
        *node_count += 1;
    }

    // Calculate average degree centrality for each category
    let mut average_degrees: HashMap<String, f64> = HashMap::new();
    for (category, (total_degree, node_count)) in category_degrees.iter() {
        let average_degree = *total_degree as f64 / *node_count as f64;
        average_degrees.insert(category.clone(), average_degree);
    }

    average_degrees
}

// Calculate average degree centrality for each category for only nodes with neigbours
pub fn compute_average_degree_centrality_for_nodes_with_neighbours(
    graph: &HashMap<String, Vec<String>>,
    node_info: &HashMap<String, (String, String)>,
) -> HashMap<String, f64> {
    let mut category_degrees: HashMap<String, (usize, usize)> = HashMap::new();

    // Calculate degree centrality for each node and accumulate it by category
    for (node, neighbors) in graph.iter() {
        if !neighbors.is_empty() {
            let (_, category) = node_info.get(node).unwrap();
            let degree = neighbors.len();

            let (total_degree, node_count) = category_degrees.entry(category.clone()).or_insert((0, 0));
            *total_degree += degree;
            *node_count += 1;
        }
    }
    

    // Calculate average degree centrality for each category
    let mut average_degrees: HashMap<String, f64> = HashMap::new();
    for (category, (total_degree, node_count)) in category_degrees.iter() {
        let average_degree = *total_degree as f64 / *node_count as f64;
        average_degrees.insert(category.clone(), average_degree);
    }

    average_degrees
}