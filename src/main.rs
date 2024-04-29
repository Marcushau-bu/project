mod data_processing;
mod graph_analysis;
use std::io;
use std::collections::HashMap;
use std::collections::HashSet;


fn main() -> io::Result<()>  {
        // Turn Amazon-meta data txt file into graph
        let graph = data_processing::data_to_graph("data/amazon-meta.txt")?;
        // Extract each node (products) corresponding item category and name.
        let node_dictionary = data_processing::node_information("data/amazon-meta.txt")?;
        
        // Count the total number of item  in each categories  
        let category_counts = data_processing::count_items_by_category(&node_dictionary);
        // Print out counts for each category
        println!("Categories");
        for (category, count) in category_counts.iter() {
            println!("{} - Item Count: {}", category, count);
        }
        println!("\n");
    

        // Get the set of nodes that exist in both graph and node_dictionary 
        let common_nodes: HashSet<&String> = graph.keys().filter(|&node| node_dictionary.contains_key(node)).collect();
        // Create a new graph and dictionary that only include nodes that exist in both
        let mut new_graph: HashMap<String, Vec<String>> = HashMap::new();
        let mut new_node_dictionary: HashMap<String, (String, String)> = HashMap::new();
    
        for node in common_nodes {
            new_graph.insert(node.clone(), graph[node].clone());
            new_node_dictionary.insert(node.clone(), node_dictionary[node].clone());
        }
        
        // Calculate the average degree centrality for each category for all nodes
        println!("Avg Degree Centrality for each category");
        let average_degree_centrality_allnodes = graph_analysis::compute_average_degree_centrality_all_nodes(&new_graph, &new_node_dictionary);
        for (category, avg_degree) in average_degree_centrality_allnodes.iter() {
            println!("Category: {} - Average Degree Centrality: {}", category, avg_degree);
        }
        println!("\n");

        // Calculate the average degree centrality for each category for only nodes with neighbors
        println!("Avg Degree Centrality for each category based only on nodes with neighbours");
        let average_degree_centrality_for_nodes_with_neighbours = graph_analysis::compute_average_degree_centrality_for_nodes_with_neighbours(&new_graph, &new_node_dictionary);
        for (category, avg_degree) in average_degree_centrality_for_nodes_with_neighbours.iter() {
            println!("Category: {} - Average Degree Centrality: {}", category, avg_degree);
        }
        println!("\n");

        // Calculate the likelihood of someone purchasing a product of the same category for each category 
        let category_likelihood = graph_analysis::category_purchasing_likelihood(&graph, &node_dictionary);
        println!("Likelihood of each category purchasing another product of its own category:");
        for (category, likelihood) in category_likelihood.iter() {
            println!("Category: {} - Likelihood: {}", category, likelihood);
        }
        println!("\n");

        // The average number of other items in each category a person purcahses along their initial item
        let average_co_purcahses = graph_analysis::average_co_purchases_per_category(&graph, &node_dictionary);
        println!("Average product category co-purchases per Category:");
        for (category, avg_recs) in average_co_purcahses.iter() {
            println!("Category: {}", category);
            for (rec_category, avg) in avg_recs.iter() {
                println!("  Co-purchases {}: {:.2}", rec_category, avg);
            }
        }
        println!("\n");
        
        Ok(())
}
