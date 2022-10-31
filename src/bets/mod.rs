mod errors;

use self::errors::ClashesWithExistingBet;
use crate::teams::Driver;
use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
};

pub type PlayerName = String;

#[derive(Eq, Hash, PartialEq, Clone, Debug)]
pub struct Player {
    name: PlayerName,
    multiplier: u8, // x3, x5, ...
}

/// The selection of possible things a player can bet on
#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub enum BetType {
    /// At which position does a driver finish the race
    Position { driver: Driver, position: u8 },
    /// A driver does not finish the race
    DNF(Driver),
    /// Which driver has the fastest lap at end of race
    FastestLap(Driver),
    /// What driver gets voted as driver of the day
    DriverOfTheDay(Driver),
    /// Will there be a Safety Car?
    WillHaveSafetyCar(bool),
}

/// The state of a single bet
#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub struct Bet {
    r#type: BetType,
    reward: u64, // 10.000
}

pub struct BettingTable {
    placed_bets: HashMap<PlayerName, HashSet<Bet>>,
}

impl BettingTable {
    pub fn new() -> Self {
        Self {
            placed_bets: HashMap::new(),
        }
    }
    pub fn place(&mut self, bet: Bet, player: PlayerName) -> Result<Bet, ClashesWithExistingBet> {
        if self
            .placed_bets
            .entry(player)
            .or_insert_with(HashSet::new)
            .insert(bet.clone())
        {
            return Ok(bet);
        }

        Err(ClashesWithExistingBet {
            existing_bet: Bet { ..bet },
        })
    }
}

#[cfg(test)]
mod tests {
    use super::{Bet, BetType, BettingTable};
    use crate::{
        bets::{errors::ClashesWithExistingBet, PlayerName},
        teams::Driver,
    };

    #[test]
    fn can_place_a_bet() {
        let mut betting_table = BettingTable::new();
        let player = PlayerName::from("Nuyts");
        let bet = Bet {
            reward: 100_000,
            r#type: BetType::DNF(Driver::ALB),
        };
        let result = betting_table.place(bet, player);

        assert!(result.is_ok())
    }
    #[test]
    fn cannot_place_the_same_bet_more_than_once() {
        let mut betting_table = BettingTable::new();
        let player = PlayerName::from("Nuyts");
        let bet = Bet {
            reward: 100_000,
            r#type: BetType::DNF(Driver::ALB),
        };
        let result = betting_table.place(bet.clone(), player.clone());
        // So far so good
        assert!(result.is_ok());
        // Place the same bet again
        let result = betting_table.place(bet.clone(), player);
        assert_eq!(
            result.unwrap_err(),
            ClashesWithExistingBet { existing_bet: bet }
        );
    }
    #[test]
    fn single_player_can_place_multiple_unique_bets() {
        let mut betting_table = BettingTable::new();
        let player = PlayerName::from("Nuyts");
        let first_bet = Bet {
            reward: 100_000,
            r#type: BetType::DNF(Driver::ALB),
        };
        let second_bet = Bet {
            reward: 100_000,
            r#type: BetType::DNF(Driver::PER),
        };

        let result = betting_table.place(first_bet, player.clone());
        assert!(result.is_ok());
        let result = betting_table.place(second_bet, player);
        assert!(result.is_ok());
    }
    #[test]
    fn this_will_pass_but_we_need_to_fix_this() {
        let mut betting_table = BettingTable::new();
        let player = PlayerName::from("Nuyts");
        let first_bet = Bet {
            reward: 100_000,                   // Different amount
            r#type: BetType::DNF(Driver::ALB), // Same bet
        };
        let second_bet = Bet {
            reward: 99,                        // Different amount
            r#type: BetType::DNF(Driver::ALB), // Same bet
        };

        let result = betting_table.place(first_bet, player.clone());
        assert!(result.is_ok());
        let result = betting_table.place(second_bet, player);
        // !This should give us an error!!
        assert!(result.is_err());

        // TODO: This gets allowed to insert, because of the different reward amount...
        // I don't think we need a HashSet for this, we need to assert the validness of a bet ourselves
        // by using a match, because each type of bet will have slightly different rules...
        // You can bet on multiple DNF, but there can only be one driver finishing at one position,
        // or there can only be one driver of the day....
    }
}
