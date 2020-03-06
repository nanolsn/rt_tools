use std::collections::{
    HashMap,
    hash_map::Entry,
};

use super::{
    get::{Get, GetMut},
    load::{Load, load_data},
};

#[derive(Debug)]
pub struct Resource<T> {
    items: Vec<T>,
    files: HashMap<String, usize>,
}

impl<T> Resource<T>
    where
        T: Load,
{
    pub fn new() -> Self { Resource::default() }

    pub fn load<S>(&mut self, file: S) -> &T
        where
            S: Into<String>,
    {
        let id = match self.files.entry(file.into()) {
            Entry::Occupied(en) => *en.get(),
            Entry::Vacant(en) => {
                let item = load_data(en.key());

                let id = self.items.len();
                self.items.push(item);
                en.insert(id);
                id
            }
        };

        &self.items[id]
    }

    pub fn get<B>(&self, by: B) -> Option<&T>
        where
            Self: Get<B, Item=T>,
    { Get::get(self, by) }

    pub fn get_mut<B>(&mut self, by: B) -> Option<&mut T>
        where
            Self: GetMut<B, Item=T>,
    { GetMut::get_mut(self, by) }

    pub fn len(&self) -> usize { self.items.len() }

    pub fn iter(&self) -> std::slice::Iter<T> { self.items.iter() }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<T> { self.items.iter_mut() }
}

impl<T> Get<usize> for Resource<T> {
    type Item = T;

    fn get(&self, by: usize) -> Option<&Self::Item> { self.items.get(by) }
}

impl<T> Get<&str> for Resource<T> {
    type Item = T;

    fn get(&self, by: &str) -> Option<&Self::Item> {
        let &id = self.files.get(by)?;
        self.get(id)
    }
}

impl<T> Get<String> for Resource<T> {
    type Item = T;

    fn get(&self, by: String) -> Option<&Self::Item> { self.get(by.as_str()) }
}

impl<T> GetMut<usize> for Resource<T> {
    type Item = T;

    fn get_mut(&mut self, by: usize) -> Option<&mut Self::Item> { self.items.get_mut(by) }
}

impl<T> GetMut<&str> for Resource<T> {
    type Item = T;

    fn get_mut(&mut self, by: &str) -> Option<&mut Self::Item> {
        let &id = self.files.get(by)?;
        self.get_mut(id)
    }
}

impl<T> GetMut<String> for Resource<T> {
    type Item = T;

    fn get_mut(&mut self, by: String) -> Option<&mut Self::Item> { self.get_mut(by.as_str()) }
}

impl<T> Default for Resource<T> {
    fn default() -> Self { Resource { items: Vec::new(), files: HashMap::new() } }
}

impl<T> AsRef<[T]> for Resource<T> {
    fn as_ref(&self) -> &[T] { &*self.items }
}

impl<T> AsMut<[T]> for Resource<T> {
    fn as_mut(&mut self) -> &mut [T] { &mut *self.items }
}

impl<T> Into<Vec<T>> for Resource<T> {
    fn into(self) -> Vec<T> { self.items }
}

#[cfg(test)]
mod tests {
    use super::{Load, Resource};

    #[derive(Debug)]
    struct Tile {
        name: String,
    }

    impl Load for Tile {
        const DIR: &'static str = "tiles";

        fn load<P>(file: P) -> Self
            where
                P: AsRef<std::path::Path>,
        {
            Tile {
                name: file
                    .as_ref()
                    .file_name()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .into()
            }
        }
    }

    #[test]
    fn load() {
        let mut res: Resource<Tile> = Resource::new();
        res.load("one.tl");
        res.load("two.tl");
        res.load("two.tl");
        res.load("one.tl");
        res.load("one.tl");
        res.load("three.tl");

        assert_eq!(res.get(0).unwrap().name, "one.tl");
        assert_eq!(res.get(1).unwrap().name, "two.tl");
        assert_eq!(res.get(2).unwrap().name, "three.tl");
        assert!(res.get(3).is_none());

        assert_eq!(res.get("one.tl").unwrap().name, "one.tl");
        assert_eq!(res.get("two.tl").unwrap().name, "two.tl");
        assert_eq!(res.get("three.tl").unwrap().name, "three.tl");
        assert!(res.get("four.tl").is_none());

        assert_eq!(res.len(), 3);

        let files = ["one.tl", "two.tl", "three.tl"];
        assert!(
            res
                .iter()
                .all(|t| files.contains(&t.name.as_str()))
        );
    }
}
