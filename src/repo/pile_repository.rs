use crate::models::pile::Pile;

trait PileRepository {
    fn create_new_pile(&self, pile: Pile);
    fn get_all_piles(&self);
    fn update_pile(&self, pile: Pile);
    // TODO: Remember to implement logic to not allow deleting of the main pile.
    fn delete_pile(&self, pile_name: &str);
}
