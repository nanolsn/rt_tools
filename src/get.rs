pub trait Get<B> {
    type Item;

    fn get(&self, by: B) -> Option<&Self::Item>;
}

pub trait GetMut<B> {
    type Item;

    fn get_mut(&mut self, by: B) -> Option<&mut Self::Item>;
}
