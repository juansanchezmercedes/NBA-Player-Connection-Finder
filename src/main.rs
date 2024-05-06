use std::io;
use std::env;
use player::Player;
use graph::AdjacencyList;
use graph::create_graph;
use bfs::bfs_with_path;
use shared_attributes::find_shared_attributes;

// Importing custom modules for different functionalities
mod player;
mod graph;
mod bfs;
mod shared_attributes;

fn main() {
    // Initial message to inform the user that the program has started
    println!("Loading...");
    // Collect command line arguments
    let args: Vec<String> = env::args().collect();
    // Ensure exactly one argument is provided, otherwise print usage and exit
    if args.len() != 2 {
        eprintln!("Usage: {} <path_to_csv>", args[0]);
        std::process::exit(1);
    }
    let file_path = &args[1];

    // Read players from the specified CSV file
    let players = player::read_players(file_path);
    // Create a graph based on the players read from the file
    let graph = create_graph(&players);
    // Confirmation that graph construction is complete
    println!("Graph created. Total players processed: {}", players.len());

    // Continuously read player names for path finding until 'exit' is entered
    while let Some(start_player) = read_player(&players, "Enter the first NBA player's name (or type 'exit' to quit):") {
        // Check for exit condition immediately after reading the player name
        if start_player.name.to_lowercase() == "exit" {
            break;
        }

        // Read the name of the second player
        if let Some(end_player) = read_player(&players, "Enter the second NBA player's name (or type 'exit' to quit):") {
            if end_player.name.to_lowercase() == "exit" {
                break;
            }

            // Find a path between the start and end player using BFS
            if let Some(path) = bfs_with_path(&graph, &start_player.name, &end_player.name) {
                println!("The path between {} and {} is {:?}", start_player.name, end_player.name, path);

                // Calculate and display shared attributes between the start and end players
                let shared = find_shared_attributes(&start_player, &end_player);
                println!("Shared attributes between {} and {}: {:?}", start_player.name, end_player.name, shared);
            } else {
                println!("No path found between {} and {}", start_player.name, end_player.name);
            }
        } else {
            println!("NBA Player not found. Please try again.");
        }
    }
}

// Function to read a player's name and return the corresponding Player object
fn read_player(players: &[Player], prompt: &str) -> Option<Player> {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let input_name = input.trim();

    // Return None to signal an 'exit' command, stopping the program gracefully
    if input_name.to_lowercase() == "exit" {
        return None;
    }

    // Fetch the player from the list based on the name entered
    players.iter().cloned().find(|p| p.name.to_lowercase() == input_name.to_lowercase())
}
