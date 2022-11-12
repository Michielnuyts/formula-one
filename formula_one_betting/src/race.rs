/// Race information
pub struct Race {
    location: Location,
    season: u16, // 2022, 2023, ...
}

/// Country location of a Grand Prix race
pub enum Location {
    Spain,
    Bahrain,
    SaudiArabia,
    Australia,
    Italy,
    Monaco,
    Azerbaijan,
    Canada,
    UK,
    Austria,
    France,
    Hungary,
    Singapore,
    Japan,
    Mexico,
    Brazil,
    AbuDhabi,
    USA,
    Belgium,
    Netherlands,
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
