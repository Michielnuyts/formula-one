mod errors;

use self::errors::ClashesWithExistingBet;
use crate::teams::Driver;
use std::{collections::HashMap, mem::discriminant};

pub type PlayerName = String;

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct Player {
    name: PlayerName,
    multiplier: u8, // x3, x5, ...
}

/// Position on the race grid, always from 1 up to 20
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct Position(u8);

impl Position {
    pub fn new(position: u8) -> Self {
        match position {
            1..=20 => Self(position),
            _ => panic!("Wrong input for position"),
        }
    }
}

/// Possible things a player can bet on
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum Bet {
    /// At which position does a driver finish the race
    FinishPosition { driver: Driver, position: Position },
    /// Which driver does not finish the race
    DoesNotFinish(Driver),
    /// Which driver has the fastest lap at end of race
    FastestLap(Driver),
    /// Which driver gets voted as driver of the day
    DriverOfTheDay(Driver),
    /// Will there be a Safety Car?
    WillHaveSafetyCar(bool),
}

/// The current state or the eventual outcome of a certain bet
/// Will be used to track live results on all matching bets
/// and to calculate final winnings after the race
pub struct Outcome {
    outcome: Bet,
    reward: u64,
}

pub struct BettingTable {
    /// The placed bets indexed by the playerName
    placed_bets: HashMap<PlayerName, Vec<Bet>>,
    /// The eventual outcomes after/during a race
    outcomes: Vec<Outcome>,
}

impl BettingTable {
    /// Create a new betting table
    pub fn new() -> Self {
        Self {
            placed_bets: HashMap::new(),
            outcomes: Vec::new(),
        }
    }
    /// Registers something that happened in the race
    pub fn register_outcome(&mut self, outcome: Outcome) {
        self.outcomes.push(outcome);
    }
    /// Places a bet for a certain player
    pub fn place(&mut self, bet: Bet, player: &PlayerName) -> Result<Bet, ClashesWithExistingBet> {
        if self.is_bet_valid(&bet, player) {
            self.placed_bets
                .entry(player.clone())
                .or_insert_with(Vec::new)
                .push(bet);

            return Ok(bet);
        }

        Err(ClashesWithExistingBet { existing_bet: bet })
    }
    /// Get the current results, based on current bets and outcomes
    pub fn results(&self) -> HashMap<PlayerName, u64> {
        let mut scores = HashMap::<PlayerName, u64>::new();

        for outcome in &self.outcomes {
            for (player_name, bets) in self.placed_bets.iter() {
                for bet in bets {
                    if bet == &outcome.outcome {
                        *scores.entry(player_name.clone()).or_insert(0) += outcome.reward;
                    }
                }
            }
        }

        scores
    }
    fn is_bet_valid(&self, bet_type: &Bet, player: &PlayerName) -> bool {
        let existing_bets = self.get_bets_for(player);
        if existing_bets.is_empty() {
            return true;
        }

        use Bet::*;
        match bet_type {
            FinishPosition { driver, position } => {
                let is_driver_free = existing_bets.iter().any(|bet| match *bet {
                    FinishPosition {
                        driver: inner_driver,
                        position: _,
                    } => *driver != inner_driver,
                    _ => true,
                });

                let is_position_free = existing_bets.iter().any(|bet| match *bet {
                    FinishPosition {
                        driver: _,
                        position: inner_position,
                    } => *position != inner_position,
                    _ => true,
                });

                is_driver_free && is_position_free
            }
            DoesNotFinish(driver) => !existing_bets
                .iter()
                .any(|bet| bet == &DoesNotFinish(*driver)),
            DriverOfTheDay(_driver) => !existing_bets
                .iter()
                .any(|bet| discriminant(bet) == discriminant(&DriverOfTheDay(Driver::MAG))),
            FastestLap(_driver) => !existing_bets
                .iter()
                .any(|bet| discriminant(bet) == discriminant(&FastestLap(Driver::MAG))),
            WillHaveSafetyCar(_val) => {
                existing_bets
                    .iter()
                    .filter(|&bet| {
                        bet == &WillHaveSafetyCar(true) || bet == &WillHaveSafetyCar(false)
                    })
                    .count()
                    == 0
            }
        }
    }
    fn get_bets_for(&self, player: &PlayerName) -> Vec<Bet> {
        self.placed_bets.get(player).unwrap_or(&vec![]).clone()
    }
}

#[cfg(test)]
mod tests {
    use super::{Bet, BettingTable};
    use crate::{
        bets::{errors::ClashesWithExistingBet, Outcome, PlayerName, Position},
        teams::Driver,
    };

    #[test]
    fn can_place_a_bet() {
        let mut betting_table = BettingTable::new();
        let player = PlayerName::from("Nuyts");
        let bet = Bet::DoesNotFinish(Driver::ALB);
        let result = betting_table.place(bet, &player);

        assert!(result.is_ok())
    }
    #[test]
    fn cannot_place_the_same_bet_more_than_once() {
        let mut betting_table = BettingTable::new();
        let player = PlayerName::from("Nuyts");
        let bet = Bet::DoesNotFinish(Driver::ALB);
        let result = betting_table.place(bet, &player);
        // So far so good
        assert!(result.is_ok());
        // Place the same bet again
        let result = betting_table.place(bet, &player);
        assert_eq!(
            result.unwrap_err(),
            ClashesWithExistingBet { existing_bet: bet }
        );
    }
    #[test]
    fn single_player_can_place_multiple_unique_bets() {
        let mut betting_table = BettingTable::new();
        let player = PlayerName::from("Nuyts");
        let first_bet = Bet::DoesNotFinish(Driver::ALB);

        let result = betting_table.place(first_bet, &player);
        assert!(result.is_ok());

        let second_bet = Bet::DoesNotFinish(Driver::PER);
        let result = betting_table.place(second_bet, &player);
        assert!(result.is_ok());
    }
    #[test]
    fn single_player_can_only_bet_once_on_safety_car() {
        let mut betting_table = BettingTable::new();
        let player = PlayerName::from("Nuyts");
        let first_bet = Bet::WillHaveSafetyCar(true);

        let result = betting_table.place(first_bet, &player);
        assert!(result.is_ok());

        let second_bet = Bet::WillHaveSafetyCar(false);
        let result = betting_table.place(second_bet, &player);
        assert!(result.is_err());
    }
    #[test]
    fn cannot_bet_on_multiple_finish_positions_for_the_same_driver() {
        let mut betting_table = BettingTable::new();
        let player = PlayerName::from("Nuyts");
        let first_bet = Bet::FinishPosition {
            driver: Driver::HAM,
            position: Position::new(1),
        };

        let result = betting_table.place(first_bet, &player);
        assert!(result.is_ok());

        let second_bet = Bet::FinishPosition {
            driver: Driver::HAM, // We already did a bet on HAM finishing first, not allowed
            position: Position::new(2),
        };
        let result = betting_table.place(second_bet, &player);
        assert!(result.is_err());
    }
    #[test]
    fn cannot_bet_on_multiple_finish_positions() {
        let mut betting_table = BettingTable::new();
        let player = PlayerName::from("Nuyts");
        let first_bet = Bet::FinishPosition {
            driver: Driver::LEC,
            position: Position::new(1),
        };

        let result = betting_table.place(first_bet, &player);
        assert!(result.is_ok());

        let second_bet = Bet::FinishPosition {
            driver: Driver::HAM,
            position: Position::new(1), // Already placed bet on LEC for position 1
        };
        let result = betting_table.place(second_bet, &player);
        assert!(result.is_err());

        let third_bet = Bet::FinishPosition {
            driver: Driver::HAM,
            position: Position::new(2), // This is valid again
        };
        let result = betting_table.place(third_bet, &player);
        assert!(result.is_ok());
    }
    #[test]
    fn many_players_can_place_many_different_bets_and_scoring_is_correct() {
        let mut betting_table = BettingTable::new();
        let michiel = PlayerName::from("michiel");
        let demi = PlayerName::from("demi");

        let result = betting_table.place(
            Bet::FinishPosition {
                driver: Driver::VER,
                position: Position::new(1),
            },
            &demi,
        );
        assert!(result.is_ok());

        let result = betting_table.place(
            Bet::FinishPosition {
                driver: Driver::VER,
                position: Position::new(1),
            },
            &michiel,
        );
        assert!(result.is_ok());

        let result = betting_table.place(
            Bet::FinishPosition {
                driver: Driver::HAM,
                position: Position::new(1),
            },
            &demi,
        );
        assert!(result.is_err());

        let result = betting_table.place(Bet::WillHaveSafetyCar(true), &demi);
        assert!(result.is_ok());
        let result = betting_table.place(Bet::WillHaveSafetyCar(false), &demi);
        assert!(result.is_err());

        let result = betting_table.place(Bet::FastestLap(Driver::LEC), &demi);
        assert!(result.is_ok());
        let result = betting_table.place(Bet::FastestLap(Driver::HAM), &demi);
        assert!(result.is_err());

        let result = betting_table.place(Bet::DriverOfTheDay(Driver::LEC), &demi);
        assert!(result.is_ok());
        let result = betting_table.place(Bet::DriverOfTheDay(Driver::HAM), &demi);
        assert!(result.is_err());

        betting_table.register_outcome(Outcome {
            outcome: Bet::FinishPosition {
                driver: Driver::VER,
                position: Position::new(1),
            },
            reward: 1000,
        });
        betting_table.register_outcome(Outcome {
            outcome: Bet::WillHaveSafetyCar(true),
            reward: 500,
        });
        betting_table.register_outcome(Outcome {
            outcome: Bet::FastestLap(Driver::LEC),
            reward: 2500,
        });
        betting_table.register_outcome(Outcome {
            outcome: Bet::DriverOfTheDay(Driver::LEC),
            reward: 5000,
        });

        let scores = betting_table.results();
        assert_eq!(scores.get(&demi).unwrap(), &9000);
        assert_eq!(scores.get(&michiel).unwrap(), &1000);
    }
}
