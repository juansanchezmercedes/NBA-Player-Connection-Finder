use std::collections::HashMap;
use crate::player::Player;

pub type AdjacencyList = HashMap<String, Vec<String>>;

// Function to create a graph based on player connections, considering team affiliation and draft year.
pub fn create_graph(players: &[Player]) -> AdjacencyList {
    let mut graph = HashMap::new();
    for player in players {
        let connections = players.iter()
            .filter(|p| p.team == player.team || p.draft_year == player.draft_year)
            .map(|p| p.name.clone())
            .collect::<Vec<String>>();
        graph.insert(player.name.clone(), connections);
    }
    println!("Graph created with {} nodes.", graph.len());
    graph
}

//TEST GRAPH.RS
#[cfg(test)]
mod tests {
    use super::*;
    use crate::player::Player;

    #[test]
    fn test_create_graph() {
        let players = vec![
            Player { name: "Michael Jordan".to_string(), team: "CHI".to_string(), draft_year: 1984, draft_round: 1, country: "USA".to_string() },
            Player { name: "LeBron James".to_string(), team: "LAL".to_string(), draft_year: 2003, draft_round: 1, country: "USA".to_string() }
        ];
        let graph = create_graph(&players);
        
        assert!(graph.contains_key("Michael Jordan"));
        assert_eq!(graph["Michael Jordan"].len(), 1); // Assuming team and draft year leads to connection
    }
}
