use std::collections::{HashMap, HashSet, VecDeque};
use crate::graph::AdjacencyList;


// Function to perform breadth-first search and find the shortest path between two nodes in a graph.
pub fn bfs_with_path(graph: &AdjacencyList, start: &str, end: &str) -> Option<Vec<String>> {
    let mut visited = HashSet::new();
    let mut parents = HashMap::new();
    let mut queue = VecDeque::new();
    
    visited.insert(start.to_string());
    queue.push_back(start.to_string());
    
    // Perform BFS traversal until the queue is empty, exploring connections of each node, 
// marking visited nodes, and updating parent-child relationships to find the shortest path.

    while let Some(current) = queue.pop_front() {
        if let Some(connections) = graph.get(&current) {
            for connection in connections {
                if !visited.contains(connection) {
                    visited.insert(connection.clone());
                    parents.insert(connection.clone(), current.clone());
                    queue.push_back(connection.clone());
                    if connection == end {
                        return Some(reconstruct_path(&parents, start, end));
                    }
                }
            }
        }
    }
    None
}

pub fn reconstruct_path(parents: &HashMap<String, String>, start: &str, end: &str) -> Vec<String> {
    let mut path = Vec::new();
    let mut current = end.to_string();
    while let Some(parent) = parents.get(&current) {
        path.push(current.clone());
        current = parent.clone();
        if current == start {
            break;
        }
    }
    path.reverse();
    path
}

//TEST BFS.RS
#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::AdjacencyList;

    #[test]
    fn test_bfs_with_path() {
        let mut graph = AdjacencyList::new();
        graph.insert("Michael Jordan".to_string(), vec!["LeBron James".to_string()]);
        graph.insert("LeBron James".to_string(), vec![]);

        let path = bfs_with_path(&graph, "Michael Jordan", "LeBron James");
        assert_eq!(path, Some(vec!["Michael Jordan".to_string(), "LeBron James".to_string()]));
    }
}
