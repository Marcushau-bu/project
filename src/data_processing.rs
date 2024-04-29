use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn data_to_graph(file_path: &str) -> std::io::Result<HashMap<String, Vec<String>>> {
    // Open the input file
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    // Initialize a HashMap to store the adjacency list
    let mut adjacency_list: HashMap<String, Vec<String>> = HashMap::new();

    // Process each line in the file
    let mut current_asin = String::new();
    let mut similar_asins: Vec<String> = Vec::new();
    for line in reader.lines() {
        let line = line.unwrap();
        if line.starts_with("ASIN: ") {
            // Extract the ASIN from the line (ASIN is a unique Amazon product number)
            current_asin = line.replace("ASIN: ", "").trim().to_string();
        } else if line.starts_with("  similar: ") {
            // Extract the similar ASINs from the line
            similar_asins = line
                .replace("  similar: ", "")
                .split_whitespace()
                .skip(1) // Skip the first element (the number of similar ASINs)
                .map(|s| s.to_string())
                .collect::<Vec<String>>();
            // Add current ASIN to the adjacency list of each similar ASIN
            for similar_asin in &similar_asins {
                adjacency_list
                    .entry(similar_asin.clone())
                    .or_insert(Vec::new())
                    .push(current_asin.clone());
            }
        } else if line.trim().is_empty() {
            // Reached the end of the item, add ASIN and similar ASINs to the adjacency list
            adjacency_list.insert(current_asin.clone(), similar_asins.clone());
            // Add similar ASINs to the adjacency list of the current ASIN
            for similar_asin in &similar_asins {
                adjacency_list
                    .entry(current_asin.clone())
                    .or_insert(Vec::new())
                    .push(similar_asin.clone());
            }
            similar_asins.clear();
        }
    }
    Ok(adjacency_list)
}


pub fn node_information(file_path: &str) -> std::io::Result<HashMap<String, (String, String)>> {
    // Open the input file
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    // Initialize a HashMap to store the node information
    let mut node_info: HashMap<String, (String, String)> = HashMap::new();

    let mut current_asin = String::new();
    let mut title = String::new();
    let mut group = String::new();
    for line in reader.lines() {
        let line = line.unwrap();
        if line.starts_with("ASIN: ") {
            // Extract the ASIN from the line
            current_asin = line.replace("ASIN: ", "").trim().to_string();
        } else if line.starts_with("  title: ") {
            // Extract the title from the line
            title = line.replace("  title: ", "").trim().to_string();
        } else if line.starts_with("  group: ") {
            // Extract the item Group from the line
            group = line.replace("  group: ", "").trim().to_string();
        } else if line.trim().is_empty() {
            // Reached the end of the item, add ASIN and node information to the HashMap
            node_info.insert(current_asin.clone(), (title.clone(), group.clone()));
        }
    }
    Ok(node_info)
}

pub fn count_items_by_category(node_dictionary: &HashMap<String, (String, String)>) -> HashMap<String, usize> {
    let mut category_counts: HashMap<String, usize> = HashMap::new();

    for (_, (_, category)) in node_dictionary.iter() {
        let count = category_counts.entry(category.clone()).or_insert(0);
        *count += 1;
    }

    category_counts
}

