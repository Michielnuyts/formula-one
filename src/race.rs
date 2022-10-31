/// Race information
pub struct Race {
    location: Location,
    season: u16, // 2022, 2023, ...
}

pub enum Location {
    USA,
    Mexico,
    Belgium,
    Netherlands,
}
