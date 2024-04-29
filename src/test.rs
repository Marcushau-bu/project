use std::collections::HashMap;
use crate::graph_analysis::compute_average_degree_centrality_all_nodes;
use crate::graph_analysis::category_purchasing_likelihood;


pub fn test_compute_average_degree_centrality_all_nodes() {
    // Create a sample graph:
        // A should have a degree centrality of 2: Category 1
        // B should have a degree centrality of 3: Category 2
        // C should have a degree centrality of 3: Category 1
        // D should have a degree centrality of 3: Category 3
        // E should have a degree centrality of 1: Category 3

    // The right answer should be:
        //  Category 1 should have a degree centrality of (2 + 3)/2 = 2.5
        //  Category 2 should have a degree centrality of 3/1 = 3.0
        //  Category 3 should have a degree centrality of (3 + 1)/2 = 2.0
    
    // Arbitrary Graph for testing
    let mut graph: HashMap<String, Vec<String>> = HashMap::new();
    graph.insert("A".to_string(), vec!["B".to_string(), "C".to_string()]);
    graph.insert("B".to_string(), vec!["A".to_string(), "C".to_string(), "D".to_string()]);
    graph.insert("C".to_string(), vec!["A".to_string(), "B".to_string(), "D".to_string()]);
    graph.insert("D".to_string(), vec!["B".to_string(), "C".to_string(), "E".to_string()]);
    graph.insert("E".to_string(), vec!["D".to_string()]);

    // Arbitrary node information for testing, only category matters
    let mut node_info: HashMap<String, (String, String)> = HashMap::new();
    node_info.insert("A".to_string(), ("title".to_string(), "1".to_string()));
    node_info.insert("B".to_string(), ("title".to_string(), "2".to_string()));
    node_info.insert("C".to_string(), ("title".to_string(), "1".to_string()));
    node_info.insert("D".to_string(), ("title".to_string(), "3".to_string()));
    node_info.insert("E".to_string(), ("title".to_string(), "3".to_string()));

    // Testing average degree centrality function
    let centrality = compute_average_degree_centrality_all_nodes(&graph, &node_info);

    println!("Average Degree Centrality:");
    for (node, centrality_value) in centrality.iter() {
        println!("Category {}: {:.2}", node, centrality_value);
    }
}

pub fn test_category_purchasing_likelihood() {
    // Create a sample graph:
        // A(1) co-purchases B(2) C(1)
        // B(2) co-purchases A(1) C(1) D(3)
        // C(3) co-purchases A(1) B(2) D(3)
        // D(3) co-purchases B(2) C(1) E(3)
        // E(3) co-purchases D(3)

    // The right answer should be:
        // Category 1: 2 co-purhcases of products that are category 1 out of 5 products. Therefore likelihood = 2/5 = 0.4
        // Category 2: 0 co-purhcases of products that are category 2 out of 3 products. Therefore likelihood = 0
        // Category 3: 3 co-purhcases of products that are category 3 out of 6 products. Therefore likelihood = 3/6 = 0.5

    // Arbitrary Graph for testing
    let mut graph: HashMap<String, Vec<String>> = HashMap::new();
    graph.insert("A".to_string(), vec!["B".to_string(), "C".to_string()]);
    graph.insert("B".to_string(), vec!["A".to_string(), "C".to_string(), "D".to_string()]);
    graph.insert("C".to_string(), vec!["A".to_string(), "B".to_string(), "D".to_string()]);
    graph.insert("D".to_string(), vec!["B".to_string(), "C".to_string(), "E".to_string()]);
    graph.insert("E".to_string(), vec!["D".to_string()]);

    // Arbitrary node information for testing, only category matters
    let mut node_info: HashMap<String, (String, String)> = HashMap::new();
    node_info.insert("A".to_string(), ("title".to_string(), "1".to_string()));
    node_info.insert("B".to_string(), ("title".to_string(), "2".to_string()));
    node_info.insert("C".to_string(), ("title".to_string(), "1".to_string()));
    node_info.insert("D".to_string(), ("title".to_string(), "3".to_string()));
    node_info.insert("E".to_string(), ("title".to_string(), "3".to_string()));

    // Testing category purchasing likelihood function
    let likelihood = category_purchasing_likelihood(&graph, &node_info);

    println!("Category puchasing likelihood:");
    for (node, likelihood_value) in likelihood.iter() {
        println!("Category {}: {:.2}", node, likelihood_value);
    }
}

pub fn run_tests() {
    test_compute_average_degree_centrality_all_nodes();
    println!("\n");
    test_category_purchasing_likelihood();
}
