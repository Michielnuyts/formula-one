#[derive(Eq, PartialEq, Clone, Debug, Hash)]
pub struct Player {
    pub name: String,
    pub multiplier: u64, // x3, x5, ...
}

impl Player {
    pub fn create(name: String, multiplier: Option<u64>) -> Self {
        Self {
            name,
            multiplier: multiplier.unwrap_or(1),
        }
    }
}
