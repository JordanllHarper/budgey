pub enum Transaction {
    PilePull {
        amount: f64,
        pile_name: String,
    },
    PilePush {
        amount: f64,
        pile_name: String,
    },
    PileMerge {
        amount: f64,
        from_pile: String,
        to_pile: String,
    },
}
