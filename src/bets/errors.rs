use std::{
    error::Error,
    fmt::{self, Display, Formatter},
};

use super::Bet;

#[derive(Debug, Eq, PartialEq)]
pub struct ClashesWithExistingBet {
    pub existing_bet: Bet,
}

impl Error for ClashesWithExistingBet {}

impl Display for ClashesWithExistingBet {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "This player already placed a bet for this betType")
    }
}
