use crate::player::Player;

pub fn find_shared_attributes(player1: &Player, player2: &Player) -> Vec<String> {
    let mut shared_attributes = Vec::new();

    if player1.team == player2.team {
        shared_attributes.push(format!("Team: {}", player1.team));
    }
    if player1.draft_year == player2.draft_year {
        shared_attributes.push(format!("Draft Year: {}", player1.draft_year));
    }
    if player1.draft_round == player2.draft_round {
        shared_attributes.push(format!("Draft Round: {}", player1.draft_round));
    }
    if player1.country == player2.country {
        shared_attributes.push(format!("Country: {}", player1.country));
    }

    shared_attributes
}
//SHARED_ATTRIBUTES.RS
#[cfg(test)]
mod tests {
    use super::*;
    use crate::player::Player;

    #[test]
    fn test_find_shared_attributes() {
        let player1 = Player { name: "Michael Jordan".to_string(), team: "CHI".to_string(), draft_year: 1984, draft_round: 1, country: "USA".to_string() };
        let player2 = Player { name: "LeBron James".to_string(), team: "LAL".to_string(), draft_year: 2003, draft_round: 1, country: "USA".to_string() };

        let shared = find_shared_attributes(&player1, &player2);
        assert!(shared.contains(&"Country: USA".to_string()));
        assert_eq!(shared.len(), 1); // Only country is shared
    }
}