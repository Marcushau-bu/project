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

pub fn category_recommendation_likelihood(graph: &HashMap<String, Vec<String>>, node_info: &HashMap<String, (String, String)>) -> HashMap<String, f64> {
    let mut category_recommendation_likelihoods: HashMap<String, f64> = HashMap::new();

    // HashMap to store the count of nodes in each category
    let mut category_node_counts: HashMap<String, usize> = HashMap::new();

    // Iterate through each node in the graph
    for (node, neighbors) in graph.iter() {
        // Get the category of the current node
        if let Some((_, category)) = node_info.get(node) {
            // Increment the count of nodes in the current category
            let count = category_node_counts.entry(category.clone()).or_insert(0);
            *count += 1;

            // Create a HashSet to store the neighbors' categories
            let mut neighbor_categories: HashSet<String> = HashSet::new();
            
            // Iterate through each neighbor of the current node
            for neighbor in neighbors {
                // Get the category of the neighbor node
                if let Some((_, neighbor_category)) = node_info.get(neighbor) {
                    // Add the neighbor's category to the HashSet
                    neighbor_categories.insert(neighbor_category.clone());
                }
            }

            // Increment the likelihood for the current category if it occurs in the neighbors' categories
            if neighbor_categories.contains(category) {
                let likelihood = category_recommendation_likelihoods.entry(category.clone()).or_insert(0.0);
                *likelihood += 1.0;
            }
        }
    }

    // Calculate the likelihood for each category
    for (category, count) in category_recommendation_likelihoods.iter_mut() {
        if let Some(node_count) = category_node_counts.get(category) {
            *count /= *node_count as f64;
        }
    }

    category_recommendation_likelihoods
}

pub fn average_recommendation_per_category(
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
