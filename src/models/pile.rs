pub struct Pile {
    name: String,
    balance: f64,
    pile_type: PileType,
}

pub enum PileType {
    Main,
    UserCreated,
}

impl Pile {
    pub fn new(name: String, balance: f64, pile_type: PileType) -> Self {
        Self {
            name,
            balance,
            pile_type,
        }
    }
}
