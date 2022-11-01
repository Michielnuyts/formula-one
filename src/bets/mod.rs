mod errors;

use self::errors::ClashesWithExistingBet;
use crate::teams::Driver;
use std::collections::HashMap;

pub type PlayerName = String;

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct Player {
    name: PlayerName,
    multiplier: u8, // x3, x5, ...
}

/// The selection of possible things a player can bet on
#[derive(Clone, Debug, Eq, PartialEq)]
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
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Bet {
    kind: BetType,
    reward: u64, // 10.000
}

pub struct BettingTable {
    placed_bets: HashMap<PlayerName, Vec<Bet>>,
}

impl BettingTable {
    pub fn new() -> Self {
        Self {
            placed_bets: HashMap::new(),
        }
    }
    pub fn place(&mut self, bet: Bet, player: &PlayerName) -> Result<Bet, ClashesWithExistingBet> {
        if self.is_bet_valid(&bet.kind, player) {
            self.placed_bets
                .entry(player.clone())
                .or_insert_with(Vec::new)
                .push(bet.clone());

            return Ok(bet);
        }

        Err(ClashesWithExistingBet {
            existing_bet: Bet { ..bet },
        })
    }
    fn is_bet_valid(&self, bet_type: &BetType, player: &PlayerName) -> bool {
        let existing_bets = self.get_bets_for(player);

        match bet_type {
            BetType::Position { driver, position } => !existing_bets.iter().any(|bet| {
                bet.kind
                    == BetType::Position {
                        driver: *driver,
                        position: *position,
                    }
            }),
            BetType::DNF(driver) => !existing_bets
                .iter()
                .any(|bet| bet.kind == BetType::DNF(*driver)),
            BetType::DriverOfTheDay(driver) => !existing_bets
                .iter()
                .any(|bet| bet.kind == BetType::DriverOfTheDay(*driver)),
            BetType::FastestLap(driver) => !existing_bets
                .iter()
                .any(|bet| bet.kind == BetType::FastestLap(*driver)),
            BetType::WillHaveSafetyCar(val) => !existing_bets
                .iter()
                .any(|bet| bet.kind == BetType::WillHaveSafetyCar(*val)),
        }
    }
    fn get_bets_for(&self, player: &PlayerName) -> Vec<Bet> {
        self.placed_bets.get(player).unwrap_or(&vec![]).clone()
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
            kind: BetType::DNF(Driver::ALB),
        };
        let result = betting_table.place(bet, &player);

        assert!(result.is_ok())
    }
    #[test]
    fn cannot_place_the_same_bet_more_than_once() {
        let mut betting_table = BettingTable::new();
        let player = PlayerName::from("Nuyts");
        let bet = Bet {
            reward: 100_000,
            kind: BetType::DNF(Driver::ALB),
        };
        let result = betting_table.place(bet.clone(), &player);
        // So far so good
        assert!(result.is_ok());
        // Place the same bet again
        let result = betting_table.place(bet.clone(), &player);
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
            kind: BetType::DNF(Driver::ALB),
        };
        let second_bet = Bet {
            reward: 100_000,
            kind: BetType::DNF(Driver::PER),
        };

        let result = betting_table.place(first_bet, &player);
        assert!(result.is_ok());
        let result = betting_table.place(second_bet, &player);
        assert!(result.is_ok());
    }
    #[test]
    fn single_player_cannot_place_multiple_unique_bets_with_only_reward_changed() {
        let mut betting_table = BettingTable::new();
        let player = PlayerName::from("Nuyts");
        let first_bet = Bet {
            reward: 100_000,                 // Different amount
            kind: BetType::DNF(Driver::ALB), // Same bet
        };
        let second_bet = Bet {
            reward: 99,                      // Different amount
            kind: BetType::DNF(Driver::ALB), // Same bet
        };

        let result = betting_table.place(first_bet, &player);
        assert!(result.is_ok());
        let result = betting_table.place(second_bet, &player);
        assert!(result.is_err());
    }
}
