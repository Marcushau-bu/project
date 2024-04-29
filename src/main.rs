mod data_processing;
use std::io;
use std::collections::HashMap;
use std::collections::HashSet;


fn main() -> io::Result<()>  {
        // Turn Amazon-meta data txt file into graph
        let graph = data_processing::data_to_graph("data/amazon-meta.txt")?;
        // Extract each node (products) corresponding item category and name.
        let node_dictionary = data_processing::node_information("data/amazon-meta.txt")?;
        

        // Count the total number of item categories there are 
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
    
        
        Ok(())
}
