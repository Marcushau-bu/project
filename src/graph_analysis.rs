// graph_analysis.rs
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

pub fn category_purchasing_likelihood(graph: &HashMap<String, Vec<String>>, node_info: &HashMap<String, (String, String)>) -> HashMap<String, f64> {
    let mut category_purchasing_likelihoods: HashMap<String, f64> = HashMap::new();
    let mut category_total_connections: HashMap<String, usize> = HashMap::new();

    // Initialize the counters for all categories to ensure each is represented in the final map
    for (_, (_, category)) in node_info.iter() {
        category_purchasing_likelihoods.entry(category.clone()).or_insert(0.0);
        category_total_connections.entry(category.clone()).or_insert(0);
    }

    // Iterate through each node and their neighbors
    for (node, neighbors) in graph.iter() {
        if let Some((_, category)) = node_info.get(node) {
            // Update the total connections for this category
            let connections = category_total_connections.entry(category.clone()).or_insert(0);
            *connections += neighbors.len();

            // Check for co-purchases within the same category
            for neighbor in neighbors {
                if let Some((_, neighbor_category)) = node_info.get(neighbor) {
                    if category == neighbor_category {
                        let likelihood = category_purchasing_likelihoods.entry(category.clone()).or_insert(0.0);
                        *likelihood += 1.0;
                    }
                }
            }
        }
    }

    // Calculate the final likelihood by dividing the co-purchase count by the total connections
    for (category, likelihood) in category_purchasing_likelihoods.iter_mut() {
        if let Some(total_connections) = category_total_connections.get(category) {
            *likelihood /= *total_connections as f64;
        }
    }
    category_purchasing_likelihoods
}

pub fn average_co_purchases_per_category(
    graph: &HashMap<String, Vec<String>>, 
    node_info: &HashMap<String, (String, String)>
) -> HashMap<String, HashMap<String, f64>> {
    let mut category_counts: HashMap<String, HashMap<String, usize>> = HashMap::new();
    let mut category_totals: HashMap<String, usize> = HashMap::new();

    // Collect recommendations for each node.
    for (node, neighbors) in graph.iter() {
        if let Some((_, node_category)) = node_info.get(node) {
            let counts = category_counts.entry(node_category.clone()).or_insert_with(HashMap::new);
            for neighbor in neighbors {
                if let Some((_, neighbor_category)) = node_info.get(neighbor) {
                    *counts.entry(neighbor_category.clone()).or_insert(0) += 1;
                }
            }
            *category_totals.entry(node_category.clone()).or_insert(0) += 1;
        }
    }

    // Compute averages for each category.
    let mut averages: HashMap<String, HashMap<String, f64>> = HashMap::new();
    for (category, counts) in category_counts {
        let total_nodes = category_totals.get(&category).unwrap_or(&1); // Avoid division by zero
        let avg_counts: HashMap<String, f64> = counts.into_iter()
            .map(|(cat, count)| (cat, count as f64 / *total_nodes as f64))
            .collect();
        averages.insert(category, avg_counts);
    }
    averages
}
