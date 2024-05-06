use std::fs::File;
use std::io::{self, BufReader};
use std::path::Path;
use csv::ReaderBuilder;
use serde::Deserialize;

/// Player struct representing essential player information.
/// Implements Deserialization, Debug, Clone, PartialEq, Eq, and Hash traits.
#[derive(Debug, Deserialize, Clone, PartialEq, Eq, Hash)]
pub struct Player {
    #[serde(rename = "player_name")]
    pub name: String,
    #[serde(rename = "team_abbreviation")]
    pub team: String,
    #[serde(rename = "draft_year")]
    pub draft_year: u32,
    #[serde(rename = "draft_round")]
    pub draft_round: u32,
    #[serde(rename = "country")]
    pub country: String,
}

pub fn read_players<P: AsRef<Path>>(file_path: P) -> Vec<Player> {
    let file = File::open(file_path).expect("Failed to open file");
    let buffered = BufReader::new(file);
    let mut rdr = ReaderBuilder::new()
        .has_headers(true)
        .from_reader(buffered);

    rdr.deserialize()
        .filter_map(Result::ok)
        .collect() // This collects all players from data
}

//TEST PLAYER.RS

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_players() {
        // Mock data or use a fixture file if available
        let data = "player_name,team_abbreviation,draft_year,draft_round,country\n\
                    Michael Jordan,CHI,1984,1,USA\n\
                    LeBron James,LAL,2003,1,USA";
        let mut rdr = csv::ReaderBuilder::new()
            .from_reader(data.as_bytes());
        let players: Vec<Player> = rdr.deserialize().filter_map(Result::ok).collect();
        
        assert_eq!(players.len(), 2);
        assert_eq!(players[0].name, "Michael Jordan");
    }
}
