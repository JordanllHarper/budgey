use crate::{models::pile::Pile, BudgeyContext};

pub trait PileIO {
    fn create_new_pile(pile: Pile, context: &BudgeyContext) -> anyhow::Result<()>;
}
pub struct PileIOImpl {}
