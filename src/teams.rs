/// List of all current drivers, can possibly change over time
#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum Driver {
    VER,
    PER,
    LEC,
    SAI,
    HAM,
    RUS,
    ALO,
    OCO,
    NOR,
    RIC,
    BOT,
    ZHO,
    STR,
    VET,
    MSC,
    MAG,
    GAS,
    TSU,
    LAT,
    ALB,
}

/// Each type implementing Team, can be considered a Constructors Team
/// # Example
/// Red Bull, Mercedes, ...
pub trait Team {
    fn drivers() -> (Driver, Driver);
}

create_team!(RedBull, (Driver::VER, Driver::PER));
create_team!(Mercedes, (Driver::HAM, Driver::RUS));
create_team!(Ferrari, (Driver::SAI, Driver::LEC));
create_team!(Alpine, (Driver::ALO, Driver::OCO));
create_team!(McLaren, (Driver::NOR, Driver::RIC));
create_team!(AlfaRomeo, (Driver::BOT, Driver::ZHO));
create_team!(AstonMartin, (Driver::STR, Driver::VET));
create_team!(Haas, (Driver::MSC, Driver::MAG));
create_team!(AlphaTauri, (Driver::GAS, Driver::TSU));
create_team!(Williams, (Driver::LAT, Driver::ALB));

#[macro_export]
macro_rules! create_team {
    ($team_name:ident, $drivers:expr) => {
        pub struct $team_name {}

        impl Team for $team_name {
            fn drivers() -> (Driver, Driver) {
                $drivers
            }
        }
    };
}

pub(crate) use create_team;
