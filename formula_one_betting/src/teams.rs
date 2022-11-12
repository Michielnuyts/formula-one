pub fn full_roster() -> HashMap<String, (Driver, Driver)> {
    HashMap::from([
        (RedBull::new().name(), RedBull::new().drivers()),
        (Mercedes::new().name(), Mercedes::new().drivers()),
        (Ferrari::new().name(), Ferrari::new().drivers()),
        (Alpine::new().name(), Alpine::new().drivers()),
        (McLaren::new().name(), McLaren::new().drivers()),
        (AlfaRomeo::new().name(), AlfaRomeo::new().drivers()),
        (AstonMartin::new().name(), AstonMartin::new().drivers()),
        (Haas::new().name(), Haas::new().drivers()),
        (AlphaTauri::new().name(), AlphaTauri::new().drivers()),
        (Williams::new().name(), Williams::new().drivers()),
    ])
}

/// List of all current drivers, can possibly change over time
#[derive(Debug, Clone, Eq, PartialEq, Copy, Hash)]
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
    fn name(&self) -> String;
    fn drivers(&self) -> (Driver, Driver);
}

create_team!(RedBull, "Red Bull", (Driver::VER, Driver::PER));
create_team!(Mercedes, "Mercedes", (Driver::HAM, Driver::RUS));
create_team!(Ferrari, "Ferrari", (Driver::SAI, Driver::LEC));
create_team!(Alpine, "Alpine", (Driver::ALO, Driver::OCO));
create_team!(McLaren, "McLaren", (Driver::NOR, Driver::RIC));
create_team!(AlfaRomeo, "Alfa Romeo", (Driver::BOT, Driver::ZHO));
create_team!(AstonMartin, "Aston Martin", (Driver::STR, Driver::VET));
create_team!(Haas, "Haas", (Driver::MSC, Driver::MAG));
create_team!(AlphaTauri, "Alpha Tauri", (Driver::GAS, Driver::TSU));
create_team!(Williams, "Williams", (Driver::LAT, Driver::ALB));

#[macro_export]
macro_rules! create_team {
    ($team_name:ident, $team_name_formatted:expr, $drivers:expr) => {
        pub struct $team_name {
            name: String,
            drivers: (Driver, Driver),
        }

        impl $team_name {
            pub fn new() -> Self {
                Self {
                    name: String::from($team_name_formatted),
                    drivers: $drivers,
                }
            }
        }

        impl Team for $team_name {
            fn name(&self) -> String {
                self.name.clone()
            }
            fn drivers(&self) -> (Driver, Driver) {
                self.drivers
            }
        }
    };
}

use std::collections::HashMap;

pub(crate) use create_team;
